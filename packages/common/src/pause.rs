use soroban_sdk::{contracterror, contractevent, symbol_short, Address, Env};

#[contracterror]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PauseError {
    ContractPaused = 1,
}

/// Emitted when the contract is paused.
#[contractevent(topics = ["paused"])]
#[derive(Clone, Debug)]
pub struct Paused {
    #[topic]
    pub by: Address,
}

/// Emitted when the contract is unpaused.
#[contractevent(topics = ["unpaused"])]
#[derive(Clone, Debug)]
pub struct Unpaused {
    #[topic]
    pub by: Address,
}

pub fn is_paused(env: &Env) -> bool {
    env.storage()
        .instance()
        .get(&symbol_short!("paused"))
        .unwrap_or(false)
}

pub fn pause(env: &Env, admin: &Address) -> Result<(), PauseError> {
    admin.require_auth();
    if is_paused(env) {
        return Ok(()); // Idempotent: already paused, no-op.
    }
    env.storage()
        .instance()
        .set(&symbol_short!("paused"), &true);
    Paused { by: admin.clone() }.publish(env);
    Ok(())
}

pub fn unpause(env: &Env, admin: &Address) -> Result<(), PauseError> {
    admin.require_auth();
    if !is_paused(env) {
        return Ok(()); // Idempotent: already unpaused, no-op.
    }
    env.storage()
        .instance()
        .set(&symbol_short!("paused"), &false);
    Unpaused { by: admin.clone() }.publish(env);
    Ok(())
}

pub fn when_not_paused(env: &Env) -> Result<(), PauseError> {
    if is_paused(env) {
        Err(PauseError::ContractPaused)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::Address as _;
    use soroban_sdk::{contract, Env};

    #[contract]
    struct PauseTestContract;

    fn setup() -> (Env, Address, Address) {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(PauseTestContract, ());
        let admin = Address::generate(&env);
        (env, contract_id, admin)
    }

    #[test]
    fn test_initially_not_paused() {
        let (env, contract_id, _admin) = setup();
        env.as_contract(&contract_id, || {
            assert!(!is_paused(&env));
            assert!(when_not_paused(&env).is_ok());
        });
    }

    #[test]
    fn test_pause_sets_paused_flag() {
        let (env, contract_id, admin) = setup();
        env.as_contract(&contract_id, || {
            pause(&env, &admin).unwrap();
            assert!(is_paused(&env));
            assert_eq!(when_not_paused(&env), Err(PauseError::ContractPaused));
        });
    }

    #[test]
    fn test_pause_is_idempotent() {
        let (env, contract_id, admin) = setup();
        // Each auth-requiring call runs in its own frame: mock auths permit
        // a single require_auth per frame ("frame is already authorized"
        // otherwise). Storage persists across frames.
        env.as_contract(&contract_id, || {
            pause(&env, &admin).unwrap();
        });
        env.as_contract(&contract_id, || {
            // Second pause is a no-op, not an error.
            pause(&env, &admin).unwrap();
            assert!(is_paused(&env));
        });
    }

    #[test]
    fn test_unpause_clears_paused_flag() {
        let (env, contract_id, admin) = setup();
        env.as_contract(&contract_id, || {
            pause(&env, &admin).unwrap();
        });
        env.as_contract(&contract_id, || {
            unpause(&env, &admin).unwrap();
        });
        env.as_contract(&contract_id, || {
            assert!(!is_paused(&env));
            assert!(when_not_paused(&env).is_ok());
        });
    }

    #[test]
    fn test_unpause_is_idempotent() {
        let (env, contract_id, admin) = setup();
        env.as_contract(&contract_id, || {
            // Unpausing a live contract is a no-op, not an error.
            unpause(&env, &admin).unwrap();
            assert!(!is_paused(&env));
        });
    }

    #[test]
    fn test_pause_unpause_full_cycle() {
        let (env, contract_id, admin) = setup();
        for _ in 0..3 {
            env.as_contract(&contract_id, || {
                pause(&env, &admin).unwrap();
            });
            env.as_contract(&contract_id, || {
                assert_eq!(when_not_paused(&env), Err(PauseError::ContractPaused));
            });
            env.as_contract(&contract_id, || {
                unpause(&env, &admin).unwrap();
            });
            env.as_contract(&contract_id, || {
                assert!(when_not_paused(&env).is_ok());
            });
        }
    }
}
