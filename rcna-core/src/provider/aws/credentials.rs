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

#[derive(Debug, Clone)]
pub enum AWSCredentials {
    /// Use the current aws config (default)
    AWSConfig { profile: Option<String> },
    ShortTerm {
        access_key_id: String,
        secret_access_key: String,
        session_token: Option<String>,
    },
    LongTerm {
        access_key_id: String,
        secret_access_key: String,
    },
    EC2Metadata {
        default_role_arn: Option<String>,
        role_arn: String,
        region: String,
    },
    IAMRole {
        role_arn: String,
        role_session_name: Option<String>,
        source_profile: Option<String>,
        region: String,
    },
}

impl Default for AWSCredentials {
    fn default() -> Self {
        Self::AWSConfig { profile: None }
    }
}
