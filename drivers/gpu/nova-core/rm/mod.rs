// SPDX-License-Identifier: GPL-2.0
//
// RM (Resource Management) API module for nova-core

pub(crate) mod common;
pub(crate) mod control;

// Re-export common types for convenience
pub(crate) use common::{RmParams, RmResponseElement};

// Re-export control types
pub(crate) use control::{RmControl /* unused for now: , RmControlHeader */};