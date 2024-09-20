// Copyright (c) [2024] SUSE LLC
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

//! Implements a client to access Agama's storage service.
use crate::base_http_client::BaseHTTPClient;
use crate::storage::StorageSettings;
use crate::ServiceError;

pub struct StorageHTTPClient {
    client: BaseHTTPClient,
}

impl StorageHTTPClient {
    pub fn new() -> Result<Self, ServiceError> {
        Ok(Self {
            client: BaseHTTPClient::new()?,
        })
    }

    pub fn new_with_base(base: BaseHTTPClient) -> Self {
        Self { client: base }
    }

    pub async fn get_config(&self) -> Result<StorageSettings, ServiceError> {
        self.client.get("/storage/config").await
    }

    pub async fn set_config(&self, config: &StorageSettings) -> Result<(), ServiceError> {
        self.client.put_void("/storage/config", config).await
    }
}