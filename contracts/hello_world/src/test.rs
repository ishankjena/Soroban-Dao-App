#![cfg(test)]

use super::*;
use soroban_sdk::{symbol_short, vec, Env};

#[test]
fn test_record_votes() {
    let env = Env::default();
    // let contract_id = env.register_contract(None, HelloContract);
    // let client = HelloContractClient::new(&env, &contract_id);
    let user = Address::random(&env);
        let selected = YES;

        // Call the record_votes function
        let result = VoteContract::record_votes(env.clone(), user.clone(), selected);

        // Assert that the result is "Recorded"
        assert_eq!(result, symbol_short!("Recorded"));
}


#[test]
fn test_view_poll() {
    let env = EnvBuilder::default().build();

    // Initialize poll data
    let poll = Poll {
        yes: 10,
        no: 5,
        total: 15,
    };
    env.storage().instance().set(&POLL, &poll);

    // Call the view_poll function
    let result = VoteContract::view_poll(env.clone());

    // Assert that the result matches the initialized poll data
    assert_eq!(result, poll);
}


#[test]
fn test_view_voter() {
    let env = EnvBuilder::default().build();
    let voter = Address::random(&env);

    // Initialize voter record
    let record = Record {
        selected: YES,
        votes: 3,
        time: 123456789,
    };
    env.storage().instance().set(&Registry::Record(voter.clone()), &record);

    // Call the view_voter function
    let result = VoteContract::view_voter(env.clone(), voter.clone());

    // Assert that the result matches the initialized voter record
    assert_eq!(result, record);
}