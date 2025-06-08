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

use k8s_openapi::api::{
    core::v1::{Namespace, ServiceAccount},
    rbac::v1::{ClusterRole, ClusterRoleBinding, RoleRef, Subject},
};
use kube::{
    Client,
    api::{Api, ObjectMeta, PostParams},
};

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
    namespace: &Namespace,
    binding_name: &str,
    cluster_role: &ClusterRole,
    service_account: &ServiceAccount,
    dry_run: bool,
    replace: bool,
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

    if dry_run {
        return Ok(binding);
    }

    let api = Api::<ClusterRoleBinding>::all(client.clone());
    let pp = PostParams::default();

    if let Some(version) = cluster_role_binding_exists(&client, binding_name).await? {
        if replace {
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
