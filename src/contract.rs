#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo,Addr, Response,  StdResult, BankMsg, Uint128, Coin, SubMsg}; //Uint128, SubMsg, CosmosMsg
use cw2::set_contract_version;
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, GetCurrentFeeStateResponse, GetConfigStateResponse };
use crate::state::{State, STATE};


const CONTRACT_NAME: &str = "crates.io:{{project-name}}";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    let state = State {
        fee: Uint128::new(_msg.fee),
        owner:_info.sender.clone(), 
        from_bank_addr: _msg.from_bank_addr, 
        from_bank_fee: Uint128::new(_msg.from_bank_fee), 
        to_bank_addr: _msg.to_bank_addr, 
        to_bank_fee: Uint128::new(_msg.to_bank_fee), 
        service_addr: _msg.service_addr, 
        service_fee: Uint128::new(_msg.service_fee) 
    };
    set_contract_version(_deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(_deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", &_info.sender.to_string())
        .add_attribute("fee", _msg.fee.to_string())
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateOnlyFee { fee } => try_update_transaction_fee(deps, info, fee),
        ExecuteMsg::UpdateConfig { fee, from_bank_addr, from_bank_fee, to_bank_addr, to_bank_fee, service_addr, service_fee } => try_update_config(deps, info, fee, from_bank_addr.to_string(), from_bank_fee, to_bank_addr.to_string(), to_bank_fee, service_addr.to_string(), service_fee ),
        ExecuteMsg::Transfer { to } => execute_transfer(deps, info, to.to_string())
    }
}

pub fn try_update_transaction_fee(deps: DepsMut, info: MessageInfo, fee: u128) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        if info.sender != state.owner {
            return Err(ContractError::Unauthorized {});
        }
        state.fee = Uint128::new(fee);
        Ok(state)
    })?;
    Ok(Response::new()
    .add_attribute("method", "update_transaction_fee")
    .add_attribute("new_transaction_fee", fee.to_string())
)
}

pub fn try_update_config(deps:DepsMut, info: MessageInfo, fee: u128, from: String, from_fee: u128,  to: String, to_fee: u128, serviss: String, serviss_fee: u128) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        if info.sender != state.owner {
            return Err(ContractError::Unauthorized{});
        }
        state.fee = Uint128::new(fee);
        state.from_bank_addr = Addr::unchecked(from.clone());
        state.from_bank_fee = Uint128::new(from_fee.clone());
        state.to_bank_addr = Addr::unchecked(to.clone());
        state.to_bank_fee = Uint128::new(to_fee.clone());
        state.service_addr = Addr::unchecked(serviss.clone());
        state.service_fee = Uint128::new(serviss_fee.clone());

        Ok(state)
    })?;
    Ok(
        Response::new()
        .add_attribute("method","updated config")
        .add_attribute("from_bank_address", from.to_string())
        .add_attribute("from_bank_percentage", from_fee.to_string())
        .add_attribute("to_bank_address", to.to_string())
        .add_attribute("to_bank_fee_percentage", to_fee.to_string())
        .add_attribute("service_fee_address", serviss.to_string())
        .add_attribute("service_fee", serviss_fee.to_string())

    )
}

pub fn execute_transfer(deps:DepsMut, mut _info:MessageInfo, recipient: String)-> Result<Response, ContractError> {
   
    if _info.funds[0].amount.is_zero(){
        return Err(ContractError::InvalidZeroAmount {});
    } else {
        let state = STATE.load(deps.storage)?;

        let sto: Uint128 = Uint128::new(100);

        let _fee = state.fee;

        let coin_name: String = _info.funds[0].denom.clone();
        
        let to = deps.api.addr_validate(&recipient)?;

        let fee_amount: Uint128 = Uint128::new((_info.funds[0].amount.u128() * _fee.u128()) / sto.u128());

        // Calculate to amount 
        _info.funds[0].amount = Uint128::new(_info.funds[0].amount.u128() - fee_amount.u128());
        
        // Create to amount transaction
        let to_transaction_msg = create_transaction(to, _info.funds[0].amount, coin_name.to_string());

        // Calculate from bank amount
        let from_bank_amount = Uint128::new(fee_amount.u128() * state.from_bank_fee.u128() / sto.u128());

        // Create from bank transaction
        let from_bank_transaction = create_transaction(state.from_bank_addr, from_bank_amount, coin_name.to_string());

        // Calculate to bank amount
        let to_bank_amount = Uint128::new(fee_amount.u128() * state.to_bank_fee.u128() / sto.u128());

        // Create to bank transaction
        let to_bank_transaction_msg = create_transaction(state.to_bank_addr, to_bank_amount, coin_name.to_string());

        // Calculate service fee amount 
        let service_amount = Uint128::new(fee_amount.u128() * state.service_fee.u128() / sto.u128());

        // Create service transaction 
        let servive_transaction_msg = create_transaction(state.service_addr, service_amount, coin_name.to_string());


        if fee_amount == Uint128::new(0) && _info.funds[0].amount.is_zero() {
            return Err(ContractError::InvalidZeroAmount {});
        } else {
            Ok(
                Response::new()
                .add_submessage(SubMsg::new(servive_transaction_msg))
                .add_submessage(SubMsg::new(from_bank_transaction))
                .add_submessage(SubMsg::new(to_bank_transaction_msg))
                .add_message(to_transaction_msg)
                .add_attribute("action", "transfer")
    
            )
        }
       
    }
}

