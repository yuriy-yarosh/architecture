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

use k8s_openapi::api::core::v1::{ConfigMap, Namespace};
use kube::{
    Client,
    api::{Api, ObjectMeta, PostParams},
};

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
    namespace: &Namespace,
    name: &str,
    dry_run: bool,
    replace: bool,
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

    if dry_run {
        return Ok(config_map);
    }

    let api = Api::<ConfigMap>::namespaced(client.clone(), ns_name);
    let pp = PostParams::default();

    if let Some(version) = config_map_exists(&client, name, ns_name).await? {
        if replace {
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
