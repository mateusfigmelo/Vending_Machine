use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub chocolate: u32,
    pub water: u32,
    pub chips: u32,
}

#[cw_serde]
pub enum ItemType {
    Chocolate,
    Water,
    Chips,
}

#[cw_serde]
pub enum ExecuteMsg {
    GetItem {
        item_type: ItemType,
    },
    Refill {
        chocolate: u32,
        water: u32,
        chips: u32,
    },
}

#[cw_serde]
pub struct ItemsResponse {
    pub chocolate: u32,
    pub water: u32,
    pub chips: u32,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ItemsResponse)]
    ItemsCount {},
}
