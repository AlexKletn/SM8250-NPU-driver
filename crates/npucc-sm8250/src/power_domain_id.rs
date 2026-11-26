// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2018, The Linux Foundation. All rights reserved.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum PowerDomainId {
    CoreGdsc = 0,
}

impl PowerDomainId {
    pub const fn gdsc_offset_bytes(self) -> u32 {
        match self {
            Self::CoreGdsc => 0x1004,
        }
    }
}
