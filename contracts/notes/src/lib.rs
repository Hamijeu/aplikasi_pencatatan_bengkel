#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec, prng::Prng};

#[contracttype]
#[derive(Clone, Debug)]
pub struct ServiceLog {
    pub id: u64,
    pub date: String,
    pub part_name: String,
    pub description: String,
    pub mileage: u32,
}

const LOG_DATA: Symbol = symbol_short!("LOG_DATA");

#[contract]
pub struct MaintenanceContract;

#[contractimpl]
impl MaintenanceContract {
    pub fn get_logs(env: Env) -> Vec<ServiceLog> {
        return env.storage().instance().get(&LOG_DATA).unwrap_or(Vec::new(&env));
    }

    pub fn create_log(
        env: Env,
        date: String,
        part_name: String,
        description: String,
        mileage: u32,
    ) -> String {
        let mut logs: Vec<ServiceLog> = env.storage().instance().get(&LOG_DATA).unwrap_or(Vec::new(&env));
        
        let log = ServiceLog {
            id: env.prng().gen::<u64>(),
            date: date,
            part_name: part_name,
            description: description,
            mileage: mileage,
        };
        
        logs.push_back(log);
        env.storage().instance().set(&LOG_DATA, &logs);
        
        return String::from_str(&env, "Service log created successfully");
    }

    pub fn delete_log(env: Env, id: u64) -> String {
        let mut logs: Vec<ServiceLog> = env.storage().instance().get(&LOG_DATA).unwrap_or(Vec::new(&env));

        for i in 0..logs.len() {
            if logs.get(i).unwrap().id == id {
                logs.remove(i);

                env.storage().instance().set(&LOG_DATA, &logs);
                return String::from_str(&env, "Service log deleted successfully");
            }
        }

        return String::from_str(&env, "Service log not found")
    }
}

mod test;