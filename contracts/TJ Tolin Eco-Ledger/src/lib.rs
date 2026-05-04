#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol, symbol_short};

#[contract]
pub struct TolinCarbonTrust;

#[contractimpl]
impl TolinCarbonTrust {
    // Log emissions: Input liters of fuel, returns kg of CO2
    // For this example, 1 liter of diesel = ~2.68 kg of CO2
    pub fn log_emission(env: Env, farmer: Address, fuel_liters: u64) -> u64 {
        farmer.require_auth();

        let coefficient = 268; // Representing 2.68 (scaled by 100)
        let total_co2 = (fuel_liters * coefficient) / 100;

        // Store the farmer's debt in contract storage
        let key = Symbol::new(&env, "debt");
        let current_debt: u64 = env.storage().persistent().get(&farmer).unwrap_or(0);
        env.storage().persistent().set(&farmer, &(current_debt + total_co2));

        total_co2
    }

    // View current carbon debt for a farmer
    pub fn get_debt(env: Env, farmer: Address) -> u64 {
        env.storage().persistent().get(&farmer).unwrap_or(0)
    }
}