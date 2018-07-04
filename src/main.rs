#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]

extern crate ring;
extern crate serde_json;

mod votes;
use votes::Vote;
use votes::MessageType;
use votes::{ signature, Sig };

use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;
use serde_json::{ Value, Error };


fn main() {
    let user_i = Sig {
        user: &String::from("i"),
        vote: Vote::Value(22),
        message: MessageType::CERT,
        signature: signature(&String::from("idasdf"))
    };
    let user_j = Sig::new(&"j", Vote::Value(33), MessageType::NEXT);
    let user_k = Sig::new(&"k", Vote::Value(33), MessageType::NEXT);
    let user_l = Sig::new(&"l", Vote::Value(33), MessageType::NEXT);
    let user_m = Sig::new(&"m", Vote::Value(33), MessageType::NEXT);
    let user_n = Sig::new(&"n", Vote::Value(33), MessageType::SOFT);
    let user_o = Sig::new(&"o", Vote::Value(33), MessageType::SOFT);
    let user_p = Sig::new(&"p", Vote::NullVote,  MessageType::NEXT);
    let user_q = Sig::new(&"q", Vote::NullVote,  MessageType::NEXT);
    let user_r = Sig::new(&"r", Vote::NullVote,  MessageType::NEXT);
    let user_s = Sig::new(&"s", Vote::NullVote,  MessageType::NEXT);
    let users = vec![
        user_j, user_k, user_l, user_m, user_n,
        user_o, user_p, user_q, user_r, user_s,
    ];
    for user in &users {
        println!("User {}: {:?}", &user.user, &user);
    }
    // STEP 1: Value Proposal
    let p = 2;
    let user_sig = algorand_agreement(p, users, user_i);
    println!("\nPropagated Vote: {:?}", &user_sig);
}




fn algorand_agreement<'a>(p: u32, users: Vec<Sig>, mut user_i: Sig<'a>) {
    //! Byzantine Agreement Protocol
    //!     Params:
    //!         p: period
    //!         users: vector of other users's Sig messages (user, vote, message, signature)
    //!         user_id: user's id

    let votes: Vec<Vote> = users.iter().map(|sig| sig.vote).collect();

    let vote_message_counts: HashMap<MessageType, HashMap<Vote, u32>> = vote_message_counter(&users);

    let mvote = majority_vote_fn(&vote_message_counts);

    let (majority_message, majority_vote, majority_message_vote_count) = mvote;
    println!("\nMajority message: {:?}\nMajority vote: {:?}\nCount: {:?}",
             &mvote.0, &mvote.1, mvote.2);

    // Page 4: Jing Chen, Sergey Gorbunov, Silvio Micali, Georgios Vlachos (2018)
    let t = 1; // Number of malicious nodes
    let next_null_vote_count = calc_next_null_vote_count(&vote_message_counts);
    let next_val_vote_count = calc_next_val_vote_count(&vote_message_counts);

    // STEP 1: [Value Proposal]
    println!("\n[STEP 1: Value Proposal]");
    // Define a dictionary lookup macro
    println!("\tUser original Vote: {:?}", &user_i);
    if (p == 1) || next_null_vote_count >= 2*t+1 {
        // If p=1 or (p >= 2 AND i has received 2t+1 next-votes for ⊥ NullVote in period p-1)
        // then i proposes vi, which he propagates together with his period p credential;
        if p == 1 { println!("\nPeriod: 1") }
        /// CODE: network broadcast
        println!("\tUser broadcasts (1): {:?}", &user_i);
    } else if (p >= 2) && next_val_vote_count >= 2*t+1 {
        // Else if 􏰀p ≥ 2􏰁 AND 􏰀i has received 2t + 1 next-votes for some value v ̸= ⊥ for period p−1􏰁
        //  i proposes v, which he propagates together with his period p credential.
        user_i.update_vote(*majority_vote);
        println!("\tUser updates Vote: {:?}", *majority_vote);
        /// CODE: network broadcast
        println!("\tUser broadcasts (2): {:?}", &user_i);
    } else {
    }


    // STEP 2: [Filtering Step]
    println!("\n[STEP 2: Filtering Step]");
    if (p == 1) || next_null_vote_count >= 2*t+1 {
        // If p=1 or (p >= 2 AND i has received 2t+1 next-votes for ⊥ NullVote in period p-1)
        // i identifies himself as leader li,p for period p
        // and soft-votes the value v proposed by li,p;
        user_i.update_message_type(MessageType::SOFT);
        println!("\tUser elects herself as leader, and SOFT-votes: {:?}", &user_i.vote);
    } else if (p >= 2) && majority_message_t(&majority_message, mvote.2, t) && !majority_vote_t(&majority_vote, mvote.2, t) {
        // STEP 2: [Filtering Step]
        println!("\tUser SOFT-votes: {:?}", &user_i.vote);
        // User i SOFT-votes v, the majority_vote
        user_i.update_message_type(MessageType::SOFT);
    } else {
    }


    // STEP 3: [Certifying Step]
    // If i sees 2t + 1 soft-votes for some value v ̸= ⊥, then i cert-votes v.
    println!("\n[STEP 3: Certifying Step]");
    if majority_message_t(&mvote.0, mvote.2, t) {
        user_i.update_message_type(MessageType::CERT);
        println!("\tUser: {:?} sees SOFT-vote majority, upgrades MessageType to: {:?}", user_i.user, user_i.message);
    }


    // STEP 4: [Period's First Finishing Step]
    println!("\n[STEP 4: First Finishing Step]");
    // If i has certified some value v for period p, he next-votes v;
    if true {
    } else {
        // Else he next-votes ⊥.
    }

    // STEP 5: [Period's Second Finishing Step]
    println!("\n[STEP 5: Second Finishing Step]");
    // If i sees 2t + 1 soft-votes for some value v ̸= ⊥ for period p
    // and has not next-voted v in Step 4, then i next-votes v.a

}


