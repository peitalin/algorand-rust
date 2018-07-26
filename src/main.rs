#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_mut)]

extern crate ring;
extern crate serde_json;
use serde_json::{ Value, Error };
use std::collections::HashMap;
use std::fmt::Debug;

mod votes;
use votes::{
    // Types
    Vote,
    MessageType,
    Sig,
    MajorityVote,
    // functions
    signature,
    gossip,
};

mod config;
use config::{
    Config,
    get_config,
};



fn main() {

    let args: Vec<String> = std::env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Please supply an argument: 'values' or 'nullvotes'.");
        std::process::exit(1);
    });

    let mut users: Vec<Sig> = gossip(&config.gossip_type);
    let users_init: Vec<Sig> = users.clone();
    // mut users: mutable vector: each sig mutates
    // user_iter is purely for iteration only.
    // Can't mutate vector which you are iterating over

    // Begin Algorand Rounds
    let mut p = 1;
    let mut t = config.num_malnodes;
    let mut halt = false;
    let mut user_halt = false;
    let mut cert_count = 0;

    while !halt && p < 10 {
        println!("\n================== BEGIN ROUND {} ===================", &p);
        cert_count = 0;
        for (i, user_i) in users.clone().into_iter().enumerate() {
            println!("\n------ USER: {:?} ------", &user_i.user);
            let (user_halt, new_user) = algorand_agreement(p, t, &users, user_i);
            if user_halt {
                cert_count = cert_count + 1;
            }
            let replace_user = users.remove(0); // pop the 1st index
            if replace_user != new_user {
                println!("\n\t>>> REPLACED USER: {:?}", replace_user);
                println!("\t>>> WITH USER: {:?}", new_user);
            }
            users.push(new_user);
        }
        println!("\n================ END ROUND {} =================\n", &p);
        if cert_count == users.len() {
            println!("Algorand halted early successfully in round: {}.", &p);
            println!("All users produced certificate votes.");
            halt = true;
        }
        p = p + 1;
        print_sigs(String::from("Initial Sigs"), &users_init);
        print_sigs(String::from("Ending Sigs"), &users);
    }
    println!("Number of Malicious nodes assumed: {:?}", t);
}



