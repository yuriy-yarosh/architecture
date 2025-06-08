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

use rcna_core::*;

pub struct Descheduler {
    context: Context,
}

impl Descheduler {
    pub fn new() -> Self {
        Self {
            context: Context::for_app(App {
                name: "descheduler".to_string(),
                git_url: "".to_string(),
                fetch: VersionFetching::Tag,
                cluster_role: true,
                service_account: true,
                init_containers: vec![],
                container: Container {
                    image: "".to_string(),
                    tag: "".to_string(),
                    args: vec![],
                },
            }),
        }
    }
}
