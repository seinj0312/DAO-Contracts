use cw_tokenfactory_issuer::{msg::StatusInfo, ContractError};
use osmosis_test_tube::Account;

use crate::test_env::{
    test_query_over_default_limit, test_query_within_default_limit, TestEnv, TokenfactoryIssuer,
};

#[test]
fn set_freezer_performed_by_contract_owner_should_pass() {
    let env = TestEnv::default();
    let owner = &env.test_accs[0];
    let non_owner = &env.test_accs[1];

    env.cw_tokenfactory_issuer
        .set_freezer(&non_owner.address(), true, owner)
        .unwrap();

    let is_freezer = env
        .cw_tokenfactory_issuer
        .query_is_freezer(&env.test_accs[1].address())
        .unwrap()
        .status;

    assert!(is_freezer);

    env.cw_tokenfactory_issuer
        .set_freezer(&non_owner.address(), false, owner)
        .unwrap();

    let is_freezer = env
        .cw_tokenfactory_issuer
        .query_is_freezer(&env.test_accs[1].address())
        .unwrap()
        .status;

    assert!(!is_freezer);
}

#[test]
fn set_freezer_performed_by_non_contract_owner_should_fail() {
    let env = TestEnv::default();
    let non_owner = &env.test_accs[1];

    let err = env
        .cw_tokenfactory_issuer
        .set_freezer(&non_owner.address(), true, non_owner)
        .unwrap_err();

    assert_eq!(
        err,
        TokenfactoryIssuer::execute_error(ContractError::Unauthorized {})
    );
}

#[test]
fn set_freezer_to_false_should_remove_it_from_storage() {
    let env = TestEnv::default();
    let owner = &env.test_accs[0];

    let mut sorted_addrs = env
        .test_accs
        .iter()
        .map(|acc| acc.address())
        .collect::<Vec<_>>();
    sorted_addrs.sort();

    env.cw_tokenfactory_issuer
        .set_freezer(&sorted_addrs[0], true, owner)
        .unwrap();
    env.cw_tokenfactory_issuer
        .set_freezer(&sorted_addrs[1], true, owner)
        .unwrap();

    assert_eq!(
        env.cw_tokenfactory_issuer
            .query_freezer_allowances(None, None)
            .unwrap()
            .freezers,
        vec![
            StatusInfo {
                address: sorted_addrs[0].clone(),
                status: true
            },
            StatusInfo {
                address: sorted_addrs[1].clone(),
                status: true
            }
        ]
    );

    env.cw_tokenfactory_issuer
        .set_freezer(&sorted_addrs[1], false, owner)
        .unwrap();

    assert_eq!(
        env.cw_tokenfactory_issuer
            .query_freezer_allowances(None, None)
            .unwrap()
            .freezers,
        vec![StatusInfo {
            address: sorted_addrs[0].clone(),
            status: true
        },]
    );

    assert!(
        !env.cw_tokenfactory_issuer
            .query_is_freezer(&sorted_addrs[1])
            .unwrap()
            .status
    );
}

#[test]
fn freeze_by_freezer_should_pass() {
    let env = TestEnv::default();
    let owner = &env.test_accs[0];
    let non_owner = &env.test_accs[1];

    env.cw_tokenfactory_issuer
        .set_freezer(&non_owner.address(), true, owner)
        .unwrap();
    env.cw_tokenfactory_issuer.freeze(true, non_owner).unwrap();

    // should be frozen after set true
    assert!(
        env.cw_tokenfactory_issuer
            .query_is_frozen()
            .unwrap()
            .is_frozen
    );

    env.cw_tokenfactory_issuer.freeze(false, non_owner).unwrap();

    // should be unfrozen after set false
    assert!(
        !env.cw_tokenfactory_issuer
            .query_is_frozen()
            .unwrap()
            .is_frozen
    );
}

#[test]
fn freeze_by_non_freezer_should_fail() {
    let env = TestEnv::default();
    let owner = &env.test_accs[0];
    let err = env.cw_tokenfactory_issuer.freeze(true, owner).unwrap_err();

    assert_eq!(
        err,
        TokenfactoryIssuer::execute_error(ContractError::Unauthorized {})
    );
}

#[test]
fn query_freezer_within_default_limit() {
    test_query_within_default_limit::<StatusInfo, _, _>(
        |(_, addr)| StatusInfo {
            address: addr.to_string(),
            status: true,
        },
        |env| {
            move |allowance| {
                let owner = &env.test_accs[0];
                env.cw_tokenfactory_issuer
                    .set_freezer(&allowance.address, true, owner)
                    .unwrap();
            }
        },
        |env| {
            move |start_after, limit| {
                env.cw_tokenfactory_issuer
                    .query_freezer_allowances(start_after, limit)
                    .unwrap()
                    .freezers
            }
        },
    );
}

#[test]
fn query_freezer_over_default_limit() {
    test_query_over_default_limit::<StatusInfo, _, _>(
        |(_, addr)| StatusInfo {
            address: addr.to_string(),
            status: true,
        },
        |env| {
            move |allowance| {
                let owner = &env.test_accs[0];
                env.cw_tokenfactory_issuer
                    .set_freezer(&allowance.address, true, owner)
                    .unwrap();
            }
        },
        |env| {
            move |start_after, limit| {
                env.cw_tokenfactory_issuer
                    .query_freezer_allowances(start_after, limit)
                    .unwrap()
                    .freezers
            }
        },
    );
}
