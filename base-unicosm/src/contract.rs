#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, from_json, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Addr, StdError, Order};
 
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{
    ExecuteMsg, 
    InstantiateMsg, 
    MigrateMsg, 
    QueryMsg, 
    EntryResponse, 
    BuyListResponse, 
    GameItemResponse
};
use crate::state::{
    CONFIG, 
    PLAYERS_SEQ,      
    PLAYERS_LIST, 
    USERS_ID,    
    GAME_ITEM_ID, 
    GAME_ITEM_SEQ, 
    GAME_ITEM, BUY_SEQ,     
    BUY_ITEM,
    Config,      
    GameItemId, 
    GameItem,
    BuyItem,
    Entry,
    UsersId
};

use cw_storage_plus::Bound; 
use std::ops::Add;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:base-unicosm";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config {
        owner: info.sender.clone(),
        update_cost: msg.update_cost,
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    CONFIG.save(deps.storage, &config)?;

    PLAYERS_SEQ.save(deps.storage, &0u64)?;    
    GAME_ITEM_SEQ.save(deps.storage, &0u64)?;
    BUY_SEQ.save(deps.storage, &0u64)?;
    //GAME_ITEM_ID.save(deps.storage, "item1".to_string(), &0u64)?;

    // Create a new player
    let id = PLAYERS_SEQ.update::<_, cosmwasm_std::StdError>(deps.storage, |id| Ok(id.add(1)))?;
    let new_entry = Entry {
        id,
        name: "Player 1".to_string(),
        address: info.sender.clone(),
        date: env.block.time,
        best_score: 0,
    };
    PLAYERS_LIST.save(deps.storage, id, &new_entry)?; 
    
    let new_entry_id = UsersId {
        address: info.sender.clone(),
        id,        
    };
    USERS_ID.save(deps.storage, info.sender.clone(), &new_entry_id)?;

    // Create a new item
    let id_item = GAME_ITEM_SEQ.update::<_, cosmwasm_std::StdError>(deps.storage, |id_item| Ok(id_item.add(1)))?;
    let new_entry_item = GameItem {
        // id_item, 
        id: id_item,
        item_id: id_item.clone().to_string(), 
        name: "Item 1".to_string(),
        image: "image.png".to_string(),
        // price: price,
    };
    GAME_ITEM.save(deps.storage, id_item, &new_entry_item)?;

    // Buy a new item
    let buy_seq = BUY_SEQ.update::<_, cosmwasm_std::StdError>(deps.storage, |buy_seq| Ok(buy_seq.add(1)))?;
    let new_item = BuyItem {
        id: buy_seq,
        name: "Item 1".to_string(),
        address: info.sender.clone(),
        player_name: "Player 1".to_string(),
        description: "Description".to_string(),
        price: vec![],
        date: env.block.time,
        item_id: "item1".to_string(),
    };
    BUY_ITEM.save(deps.storage, buy_seq, &new_item)?;


    // Index the entry by address
    let game_item_id = GameItemId {
        id: buy_seq.clone(),
        item_id: buy_seq.to_string(), 

    };
    GAME_ITEM_ID.save(deps.storage, buy_seq.clone().to_string(), &game_item_id)?;
 

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.count.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::NewPlayer {
            name
        } => execute::execute_create_new_player(deps, env, info, name),
        ExecuteMsg::UpdatePlayerScore {
            score
        } => execute::execute_update_player_score(deps, info, score),
        ExecuteMsg::BuyItemEntry {
            name,
            description,
            item_id
        } => execute::execute_buy_item(deps, env, info, name, description, item_id.to_string()),
    } 
}

pub mod execute {
    use super::*;

