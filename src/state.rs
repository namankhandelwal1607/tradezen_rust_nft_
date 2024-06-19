use cw_storage_plus::Item;
use cosmwasm_std::Uint64;
use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct ContractInfo{
    pub token_name : String,
   pub token_symbol: String,
   pub price_per_share: Uint64,
   pub stocks: Uint64,
}

pub const CONTRACT_INFO : Item<ContractInfo> = Item::new("contract_info");