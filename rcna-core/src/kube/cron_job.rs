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

use k8s_openapi::api::batch::v1::{
    CronJob, CronJobSpec as K8SCronJobSpec, JobSpec, JobTemplateSpec,
};
use k8s_openapi::api::core::v1::{Container, Namespace, PodSpec};
use kube::{
    Client,
    api::{Api, ObjectMeta, PostParams},
};

use crate::resources::context::Context;

use super::ContainerSpec;

pub struct CronJobSpec {
    pub schedule: String,
    pub container: ContainerSpec,
    pub concurrency_policy: Option<String>,
    pub successful_jobs_history_limit: Option<i32>,
    pub failed_jobs_history_limit: Option<i32>,
}

impl From<CronJobSpec> for K8SCronJobSpec {
    fn from(builder: CronJobSpec) -> Self {
        let container = Container {
            name: builder.container.name,
            image: Some(builder.container.image),
            args: Some(builder.container.args),
            env: Some(
                builder
                    .container
                    .env
                    .into_iter()
                    .map(|e| k8s_openapi::api::core::v1::EnvVar {
                        name: e.name,
                        value: e.value,
                        value_from: e.value_from.map(|s| {
                            k8s_openapi::api::core::v1::EnvVarSource {
                                config_map_key_ref: s.config_map_key_ref.map(|k| {
                                    k8s_openapi::api::core::v1::ConfigMapKeySelector {
                                        name: k.name,
                                        key: k.key,
                                        optional: None,
                                    }
                                }),
                                ..Default::default()
                            }
                        }),
                        ..Default::default()
                    })
                    .collect(),
            ),
            ..Default::default()
        };

        K8SCronJobSpec {
            schedule: builder.schedule,
            job_template: JobTemplateSpec {
                spec: Some(JobSpec {
                    template: k8s_openapi::api::core::v1::PodTemplateSpec {
                        spec: Some(PodSpec {
                            containers: vec![container],
                            restart_policy: Some("OnFailure".into()),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
                ..Default::default()
            },
            concurrency_policy: builder.concurrency_policy,
            successful_jobs_history_limit: builder.successful_jobs_history_limit,
            failed_jobs_history_limit: builder.failed_jobs_history_limit,
            ..Default::default()
        }
    }
}

pub async fn cron_job_exists(
    client: &Client,
    namespace: &str,
    name: &str,
) -> Result<Option<String>, anyhow::Error> {
    Api::<CronJob>::namespaced(client.clone(), namespace)
        .get(name)
        .await
        .map(|r| r.metadata.resource_version)
        .map_err(Into::into)
}

pub async fn create_cron_job(
    client: Client,
    namespace: &Namespace,
    name: &str,
    spec: CronJobSpec,
    dry_run: bool,
    replace: bool,
) -> Result<CronJob, anyhow::Error> {
    if name.is_empty() {
        return Err(anyhow::anyhow!("CronJob name must not be empty"));
    }

    let ns_name = namespace
        .metadata
        .name
        .as_deref()
        .ok_or_else(|| anyhow::anyhow!("Namespace must have a name"))?;

    let cron_job = CronJob {
        metadata: ObjectMeta {
            name: Some(name.into()),
            namespace: Some(ns_name.into()),
            ..Default::default()
        },
        spec: Some(spec.into()),
        ..Default::default()
    };

    if dry_run {
        return Ok(cron_job);
    }

    let api = Api::<CronJob>::namespaced(client.clone(), ns_name);
    let pp = PostParams::default();

    if let Some(version) = cron_job_exists(&client, name, ns_name).await? {
        if replace {
            let mut cron_job_with_version = cron_job.clone();
            cron_job_with_version.metadata.resource_version = Some(version);
            api.replace(name, &pp, &cron_job_with_version)
                .await
                .map_err(Into::into)
        } else {
            Ok(cron_job)
        }
    } else {
        api.create(&pp, &cron_job).await.map_err(Into::into)
    }
}
