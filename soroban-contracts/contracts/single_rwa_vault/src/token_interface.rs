//! External contract interface for zkMe verification.
//!
//! Mirrors `IZKMEVerify` from `ISingleRWA_Vault.sol`.

use soroban_sdk::{contractclient, Address, Env};

/// Minimal interface for the zkMe on-chain verification oracle.
#[allow(dead_code)]
#[contractclient(name = "ZkmeVerifyClient")]
pub trait ZkmeVerifyTrait {
    /// Returns true when `cooperator` has approved `user`.
    fn has_approved(env: Env, cooperator: Address, user: Address) -> bool;
}
