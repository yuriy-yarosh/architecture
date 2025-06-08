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

use anyhow::*;
use super::App;
use super::Context as DeploymentContext;

#[derive(Debug, Clone)]
pub struct ContainerSpec {
    pub name: String,
    pub image: String,
    pub args: Vec<String>,
    pub env: Vec<EnvVar>,
}

#[derive(Debug, Clone)]
pub struct EnvVar {
    pub name: String,
    pub value: Option<String>,
    pub value_from: Option<EnvVarSource>,
}

#[derive(Debug, Clone)]
pub struct EnvVarSource {
    pub config_map_key_ref: Option<ConfigMapKeySelector>,
}

#[derive(Debug, Clone)]
pub struct ConfigMapKeySelector {
    pub name: String,
    pub key: String,
}

pub fn ensure(app: App, ctx: DeploymentContext) -> Result<()> {
    Ok(())
}