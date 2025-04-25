#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Address, String, Vec, log};

#[contracttype]
#[derive(Clone)]
pub struct ActivityEntry {
    pub user: Address,
    pub activity: String,
    pub duration_minutes: u32,
    pub calories_burned: u32,
    pub timestamp: u64,
}

#[contracttype]
pub enum ActivityKey {
    ByUser(Address),
}

#[contract]
pub struct DecentralizedFitnessTracker;

#[contractimpl]
impl DecentralizedFitnessTracker {
    // Log a fitness activity
    pub fn log_activity(
        env: Env,
        user: Address,
        activity: String,
        duration_minutes: u32,
        calories_burned: u32,
    ) {
        user.require_auth();

        let mut activities: Vec<ActivityEntry> =
            env.storage().instance().get(&ActivityKey::ByUser(user.clone())).unwrap_or(Vec::new(&env));

        let entry = ActivityEntry {
            user: user.clone(),
            activity,
            duration_minutes,
            calories_burned,
            timestamp: env.ledger().timestamp(),
        };

        activities.push_back(entry);
        env.storage().instance().set(&ActivityKey::ByUser(user), &activities);
        log!(&env, "Activity logged");
    }

    // Get activity history for a user
    pub fn get_activities(env: Env, user: Address) -> Vec<ActivityEntry> {
        env.storage().instance().get(&ActivityKey::ByUser(user)).unwrap_or(Vec::new(&env))
    }
}
