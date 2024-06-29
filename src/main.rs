mod election;
mod voter;
mod zkproof;

use election::Election;
use voter::Voter;

fn main() {
    let mut election = Election::new();

    let voter1 = Voter::new();
    let voter2 = Voter::new();

    election.register_voter(voter1.clone());
    election.register_voter(voter2.clone());

    election.cast_vote(&voter1, "Candidate A".to_string());
    election.cast_vote(&voter2, "Candidate B".to_string());

    let verified_votes = election.verify_votes();
    println!("{}", serde_json::to_string_pretty(&verified_votes).unwrap());
}
