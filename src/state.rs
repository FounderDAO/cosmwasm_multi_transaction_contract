use schemars::JsonSchema;
use cosmwasm_std::{Addr, Uint128};  // Decimal
use serde::{Deserialize, Serialize};

use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub fee: Uint128,
    pub from_bank_addr: Addr,
    pub from_bank_fee: Uint128,
    pub to_bank_addr: Addr,
    pub to_bank_fee: Uint128,
    pub service_addr: Addr,
    pub service_fee: Uint128,
    pub owner: Addr,
}

pub const STATE: Item<State> = Item::new("state");
