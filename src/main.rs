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
    let user_j = SIG::new(&"j", Vote::Value(33), MessageType::SOFT);
    let user_k = SIG::new(&"k", Vote::Value(33), MessageType::CERT);
    let user_l = SIG::new(&"l", Vote::Value(33), MessageType::NEXT);
    let user_m = SIG::new(&"m", Vote::Value(33), MessageType::NEXT);
    let user_n = SIG::new(&"n", Vote::Value(44), MessageType::NEXT);
    let user_o = SIG::new(&"o", Vote::NullVote,  MessageType::NEXT);
    let user_p = SIG::new(&"p", Vote::NullVote,  MessageType::NEXT);
    let user_q = SIG::new(&"q", Vote::NullVote,  MessageType::NEXT);
    let user_r = SIG::new(&"r", Vote::NullVote,  MessageType::NEXT);
    let users = vec![
        user_i, user_j, user_k, user_l, user_m,
        user_n, user_o, user_p, user_q, user_r,
    ];
    for user in &users {
        println!("User {}: {:?}", &user.user, &user);
    }
    // STEP 1: Value Proposal
    let (propagated_vote, user_signature) = algorand_agreement(2, users, String::from("i"));
    println!("Propagated Vote: {:?}\tSignature: {}", propagated_vote, user_signature);
}




fn algorand_agreement<'a>(p: u32, users: Vec<SIG>, user_id: String) -> (Vote, String) {
    //! Byzantine Agreement Protocol
    //!     Params:
    //!         p: period
    //!         users: vector of other users's SIG messages (user, vote, message, signature)
    //!         user_id: user's id
    // let user: Vec<&SIG> = users.iter().filter(|user| user.user == user_id).collect();
    let user = users.iter()
        .filter(|user| user.user == user_id)
        .collect::<Vec<&SIG>>()[0];

    let votes: Vec<Vote> = users.iter().map(|sig| sig.vote).collect();

    let voteCounter = vote_counter(&votes);

    let (majority_vote, majority_vote_count) = majority_vote(&voteCounter);

    println!("\nUser: {:?}\nVoteCounter: {:?}", user, voteCounter);

    // STEP 1: [Value Proposal]
    if majority_votes_next_null(p, &users) {
        println!("\nMajority voted: {:?} {:?} times, user {} propagates: {:?}",
                 majority_vote, majority_vote_count, &user_id, &user.vote);
        // then i proposes vi, which he propagates together with his period p credential;
        return (user.vote, user.signature.clone())
    } else {
        println!("\nMajority voted: {:?} {:?} times", majority_vote, majority_vote_count);
        // then i proposes v, which he propagates together with his period p credential.
        return (majority_vote, user.signature.clone())
    }
}


fn majority_votes_next_null(p: u32, users: &Vec<SIG>) -> bool {
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
            if nnvc > 2*t+1 {
                println!("\nOver 2*t+1 nodes voted NEXT:");
                println!("next_null_vote_count: {:?}\t t: {:?}", next_null_vote_count, t);
                true
            } else {
                false
            }
        }
    }
}



fn vote_counter(votes: &Vec<Vote>) -> HashMap<Vote, u32> {
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


fn majority_vote(voteCounter: &HashMap<Vote, u32>) -> (Vote, u32) {
    //! DESCRIPTION:
    //!     Check if user i received 2t + 1 next-votes for ⊥ (NullVote) in period p - 1
    //!     count number of NullVotes, return majority: v or NullVote
    //! DESCRIPTION: Return the (Key, Value) pair with the largest value in the hash_map
    let mut maxKey = Vote::NullVote;
    let mut maxVal = 0;
    for (key, value) in voteCounter {
        if value > &maxVal {
            maxKey = *key;
            maxVal = *value;
        }
    }
    (maxKey, maxVal)
}



// fn maxHashMap<K, V>(hash_map: HashMap<K, V>) -> (HashMap<K, V>, K, V)
// where K: Hash + Eq + Debug + Default, V: Ord + Debug + Default {
//     //! DESCRIPTION: Return the (Key, Value) pair with the largest value in the hash_map
//     let mut maxKey: K = K::default();
//     let mut maxVal: V = V::default();
//     for (key, value) in hash_map {
//         if value > maxVal {
//             maxKey = key;
//             maxVal = value;
//         }
//     }
//     let totalVotes  = voteCounts.iter().map(|(k, &v)| v).fold(0, |acc, i| acc+i);
//     let countValue = votes.iter().filter(|&n| *n != &Vote::NullVote).count() as i32;
//     let countNull = votes.iter().filter(|&n| *n == &Vote::NullVote).count() as i32;
//     // Ideally avoid taking ownership, just borrow
//     (hash_map, maxKey, maxVal)
// }

