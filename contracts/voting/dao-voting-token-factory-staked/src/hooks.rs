use crate::state::HOOKS;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{to_binary, Addr, StdResult, Storage, SubMsg, Uint128, WasmMsg};
use token_bindings::TokenFactoryMsg;

#[cw_serde]
pub enum StakeChangedHookMsg {
    Stake { addr: Addr, amount: Uint128 },
    Unstake { addr: Addr, amount: Uint128 },
}

pub fn stake_hook_msgs(
    storage: &dyn Storage,
    addr: Addr,
    amount: Uint128,
) -> StdResult<Vec<SubMsg<TokenFactoryMsg>>> {
    let msg = to_binary(&StakeChangedExecuteMsg::StakeChangeHook(
        StakeChangedHookMsg::Stake { addr, amount },
    ))?;
    HOOKS.prepare_hooks_custom_msg(storage, |a| {
        let execute = WasmMsg::Execute {
            contract_addr: a.to_string(),
            msg: msg.clone(),
            funds: vec![],
        };
        Ok(SubMsg::<TokenFactoryMsg>::new(execute))
    })
}

pub fn unstake_hook_msgs(
    storage: &dyn Storage,
    addr: Addr,
    amount: Uint128,
) -> StdResult<Vec<SubMsg<TokenFactoryMsg>>> {
    let msg = to_binary(&StakeChangedExecuteMsg::StakeChangeHook(
        StakeChangedHookMsg::Unstake { addr, amount },
    ))?;
    HOOKS.prepare_hooks_custom_msg(storage, |a| {
        let execute = WasmMsg::Execute {
            contract_addr: a.to_string(),
            msg: msg.clone(),
            funds: vec![],
        };
        Ok(SubMsg::<TokenFactoryMsg>::new(execute))
    })
}

// This is just a helper to properly serialize the above message
#[cw_serde]
enum StakeChangedExecuteMsg {
    StakeChangeHook(StakeChangedHookMsg),
}