fn create_transaction(to_address: Addr, amount: Uint128, denom: String ) -> BankMsg {
    let transaction = BankMsg::Send {
        to_address: to_address.clone().into(),
        amount: vec![Coin {
            denom: denom,
            amount: amount,
        }],
    };
    return transaction;
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCurrentFeeState {} => to_binary(&query_fee_state(deps)?),
        QueryMsg::GetConfigState {} => to_binary(&query_config_state(deps)?),
     }
}

fn query_fee_state(deps: Deps) -> StdResult<GetCurrentFeeStateResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(GetCurrentFeeStateResponse { fee: state.fee.u128() })
}

fn query_config_state(deps: Deps) -> StdResult<GetConfigStateResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(GetConfigStateResponse {
        fee: state.fee.u128(),
        from_bank_addr: state.from_bank_addr,
        from_bank_fee: state.from_bank_fee.u128(),
        to_bank_addr: state.to_bank_addr,
        to_bank_fee: state.to_bank_fee.u128(),
        service_addr: state.service_addr,
        service_fee: state.service_fee.u128(),
        owner: state.owner
    })
}

#[cfg(test)]
mod tests {

    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary, Addr};

    const FROM_BANK: &str = "from_bank";
    const TO_BANK: &str = "to_bank";
    const SERVICE_ADDR: &str = "service_addr";
    const ADMIN: &str = "ADMIN";
    const NATIVE_DENOM: &str = "stake";

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();
        
        let msg = InstantiateMsg { fee: 1u128, from_bank_addr: Addr::unchecked(FROM_BANK), from_bank_fee: 20u128, to_bank_addr: Addr::unchecked(TO_BANK), to_bank_fee: 40u128, service_addr: Addr::unchecked(SERVICE_ADDR), service_fee: 40u128 };
        let info = mock_info(ADMIN, &coins(1000, "stake"));
    
        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCurrentFeeState{}).unwrap();
        let value: GetCurrentFeeStateResponse = from_binary(&res).unwrap();
        assert_eq!(1u128, value.fee);
    }

    #[test]
    fn update_config() {
        let mut deps = mock_dependencies();

        let msg = ExecuteMsg::UpdateConfig { fee: 10u128, from_bank_addr: Addr::unchecked(FROM_BANK), from_bank_fee: 30u128, to_bank_addr: Addr::unchecked(TO_BANK), to_bank_fee: 35u128, service_addr: Addr::unchecked(SERVICE_ADDR), service_fee: 35u128 };
        let info = mock_info(ADMIN, &coins(2, NATIVE_DENOM));
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let info = mock_info("anyone", &coins(2, "stake"));
        let msg = ExecuteMsg::UpdateOnlyFee {fee: 24u128};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // should increase counter by 1
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCurrentFeeState {}).unwrap();
        let value: GetConfigStateResponse = from_binary(&res).unwrap();
        assert_eq!(24u128, value.fee);
    }

    #[test]
    fn reset() {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg { fee: 10u128, from_bank_addr: Addr::unchecked(FROM_BANK), from_bank_fee: 30u128, to_bank_addr: Addr::unchecked(TO_BANK), to_bank_fee: 35u128, service_addr: Addr::unchecked(SERVICE_ADDR), service_fee: 35u128};
        let info = mock_info(ADMIN, &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let unauth_info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::UpdateConfig { fee: 10u128, from_bank_addr: Addr::unchecked(FROM_BANK), from_bank_fee: 30u128, to_bank_addr: Addr::unchecked(TO_BANK), to_bank_fee: 35u128, service_addr: Addr::unchecked(SERVICE_ADDR), service_fee: 35u128};
        let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
        match res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // should now be 5
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetConfigState{}).unwrap();
        let value: GetConfigStateResponse = from_binary(&res).unwrap();
        assert_eq!(Addr::unchecked(FROM_BANK), value.from_bank_addr);
    }
}
