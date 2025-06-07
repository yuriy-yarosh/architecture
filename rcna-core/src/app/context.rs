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
use envy;
use serde::*;

use super::App;

/// Target environment, user-specific for Development and Testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Environment {
    Development { user: String },
    Testing { user: String },
    Staging,
    Production,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    organization: String,
    environment: Environment,
    unit: Option<String>,
    app: App,
    dry_run: bool,
    replace: bool,
}

/// Struct for direct env parsing (flat, simple)
#[derive(Debug, Clone, Deserialize)]
struct ContextEnv {
    #[serde(default = "default_org")]
    organization: String,
    env: Option<String>,
    unit: Option<String>,
    #[serde(default)]
    dry_run: bool,
    #[serde(default)]
    replace: bool,
}

fn default_org() -> String {
    "rcna".to_string()
}

impl Context {
    pub fn for_app(app: App) -> Result<Self> {
        let env_config = envy::from_env::<ContextEnv>()?;
        let environment = parse_environment(env_config.env.as_deref())?;
        Ok(Self {
            organization: env_config.organization,
            environment,
            unit: env_config.unit,
            dry_run: env_config.dry_run,
            replace: env_config.replace,
            app,
        })
    }

    pub fn with_dry_run(&mut self) -> &mut Self {
        self.dry_run = true;
        self
    }

    pub fn with_replace(&mut self) -> &mut Self {
        self.replace = true;
        self
    }
}

/// Parse environment string into Environment enum
fn parse_environment(env: Option<&str>) -> Result<Environment> {
    match env.map(|s| s.trim().to_ascii_lowercase()) {
        Some(ref s) if s == "staging" => Ok(Environment::Staging),
        Some(ref s) if s == "production" => Ok(Environment::Production),
        Some(s) if s.starts_with("development:") => {
            let user = s.splitn(2, ':').nth(1).unwrap_or("").to_string();
            if user.is_empty() {
                Err(anyhow!("User is not specified for development environment"))
            } else {
                Ok(Environment::Development { user })
            }
        }
        Some(s) if s.starts_with("testing:") => {
            let user = s.splitn(2, ':').nth(1).unwrap_or("").to_string();
            if user.is_empty() {
                Err(anyhow!("User is not specified for testing environment"))
            } else {
                Ok(Environment::Testing { user })
            }
        }
        Some(s) => Err(anyhow!("Unknown environment: {}", s)),
        None => Err(anyhow!("ENV variable is not set")),
    }
}
