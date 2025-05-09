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

mod initiator;
pub use initiator::InitiatorProxy;

mod node;
pub use node::NodeProxy;

use zbus::proxy;
#[proxy(
    default_service = "org.opensuse.Agama.Storage1",
    default_path = "/org/opensuse/Agama/Storage1/ISCSI",
    interface = "org.opensuse.Agama.Storage1.ISCSI",
    assume_defaults = true
)]
pub trait ISCSI {
    /// SetConfig method
    fn set_config(&self, serialized_config: &str) -> zbus::Result<u32>;
}
