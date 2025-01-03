use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VendingMachineState {
    pub owner: Addr,
    pub chocolate: u32,
    pub water: u32,
    pub chips: u32,
}

pub const STATE: Item<VendingMachineState> = Item::new("vending_machine_state");
