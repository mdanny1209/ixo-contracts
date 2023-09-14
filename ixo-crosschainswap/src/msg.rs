use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Uint128;
use cw20::Expiration;

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InstantiateMsg {
    /// Owner if none set to info.sender.
    pub owner: Option<String>,
    pub cw20_token_address: String,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(crate::msg::GetCountResponse)]
    GetCount {
        // The ID of the LOCAL channel you'd like to query the count
        // for.
        channel: String,
    },
    // GetTimeoutCount returns the number of timeouts have occurred on
    // the LOCAL channel `channel`.
    #[returns(crate::msg::GetCountResponse)]
    GetTimeoutCount { channel: String },
}
// We define a custom struct for each query response
#[cw_serde]
pub struct GetCountResponse {
    pub count: u32,
}