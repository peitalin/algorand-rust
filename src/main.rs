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
        message: MessageType::NEXT,
        signature: signature(&String::from("idasdf"))
    };
    // // NEXT demo
    // let user_j = Sig::new(&"j", Vote::Value(33), MessageType::NEXT);
    // let user_k = Sig::new(&"k", Vote::Value(33), MessageType::NEXT);
    // let user_l = Sig::new(&"l", Vote::Value(33), MessageType::NEXT);
    // let user_m = Sig::new(&"m", Vote::Value(33), MessageType::NEXT);
    // let user_n = Sig::new(&"n", Vote::Value(33), MessageType::SOFT);
    // let user_o = Sig::new(&"o", Vote::Value(33), MessageType::SOFT);
    // let user_p = Sig::new(&"p", Vote::NullVote,  MessageType::NEXT);
    // let user_q = Sig::new(&"q", Vote::NullVote,  MessageType::NEXT);
    // let user_r = Sig::new(&"r", Vote::NullVote,  MessageType::NEXT);
    // let user_s = Sig::new(&"s", Vote::NullVote,  MessageType::NEXT);
    // SOFT demo
    let user_j = Sig::new(&"j", Vote::Value(33), MessageType::SOFT);
    let user_k = Sig::new(&"k", Vote::Value(33), MessageType::SOFT);
    let user_l = Sig::new(&"l", Vote::Value(33), MessageType::SOFT);
    let user_m = Sig::new(&"m", Vote::Value(33), MessageType::SOFT);
    let user_n = Sig::new(&"n", Vote::Value(33), MessageType::SOFT);
    let user_o = Sig::new(&"o", Vote::Value(33), MessageType::SOFT);
    let user_p = Sig::new(&"p", Vote::NullVote,  MessageType::SOFT);
    let user_q = Sig::new(&"q", Vote::NullVote,  MessageType::SOFT);
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
    //! DESCRIPTION:
    //!     Algorand's Byzantine Agreement Protocol
    //!     Page 4: Jing Chen, Sergey Gorbunov, Silvio Micali, Georgios Vlachos (2018)
    //! PARAMS:
    //!     p: period
    //!     users: vector of other users's Sig messages (user, vote, message, signature)
    //!     user_i: user's Sig
    let votes: Vec<Vote> = users.iter().map(|sig| sig.vote).collect();

    let vote_message_counts: HashMap<MessageType, HashMap<Vote, u32>> = vote_message_counter(&users);

    let (majority_message, majority_vote, majority_message_vote_count) = calc_majority_vote(&vote_message_counts);
    println!("\nMajority Vote Observed:\n\tMajority message: {:?}\n\tMajority vote: {:?}\n\tCount: {:?}",
             majority_message, majority_vote, majority_message_vote_count);

    let t = 1; // Number of malicious nodes
    // How do you know how many malicious nodes there are?

    if halting_condition(t, &majority_message, &majority_vote, &majority_message_vote_count) {
        return
    } else {
        println!("No halting condition (majority CERT-vote) encountered, resuming consensus protocol.");
    }

    // STEP 1: [Value Proposal]
    println!("\n[STEP 1: Value Proposal]");
    println!("\tUser original Vote: {:?}", &user_i);
    if (p == 1) || (majority_message == MessageType::NEXT
                    && majority_vote == Vote::NullVote
                    && majority_message_vote_count >= 2*t+1) {
        // If p=1 or (p >= 2 AND i has received 2t+1 next-votes for ⊥ NullVote in period p-1)
        // then i proposes vi, which he propagates together with his period p credential;
        if p == 1 { println!("\nPeriod: 1") }
        /// CODE: network broadcast
        println!("\tUser broadcasts (1a): {:?}", &user_i);
    } else if (p >= 2) && majority_message == MessageType::NEXT
                        && majority_vote != Vote::NullVote
                        && majority_message_vote_count >= 2*t+1 {
        // Else if 􏰀p ≥ 2􏰁 AND 􏰀i has received 2t + 1 next-votes for some value v ̸= ⊥ for period p−1􏰁
        //  i proposes v, which he propagates together with his period p credential.
        user_i.update_vote(majority_vote);
        println!("\tUser updates Vote: {:?}", majority_vote);
        /// CODE: network broadcast
        println!("\tUser broadcasts (1b): {:?}", &user_i);
    } else {
    }

    // STEP 2: [Filtering Step]
    println!("\n[STEP 2: Filtering Step]");
    if (p == 1) || (majority_message == MessageType::NEXT
                    && majority_vote == Vote::NullVote
                    && majority_message_vote_count >= 2*t+1) {
        // If p=1 or (p >= 2 AND i has received 2t+1 next-votes for ⊥ NullVote in period p-1)
        // i identifies himself as leader li,p for period p
        // and soft-votes the value v proposed by li,p;
        user_i.update_message_type(MessageType::SOFT);
        println!("\tUser elects herself as leader, and SOFT-votes: {:?}", &user_i.vote);
    } else if majority_message == MessageType::NEXT
            && majority_vote != Vote::NullVote
            && majority_message_vote_count >= 2*t+1 {
        // STEP 2: [Filtering Step]
        println!("\tUser SOFT-votes observed majority vote: {:?}", &user_i.vote);
        // User i SOFT-votes v, the majority_vote
        user_i.update_message_type(MessageType::SOFT);
    } else {
    }

    // STEP 3: [Certifying Step]
    // If i sees 2t + 1 soft-votes for some value v ̸= ⊥, then i cert-votes v.
    println!("\n[STEP 3: Certifying Step]");
    let mut has_certified_vote = false;
    if majority_message == MessageType::SOFT
        && majority_vote != Vote::NullVote
        && majority_message_vote_count >= 2*t+1 {
        user_i.update_message_type(MessageType::CERT);
        has_certified_vote = true;
        println!("\tUser: {:?} sees SOFT-vote majority, upgrades MessageType to: {:?}", user_i.user, user_i.message);
        println!("\tUser broadcasts (3): {:?}", &user_i);
    } else {
    }


    // STEP 4: [Period's First Finishing Step]
    println!("\n[STEP 4: First Finishing Step]");
    // If i has certified some value v for period p, he next-votes v;
    if has_certified_vote {
        user_i.update_message_type(MessageType::NEXT);
        user_i.update_vote(majority_vote);
        println!("\tUser NEXT-votes (4): {:?}", &user_i.vote);
    } else {
        // Else he next-votes ⊥.
        user_i.update_message_type(MessageType::NEXT);
        user_i.update_vote(Vote::NullVote);
        println!("\tUser broadcasts (4): {:?}", &user_i);
    }

    // STEP 5: [Period's Second Finishing Step]
    println!("\n[STEP 5: Second Finishing Step]");
    if majority_message == MessageType::SOFT
        && majority_message_vote_count >= 2*t+1
        && majority_vote != Vote::NullVote
        && !has_certified_vote {
        // If i sees 2t + 1 soft-votes for some value v ̸= ⊥ for period p
        // and has not next-voted v in Step 4, then i next-votes v.
        user_i.update_message_type(MessageType::NEXT);
        user_i.update_vote(majority_vote);
        println!("\tUser NEXT-votes (4): {:?}", &user_i.vote);
    }

}




