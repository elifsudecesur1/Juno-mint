use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin: Addr,
    pub mint_enabled: bool,
    pub minters: Vec<Addr>,
    pub supply_cap: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Token {
    pub balance: Uint128,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const TOTAL_SUPPLY: Item<Uint128> = Item::new("total_supply");
pub const TOKEN_STORAGE: Map<&Addr, Token> = Map::new("token_storage");
