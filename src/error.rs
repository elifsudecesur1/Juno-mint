use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Minting is currently disabled")]
    MintingDisabled {},

    #[error("Supply cap exceeded")]
    SupplyCapExceeded {},

    #[error("Minter already exists")]
    MinterAlreadyExists {},

    #[error("Minter not found")]
    MinterNotFound {},
}
