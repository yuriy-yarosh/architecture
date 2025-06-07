use k8s_openapi::api::core::v1::Namespace;
use k8s_openapi::api::core::v1::ServiceAccount;
use kube::{
    Api, Client,
    api::{ObjectMeta, PostParams},
};

use crate::resources::context::Context;

pub async fn service_account_exists(
    client: &Client,
    name: &str,
) -> Result<Option<String>, anyhow::Error> {
    Api::<ServiceAccount>::all(client.clone())
        .get(name)
        .await
        .map(|r| r.metadata.resource_version)
        .map_err(Into::into)
}

pub async fn create_service_account(
    client: &Client,
    ctx: &Context,
    namespace: &Namespace,
    name: &str,
) -> Result<ServiceAccount, anyhow::Error> {
    let namespace_name = namespace
        .metadata
        .name
        .as_deref()
        .ok_or_else(|| anyhow::anyhow!("Namespace must have a name"))?;

    let api: Api<ServiceAccount> = Api::namespaced(client.clone(), namespace_name);
    let pp = PostParams::default();

    let mut service_account = ServiceAccount {
        metadata: ObjectMeta {
            name: Some(name.into()),
            namespace: Some(namespace_name.into()),
            ..Default::default()
        },
        ..Default::default()
    };

    if ctx.dry_run {
        return Ok(service_account);
    }

    match service_account_exists(client, name).await? {
        Some(version) if ctx.replace => {
            service_account.metadata.resource_version = Some(version);
            api.create(&pp, &service_account).await.map_err(Into::into)
        }
        Some(_) => Ok(service_account),
        None => api.create(&pp, &service_account).await.map_err(Into::into),
    }
}
