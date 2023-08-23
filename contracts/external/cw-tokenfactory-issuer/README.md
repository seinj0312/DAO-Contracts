# `cw-tokenfactory-issuer`

Forked from [osmosis-labs/cw-tokenfactory-issuer](https://github.com/osmosis-labs/cw-tokenfactory-issuer).

This repo contains a set of contracts that when used in conjunction with the x/tokenfactory module in Osmosis, Juno, and many other chains will enable a centrally issued stablecoin with many features:
- Creating a new Token Factory token or using an existing one
- Granting and revoking allowances for the minting and burning of tokens
- Updating token metadata
- Freezing and unfreezing transfers, with a whitelist to allow some address to continue to transfer
- Force transfering tokens via the contract owner
- Updating the contract owner or Token Factory admin

It is intended to work on multiple chains supporting Token Factory, and has been tested on Juno Network and Osmosis.

The contract has an owner (which can be removed or updated via `ExecuteMsg::ChangeContractOwner {}`), but it can delegate capabilities to other acccounts. For example, the owner of a contract can delegate minting allowance of 1000 tokens to a new address.

The contract is also the admin of the newly created Token Factory denom. For minting and burning, users then interact with the contract using its own ExecuteMsgs which trigger the contract's access control logic, and the contract then dispatches tokenfactory sdk.Msgs from its own contract account.

NOTE: this contract contains a `SudoMsg::BlockBeforeSend` hook that allows for the blacklisting of specific accounts as well as the freezing of all transfers if necessary. This feature is not enabled on every chain using Token Factory, and so blacklisting and freezing features are disabled if `MsgBeforeSendHook` is not supported. DAOs wishing to leverage these features on chains after support is added can call `ExecuteMsg::SetBeforeSendHook {}`.

## Instantiation
When instantiating `cw-tokenfactory-issuer`, you can either create a `new_token` or an `existing_token`.

### Creating a new Token Factory token
To create a new Token Factory token, simply instantiate the contract with a `subdenom`, this will create a new contract as well as a token with a denom formatted as `factory/{contract_address}/{subdenom}`.

Example instantiate message:

``` json
{
  "new_token": {
    "subdenom": "test"
  }
}
```

All other updates can be preformed afterwards via this contract's `ExecuteMsg` enum. See `src/msg.rs` for available methods.

### Using an Existing Token
You can also instantiate this contract with an existing token, however most features will not be available until the previous Token Factory admin transfers admin rights to the instantiated contract and optionally calls `ExecuteMsg::SetBeforeSendHook {}` to enable dependent features.

Example instantiate message:
``` json
{
  "existing_token": {
    "denom": "factory/{contract_address}/{subdenom}"
  }
}
```
