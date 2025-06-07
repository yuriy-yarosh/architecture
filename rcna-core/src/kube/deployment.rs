use crate::resources::context::Context;
use k8s_openapi::api::apps::v1::{Deployment, DeploymentSpec as K8SDeploymentSpec};
use k8s_openapi::api::core::v1::{
    Container, EnvVar as K8sEnvVar, Namespace, PodSpec, PodTemplateSpec,
};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::{LabelSelector, ObjectMeta};
use kube::{
    Client,
    api::{Api, PostParams},
};
use std::collections::BTreeMap;

use super::ContainerSpec;

#[derive(Debug, Clone)]
pub struct EnvVar {
    pub name: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DeploymentSpec {
    pub replicas: Option<i32>,
    pub container: ContainerSpec,
    pub labels: Option<BTreeMap<String, String>>,
}

impl From<DeploymentSpec> for K8SDeploymentSpec {
    fn from(spec: DeploymentSpec) -> Self {
        let container = Container {
            name: spec.container.name,
            image: Some(spec.container.image),
            args: Some(spec.container.args),
            env: Some(
                spec.container
                    .env
                    .into_iter()
                    .map(|e| K8sEnvVar {
                        name: e.name,
                        value: e.value,
                        ..Default::default()
                    })
                    .collect(),
            ),
            ..Default::default()
        };

        // Avoid double-cloning labels
        let labels = spec.labels.unwrap_or_default();

        K8SDeploymentSpec {
            replicas: spec.replicas,
            selector: LabelSelector {
                match_labels: Some(labels.clone()),
                ..Default::default()
            },
            template: PodTemplateSpec {
                metadata: Some(ObjectMeta {
                    labels: Some(labels),
                    ..Default::default()
                }),
                spec: Some(PodSpec {
                    containers: vec![container],
                    ..Default::default()
                }),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

pub async fn deployment_exists(
    client: &Client,
    name: &str,
    namespace: &str,
) -> Result<Option<String>, anyhow::Error> {
    Api::<Deployment>::namespaced(client.clone(), namespace)
        .get(name)
        .await
        .map(|r| r.metadata.resource_version)
        .map_err(Into::into)
}

pub async fn create_deployment(
    client: &Client,
    ctx: &Context,
    namespace: &Namespace,
    name: &str,
    spec: DeploymentSpec,
) -> Result<Deployment, anyhow::Error> {
    if name.is_empty() {
        anyhow::bail!("Deployment name must not be empty");
    }

    let ns_name = namespace
        .metadata
        .name
        .as_deref()
        .ok_or_else(|| anyhow::anyhow!("Namespace must have a name"))?;

    let deployment = Deployment {
        metadata: ObjectMeta {
            name: Some(name.into()),
            namespace: Some(ns_name.into()),
            ..Default::default()
        },
        spec: Some(spec.into()),
        ..Default::default()
    };

    let api = Api::<Deployment>::namespaced(client.clone(), ns_name);
    let pp = PostParams::default();

    if let Some(version) = deployment_exists(&client, name, ns_name).await? {
        if ctx.replace {
            let mut deployment_with_version = deployment.clone();
            deployment_with_version.metadata.resource_version = Some(version);
            api.replace(name, &pp, &deployment_with_version)
                .await
                .map_err(Into::into)
        } else {
            Ok(deployment)
        }
    } else {
        api.create(&pp, &deployment).await.map_err(Into::into)
    }
}
