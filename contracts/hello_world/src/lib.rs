#![no_std]
use soroban_sdk::{contract, Address, contractimpl, String, Env};
use crate::datatypes::{Proposal, VoterInfo, Datakey};



#[contract]
pub struct VotingContract;



#[contractimpl]
impl VotingContract {

    // create proposal
    pub fn create_proposal(env: Env, title: String, description: String) -> u32 {
        let proposal_count: u32 = env.storage().instance().get(&Datakey::ProposalCount(0)).unwrap_or(0);
        let mut _newCount: u32 = proposal_count + 1;  

        let proposal: datatypes::Proposal  = Proposal {
            id: _newCount,
            title: title,
            description: description,
            vote_count: 0,
        };


        // set proposal
        env.storage().instance().set(&datatypes::Datakey::Proposal(_newCount), &proposal);
        // set proposal count
        env.storage().instance().set(&datatypes::Datakey::ProposalCount(0), &_newCount);
        
        _newCount
    }


    // vote proposal
    pub fn vote(env: Env, voter: Address, proposal_id: u32)  {

        let mut proposal: Proposal = env.storage().instance().get(&Datakey::Proposal(proposal_id)).unwrap_or_else(
        || panic!("Proposal not found")
        );

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


}

mod test;
mod datatypes;
