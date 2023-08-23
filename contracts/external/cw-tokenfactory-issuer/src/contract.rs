use std::convert::TryInto;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, SubMsg, SubMsgResult,
};
use cosmwasm_std::{CosmosMsg, Reply};
use cw2::set_contract_version;
use osmosis_std::types::osmosis::tokenfactory::v1beta1::{
    MsgCreateDenom, MsgCreateDenomResponse, MsgSetBeforeSendHook,
};
use token_bindings::{TokenFactoryMsg, TokenFactoryQuery};

use crate::error::ContractError;
use crate::execute;
use crate::hooks;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg, SudoMsg};
use crate::queries;
use crate::state::{BEFORE_SEND_HOOK_FEATURES_ENABLED, DENOM, IS_FROZEN, OWNER};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-tokenfactory-issuer";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const CREATE_DENOM_REPLY_ID: u64 = 1;
const BEFORE_SEND_HOOK_REPLY_ID: u64 = 2;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<TokenFactoryQuery>,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response<TokenFactoryMsg>, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    OWNER.save(deps.storage, &info.sender)?;
    IS_FROZEN.save(deps.storage, &false)?;

    match msg {
        InstantiateMsg::NewToken { subdenom } => {
            Ok(Response::new()
                .add_attribute("action", "instantiate")
                .add_attribute("owner", info.sender)
                .add_attribute("subdenom", subdenom.clone())
                .add_submessage(
                    // create new denom, if denom is created successfully,
                    // set beforesend listener to this contract on reply
                    SubMsg::reply_on_success(
                        <CosmosMsg<TokenFactoryMsg>>::from(MsgCreateDenom {
                            sender: env.contract.address.to_string(),
                            subdenom,
                        }),
                        CREATE_DENOM_REPLY_ID,
                    ),
                ))
        }
        InstantiateMsg::ExistingToken { denom } => {
            DENOM.save(deps.storage, &denom)?;

            // BeforeSendHook cannot be set with existing tokens
            // features that rely on it are disabled
            BEFORE_SEND_HOOK_FEATURES_ENABLED.save(deps.storage, &false)?;

            Ok(Response::new()
                .add_attribute("action", "instantiate")
                .add_attribute("owner", info.sender)
                .add_attribute("denom", denom))
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<TokenFactoryQuery>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<TokenFactoryMsg>, ContractError> {
    match msg {
        // Executive Functions
        ExecuteMsg::Mint { to_address, amount } => {
            execute::mint(deps, env, info, to_address, amount)
        }
        ExecuteMsg::Burn {
            amount,
            from_address: address,
        } => execute::burn(deps, env, info, amount, address),
        ExecuteMsg::Blacklist { address, status } => {
            execute::blacklist(deps, info, address, status)
        }
        ExecuteMsg::Whitelist { address, status } => {
            execute::whitelist(deps, info, address, status)
        }
        ExecuteMsg::Freeze { status } => execute::freeze(deps, info, status),
        ExecuteMsg::ForceTransfer {
            amount,
            from_address,
            to_address,
        } => execute::force_transfer(deps, env, info, amount, from_address, to_address),

        // Admin functions
        ExecuteMsg::ChangeTokenFactoryAdmin { new_admin } => {
            execute::change_tokenfactory_admin(deps, info, new_admin)
        }
        ExecuteMsg::ChangeContractOwner { new_owner } => {
            execute::change_contract_owner(deps, info, new_owner)
        }
        ExecuteMsg::SetMinterAllowance { address, allowance } => {
            execute::set_minter(deps, info, address, allowance)
        }
        ExecuteMsg::SetBurnerAllowance { address, allowance } => {
            execute::set_burner(deps, info, address, allowance)
        }
        ExecuteMsg::SetBeforeSendHook {} => execute::set_before_send_hook(deps, env, info),
        ExecuteMsg::SetBlacklister { address, status } => {
            execute::set_blacklister(deps, info, address, status)
        }
        ExecuteMsg::SetWhitelister { address, status } => {
            execute::set_whitelister(deps, info, address, status)
        }
        ExecuteMsg::SetFreezer { address, status } => {
            execute::set_freezer(deps, info, address, status)
        }
        ExecuteMsg::SetDenomMetadata { metadata } => {
            execute::set_denom_metadata(deps, env, info, metadata)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(
    deps: DepsMut<TokenFactoryQuery>,
    _env: Env,
    msg: SudoMsg,
) -> Result<Response, ContractError> {
    match msg {
        SudoMsg::BlockBeforeSend { from, to, amount } => {
            hooks::beforesend_hook(deps, from, to, amount)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<TokenFactoryQuery>, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::IsFrozen {} => to_binary(&queries::query_is_frozen(deps)?),
        QueryMsg::Denom {} => to_binary(&queries::query_denom(deps)?),
        QueryMsg::Owner {} => to_binary(&queries::query_owner(deps)?),
        QueryMsg::BurnAllowance { address } => {
            to_binary(&queries::query_burn_allowance(deps, address)?)
        }
        QueryMsg::BurnAllowances { start_after, limit } => {
            to_binary(&queries::query_burn_allowances(deps, start_after, limit)?)
        }
        QueryMsg::MintAllowance { address } => {
            to_binary(&queries::query_mint_allowance(deps, address)?)
        }
        QueryMsg::MintAllowances { start_after, limit } => {
            to_binary(&queries::query_mint_allowances(deps, start_after, limit)?)
        }
        QueryMsg::IsBlacklisted { address } => {
            to_binary(&queries::query_is_blacklisted(deps, address)?)
        }
        QueryMsg::Blacklistees { start_after, limit } => {
            to_binary(&queries::query_blacklistees(deps, start_after, limit)?)
        }
        QueryMsg::IsBlacklister { address } => {
            to_binary(&queries::query_is_blacklister(deps, address)?)
        }
        QueryMsg::BlacklisterAllowances { start_after, limit } => to_binary(
            &queries::query_blacklister_allowances(deps, start_after, limit)?,
        ),
        QueryMsg::IsWhitelisted { address } => {
            to_binary(&queries::query_is_whitelisted(deps, address)?)
        }
        QueryMsg::Whitelistees { start_after, limit } => {
            to_binary(&queries::query_whitelistees(deps, start_after, limit)?)
        }
        QueryMsg::IsWhitelister { address } => {
            to_binary(&queries::query_is_whitelister(deps, address)?)
        }
        QueryMsg::WhitelisterAllowances { start_after, limit } => to_binary(
            &queries::query_whitelister_allowances(deps, start_after, limit)?,
        ),
        QueryMsg::IsFreezer { address } => to_binary(&queries::query_is_freezer(deps, address)?),
        QueryMsg::FreezerAllowances { start_after, limit } => to_binary(
            &queries::query_freezer_allowances(deps, start_after, limit)?,
        ),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(
    deps: DepsMut<TokenFactoryQuery>,
    _env: Env,
    _msg: MigrateMsg,
) -> Result<Response, ContractError> {
    // Set contract to version to latest
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::new().add_attribute("action", "migrate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(
    deps: DepsMut<TokenFactoryQuery>,
    env: Env,
    msg: Reply,
) -> Result<Response<TokenFactoryMsg>, ContractError> {
    match msg.id {
        CREATE_DENOM_REPLY_ID => {
            let MsgCreateDenomResponse { new_token_denom } = msg.result.try_into()?;
            DENOM.save(deps.storage, &new_token_denom)?;

            // SetBeforeSendHook to this contract
            // this will trigger sudo endpoint before any bank send
            // which makes blacklisting / freezing possible
            let msg_set_beforesend_hook: CosmosMsg<TokenFactoryMsg> = MsgSetBeforeSendHook {
                sender: env.contract.address.to_string(),
                denom: new_token_denom.clone(),
                cosmwasm_address: env.contract.address.to_string(),
            }
            .into();

            Ok(Response::new()
                .add_attribute("denom", new_token_denom)
                .add_submessage(SubMsg::reply_always(
                    msg_set_beforesend_hook,
                    BEFORE_SEND_HOOK_REPLY_ID,
                )))
        }
        BEFORE_SEND_HOOK_REPLY_ID => match msg.result {
            SubMsgResult::Ok(_) => {
                // Enable features with BeforeSendHook requirement
                BEFORE_SEND_HOOK_FEATURES_ENABLED.save(deps.storage, &true)?;

                Ok(Response::new().add_attribute("extra_features", "enabled"))
            }
            SubMsgResult::Err(_) => {
                // MsgSetBeforeSendHook failed, disable extra features that require it
                BEFORE_SEND_HOOK_FEATURES_ENABLED.save(deps.storage, &false)?;

                Ok(Response::new().add_attribute("extra_features", "disabled"))
            }
        },
        _ => Err(ContractError::UnknownReplyId { id: msg.id }),
    }
}
