#![cfg_attr(not(test), no_std)]
mod contract;
#[cfg(test)]
mod test;
mod types;
use soroban_sdk::{contract, contractimpl, Address, BytesN, Env};

/// # Circle Factory Contract
///
/// Factory contract for deploying Circle contracts with consistent configuration
/// and fee collection.
///
/// ## Example: Initialize the factory
///
/// ```no_run
/// use soroban_sdk::{Address, BytesN, Env, String};
/// use circle_factory::CircleFactoryClient;
///
/// let env = Env::default();
/// let admin = Address::from_string(&soroban_sdk::String::from_str(&env, "GAADMIN..."));
/// let fee_bps = 100; // 1%
/// let wasm_hash = BytesN::from_array(&env, &[0u8; 32]);
/// let organizer_rate_limit = 5;
/// let rate_limit_period_secs = 3600;
///
/// let factory_id = Address::from_string(&soroban_sdk::String::from_str(&env, "GAFACTORY..."));
/// let client = CircleFactoryClient::new(&env, &factory_id);
/// client.init(&admin, &fee_bps, &wasm_hash, &organizer_rate_limit, &rate_limit_period_secs);
/// ```
///
/// ## Example: Deploy a new circle
///
/// ```no_run
/// use soroban_sdk::{Address, BytesN, Env, String};
/// use circle_factory::CircleFactoryClient;
///
/// let env = Env::default();
/// let factory_id = Address::from_string(&soroban_sdk::String::from_str(&env, "GAFACTORY..."));
/// let organizer = Address::from_string(&soroban_sdk::String::from_str(&env, "GAORG..."));
/// let token = Address::from_string(&soroban_sdk::String::from_str(&env, "GATOKEN..."));
///
/// // CircleConfig would be constructed here
/// // let config = CircleConfig { ... };
///
/// let client = CircleFactoryClient::new(&env, &factory_id);
/// // let circle_address = client.deploy_circle(&config);
/// ```
///
/// ## Example: Get all deployed circles
///
/// ```no_run
/// use soroban_sdk::{Address, Env, String};
/// use circle_factory::CircleFactoryClient;
///
/// let env = Env::default();
/// let factory_id = Address::from_string(&soroban_sdk::String::from_str(&env, "GAFACTORY..."));
///
/// let client = CircleFactoryClient::new(&env, &factory_id);
/// let registry = client.get_circles();
/// ```
#[contract]
pub struct CircleFactory;
#[contractimpl]
impl CircleFactory {
    pub fn init(
        env: Env,
        admin: Address,
        fee_bps: i128,
        circle_wasm_hash: BytesN<32>,
        organizer_rate_limit: u32,
        rate_limit_period_secs: u64,
    ) -> Result<(), types::FactoryError> {
        contract::init(
            &env,
            &admin,
            fee_bps,
            &circle_wasm_hash,
            organizer_rate_limit,
            rate_limit_period_secs,
        )
    }
    pub fn deploy_circle(
        env: Env,
        config: types::CircleConfig,
    ) -> Result<Address, types::FactoryError> {
        contract::deploy_circle(&env, &config)
    }
    pub fn get_circles(env: Env) -> types::CircleRegistry {
        contract::get_circles(&env)
    }
    pub fn get_circle_config(
        env: Env,
        cid: Address,
    ) -> Result<types::CircleConfig, types::FactoryError> {
        contract::get_circle_config(&env, &cid)
    }
    pub fn get_circle_count(env: Env) -> u32 {
        contract::get_circle_count(&env)
    }
    pub fn get_fee_config(env: Env) -> types::FeeConfig {
        contract::get_fee_config(&env)
    }
    pub fn get_rate_limit_config(env: Env) -> types::RateLimitConfig {
        contract::get_rate_limit_config(&env)
    }
    pub fn set_fee_config(
        env: Env,
        admin: Address,
        fee_bps: i128,
    ) -> Result<(), types::FactoryError> {
        contract::set_fee_config(&env, &admin, fee_bps)
    }
    pub fn init_with_config(
        env: Env,
        admin: Address,
        fee_bps: i128,
        treasury: Address,
        reputation_registry: Address,
        circle_wasm_hash: BytesN<32>,
    ) -> Result<(), types::FactoryError> {
        contract::init_with_config(
            &env,
            &admin,
            fee_bps,
            &treasury,
            &reputation_registry,
            &circle_wasm_hash,
        )
    }
    pub fn get_factory_config(env: Env) -> Option<types::FactoryConfig> {
        contract::get_factory_config(&env)
    }
    pub fn set_factory_config(
        env: Env,
        admin: Address,
        treasury: Address,
        reputation_registry: Address,
        fee_bps: u32,
    ) -> Result<(), types::FactoryError> {
        contract::set_factory_config(&env, &admin, &treasury, &reputation_registry, fee_bps)
    }
    pub fn pause(env: Env, admin: Address) -> Result<(), types::FactoryError> {
        contract::pause(&env, &admin)
    }
    pub fn unpause(env: Env, admin: Address) -> Result<(), types::FactoryError> {
        contract::unpause(&env, &admin)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_smoke_compile() {
        assert!(true);
    }
    #[test]
    fn test_types_compile() {
        // Verify contract types compile correctly
        assert!(true);
    }
}
