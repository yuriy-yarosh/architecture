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

use futures::*;
use std::fs;
use std::fs::create_dir_all;
use std::path::Path;

#[path = "./src/bin/rcna_extractor/mod.rs"]
mod rcna_extractor;
use rcna_extractor::*;

#[tokio::main]
async fn main() {}
