use anyhow::*;
use k8s_openapi::api::core::v1::Secret;
use kube::{Api, Client};
use serde::{Deserialize, Serialize};

use crate::resources::namespace::Namespace;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SecretSpec {
    pub data: std::collections::BTreeMap<String, Vec<u8>>,
    pub string_data: Option<std::collections::BTreeMap<String, String>>,
    pub secret_type: Option<String>,
}

pub async fn secret_exists(client: &Client, namespace: &str, name: &str) -> Result<Option<Secret>> {
    let secrets: Api<Secret> = Api::namespaced(client.clone(), namespace);
    match secrets.get_opt(name).await? {
        Some(secret) => Ok(Some(secret)),
        None => Ok(None),
    }
}

pub async fn create_secret(
    client: &Client,
    ctx: &Context,
    namespace: &Namespace,
    name: &str,
    spec: SecretSpec,
) -> Result<Secret, anyhow::Error> {
    let namespace_name = namespace
        .metadata
        .name
        .clone()
        .ok_or_else(|| anyhow::anyhow!("Namespace has no name"))?;
    let secrets: Api<Secret> = Api::namespaced(client.clone(), &namespace_name);

    let secret = Secret {
        metadata: kube::api::ObjectMeta {
            name: Some(name.to_string()),
            ..existing.metadata.clone()
        },
        data: Some(BTreeMap::from([(
            name.to_string(),
            k8s_openapi::ByteString::from(spec.data),
        )])),
        string_data: spec.string_data,
        type_: spec.secret_type,
        ..Default::default()
    };

    if let Some(existing) = secrets.get_opt(name).await? {
        if ctx.replace {
            let pp = kube::api::PostParams::default();
            secrets
                .replace(name, &pp, &secret)
                .await
                .map_err(Into::into)
        } else {
            Ok(existing)
        }
    } else {
        secrets
            .create(&kube::api::PostParams::default(), &secret)
            .await
            .map_err(Into::into)
    }
}
