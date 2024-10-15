#![no_std]
use soroban_sdk::{contract, Address, contractimpl, String, Env, Vec};
use crate::datatypes::{Proposal, VoterInfo, Datakey};

#[contract]
pub struct VotingContract;

#[contractimpl]
impl VotingContract {

    // get a proposal by id
    pub fn get_proposal(env: &Env, proposal_id: u32) -> Proposal {
        env.storage().instance().get(&Datakey::Proposal(proposal_id))
            .unwrap_or_else(|| panic!("Proposal not found"))
    }


    // get the number of proposals
    pub fn get_proposal_count(env: &Env) -> u32 {
        env.storage().instance().get(&Datakey::ProposalCount(0)).unwrap_or(0)
    }

    // create a new proposal
    pub fn create_proposal(env: &Env, title: String, description: String) -> u32 {
        let proposal_count: u32 = Self::get_proposal_count(env);
        let new_count: u32 = proposal_count + 1;
        let proposal = Proposal {
            id: new_count,
            title,
            description,
            vote_count: 0,
        };
        env.storage().instance().set(&Datakey::Proposal(new_count), &proposal);
        env.storage().instance().set(&Datakey::ProposalCount(0), &new_count);
        new_count
    }

    // vote for a proposal
    pub fn vote(env: &Env, voter: Address, proposal_id: u32) {
        let mut proposal: Proposal = Self::get_proposal(env, proposal_id);
        let mut voter_info: VoterInfo = env.storage().instance().get(&Datakey::VoterInfo(voter.clone()))
            .unwrap_or(VoterInfo {
                has_voted: false,
                voted_for: 0,
            });
        if voter_info.has_voted {
            panic!("Already voted");
        }
        proposal.vote_count += 1;
        voter_info.has_voted = true;
        voter_info.voted_for = proposal_id;

        env.storage().instance().set(&Datakey::Proposal(proposal_id), &proposal);
        env.storage().instance().set(&Datakey::VoterInfo(voter.clone()), &voter_info);
    }

    // get all proposals
    pub fn get_all_proposals(env: &Env) -> Vec<Proposal> {
        let proposal_count: u32 = Self::get_proposal_count(env);
        let mut proposals = Vec::new(env);
        for i in 1..=proposal_count {
            let proposal: Proposal = Self::get_proposal(env, i);
            proposals.push_back(proposal);
        }
        proposals
    }

    // get the winning proposal
    // pub fn get_winning_proposal(env: &Env) -> Proposal {
    //     let proposal_count: u32 = Self::get_proposal_count(env);
    //     let mut winning_proposal = Proposal {
    //         id: 0,
    //         title: String::from_str(env, ""),
    //         description: String::from_str(env, ""),
    //         vote_count: 0,
    //     };
    //     for i in 1..=proposal_count {
    //         let proposal: Proposal = Self::get_proposal(env, i);
    //         if proposal.vote_count > winning_proposal.vote_count {
    //             winning_proposal = proposal;
    //         }
    //     }
    //     winning_proposal
    // }
}

mod test;
mod datatypes;