use schemars::JsonSchema;
use cosmwasm_std::{Addr, Coin }; // Decimal
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub fee: u128,
    pub from_bank_addr: Addr,
    pub from_bank_fee: u128,
    pub to_bank_addr: Addr,
    pub to_bank_fee: u128,
    pub service_addr: Addr,
    pub service_fee: u128
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    UpdateOnlyFee { fee: u128 },
    UpdateConfig { fee: u128, from_bank_addr: Addr, from_bank_fee: u128, to_bank_addr: Addr, to_bank_fee: u128, service_addr: Addr, service_fee: u128},
    Transfer { to: Addr, },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetCurrentFeeState {},
    GetConfigState {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetCurrentFeeStateResponse {
    pub fee: u128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetConfigStateResponse {
    pub fee: u128,
    pub from_bank_addr: Addr,
    pub from_bank_fee: u128,
    pub to_bank_addr: Addr,
    pub to_bank_fee: u128,
    pub service_addr: Addr,
    pub service_fee: u128,
    pub owner: Addr
}


#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub enum Balance {
    Native(Vec<Coin>)
}