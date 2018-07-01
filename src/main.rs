#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]

extern crate ring;
extern crate serde_json;

mod votes;
use votes::Vote;
use votes::MessageType;
use votes::{ signature, SIG };

use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;
use serde_json::{ Value, Error };



fn main() {

    let user_i = SIG {
        user: &String::from("i"),
        vote: Vote::Value(33),
        message: MessageType::CERT,
        signature: signature(&String::from("idasdf"))
    };
    // let user_i = SIG::new(&"i", Vote::Value(1) MessageType::CERT);
    let user_j = SIG::new(&"j", Vote::Value(33),  MessageType::SOFT);
    let user_k = SIG::new(&"k", Vote::Value(33), MessageType::CERT);
    let user_l = SIG::new(&"l", Vote::Value(33), MessageType::NEXT);
    let user_m = SIG::new(&"m", Vote::Value(33),  MessageType::NEXT);
    let user_n = SIG::new(&"n", Vote::Value(44),  MessageType::NEXT);
    let user_o = SIG::new(&"o", Vote::NullVote,  MessageType::NEXT);
    let user_p = SIG::new(&"p", Vote::NullVote,  MessageType::NEXT);
    let user_q = SIG::new(&"q", Vote::NullVote,  MessageType::NEXT);
    let user_r = SIG::new(&"r", Vote::NullVote,  MessageType::NEXT);

    let users = vec![
        &user_i, &user_j, &user_k, &user_l, &user_m,
        &user_n, &user_o, &user_p, &user_q, &user_r,
    ];
    for &user in &users {
        println!("User {}: {:?}", &user.user, &user);
    }

    // STEP 1: Value Proposal
    let vt = BA1(2, users);
    // println!("v: {}\tsig: {}", v, sig);
}




fn BA1<'a>(p: u32, users: Vec<&SIG>) -> (Vote, u32) {
    //! Byzantine Agreement Protocol

    let votes: Vec<&Vote> = users.iter().map(|&user: &&SIG| &user.vote).collect();
    let voteCounter = vote_counter(votes);
    println!("\nVoteCounter: {:?}\n", voteCounter);
    let majority_vote = majority_vote(voteCounter);

    // STEP 1: [Value Proposal]
    if majority_votes_next_null(p, &users) {
        println!("\nMajority voted: {:?} {:?} times ", majority_vote.0, majority_vote.1);
        // then i proposes vi, which he propagates together with his period p credential;
        return majority_vote
    } else {
        println!("\nMajority voted: {:?} {:?} times ", majority_vote.0, majority_vote.1);
        // then i proposes v, which he propagates together with his period p credential.
        return majority_vote
    }

}


fn majority_votes_next_null(p: u32, users: &Vec<&SIG>) -> bool {
    //! Description: checks if 2*t+1 (t: malicious nodes) votes are
    //!     votes from previous period p-1 are (next-vote, NullVote)
    //! Params:
    //!     p: period
    //!     votes: vector of peer votes from previous period p-1
    let t = 1; // Number of Malicious Nodes
    // HOW do you know how big t should be?
    let mut next_null_vote_count = 0;
    for user in users {
        // counter all (Next-vote, NullVotes)
        match (&user.vote, &user.message) {
            (Vote::NullVote, MessageType::NEXT) => next_null_vote_count += 1,
            _ => continue,
        }
    }
    // Page 4, Jing Chen, Sergey Gorbunov, Silvio Micali, Georgios Vlachos
    // If p=1 or (p >= 2 AND i has received 2t+1 next-votes for ⊥ NullVote in period p-1)
    match (p, next_null_vote_count) {
        (1, _) => true,
        (_, nnvc) => {
            if nnvc > 2*t {
                println!("\nOver 2*t nodes voted NEXT:");
                println!("next_null_vote_count: {:?}\t t: {:?}", next_null_vote_count, t);
                true
            } else { false }
        },
    }
}


fn vote_counter<'a>(votes: Vec<&Vote>) -> HashMap<Vote, u32> {
    //! Creates a Hashmap of votes, and their counts
    let mut counter: HashMap<Vote, u32> = HashMap::new();
    use Vote::*;
    for v in votes {
        // iterate and count votes for each value
        match v {
            Vote::Value(n) => *counter.entry(Value(*n)).or_insert(0) += 1,
            _ => *counter.entry(NullVote).or_insert(0) += 1,
        }
    }
    counter
}



fn maxHashMap<K, V>(hash_map: HashMap<K, V>) -> (K, V)
where K: Hash + Eq + Debug + Default, V: Ord + Debug + Default {
    //! DESCRIPTION: Return the (Key, Value) pair with the largest value in the hash_map
    let mut maxKey: K = K::default();
    let mut maxVal: V = V::default();
    for (key, value) in hash_map {
        if value > maxVal {
            maxKey = key;
            maxVal = value;
        }
    }
    (maxKey, maxVal)
}
fn majority_vote(voteCounts: HashMap<Vote, u32>) -> (Vote, u32) {
    //! DESCRIPTION:
    //!     Check if user i received 2t + 1 next-votes for ⊥ (NullVote) in period p - 1
    //!     count number of NullVotes, return majority: v or NullVote
    let (maxVoteKey, maxVoteVal) = maxHashMap(voteCounts);
    // let totalVotes  = voteCounts.iter().map(|(k, &v)| v).fold(0, |acc, i| acc+i);
    // let countValue = votes.iter().filter(|&n| *n != &Vote::NullVote).count() as i32;
    // let countNull = votes.iter().filter(|&n| *n == &Vote::NullVote).count() as i32;
    (maxVoteKey, maxVoteVal)
}