    #[entry_point]
    pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
        let ver = cw2::get_contract_version(deps.storage)?;
        if ver.contract != CONTRACT_NAME.to_string() {
            return Err(StdError::generic_err("Can only upgrade from same type").into());
        }
        cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
        Ok(Response::default())
    }

    pub fn execute_create_new_player(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        name: String,
    ) -> Result<Response, ContractError> {
        let address = info.sender;
       
        if (USERS_ID.may_load(deps.storage, address.clone())?).is_some() {
            return Err(ContractError::PlayerAlreadyExists {});
        }
 
        let id = PLAYERS_SEQ.update::<_, cosmwasm_std::StdError>(deps.storage, |id| Ok(id.add(1)))?;
        let new_entry = Entry {
            id,
            name,
            address: address.clone(),
            date: env.block.time,
            best_score: 0,
        };
        PLAYERS_LIST.save(deps.storage, id, &new_entry)?;

        // Index the entry by address
        let new_entry_id = UsersId {
            id,
            address: address.clone(),
        };
        USERS_ID.save(deps.storage, address, &new_entry_id)?;

        Ok(Response::new()
            .add_attribute("method", "execute_create_new_player")
            .add_attribute("new_player_id", id.to_string()))
    }

    pub fn execute_update_player_score(
        deps: DepsMut,
        info: MessageInfo, 
        score: u64,
    ) -> Result<Response, ContractError> {
        let _config = CONFIG.load(deps.storage)?;
        let address_sender = info.sender.clone();
        let entry_user = USERS_ID.load(deps.storage, address_sender.clone())
            .map_err(|_| ContractError::PlayerNotFound {})?;

        // Solution 1
        /* assert_sent_sufficient_coin(&info.funds, config.update_cost)?; */

        // Solution 2
        /* let owner = CONFIG.load(deps.storage)?.owner;
        if address != owner {
            return Err(ContractError::Unauthorized {});
        } */     
         
        // Solution 3 create a logical to update the player score
        let current_entry = PLAYERS_LIST.load(deps.storage, entry_user.id)?;
        // Only update if the new score is higher than the current best score
        if score <= current_entry.best_score {
            return Err(ContractError::ScoreNotHighEnough {});
        }
    
        let entry_user = USERS_ID.load(deps.storage, address_sender.clone())?;
        let entry = PLAYERS_LIST.load(deps.storage, entry_user.id)?;
        let updated_entry = Entry {
            id: entry_user.id,
            name: entry.name,
            address: address_sender,
            date: entry.date,
            best_score: score,
        };
        PLAYERS_LIST.save(deps.storage, entry_user.id, &updated_entry)?;
        Ok(Response::new()
            .add_attribute("method", "execute_update_player")
            .add_attribute("updated_player_id", entry_user.id.to_string()))
    }

    pub fn execute_buy_item(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        name: String,
        description: String,
        item_id: String,
    ) -> Result<Response, ContractError> {
        let address_sender = info.sender;
 

        //let entry_item_id = GAME_ITEM_ID.load(deps.storage, item_id.clone().to_string())?;
        //let _entry_item = GAME_ITEM.load(deps.storage, entry_item_id.id)?;     
        // Send token before buy
        //send_token_before_buy(&info.funds, entry_item.price.clone())?;

        let entry_user = USERS_ID.load(deps.storage, address_sender.clone())?;
        let entry = PLAYERS_LIST.load(deps.storage, entry_user.id)?;

        // Index the entry by address
        let id = BUY_SEQ.update::<_, cosmwasm_std::StdError>(deps.storage, |id| Ok(id.add(1)))?;

        let new_entry_item = BuyItem {
            id,
            name: name.clone(),
            address: address_sender.clone(),
            player_name: entry.name,
            description: description,
            price: info.funds,
            date: env.block.time,
            item_id: item_id.to_string(),
        };
        BUY_ITEM.save(deps.storage, id, &new_entry_item)?;
        Ok(Response::new()
            .add_attribute("method", "execute_buy_item")
            .add_attribute("execute_buy_item", name.to_string())
            .add_attribute("execute_from", address_sender.clone())
        )
    }


}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::SearchPlayerId { id } => to_json_binary(&query::search_player_id(deps, id)?),
        QueryMsg::SearchPlayerAddress { address } => to_json_binary(&query::search_player_address(deps, address)?),
        QueryMsg::GameItemsList { start_after, limit} => {
            to_json_binary(&query::get_all_items(deps, start_after, limit)?)
        }
        QueryMsg::BuyList { start_after, limit } => {
            to_json_binary(&query::buy_list(deps, start_after, limit)?)
        },
        QueryMsg::BuyByAddress { address } => to_json_binary(&query::buy_by_address(deps, env, address)?),
    }
}

pub mod query {
    use super::*;

    // Limits for pagination
    const MAX_LIMIT: u32 = 10000;
    const DEFAULT_LIMIT: u32 = 10;

    pub fn search_player_id(deps: Deps, id: u64) -> StdResult<EntryResponse> {
        let entry = PLAYERS_LIST.load(deps.storage, id)?;
        Ok(EntryResponse {
            id: entry.id,
            name: entry.name,
            address: entry.address,
            best_score: entry.best_score,
        })
    }
    pub fn search_player_address(deps: Deps, address: Addr) -> StdResult<EntryResponse> {
        let entry_user = USERS_ID.load(deps.storage, address)?;
        let entry = PLAYERS_LIST.load(deps.storage, entry_user.id)?;
        Ok(EntryResponse {
            id: entry.id,
            name: entry.name,
            address: entry.address,
            best_score: entry.best_score,
        })
    }
    pub fn get_all_items(deps: Deps, start_after: Option<u64>, limit: Option<u32>) -> StdResult<GameItemResponse> {
        let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
        let start = start_after.map(Bound::exclusive);

        let entries: StdResult<Vec<_>> = GAME_ITEM
            .range(deps.storage, start, None, Order::Ascending)
            .take(limit)
            .collect();

        let result = GameItemResponse {
            entries: entries?.into_iter().map(|l| l.1).collect(),
        };
        Ok(result)
    }
    pub fn buy_list(deps: Deps, start_after: Option<u64>, limit: Option<u32>) -> StdResult<BuyListResponse> {
        let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
        let start = start_after.map(Bound::exclusive);

        let entries: StdResult<Vec<_>> = BUY_ITEM
            .range(deps.storage, start, None, Order::Ascending)
            .take(limit)
            .collect();

        let result = BuyListResponse {
            entries: entries?.into_iter().map(|l| l.1).collect(),
        };
        Ok(result)
    }
    pub fn buy_by_address(deps: Deps, env: Env, address: Addr) -> StdResult<BuyListResponse> {

        let res = query(deps, env, QueryMsg::BuyList { start_after: Some(0), limit: Some(30) }).unwrap();
        let value: BuyListResponse = from_json(&res).unwrap();
        let mut current_buy = Vec::new();

        for buy_entry in value.entries.iter() {
            if buy_entry.address == Addr::unchecked(address.clone()) {
                current_buy.push(buy_entry.clone());
            }
        }
        let result = BuyListResponse { entries: current_buy };
        Ok(result)
    }
}