fn calc_next_null_vote_count(vote_message_counts: &HashMap<MessageType, HashMap<Vote, u32>>) -> u32 {
    let next_null_vote_count = *vote_message_counts
        .get(&MessageType::NEXT).unwrap()
        .get(&Vote::NullVote).unwrap();
    next_null_vote_count
}

fn calc_next_val_vote_count(vote_message_counts: &HashMap<MessageType, HashMap<Vote, u32>>) -> u32 {
    let mut next_val_vote_count = 0;
    let mut next_val_key = Vote::Value(0);
    for (key, val) in vote_message_counts.get(&MessageType::NEXT).unwrap() {
        if val > &next_val_vote_count {
            next_val_vote_count = *val;
            next_val_key = *key;
        }
    }
    next_val_vote_count
}

fn majority_message_t(message: &MessageType, message_count: u32, t: u32) -> bool {
    //! Description: checks if 2*t+1 (t: malicious nodes) messages
    //!     from previous period p-1 are SOFT-messages
    //! Params:
    //!     users: vector of peer votes (Sig) from previous period p-1
    //!     t: Number of Malicious Nodes. HOW do you know how big t is?
    if message_count > 2*t { true } else { false }
}

fn majority_vote_t(vote: &Vote, vote_count: u32, t: u32) -> bool {
    //! Description: checks if 2*t+1 (t: malicious nodes) votes
    //!     from previous period p-1 are votes for NullVote
    //! Params:
    //!     users: vector of peer votes (Sig) from previous period p-1
    //!     t: Number of Malicious Nodes. HOW do you know how big t is?
    if vote_count > 2*t { true } else { false }
}




