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

use crate::provider::CloudProvider;

pub struct App {
    deployed: Vec<(CloudProvider, String)>,
}

impl App {
    pub fn deploy(cloud_provider: CloudProvider, credentials: String) -> Self {
        Self {
            deployed: vec![(cloud_provider, credentials)],
        }
    }

    pub fn deploy_more(&mut self, cloud_provider: CloudProvider, credentials: String) {
        self.deployed.push((cloud_provider, credentials));
    }
}
