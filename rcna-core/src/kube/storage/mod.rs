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

pub struct AwsRdsPostgresCluster {}

pub enum PostgreSQLCluster {
    AwsRds(AwsRdsPostgresCluster),
    Stackgres,
    CNPG,
}

pub enum CassandraCluster {
    K8ssandra,
    Scylla,
    AwsKeyspace
}

pub enum AwsVolume {
    AwsEbs,
    AwsEfs,
    AwsS3,
}

pub enum Volume {
    Aws(AwsVolume),
    LocalProvisioner,
}

pub enum VeleroBackups {
    Plugin,
    Hook,
}

pub fn ensure(app: App, credentials: KubernetesCredentials) -> Result<()> {
    Ok(())
}