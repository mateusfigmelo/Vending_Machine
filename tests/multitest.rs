#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        from_json,
        testing::{message_info, mock_dependencies, mock_env},
        Addr,
    };
    use vending_machine::contract::{execute, instantiate, query};
    use vending_machine::error::ContractError;
    use vending_machine::msg::{ExecuteMsg, InstantiateMsg, ItemType, ItemsResponse, QueryMsg};
    use vending_machine::state::{VendingMachineState, STATE};

    #[test]
    fn test_initilization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            chocolate: 10,
            water: 20,
            chips: 30,
        };
        let info = message_info(&Addr::unchecked("owner"), &[]);
        let env = mock_env();
        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        let state: VendingMachineState = STATE.load(&deps.storage).unwrap();
        assert_eq!(state.chocolate, 10);
        assert_eq!(state.water, 20);
        assert_eq!(state.chips, 30);
        assert_eq!(state.owner.as_str(), "owner");

        assert_eq!(res.attributes.len(), 2);
        assert_eq!(res.attributes[0].key, "method");
        assert_eq!(res.attributes[0].value, "instantiate");
        assert_eq!(res.attributes[1].key, "owner");
        assert_eq!(res.attributes[1].value, "owner");
    }

    #[test]
    fn test_get_item() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            chocolate: 10,
            water: 20,
            chips: 30,
        };
        let info = message_info(&Addr::unchecked("owner"), &[]);
        let env = mock_env();
        instantiate(deps.as_mut(), env, info, msg).unwrap();

        let exec_msg = ExecuteMsg::GetItem {
            item_type: ItemType::Chocolate,
        };
        let info = message_info(&Addr::unchecked("user"), &[]);
        let res = execute(deps.as_mut(), mock_env(), info, exec_msg).unwrap();

        let state: VendingMachineState = STATE.load(&deps.storage).unwrap();
        assert_eq!(state.chocolate, 9);
        assert_eq!(res.attributes[0].key, "method");
        assert_eq!(res.attributes[0].value, "get_item");
    }
    #[test]
    fn test_get_out_of_stock_item() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            chocolate: 0,
            water: 0,
            chips: 1,
        };
        let info = message_info(&Addr::unchecked("owner"), &[]);
        let env = mock_env();
        instantiate(deps.as_mut(), env, info, msg).unwrap();

        let exec_msg = ExecuteMsg::GetItem {
            item_type: ItemType::Chips,
        };
        let info = message_info(&Addr::unchecked("user"), &[]);
        let res = execute(deps.as_mut(), mock_env(), info.clone(), exec_msg).unwrap();
        assert_eq!(res.attributes[0].value, "get_item");

        let exec_msg = ExecuteMsg::GetItem {
            item_type: ItemType::Chips,
        };
        let res = execute(deps.as_mut(), mock_env(), info, exec_msg);

        match res {
            Err(ContractError::OutOfStock { item_type }) => assert_eq!(item_type, "Chips"),
            _ => panic!("Expected OutOfStock error"),
        }
    }

    #[test]
    fn test_refill() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            chocolate: 0,
            water: 0,
            chips: 0,
        };
        let info = message_info(&Addr::unchecked("owner"), &[]);
        let env = mock_env();
        instantiate(deps.as_mut(), env, info, msg).unwrap();

        let exec_msg = ExecuteMsg::Refill {
            chocolate: 10,
            water: 20,
            chips: 30,
        };
        let info = message_info(&Addr::unchecked("owner"), &[]);
        let res = execute(deps.as_mut(), mock_env(), info, exec_msg).unwrap();

        let state: VendingMachineState = STATE.load(&deps.storage).unwrap();
        assert_eq!(state.chocolate, 10);
        assert_eq!(state.water, 20);
        assert_eq!(state.chips, 30);

        assert_eq!(res.attributes[0].key, "method");
        assert_eq!(res.attributes[0].value, "refill");
    }

    #[test]
    fn test_refill_non_owner() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            chocolate: 0,
            water: 0,
            chips: 0,
        };
        let info = message_info(&Addr::unchecked("owner"), &[]);
        let env = mock_env();
        instantiate(deps.as_mut(), env, info, msg).unwrap();

        let exec_msg = ExecuteMsg::Refill {
            chocolate: 10,
            water: 20,
            chips: 30,
        };
        let info = message_info(&Addr::unchecked("user"), &[]);
        let res = execute(deps.as_mut(), mock_env(), info, exec_msg);

        match res {
            Err(ContractError::Unauthorized {}) => (),
            _ => panic!("Expected Unauthorized error"),
        }
    }

    #[test]
    fn test_refill_with_invalid_amount() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            chocolate: 10,
            water: 20,
            chips: 30,
        };
        let info = message_info(&Addr::unchecked("owner"), &[]);
        let env = mock_env();
        instantiate(deps.as_mut(), env, info, msg).unwrap();

        let exec_msg = ExecuteMsg::Refill {
            chocolate: 0,
            water: 0,
            chips: 0,
        };
        let info = message_info(&Addr::unchecked("owner"), &[]);
        let res = execute(deps.as_mut(), mock_env(), info, exec_msg);

        match res {
            Err(ContractError::RefillInvalidAmount {}) => (),
            _ => panic!("Expected Unauthorized error"),
        }
    }

    #[test]
    fn test_query_items_count() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            chocolate: 10,
            water: 20,
            chips: 30,
        };
        let info = message_info(&Addr::unchecked("owner"), &[]);
        let env = mock_env();
        instantiate(deps.as_mut(), env, info, msg).unwrap();

        let query_msg = QueryMsg::ItemsCount {};
        let res = query(deps.as_ref(), mock_env(), query_msg).unwrap();
        let items_count: ItemsResponse = from_json(res).unwrap();

        assert_eq!(items_count.chocolate, 10);
        assert_eq!(items_count.water, 20);
        assert_eq!(items_count.chips, 30);
    }
}
