use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint64;


#[cw_serde]
pub struct InstantiateMsg {
    pub token_name : String,
    pub token_symbol: String,
    pub price_per_share: Uint64,
    pub stocks: Uint64,
}

#[cw_serde]
pub enum QueryMsg {
    GetDetails {},
}