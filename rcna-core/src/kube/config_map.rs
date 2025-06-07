use k8s_openapi::api::core::v1::{ConfigMap, Namespace};
use kube::{
    Client,
    api::{Api, ObjectMeta, PostParams},
};

use crate::resources::context::Context;

pub async fn config_map_exists(
    client: &Client,
    namespace: &str,
    name: &str,
) -> Result<Option<String>, anyhow::Error> {
    Api::<ConfigMap>::namespaced(client.clone(), namespace)
        .get(name)
        .await
        .map(|r| r.metadata.resource_version)
        .map_err(Into::into)
}

pub async fn create_config_map(
    client: &Client,
    ctx: &Context,
    namespace: &Namespace,
    name: &str,
) -> Result<ConfigMap, anyhow::Error> {
    if name.is_empty() {
        return Err(anyhow::anyhow!("ConfigMap name must not be empty"));
    }

    let ns_name = namespace
        .metadata
        .name
        .as_deref()
        .ok_or_else(|| anyhow::anyhow!("Namespace must have a name"))?;

    let config_map = ConfigMap {
        metadata: ObjectMeta {
            name: Some(name.into()),
            namespace: Some(ns_name.into()),
            ..Default::default()
        },
        ..Default::default()
    };

    if ctx.dry_run {
        return Ok(config_map);
    }

    let api = Api::<ConfigMap>::namespaced(client.clone(), ns_name);
    let pp = PostParams::default();

    if let Some(version) = config_map_exists(&client, name, ns_name).await? {
        if ctx.replace {
            let mut config_map_with_version = config_map.clone();
            config_map_with_version.metadata.resource_version = Some(version);
            api.replace(name, &pp, &config_map_with_version)
                .await
                .map_err(Into::into)
        } else {
            Ok(config_map)
        }
    } else {
        api.create(&pp, &config_map).await.map_err(Into::into)
    }
}
