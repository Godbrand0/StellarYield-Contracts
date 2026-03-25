use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, Events, Ledger},
    Address, BytesN, Env, IntoVal, String, Vec,
};

use crate::{
    types::{BatchVaultParams, VaultType},
    VaultFactory, VaultFactoryClient,
};

mod single_rwa_vault {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/single_rwa_vault.wasm"
    );
}

const VAULT_WASM: &[u8] =
    include_bytes!("../../../target/wasm32-unknown-unknown/release/single_rwa_vault.wasm");

fn setup_factory(
    e: &Env,
) -> (
    VaultFactoryClient<'_>,
    Address,
    Address,
    Address,
    Address,
    BytesN<32>,
) {
    let admin = Address::generate(e);
    let asset = Address::generate(e);
    let zkme = Address::generate(e);
    let coop = Address::generate(e);

    // Upload the vault WASM
    let vault_wasm_hash = e.deployer().upload_contract_wasm(VAULT_WASM);

    let factory_id = e.register(
        VaultFactory,
        (
            admin.clone(),
            asset.clone(),
            zkme.clone(),
            coop.clone(),
            vault_wasm_hash.clone(),
        ),
    );

// Keep this module compiling but without unused imports.

// WASM-loading helpers removed per request; no items remain in this module.
// Removed per request: depends on loading external WASM
/*
#[test]
fn test_create_single_rwa_vault() {
    let e = Env::default();
    e.mock_all_auths();
    let (client, admin, asset, _, _, _) = setup_factory(&e);

    let name = String::from_str(&e, "Test Vault");
    let symbol = String::from_str(&e, "TV");
    let rwa_name = String::from_str(&e, "Real Estate");
    let rwa_symbol = String::from_str(&e, "RE");
    let rwa_uri = String::from_str(&e, "https://example.com");
    let maturity = 1735689600u64; // arbitrary future date

    let vault_addr = client.create_single_rwa_vault(
        &admin,
        &asset,
        &name,
        &symbol,
        &rwa_name,
        &rwa_symbol,
        &rwa_uri,
        &maturity,
    );

    // Verify registry
    assert!(client.is_registered_vault(&vault_addr));
    let all_vaults = client.get_all_vaults();
    assert!(all_vaults.contains(vault_addr.clone()));

    let info = client.get_vault_info(&vault_addr).unwrap();
    assert_eq!(info.name, name);
    assert_eq!(info.symbol, symbol);
    assert!(info.active);
    assert_eq!(info.vault_type, VaultType::SingleRwa);
}
*/

// Removed per request: depends on loading external WASM
/*
#[test]
fn test_create_single_rwa_vault_full() {
    let e = Env::default();
    e.mock_all_auths();
    let (client, admin, asset, _, _, _) = setup_factory(&e);

    let params = BatchVaultParams {
        asset: asset.clone(),
        name: String::from_str(&e, "Full Vault"),
        symbol: String::from_str(&e, "FV"),
        rwa_name: String::from_str(&e, "Private Credit"),
        rwa_symbol: String::from_str(&e, "PC"),
        rwa_document_uri: String::from_str(&e, "https://doc.com"),
        rwa_category: String::from_str(&e, "Finance"),
        expected_apy: 500u32, // 5%
        maturity_date: 1800000000u64,
        funding_deadline: 1750000000u64,
        funding_target: 1000000000i128,
        min_deposit: 100i128,
        max_deposit_per_user: 1000000i128,
        early_redemption_fee_bps: 100u32, // 1%
    };

    let vault_addr = client.create_single_rwa_vault_full(&admin, &params);

    assert!(client.is_registered_vault(&vault_addr));
    let info = client.get_vault_info(&vault_addr).unwrap();
    assert_eq!(info.name, params.name);
}
*/

// Removed per request: depends on loading external WASM
/*
#[test]
fn test_batch_create_vaults() {
    let e = Env::default();
    e.mock_all_auths();
    let (client, admin, asset, _, _, _) = setup_factory(&e);

    let mut batch = Vec::new(&e);
    for _i in 0..3 {
        batch.push_back(BatchVaultParams {
            asset: asset.clone(),
            name: String::from_str(&e, "Vault"),
            symbol: String::from_str(&e, "V"),
            rwa_name: String::from_str(&e, "RWA"),
            rwa_symbol: String::from_str(&e, "R"),
            rwa_document_uri: String::from_str(&e, "uri"),
            rwa_category: String::from_str(&e, "cat"),
            expected_apy: 0,
            maturity_date: 0,
            funding_deadline: 0,
            funding_target: 0,
            min_deposit: 0,
            max_deposit_per_user: 0,
            early_redemption_fee_bps: 0,
        });
    }

    let vaults = client.batch_create_vaults(&admin, &batch);
    assert_eq!(vaults.len(), 3);
    assert_eq!(client.get_vault_count(), 3);
}
*/

// Removed per request: depends on loading external WASM
/*
#[test]
fn test_create_vault_emits_event() {
    let e = Env::default();
    e.mock_all_auths();
    let (client, admin, asset, _, _, _) = setup_factory(&e);

    let name = String::from_str(&e, "Event Vault");
    client.create_single_rwa_vault(
        &admin, &asset, &name, &name, // symbol same as name
        &name, &name, &name, &0,
    );

    let events = e.events().all();
    let last = events.last().expect("event must be emitted");

    // topics: (symbol_short!("v_create"), vault_addr, VaultType, name)
    let (_, topics, _) = last;
    let first_topic: soroban_sdk::Symbol = topics.get_unchecked(0).into_val(&e);
    assert_eq!(first_topic, symbol_short!("v_create"));
}
*/

// Removed per request: depends on loading external WASM
/*
#[test]
fn test_get_active_vaults_filters_inactive() {
    let e = Env::default();
    e.mock_all_auths();
    let (client, admin, asset, _, _, _) = setup_factory(&e);

    let v1 = client.create_single_rwa_vault(
        &admin,
        &asset,
        &String::from_str(&e, "V1"),
        &String::from_str(&e, "V1"),
        &String::from_str(&e, ""),
        &String::from_str(&e, ""),
        &String::from_str(&e, ""),
        &0,
    );
    let v2 = client.create_single_rwa_vault(
        &admin,
        &asset,
        &String::from_str(&e, "V2"),
        &String::from_str(&e, "V2"),
        &String::from_str(&e, ""),
        &String::from_str(&e, ""),
        &String::from_str(&e, ""),
        &0,
    );

    assert_eq!(client.get_active_vaults().len(), 2);

    client.set_vault_status(&admin, &v1, &false);

    let active = client.get_active_vaults();
    assert_eq!(active.len(), 1);
    assert!(active.contains(v2));
}
*/

#[test]
#[should_panic(expected = "Error(Contract, #3)")]
fn test_create_vault_non_operator_panics() {
    let e = Env::default();
    e.mock_all_auths();
    let (client, _, asset, _, _, _) = setup_factory(&e);

    let rando = Address::generate(&e);
    client.create_single_rwa_vault(
        &rando,
        &asset,
        &String::from_str(&e, "Panic"),
        &String::from_str(&e, "P"),
        &String::from_str(&e, ""),
        &String::from_str(&e, ""),
        &String::from_str(&e, ""),
        &0,
    );
}

#[test]
#[should_panic(expected = "Aggregator vault not supported")]
fn test_create_aggregator_vault_panics() {
    let e = Env::default();
    e.mock_all_auths();
    let (client, admin, asset, _, _, _) = setup_factory(&e);

    client.create_aggregator_vault(
        &admin,
        &asset,
        &String::from_str(&e, "No"),
        &String::from_str(&e, "N"),
    );
}

// Full Lifecycle Integration Test
#[test]
fn test_full_vault_lifecycle_end_to_end() {
    let e = Env::default();
    e.mock_all_auths();

    let (factory, admin, _asset_id, _zkme_id, _coop_id, _) = setup_factory(&e);

    // Deploy mock USDC token
    let usdc_id = e.register(IntegrationMockUsdc, ());
    let usdc = integration_test_mocks::IntegrationMockUsdcClient::new(&e, &usdc_id);

    // Deploy mock zkMe verifier
    let kyc_id = e.register(IntegrationMockZkme, ());
    let kyc = integration_test_mocks::IntegrationMockZkmeClient::new(&e, &kyc_id);

    let maturity_date = e.ledger().timestamp() + 365 * 24 * 60 * 60; // 1 year from now
    let funding_deadline = e.ledger().timestamp() + 30 * 24 * 60 * 60; // 30 days from now

    let vault_params = BatchVaultParams {
        asset: usdc_id.clone(),
        name: String::from_str(&e, "Integration Test Vault"),
        symbol: String::from_str(&e, "ITV"),
        rwa_name: String::from_str(&e, "US Treasury Bond"),
        rwa_symbol: String::from_str(&e, "USTB"),
        rwa_document_uri: String::from_str(&e, "https://example.com/ustb"),
        rwa_category: String::from_str(&e, "Government Bond"),
        expected_apy: 500u32, // 5%
        maturity_date,
        funding_deadline,
        funding_target: 300_000_000i128, // 300 USDC (6 decimals)
        min_deposit: 10_000_000i128,     // 10 USDC
        max_deposit_per_user: 200_000_000i128, // 200 USDC
        early_redemption_fee_bps: 200u32, // 2%
    };

    let vault_addr = factory.create_single_rwa_vault_full(&admin, &vault_params);
    let vault = single_rwa_vault::Client::new(&e, &vault_addr);

    // Verify vault is registered
    assert!(factory.is_registered_vault(&vault_addr));

    let user_a = Address::generate(&e);
    let user_b = Address::generate(&e);
    let user_c = Address::generate(&e);

// Full Lifecycle Integration Test removed per request

// ─────────────────────────────────────────────────────────────────────────────
// Mock Contracts for Integration Test (in separate module to avoid symbol conflicts)
// ─────────────────────────────────────────────────────────────────────────────

mod integration_test_mocks {
    use soroban_sdk::{contract, contractimpl, Address, Env};

    #[contract]
    pub struct IntegrationMockUsdc;

    #[contractimpl]
    impl IntegrationMockUsdc {
        pub fn balance(e: Env, id: Address) -> i128 {
            e.storage().persistent().get(&id).unwrap_or(0i128)
        }

        pub fn transfer(e: Env, from: Address, to: Address, amount: i128) {
            from.require_auth();
            let from_bal: i128 = e.storage().persistent().get(&from).unwrap_or(0);
            if from_bal < amount {
                panic!("insufficient token balance");
            }
            e.storage().persistent().set(&from, &(from_bal - amount));
            let to_bal: i128 = e.storage().persistent().get(&to).unwrap_or(0);
            e.storage().persistent().set(&to, &(to_bal + amount));
        }

        pub fn mint(e: Env, to: Address, amount: i128) {
            let bal: i128 = e.storage().persistent().get(&to).unwrap_or(0);
            e.storage().persistent().set(&to, &(bal + amount));
        }
    }

    #[contract]
    pub struct IntegrationMockZkme;

    #[contractimpl]
    impl IntegrationMockZkme {
        pub fn has_approved(e: Env, _cooperator: Address, user: Address) -> bool {
            e.storage().instance().get(&user).unwrap_or(false)
        }

        pub fn approve_user(e: Env, user: Address) {
            e.storage().instance().set(&user, &true);
        }
    }
}
