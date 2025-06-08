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

use anyhow::Context;
use kube::{Client, Config};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum KubernetesCredentials {
    /// Use the current kubeconfig context (default)
    KubeConfig {
        context: Option<String>, // None = current context
        cluster: Option<String>, // None = default cluster
        user: Option<String>,    // None = default user
    },
    /// Use manually provided endpoint and certs
    Manual {
        server: String,
        ca_cert: Option<Vec<Vec<u8>>>,
        client_cert: Option<String>,
        client_key: Option<String>,
        token: Option<String>,
    },
}

impl KubernetesCredentials {
    /// Connect to the Kubernetes cluster using the provided credentials
    pub async fn client(&self) -> anyhow::Result<Client> {
        if let Ok(in_cluster_client) =
            Client::try_from(Config::incluster().context("Failed to load in-cluster config")?)
                .context("Failed to create client from in-cluster config")
        {
            return Ok(in_cluster_client);
        }

        let config = match self {
            KubernetesCredentials::KubeConfig {
                context,
                cluster,
                user,
            } => Config::from_kubeconfig(&kube::config::KubeConfigOptions {
                context: context.clone(),
                cluster: cluster.clone(),
                user: user.clone(),
            })
            .await
            .context("Failed to load kubeconfig")?,
            KubernetesCredentials::Manual {
                server,
                ca_cert,
                client_cert,
                client_key,
                token,
            } => {
                let mut config = Config::new(
                    server
                        .clone()
                        .parse()
                        .context("Failed to parse server URL")?,
                );
                if let Some(ca) = ca_cert {
                    config.root_cert = Some(ca.clone());
                }
                if let Some(cert) = client_cert {
                    config.auth_info.client_certificate = Some(cert.clone());
                }
                if let Some(key) = client_key {
                    config.auth_info.client_key = Some(key.clone());
                }
                if let Some(t) = token {
                    config.auth_info.token = Some(t.clone().into());
                }
                config
            }
        };
        Ok(Client::try_from(config)?)
    }
}

impl Default for KubernetesCredentials {
    fn default() -> Self {
        Self::KubeConfig {
            context: None,
            cluster: None,
            user: None,
        }
    }
}

impl std::fmt::Display for KubernetesCredentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::future::join_all;

    const LOCAL_SERVER: &str = "https://localhost:6443";

    #[tokio::test]
    async fn test_kubeconfig_default() {
        let credentials = vec![
            KubernetesCredentials::KubeConfig {
                context: None,
                cluster: None,
                user: None,
            },
            KubernetesCredentials::Manual {
                server: LOCAL_SERVER.to_string(),
                ca_cert: None,
                client_cert: None,
                client_key: None,
                token: None,
            },
            KubernetesCredentials::Manual {
                server: LOCAL_SERVER.to_string(),
                ca_cert: None,
                client_cert: None,
                client_key: None,
                token: Some("dummy-token".to_string()),
            },
        ];

        assert!(
            join_all(credentials.iter().map(|c| c.client()))
                .await
                .iter()
                .all(|r| r.is_ok() || r.is_err())
        );
    }
}
