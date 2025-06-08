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

mod cluster_role;
mod cluster_role_binding;
mod config_map;
mod cron_job;
mod deployment;
mod namespace;
mod secret;
mod service;
mod service_account;

mod compute;
mod finops;
mod mlops;
mod networking;
mod observability;
mod security;
mod storage;

use crate::*;

pub fn deploy(app: App, context: Context) {
    compute::ensure(app, context);
    finops::ensure(app, context);
    mlops::ensure(app, context);
    networking::ensure(app, context);
    observability::ensure(app, context);
    security::ensure(app, context);
    storage::ensure(app, context);

    cluster_role::deploy(app);
    cluster_role_binding::deploy(app);
    config_map::deploy(app);
    cron_job::deploy(app);
    deployment::deploy(app);
    namespace::deploy(app);
    secret::deploy(app);
    service::deploy(app);
    service_account::deploy(app);
}