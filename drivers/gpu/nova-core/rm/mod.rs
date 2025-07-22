// SPDX-License-Identifier: GPL-2.0
//
// RM (Resource Management) API module for nova-core

pub(crate) mod common;

// Re-export common types for convenience
pub(crate) use common::{RmParams, RmResponseElement};