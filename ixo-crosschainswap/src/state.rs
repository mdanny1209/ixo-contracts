use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};
use cw20::Expiration;
use cw_storage_plus::{Item, Map};

/// (channel_id) -> count. Reset on channel closure.
pub const CONNECTION_COUNTS: Map<String, u32> = Map::new("connection_counts");
/// (channel_id) -> timeout_count. Reset on channel closure.
pub const TIMEOUT_COUNTS: Map<String, u32> = Map::new("timeout_count");