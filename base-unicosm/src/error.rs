use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Player already exists")]
    PlayerAlreadyExists {},

    #[error("Insufficient funds sent")]
    InsufficientFundsSend {},    

    #[error("Score Not High Enough")]
    ScoreNotHighEnough {},   

    #[error("Player Not Found")]
    PlayerNotFound {},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
