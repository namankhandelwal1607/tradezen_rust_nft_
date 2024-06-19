#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary,  Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ InstantiateMsg, QueryMsg};
use crate::state::{CONTRACT_INFO, ContractInfo};

const CONTRACT_NAME: &str = "crates.io:stock-nft";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let contract_info=ContractInfo{
    token_name : msg.token_name.to_owned(),
   token_symbol: msg.token_symbol.to_owned(),
   price_per_share: msg.price_per_share,
   stocks: msg.stocks,
    };

    CONTRACT_INFO.save(deps.storage, &contract_info)?;

    
    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("token_name", msg.token_name)
        .add_attribute("token_symbol", msg.token_symbol)
        .add_attribute("price_per_share", msg.price_per_share)
        .add_attribute("stocks", msg.stocks))
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg{
        QueryMsg::GetDetails {  } => to_json_binary(&query::get_details(deps)?),
    }
}

pub mod query {
    use cosmwasm_std::StdResult;
    use super::*;

    pub fn get_details(deps : Deps) -> StdResult<ContractInfo>{
        let contract_info = CONTRACT_INFO.load(deps.storage)?;
        Ok(contract_info)
    }
}

#[cfg(test)]
mod tests {}
