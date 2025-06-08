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

use k8s_openapi::api::core::v1::Namespace;
use kube::{
    Api, Client,
    api::{ObjectMeta, PostParams},
};

pub async fn namespace_exists(
    client: &Client,
    name: &str,
) -> Result<Option<String>, anyhow::Error> {
    Api::<Namespace>::all(client.clone())
        .get(name)
        .await
        .map(|r| r.metadata.resource_version)
        .map_err(Into::into)
}

pub async fn create_namespace(
    client: &Client,
    name: &str,
    dry_run: bool,
    replace: bool,
) -> Result<Namespace, anyhow::Error> {
    if name.is_empty() {
        return Err(anyhow::anyhow!("Namespace name must not be empty"));
    }

    let namespace = Namespace {
        metadata: ObjectMeta {
            name: Some(name.into()),
            ..Default::default()
        },
        ..Default::default()
    };

    if dry_run {
        return Ok(namespace);
    }

    let api = Api::<Namespace>::all(client.clone());
    let pp = PostParams::default();

    if let Some(version) = namespace_exists(&client, name).await? {
        if ctx.replace {
            let mut namespace_with_version = namespace.clone();
            namespace_with_version.metadata.resource_version = Some(version);
            api.replace(name, &pp, &namespace_with_version)
                .await
                .map_err(Into::into)
        } else {
            Ok(namespace)
        }
    } else {
        api.create(&pp, &namespace).await.map_err(Into::into)
    }
}
