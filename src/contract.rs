use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128, Deps, Binary, StdResult, to_binary};
use cosmwasm_std::entry_point;
use crate::state::{Config, Token, CONFIG, TOKEN_STORAGE, TOTAL_SUPPLY};
use crate::msg::{InstantiateMsg, ExecuteMsg, QueryMsg};
use crate::error::ContractError;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config {
        admin: info.sender.clone(),
        mint_enabled: msg.mint_enabled,
        minters: vec![info.sender.clone()],
        supply_cap: msg.supply_cap,
    };
    CONFIG.save(deps.storage, &config)?;
    TOTAL_SUPPLY.save(deps.storage, &Uint128::zero())?;
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Mint { recipient, amount } => execute_mint(deps, env, info, recipient, amount),
        ExecuteMsg::AddMinter { minter } => execute_add_minter(deps, info, minter),
        ExecuteMsg::RemoveMinter { minter } => execute_remove_minter(deps, info, minter),
    }
}

fn execute_mint(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    recipient: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    if !config.mint_enabled {
        return Err(ContractError::MintingDisabled {});
    }

    if !config.minters.contains(&info.sender) {
        return Err(ContractError::Unauthorized {});
    }

    let mut total_supply = TOTAL_SUPPLY.load(deps.storage)?;
    if total_supply + amount > config.supply_cap {
        return Err(ContractError::SupplyCapExceeded {});
    }

    let recipient_addr = deps.api.addr_validate(&recipient)?;
    let mut token = TOKEN_STORAGE.may_load(deps.storage, &recipient_addr)?.unwrap_or(Token { balance: Uint128::zero() });
    token.balance += amount;

    TOKEN_STORAGE.save(deps.storage, &recipient_addr, &token)?;
    total_supply += amount;
    TOTAL_SUPPLY.save(deps.storage, &total_supply)?;

    Ok(Response::new()
        .add_attribute("action", "mint")
        .add_attribute("recipient", recipient)
        .add_attribute("amount", amount.to_string())
        .add_attribute("total_supply", total_supply.to_string()))
}

fn execute_add_minter(deps: DepsMut, info: MessageInfo, minter: String) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    if info.sender != config.admin {
        return Err(ContractError::Unauthorized {});
    }

    let minter_addr = deps.api.addr_validate(&minter)?;
    if config.minters.contains(&minter_addr) {
        return Err(ContractError::MinterAlreadyExists {});
    }

    config.minters.push(minter_addr);
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("action", "add_minter")
        .add_attribute("minter", minter))
}

fn execute_remove_minter(deps: DepsMut, info: MessageInfo, minter: String) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    if info.sender != config.admin {
        return Err(ContractError::Unauthorized {});
    }

    let minter_addr = deps.api.addr_validate(&minter)?;
    if !config.minters.contains(&minter_addr) {
        return Err(ContractError::MinterNotFound {});
    }

    config.minters.retain(|m| m != &minter_addr);
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("action", "remove_minter")
        .add_attribute("minter", minter))
}

#[entry_point]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Balance { address } => to_binary(&query_balance(deps, address)?),
    }
}

fn query_balance(deps: Deps, address: String) -> StdResult<Uint128> {
    let addr = deps.api.addr_validate(&address)?;
    let token = TOKEN_STORAGE.may_load(deps.storage, &addr)?.unwrap_or(Token { balance: Uint128::zero() });
    Ok(token.balance)
}
