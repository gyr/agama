// Copyright (c) [2025] SUSE LLC
//
// All Rights Reserved.
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by the Free
// Software Foundation; either version 2 of the License, or (at your option)
// any later version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
// FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for
// more details.
//
// You should have received a copy of the GNU General Public License along
// with this program; if not, contact SUSE LLC.
//
// To contact SUSE LLC about this file by physical or electronic mail, you may
// find current contact information at www.suse.com.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Software service configuration (product, patterns, etc.).
#[derive(Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct SoftwareConfig {
    /// A map where the keys are the pattern names and the values whether to install them or not.
    pub patterns: Option<HashMap<String, bool>>,
    /// Packages to install.
    pub packages: Option<Vec<String>>,
    /// Name of the product to install.
    pub product: Option<String>,
}

/// Software resolvable type (package or pattern).
#[derive(Deserialize, Serialize, strum::Display, utoipa::ToSchema)]
#[strum(serialize_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub enum ResolvableType {
    Package = 0,
    Pattern = 1,
}

/// Resolvable list specification.
#[derive(Deserialize, Serialize, utoipa::ToSchema)]
pub struct ResolvableParams {
    /// List of resolvables.
    pub names: Vec<String>,
    /// Resolvable type.
    pub r#type: ResolvableType,
    /// Whether the resolvables are optional or not.
    pub optional: bool,
}

/// Repository list specification.
#[derive(Deserialize, Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Repository {
    /// repository identifier
    pub id: i32,
    /// repository alias. Has to be unique
    pub alias: String,
    /// repository name
    pub name: String,
    /// Repository url (raw format without expanded variables)
    pub url: String,
    /// product directory (currently not used, valid only for multiproduct DVDs)
    pub product_dir: String,
    /// Whether the repository is enabled
    pub enabled: bool,
    /// Whether the repository is loaded
    pub loaded: bool,
}
