// SPDX-License-Identifier: GPL-2.0-only

//! Rust descriptors for the Linux NPU clock controller adapter.

#![allow(missing_docs)]

#[path = "../crates/npucc-sm8250/src/branch/mod.rs"]
mod branch;
#[path = "../crates/npucc-sm8250/src/clock_id.rs"]
mod clock_id;

pub use branch::{ClockBranch, HaltCheck};
pub use clock_id::ClockId;

#[path = "npucc/clock_branch_data.rs"]
mod clock_branch_data;

#[path = "../crates/npucc-sm8250/src/rate/mod.rs"]
mod rate;
pub use rate::{ClockParent, ClockRate, ClockSource};

#[path = "npucc/clock_source_data.rs"]
mod clock_source_data;

#[path = "../crates/npucc-sm8250/src/pll/mod.rs"]
mod pll;
pub use pll::{CalibrationDivider, LucidPll, PostDivider};

#[path = "npucc/pll_data.rs"]
mod pll_data;

#[path = "../crates/npucc-sm8250/src/power_domain_id.rs"]
mod power_domain_id;
#[path = "../crates/npucc-sm8250/src/reset_id.rs"]
mod reset_id;
#[path = "../crates/npucc-sm8250/src/reset_line.rs"]
mod reset_line;
pub use power_domain_id::PowerDomainId;
pub use reset_id::ResetId;
pub use reset_line::ResetLine;

#[path = "npucc/reset_data.rs"]
mod reset_data;

#[path = "../crates/npucc-sm8250/src/hm/mod.rs"]
mod hm;
pub use hm::{HmPair, HmPairOps};

#[path = "npucc/hm_pair_adapter.rs"]
mod hm_pair_adapter;
