use soroban_sdk::{contracttype, Address, String};

use crate::multisig::MultisigConfig;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Contributor(Address),
    GitHubIndex(String),
    MultisigConfig,
    Proposal(u64),
    NextProposalId,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContributorData {
    pub address: Address,
    pub github_handle: String,
    pub reputation_score: u64,
    pub registered_timestamp: u64,
}
