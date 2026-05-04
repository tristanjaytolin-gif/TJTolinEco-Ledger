#![cfg(test)]
use super::*;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{Env, Address};

#[test]
fn test_emission_calculation() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TolinCarbonTrust);
    let client = TolinCarbonTrustClient::new(&env, &contract_id);

    let farmer = Address::generate(&env);

    // Mock authentication for the farmer
    env.mock_all_auths();

    // 1. Log 100 liters of fuel
    // Expected: (100 * 268) / 100 = 268 kg CO2
    let emission = client.log_emission(&farmer, &100);
    assert_eq!(emission, 268);

    // 2. Verify storage updated correctly
    let debt = client.get_debt(&farmer);
    assert_eq!(debt, 268);

    // 3. Add more emissions
    client.log_emission(&farmer, &50);
    let total_debt = client.get_debt(&farmer);
    assert_eq!(total_debt, 402); // 268 + 134
}