fn vote_message_counter<'a>(users: &Vec<Sig>) -> HashMap<MessageType, HashMap<Vote, u32>> {
    //! DESCRIPTION:
    //!     Creates a HashMap of MessageType[Vote], and respective counts
    //! PARAMS:
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



fn calc_majority_vote<'a>(vote_message_counter: &HashMap<MessageType, HashMap<Vote, u32>>) -> (MessageType, Vote, u32) {
    //! DESCRIPTION:
    //!     Check if user i received 2t + 1 next-votes for ⊥ (NullVote) in period p - 1
    //!     count number of NullVotes, return majority: v or NullVote
    //! PARAMS:
    //!     vote_message_counter: reference to a HashMap of a HashMap: MessageType[Vote]
    //! RETURN: Returns the (Key, Value) pair with the largest value in the hash_map
    let mut maxMsg = &MessageType::SOFT;
    let mut maxVote = &Vote::NullVote;
    let mut maxVal = 0;
    for (message_type, vote_dict) in vote_message_counter {
        for (voteKey, val) in vote_dict {
            if val > &maxVal {
                maxMsg = message_type;
                maxVote = voteKey;
                maxVal = *val
            }
        }
    }
    (maxMsg.clone(), maxVote.clone(), maxVal)
}


fn maxHashMap<K, V>(hash_map: HashMap<K, V>) -> (K, V)
where K: Hash + Eq + Debug + Default, V: Ord + Debug + Default {
    //! DESCRIPTION:
    //!     Return the (Key, Value) pair of the entry with the largest value in the hash_map
    //! PARAMS:
    //!     hash_map: Generic HashMap
    let mut maxKey: K = K::default();
    let mut maxVal: V = V::default();
    for (key, value) in hash_map {
        if value > maxVal {
            maxKey = key;
            maxVal = value;
        }
    }
    // let totalVotes  = voteCounts.iter().map(|(k, &v)| v).fold(0, |acc, i| acc+i);
    // let countValue = votes.iter().filter(|&n| *n != &Vote::NullVote).count() as i32;
    // let countNull = votes.iter().filter(|&n| *n == &Vote::NullVote).count() as i32;
    // Ideally avoid taking ownership, just borrow
    (maxKey, maxVal)
}



fn halting_condition(t: u32, majority_message: &MessageType, majority_vote: &Vote, majority_message_vote_count: &u32) -> bool {
    // User i HALTS the moment he sees 2t + 1 cert-votes for some value v for the same period p,
    // and sets v to be his output. Those cert-votes form a certificate for v.
    if *majority_message == MessageType::CERT
        && *majority_message_vote_count >= 2*t+1 && *majority_vote != Vote::NullVote {
        println!("User sees 2t + 1 CERT-votes for some value v");
        true
    } else {
        false
    }
}

