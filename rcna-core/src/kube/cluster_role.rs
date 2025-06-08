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

use k8s_openapi::api::rbac::v1::{ClusterRole, PolicyRule as K8SPolicyRule};
use kube::{
    Client,
    api::{Api, ObjectMeta, PostParams},
};

pub struct PolicyRule {
    pub api_groups: Vec<String>,
    pub resources: Vec<String>,
    pub verbs: Vec<String>,
}

impl From<PolicyRule> for K8SPolicyRule {
    fn from(rule: PolicyRule) -> Self {
        K8SPolicyRule {
            api_groups: Some(rule.api_groups),
            resources: Some(rule.resources),
            verbs: rule.verbs,
            non_resource_urls: None,
            resource_names: None,
        }
    }
}

pub async fn cluster_role_exists(
    client: &Client,
    name: &str,
) -> Result<Option<String>, anyhow::Error> {
    Api::<ClusterRole>::all(client.clone())
        .get(name)
        .await
        .map(|r| r.metadata.resource_version)
        .map_err(Into::into)
}

pub async fn create_cluster_role(
    client: &Client,
    role_name: &str,
    rules: Vec<PolicyRule>,
    dry_run: bool,
    replace: bool,
) -> Result<ClusterRole, anyhow::Error> {
    if role_name.is_empty() {
        return Err(anyhow::anyhow!("ClusterRole name must not be empty"));
    }

    let role = ClusterRole {
        metadata: ObjectMeta {
            name: Some(role_name.to_string()),
            ..Default::default()
        },
        rules: Some(rules.into_iter().map(|r| r.into()).collect()),
        ..Default::default()
    };

    if dry_run {
        return Ok(role);
    }

    let api = Api::<ClusterRole>::all(client.clone());
    let pp = PostParams::default();

    if let Some(version) = cluster_role_exists(&client, role_name).await? {
        if replace {
            let mut role_with_version = role.clone();
            role_with_version.metadata.resource_version = Some(version);
            api.replace(&role_name, &pp, &role_with_version)
                .await
                .map_err(Into::into)
        } else {
            Ok(role)
        }
    } else {
        api.create(&pp, &role).await.map_err(Into::into)
    }
}
