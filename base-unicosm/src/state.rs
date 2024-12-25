use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cosmwasm_std::Coin;

use cosmwasm_std::Timestamp;

use cw_storage_plus::Item;
use cw_storage_plus::Map;


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub update_cost: Option<Coin>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Entry {
    pub id: u64,
    pub name: String,
    pub address: Addr,
    pub date: Timestamp,
    pub best_score: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct GameItem {
    pub id: u64,
    pub item_id: String,
    pub name: String,
    pub image: String,
    //pub price: Option<Coin>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct UsersId {
    pub address: Addr,
    pub id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct GameItemId {
    pub item_id: String,
    pub id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct BuyItem {
    pub id: u64,
    pub name: String,
    pub address: Addr,
    pub player_name: String,
    pub description: String,
    pub price: Vec<Coin>,
    pub date: Timestamp,
    pub item_id: String,
}

pub const CONFIG: Item<Config> = Item::new("config");

// Start game
pub const PLAYERS_SEQ: Item<u64> = Item::new("players_seq");
pub const PLAYERS_LIST: Map<u64, Entry> = Map::new("list");
pub const USERS_ID: Map<Addr, UsersId> = Map::new("users_id");

pub const BUY_ITEM: Map<u64, BuyItem> = Map::new("buy_item");
pub const BUY_SEQ: Item<u64> = Item::new("buy_seq");

pub const GAME_ITEM_SEQ: Item<u64> = Item::new("game_item_seq");
pub const GAME_ITEM: Map<u64, GameItem> = Map::new("game_item");
pub const GAME_ITEM_ID: Map<String, GameItemId> = Map::new("game_item_id");