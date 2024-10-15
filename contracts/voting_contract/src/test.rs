#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, Env};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, VotingContract);
    let client = VotingContractClient::new(&env, &contract_id);

    // Test creating proposals
    let proposal1_id = client.create_proposal(&String::from_str(&env, "Cook"), &String::from_str(&env, "We need to cook"));
    let proposal2_id = client.create_proposal(&String::from_str(&env, "Sleep"), &String::from_str(&env, "We need to rest"));
    assert_eq!(proposal1_id, 1);
    assert_eq!(proposal2_id, 2);

    // Test getting proposal count
    assert_eq!(client.get_proposal_count(), 2);

    // Test getting a specific proposal
    let proposal1 = client.get_proposal(&1);
    assert_eq!(proposal1.title, String::from_str(&env, "Cook"));
    assert_eq!(proposal1.description, String::from_str(&env, "We need to cook"));
    assert_eq!(proposal1.vote_count, 0);

    // Test getting all proposals
    let all_proposals = client.get_all_proposals();
    assert_eq!(all_proposals.len(), 2);


    // Generate voter addresses
    let voter1 = Address::generate(&env);
    let voter2 = Address::generate(&env);
    let voter3 = Address::generate(&env);

    // Test voting
    client.vote(&voter1, &1);
    client.vote(&voter2, &1);
    client.vote(&voter3, &2);

    
    // Verify vote counts for both proposals
    let updated_proposal1 = client.get_proposal(&1);
    let updated_proposal2 = client.get_proposal(&2);
    assert_eq!(updated_proposal1.vote_count, 2);
    assert_eq!(updated_proposal2.vote_count, 1);


    // Test getting winning proposal after voting
    // let final_winning_proposal = client.get_winning_proposal();
    // assert_eq!(final_winning_proposal.id, 1);
    // assert_eq!(final_winning_proposal.vote_count, 2);

}