fn vote_message_counter<'a>(users: &Vec<Sig>) -> HashMap<MessageType, HashMap<Vote, u32>> {
    //! Description:
    //!     Creates a HashMap of MessageType[Vote], and respective counts
    //! Params:
    //!     users: vector of peer votes (Sig) from previous period p-1
    let mut messageDict: HashMap<MessageType, HashMap<Vote, u32>> = HashMap::new();
    let mut voteDictSOFT: HashMap<Vote, u32> = HashMap::new();
    let mut voteDictCERT: HashMap<Vote, u32> = HashMap::new();
    let mut voteDictNEXT: HashMap<Vote, u32> = HashMap::new();
    // HashMap::new() returns address, need to deference to mutate
    use MessageType::{ SOFT, CERT, NEXT };
    use Vote::{ Value, NullVote };
    for u in users {
        // iterate and count votes for each value.
        match (&u.message, &u.vote ) {
            (SOFT, Vote::Value(n)) => *voteDictSOFT.entry(Vote::Value(*n)).or_insert(0) += 1,
            (CERT, Vote::Value(n)) => *voteDictCERT.entry(Vote::Value(*n)).or_insert(0) += 1,
            (NEXT, Vote::Value(n)) => *voteDictNEXT.entry(Vote::Value(*n)).or_insert(0) += 1,
            (SOFT, Vote::NullVote) => *voteDictSOFT.entry(NullVote).or_insert(0) += 1,
            (CERT, Vote::NullVote) => *voteDictCERT.entry(NullVote).or_insert(0) += 1,
            (NEXT, Vote::NullVote) => *voteDictNEXT.entry(NullVote).or_insert(0) += 1,
        }
    }
    messageDict.insert(SOFT, voteDictSOFT);
    messageDict.insert(CERT, voteDictCERT);
    messageDict.insert(NEXT, voteDictNEXT);
    messageDict
}

// fn next_val_vote_counter(vote_message_counts: HashMap<>) {
//     let mut next_val_vote_count = 0;
//     let mut next_val_key = Vote::Value(0);
//     for (key, val) in vote_message_counts.get(&MessageType::NEXT).unwrap() {
//         if val > &next_val_vote_count {
//             next_val_vote_count = *val;
//             next_val_key = *key;
//         }
//     }
//     next_val_vote_count
// }


// fn vote_counter(votes: &Vec<Vote>) -> HashMap<Vote, u32> {
//     //! Creates a Hashmap of votes, and their counts
//     let mut counter: HashMap<Vote, u32> = HashMap::new();
//     use Vote::*;
//     for v in votes {
//         // iterate and count votes for each value
//         match v {
//             Vote::Value(n) => *counter.entry(Value(*n)).or_insert(0) += 1,
//             _ => *counter.entry(NullVote).or_insert(0) += 1,
//         }
//     }
//     counter
// }


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
//     // let totalVotes  = voteCounts.iter().map(|(k, &v)| v).fold(0, |acc, i| acc+i);
//     // let countValue = votes.iter().filter(|&n| *n != &Vote::NullVote).count() as i32;
//     // let countNull = votes.iter().filter(|&n| *n == &Vote::NullVote).count() as i32;
//     // Ideally avoid taking ownership, just borrow
//     (hash_map, maxKey, maxVal)
// }

fn majority_vote_fn<'a>(vote_message_counter: &HashMap<MessageType, HashMap<Vote, u32>>) -> (&MessageType, &Vote, u32) {
    //! DESCRIPTION:
    //!     Check if user i received 2t + 1 next-votes for ⊥ (NullVote) in period p - 1
    //!     count number of NullVotes, return majority: v or NullVote
    //! DESCRIPTION: Return the (Key, Value) pair with the largest value in the hash_map
    let mut maxMsg = &MessageType::SOFT;
    let mut maxVote = &Vote::NullVote;
    let mut maxVal = 0;
    for (message_type, vote_dict) in vote_message_counter {
        for (voteKey, val) in vote_dict {
            if val > &maxVal {
                println!("New Largest Count -- {:?}: {:?}: {:?}", message_type, voteKey, val);
                maxMsg = message_type;
                maxVote = voteKey;
                maxVal = *val
            }
        }
    }
    (maxMsg, maxVote, maxVal)
}




