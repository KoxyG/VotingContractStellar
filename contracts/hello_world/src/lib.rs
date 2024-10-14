#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, String,  symbol_short, vec, Env, Symbol, Vec};

const VOTE_ID: Symbol = symbol_short!("VOTE_ID");
const PRO_ID: Symbol = symbol_short!("PRO_ID");



#[contract]
pub struct VotingContract;


#[contracttype]
pub struct Proposal {
    id: u32,
    title: String,
    description: String,

    vote_count: u32,
}


#[contracttype]
pub struct VoterInfo {
    has_voted: bool,
    voted_for: u32,
}


#[contractimpl]
impl VotingContract {

    // create proposal
    pub fn create_proposal(env: Env, title: String, description: String) -> u32 {
        
        let mut proposal_id: u32 = env.storage().instance().get(&PRO_ID).unwrap_or(0);

        proposal_id += 1;  

        let proposal = Proposal {
            id: proposal_id,
            title: title,
            description: description,
            vote_count: 0,
        };


        env.storage().instance().set(&PRO_ID, &proposal_id);
        
        proposal_id
    }


    // vote proposal
    pub fn vote(env: Env, proposal_id: u32) {

        let mut voter_info = env.storage().instance().get(&VOTE_ID).unwrap_or(
            VoterInfo {
                has_voted: false,
                voted_for: 0,
            }  
        );

        if voter_info.has_voted {
            panic!("Already voted");
        }

        let mut proposal: Proposal = env.storage().instance().get(&proposal_id).unwrap();

        proposal.vote_count += 1;
        voter_info.has_voted = true;
        voter_info.voted_for  = proposal_id;

       
        env.storage().instance().set(&proposal_id, &proposal);
        env.storage().instance().set(&VOTE_ID, &voter_info);
    }

}

mod test;
