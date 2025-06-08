/*
 * Copyright (C) 2016-2025 Yuriy Yarosh
 * All rights reserved.
 *
 * SPDX-License-Identifier: MPL-2.0
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::resources::context::Context;
use k8s_openapi::{
    api::core::v1::{Namespace, Service, ServicePort, ServiceSpec as K8SServiceSpec},
    apimachinery::pkg::util::intstr::IntOrString,
};
use kube::{
    Client,
    api::{Api, ObjectMeta, PostParams},
};

pub struct ServicePortSpec {
    pub port: i32,
    pub target_port: Option<i32>,
    pub protocol: Option<String>,
    pub name: Option<String>,
}

pub struct ServiceSpec {
    pub selector: Option<std::collections::BTreeMap<String, String>>,
    pub ports: Vec<ServicePortSpec>,
    pub type_: Option<String>,
}

impl From<ServiceSpec> for K8SServiceSpec {
    fn from(spec: ServiceSpec) -> Self {
        K8SServiceSpec {
            selector: spec.selector,
            ports: Some(
                spec.ports
                    .into_iter()
                    .map(|p| ServicePort {
                        port: p.port,
                        target_port: p.target_port.map(IntOrString::Int),
                        protocol: p.protocol,
                        name: p.name,
                        ..Default::default()
                    })
                    .collect(),
            ),
            type_: spec.type_,
            ..Default::default()
        }
    }
}

pub async fn service_exists(
    client: &Client,
    namespace: &str,
    name: &str,
) -> Result<Option<String>, anyhow::Error> {
    Api::<Service>::namespaced(client.clone(), namespace)
        .get(name)
        .await
        .map(|r| r.metadata.resource_version)
        .map_err(Into::into)
}

pub async fn create_service(
    client: Client,
    namespace: &Namespace,
    name: &str,
    spec: ServiceSpec,
    dry_run: bool,
    replace: bool,
) -> Result<Service, anyhow::Error> {
    if name.is_empty() {
        return Err(anyhow::anyhow!("Service name must not be empty"));
    }

    let ns_name = namespace
        .metadata
        .name
        .as_deref()
        .ok_or_else(|| anyhow::anyhow!("Namespace must have a name"))?;

    let service = Service {
        metadata: ObjectMeta {
            name: Some(name.into()),
            namespace: Some(ns_name.into()),
            ..Default::default()
        },
        spec: Some(spec.into()),
        ..Default::default()
    };

    if dry_run {
        return Ok(service);
    }

    let api = Api::<Service>::namespaced(client.clone(), ns_name);
    let pp = PostParams::default();

    if let Some(version) = service_exists(&client, name, ns_name).await? {
        if replace {
            let mut service_with_version = service.clone();
            service_with_version.metadata.resource_version = Some(version);
            api.replace(name, &pp, &service_with_version)
                .await
                .map_err(Into::into)
        } else {
            Ok(service)
        }
    } else {
        api.create(&pp, &service).await.map_err(Into::into)
    }
}
