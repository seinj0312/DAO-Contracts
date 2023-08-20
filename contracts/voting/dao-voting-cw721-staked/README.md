# `dao-voting-cw721-staked`

This is a basic implementation of an NFT staking contract.

Staked tokens can be unbonded with a configurable unbonding period. Staked balances can be queried at any arbitrary height by external contracts. This contract implements the interface needed to be a DAO DAO [voting module](https://github.com/DA0-DA0/dao-contracts/wiki/DAO-DAO-Contracts-Design#the-voting-module).

`dao-voting-cw721-staked` can be used with existing NFT collections or create new `cw721` or `sg721` collections upon instantiation (with the DAO as admin and `minter`).