fn algorand_agreement<'a>(p: u32, t: u32, users: &Vec<Sig>, mut user_i: Sig<'a>) -> (bool, Sig<'a>) {
    //! DESCRIPTION:
    //!     Algorand's Byzantine Agreement Protocol
    //!     Page 4: Jing Chen, Sergey Gorbunov, Silvio Micali, Georgios Vlachos (2018)
    //! PARAMS:
    //!     p: period
    //!     t: number of malicious nodes (How to know how many malicious-nodes exist?)
    //!     users: vector of other users's Sig messages (user, vote, message, signature)
    //!     user_i: user's Sig

    let majority = MajorityVote::new(&users);

    println!("Majority Vote Observed:\n\tMajority message: {:?}\n\tMajority vote: {:?}\n\tCount: {:?}",
             majority.message, majority.vote, majority.count);
    println!("User:\n\tmessage: {:?}\n\tvote: {:?}", user_i.message, user_i.vote);

    if p == 1 { println!("\nPeriod: 1") }
    if halting_condition(t, &majority) {
        user_i.update_vote(majority.vote);
        user_i.update_message_type(majority.message);
        return (true, user_i)
    } else {
        println!("No halting condition (majority CERT-vote) encountered, resuming consensus protocol.");
    }


    // STEP 1: [Value Proposal]
    if (p == 1) || (majority.message == MessageType::NEXT
                    && majority.vote == Vote::NullVote
                    && majority.count >= 2*t+1) {
        // If p=1 or (p >= 2 AND i has received 2t+1 next-votes for ⊥ NullVote in period p-1)
        // then i proposes vi, which he propagates together with his period p credential;
        /// CODE: network broadcast
        println!("\t[Step 1a] User broadcasts: {:?}", &user_i);
    } else if (p >= 2) && majority.message == MessageType::NEXT
                        && majority.vote != Vote::NullVote
                        && majority.count >= 2*t+1 {
        // Else if p ≥ 2 AND i has received 2t + 1 next-votes for some value v ̸= ⊥ for period p - 1
        //  i proposes v, which he propagates together with his period p credential.
        user_i.update_vote(majority.vote);
        /// CODE: network broadcast
        println!("\t[Step 1b] User broadcasts: {:?}", &user_i);
    } else {
    }

    // STEP 2: [Filtering Step]
    if (p == 1) || (majority.message == MessageType::NEXT
                    && majority.vote == Vote::NullVote
                    && majority.count >= 2*t+1) {
        // If p=1 or (p >= 2 AND i has received 2t+1 next-votes for ⊥ NullVote in period p-1)
        // i identifies himself as leader li,p for period p
        // and soft-votes the value v proposed by li,p;
        user_i.update_message_type(MessageType::SOFT);
        println!("\t[Step 2a] User elects herself as leader, and SOFT-votes: {:?}", &user_i.vote);
    } else if majority.message == MessageType::NEXT
            && majority.vote != Vote::NullVote
            && majority.count >= 2*t+1 {
        // User i SOFT-votes v, the majority.vote
        println!("\t[Step 2b] User SOFT-votes observed majority vote: {:?}", &user_i.vote);
        user_i.update_message_type(MessageType::SOFT);
    } else {
    }

    // STEP 3: [Certifying Step]
    // If i sees 2t + 1 soft-votes for some value v ̸= ⊥, then i cert-votes v.
    let mut has_certified_vote = false;
    if (majority.message == MessageType::SOFT && majority.vote != Vote::NullVote && majority.count >= 2*t+1) {
        user_i.update_message_type(MessageType::CERT);
        has_certified_vote = true;
        println!("\t[Step 3] User: {:?} sees SOFT-vote majority, upgrades MessageType to: {:?}", user_i.user, user_i.message);
        println!("\t[Step 3] User broadcasts: {:?}", &user_i);
    } else {
    }

    // STEP 4: [Period's First Finishing Step]
    // If i has certified some value v for period p, he next-votes v;
    if has_certified_vote {
        // user_i.update_message_type(MessageType::NEXT);
        user_i.update_vote(majority.vote);
        println!("\t[Step 4a] User CERT-votes: {:?}", &user_i.vote);
    } else {
        // Else he next-votes ⊥.
        user_i.update_message_type(MessageType::NEXT);
        user_i.update_vote(Vote::NullVote);
        println!("\t[Step 4b] User broadcasts: {:?}", &user_i);
    }

    // STEP 5: [Period's Second Finishing Step]
    if majority.message == MessageType::SOFT
        && majority.vote != Vote::NullVote
        && majority.count >= 2*t+1
        && !has_certified_vote {
        // If i sees 2t + 1 soft-votes for some value v ̸= ⊥ for period p
        // and has not next-voted v in Step 4, then i next-votes v.
        user_i.update_message_type(MessageType::NEXT);
        user_i.update_vote(majority.vote);
        println!("\t[Step 5] User NEXT-votes: {:?}", &user_i.vote);
    }

    // Return (Halting condition, User_sig)
    return (false, user_i)
}


pub fn print_sigs(period: String, users: &Vec<Sig>) {
    println!("{}:", period);
    for user in users.iter() {
        println!("\tuser: {:?}\tvote: {:?}\tmessage: {:?}", user.user, user.vote, user.message);
    }
}


pub fn halting_condition(t: u32, m: &MajorityVote) -> bool {
    // User i HALTS the moment he sees 2t + 1 cert-votes for some value v for the same period p,
    // and sets v to be his output. Those cert-votes form a certificate for v.
    if m.message == MessageType::CERT
        && m.count >= 2*t+1
        && m.vote != Vote::NullVote {
        println!("User sees 2t + 1 CERT-votes for value: {:?}", m.vote);
        true
    } else {
        false
    }
}


