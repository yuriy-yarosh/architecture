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

use serde::*;

mod credentials;
mod context;
mod git;

pub use context::*;
pub use credentials::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Container {
    pub image: String,
    pub tag: String,
    pub args: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VersionFetching {
    FilteredRelease { omit: Vec<String> },
    FilteredTag { omit: Vec<String> },
    Release,
    Tag,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App {
    pub name: String,

    pub git_url: String,
    pub fetch: VersionFetching,

    pub cluster_role: bool,
    pub service_account: bool,

    pub init_containers: Vec<Container>,
    pub container: Container,
}

impl std::fmt::Display for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
