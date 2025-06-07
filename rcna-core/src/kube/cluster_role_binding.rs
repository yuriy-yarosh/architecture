use k8s_openapi::api::{
    core::v1::{Namespace, ServiceAccount},
    rbac::v1::{ClusterRole, ClusterRoleBinding, RoleRef, Subject},
};
use kube::{
    Client,
    api::{Api, ObjectMeta, PostParams},
};

use crate::resources::context::Context;

pub async fn cluster_role_binding_exists(
    client: &Client,
    name: &str,
) -> Result<Option<String>, anyhow::Error> {
    Api::<ClusterRoleBinding>::all(client.clone())
        .get(name)
        .await
        .map(|r| r.metadata.resource_version)
        .map_err(Into::into)
}

pub async fn create_cluster_role_binding(
    client: &Client,
    ctx: &Context,
    namespace: &Namespace,
    binding_name: &str,
    cluster_role: &ClusterRole,
    service_account: &ServiceAccount,
) -> Result<ClusterRoleBinding, anyhow::Error> {
    if binding_name.is_empty() {
        return Err(anyhow::anyhow!("ClusterRoleBinding name must not be empty"));
    }

    let role_name = cluster_role
        .metadata
        .name
        .as_deref()
        .ok_or_else(|| anyhow::anyhow!("ClusterRole must have a name"))?;

    let sa_name = service_account
        .metadata
        .name
        .as_deref()
        .ok_or_else(|| anyhow::anyhow!("ServiceAccount must have a name"))?;

    let ns_name = namespace
        .metadata
        .name
        .as_deref()
        .ok_or_else(|| anyhow::anyhow!("Namespace must have a name"))?;

    let binding = ClusterRoleBinding {
        metadata: ObjectMeta {
            name: Some(binding_name.to_string()),
            namespace: Some(ns_name.to_string()),
            ..Default::default()
        },
        role_ref: RoleRef {
            name: role_name.to_string(),
            kind: "ClusterRole".into(),
            api_group: "rbac.authorization.k8s.io".into(),
        },
        subjects: Some(vec![Subject {
            name: sa_name.to_string(),
            namespace: Some(ns_name.to_string()),
            kind: "ServiceAccount".into(),
            api_group: Some("v1".into()),
        }]),
        ..Default::default()
    };

    if ctx.dry_run {
        return Ok(binding);
    }

    let api = Api::<ClusterRoleBinding>::all(client.clone());
    let pp = PostParams::default();

    if let Some(version) = cluster_role_binding_exists(&client, binding_name).await? {
        if ctx.replace {
            let mut binding_with_version = binding.clone();
            binding_with_version.metadata.resource_version = Some(version);
            api.replace(binding_name, &pp, &binding_with_version)
                .await
                .map_err(Into::into)
        } else {
            Ok(binding)
        }
    } else {
        api.create(&pp, &binding).await.map_err(Into::into)
    }
}
