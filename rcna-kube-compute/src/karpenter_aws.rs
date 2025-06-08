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

pub struct KarpenterAWS {
    context: Context,
}

impl KarpenterAWS {
    pub fn new() -> Self {
        Self {
            context: Context::for_app(App {
                name: "karpenter-aws".to_string(),
                git_url: "".to_string(),
                fetch: VersionFetching::Tag,
                cluster_role: true,
                service_account: true,
                init_containers: vec![],
                container: Container {
                    image: "public.ecr.aws/karpenter/karpenter-aws".to_string(),
                    tag: "v0.25.0".to_string(),
                    args: vec![],
                },
            }),
        }
    }
}
