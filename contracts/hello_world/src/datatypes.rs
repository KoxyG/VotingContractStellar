#![no_std]
use soroban_sdk::{contracttype, Address, String};


#[contracttype]
#[derive(Clone, PartialEq, Eq)]
pub enum Datakey {
    Proposal(u32),
    VoteCount(u32),
    ProposalCount(u32),
    VoterInfo(Address),   
}

#[contracttype]
#[derive(Clone, PartialEq, Eq)]
pub struct Proposal {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub vote_count: u32,
}


#[contracttype]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VoterInfo {
    pub has_voted: bool,
    pub voted_for: u32,
}