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
use k8s_openapi::api::core::v1::ServiceAccount;
use kube::{
    Api, Client,
    api::{ObjectMeta, PostParams},
};

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
    namespace: &Namespace,
    name: &str,
    dry_run: bool,
    replace: bool,
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

    if dry_run {
        return Ok(service_account);
    }

    match service_account_exists(client, name).await? {
        Some(version) if replace => {
            service_account.metadata.resource_version = Some(version);
            api.create(&pp, &service_account).await.map_err(Into::into)
        }
        Some(_) => Ok(service_account),
        None => api.create(&pp, &service_account).await.map_err(Into::into),
    }
}
