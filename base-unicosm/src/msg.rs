use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Coin;
use cosmwasm_std::Addr; 

use crate::state::BuyItem;
use crate::state::GameItem;




#[cw_serde]
pub struct InstantiateMsg {
    pub count: i32,
    pub update_cost: Option<Coin>,
}
#[cw_serde]
pub struct MigrateMsg {
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Create a new player
    NewPlayer {
        name: String
    },
    /// Update player on contract
    UpdatePlayerScore {
        score: u64
    },
    /// Create a new buy item
    BuyItemEntry {
        name: String,
        description: String,
        item_id: u64
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Search player by id
    #[returns(EntryResponse)]
    SearchPlayerId {
        /// id to query
        id: u64
    },
    /// Search player by address
    #[returns(EntryResponse)]
    SearchPlayerAddress { address: Addr },
    /// List all buy items
    #[returns(BuyListResponse)]
    BuyList {
        start_after: Option<u64>,
        limit: Option<u32>,
    },
    /// List all buy items by address
    #[returns(BuyListResponse)]
    BuyByAddress {
        address: Addr
    },
    /// List all items
    #[returns(GameItemResponse)]
    GameItemsList {
        start_after: Option<u64>,
        limit: Option<u32>,
    }
}

// We define a custom struct for each query response

#[cw_serde]
pub struct GetJwtTokenResponse {
    pub validated: String,
}

#[cw_serde]
pub struct EntryResponse {
    pub id: u64,
    pub name: String,
    pub address: Addr,
    pub best_score: u64,
}

#[cw_serde]
pub struct BuyListResponse {
    pub entries: Vec<BuyItem>,
}

#[cw_serde]
pub struct GameItemResponse {
    pub entries: Vec<GameItem>,
}