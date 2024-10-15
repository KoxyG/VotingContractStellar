#![no_std]
use soroban_sdk::{contract, Address, contractimpl, String, Env, Vec};
use crate::datatypes::{Proposal, VoterInfo, Datakey};



#[contract]
pub struct VotingContract;



#[contractimpl]
impl VotingContract {

    // get proposal
    pub fn get_proposal(env: Env, proposal_id: u32) -> Proposal {
        env.storage().instance().get(&Datakey::Proposal(proposal_id)).unwrap_or_else(
        || panic!("Proposal not found")
        )
    }

    // create proposal
    pub fn create_proposal(env: Env, title: String, description: String) -> u32 {
        let proposal_count: u32 = env.storage().instance().get(&Datakey::ProposalCount(0)).unwrap_or(0);
        
        let mut _new_count: u32 = proposal_count + 1;  

        let proposal: datatypes::Proposal  = Proposal {
            id: _new_count,
            title,
            description,
            vote_count: 0,
        };


        // set proposal
        env.storage().instance().set(&datatypes::Datakey::Proposal(_new_count), &proposal);
        // set proposal count
        env.storage().instance().set(&datatypes::Datakey::ProposalCount(0), &_new_count);
        
        _new_count
    }


    // vote proposal
    pub fn vote(env: Env, voter: Address, proposal_id: u32)  {

        let mut proposal: Proposal = Self::get_proposal(env.clone(), proposal_id);

        let mut voter_info: VoterInfo = env.storage().instance().get(&Datakey::VoterInfo(voter.clone())).unwrap_or(
            VoterInfo {
                has_voted: false,
                voted_for: 0,
            }  
        );


        if voter_info.has_voted {
            panic!("Already voted");
        }


        proposal.vote_count += 1;
        voter_info.has_voted = true;
        voter_info.voted_for  = proposal_id;

       
        env.storage().instance().set(&Datakey::Proposal(proposal_id), &proposal);
        env.storage().instance().set(&Datakey::VoterInfo(voter.clone()), &voter_info);
   
    }

    // get all proposals
    pub fn get_all_proposals(env: Env) -> Vec<Proposal> {
        let proposal_count: u32 = env.storage().instance().get(&Datakey::ProposalCount(0)).unwrap_or(0);
        
        let mut proposals = Vec::new(&env);

        for i in 1..proposal_count + 1 {
            let proposal: Proposal = Self::get_proposal(env.clone(), i);
            proposals.push_back(proposal);
        }
        
        proposals
    }


    

}

mod test;
mod datatypes;
