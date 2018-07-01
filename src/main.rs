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
    // Value proposal

    let votes: Vec<&Vote> = users.iter().map(|&user: &&SIG| &user.vote).collect();
    let voteCounts = vote_counter(votes);
    let vt = majority_vote(voteCounts);

    if check_next_null_votes(p, &users) {
        if p > 1 {
        }
        println!("\nMajority voted: {:?} {:?} times ", vt.0, vt.1);
        return vt
    } else {
        // else if p == 2 && !majority_vote()
        println!("\nMajority voted: {:?} {:?} times ", vt.0, vt.1);
        return vt
    }

}


fn check_next_null_votes(p: u32, users: &Vec<&SIG>) -> bool {
    //! Description: checks if 2*t+1 (t: malicious nodes) votes are
    //!     votes from previous period p-1 are (next-vote, NullVote)
    //! Params:
    //!     p: period
    //!     votes: vector of peer votes from previous period p-1
    let t = 1; // Number of Malicious Nodes
    // HOW do you know how big t should be?
    let mut next_null_vote_count = 0;
    for user in users {
        match (&user.vote, &user.message) {
            (Vote::NullVote, MessageType::NEXT) => next_null_vote_count += 1,
            _ => continue,
        }
    }
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
        match v {
            Vote::Value(n) => *counter.entry(Value(*n)).or_insert(0) += 1,
            _ => *counter.entry(NullVote).or_insert(0) += 1,
        }
    }
    println!("\nVoteCounter: {:?}\n", counter);
    counter
}


fn maxHashMap<K, V>(hash_map: HashMap<K, V>) -> (K, V)
where K: Hash + Eq + Debug + Default, V: Ord + Debug + Default {
    //! Return the (Key, Value) pair with the largest value in the hash_map
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
    //! Has user i received 2t + 1 next-votes for ⊥ in period p - 1
    //! count number of NullVotes, return majority: v or NullVote
    let (maxVoteKey, maxVoteVal) = maxHashMap(voteCounts);
    // let totalVotes  = voteCounts.iter().map(|(k, &v)| v).fold(0, |acc, i| acc+i);
    // let countValue = votes.iter().filter(|&n| *n != &Vote::NullVote).count() as i32;
    // let countNull = votes.iter().filter(|&n| *n == &Vote::NullVote).count() as i32;
    (maxVoteKey, maxVoteVal)
}




