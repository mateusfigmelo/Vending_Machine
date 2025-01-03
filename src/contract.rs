#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, ItemType, ItemsResponse, QueryMsg};
use crate::state::{VendingMachineState, STATE};
pub struct VendingMachineContract;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = VendingMachineState {
        owner: info.sender.clone(),
        chocolate: msg.chocolate,
        water: msg.water,
        chips: msg.chips,
    };

    STATE.save(deps.storage, &state)?;
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::GetItem { item_type } => get_item(deps, item_type),
        ExecuteMsg::Refill {
            chocolate,
            water,
            chips,
        } => refill(deps, info, chocolate, water, chips),
    }
}

fn get_item(deps: DepsMut, item_type: ItemType) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        match item_type {
            ItemType::Chocolate if state.chocolate > 0 => state.chocolate -= 1,
            ItemType::Water if state.water > 0 => state.water -= 1,
            ItemType::Chips if state.chips > 0 => state.chips -= 1,
            _ => {
                return Err(ContractError::OutOfStock {
                    item_type: format!("{:?}", item_type),
                })
            }
        };
        Ok(state)
    })?;

    Ok(Response::new().add_attribute("method", "get_item"))
}

fn refill(
    deps: DepsMut,
    info: MessageInfo,
    chocolate: u32,
    water: u32,
    chips: u32,
) -> Result<Response, ContractError> {
    if chocolate == 0 && water == 0 && chips == 0 {
        return Err(ContractError::RefillInvalidAmount {});
    }

    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        if info.sender != state.owner {
            return Err(ContractError::Unauthorized {});
        }
        state.chocolate += chocolate;
        state.water += water;
        state.chips += chips;
        Ok(state)
    })?;

    Ok(Response::new().add_attribute("method", "refill"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::ItemsCount {} => to_json_binary(&query_items_count(deps)?),
    }
}

fn query_items_count(deps: Deps) -> StdResult<ItemsResponse> {
    let state = STATE.load(deps.storage)?;

    Ok(ItemsResponse {
        chocolate: state.chocolate,
        water: state.water,
        chips: state.chips,
    })
}
