use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Refill amounts must be greater than zero")]
    RefillInvalidAmount {},

    #[error("Unauthorized access: only the contract owner can perform this action.")]
    Unauthorized {},

    #[error("The requested item '{item_type}' is out of stock.")]
    OutOfStock { item_type: String },
}
