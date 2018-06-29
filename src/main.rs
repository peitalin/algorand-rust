#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]

extern crate ring;

mod votes;
use votes::Vote;
use votes::MessageType;
use votes::{ signature, SIG };


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
    let user_m = SIG::new(&"m", Vote::NullVote,  MessageType::NEXT);
    let user_n = SIG::new(&"n", Vote::NullVote,  MessageType::NEXT);
    let user_o = SIG::new(&"o", Vote::NullVote,  MessageType::NEXT);

    let users = vec![&user_i, &user_j, &user_k, &user_l, &user_m, &user_n, &user_o];
    for (i, user) in users.iter().enumerate() {
        println!("User {}: {:?}", &user.user, &user);
        if i+1 == users.len() {
            println!("\n");
        }
    }

    // STEP 1: Value Proposal
    let vt = BA1(2, users);
    // println!("v: {}\tsig: {}", v, sig);
}




fn BA1<'a>(p: i32, users: Vec<&SIG>) -> (Vote, i32) {
    // Value proposal
    let vt = majority_vote( users.iter().map(|&user| &user.vote).collect() );

    if p == 1 {
        println!("\nMajority voted: {:?} {:?} times ", vt.0, vt.1);
        return vt
    } else {
        // else if p == 2 && !majority_vote()
        println!("\nMajority voted: {:?} {:?} times ", vt.0, vt.1);
        return vt
    }

}




fn majority_vote(votes: Vec<&Vote>) -> (Vote, i32) {
    // Has user i received 2t + 1 next-votes for ⊥ in period p - 1
    // count number of NullVotes, return majority: v or NullVote
    let countValue = votes.iter().filter(|&n| *n == &Vote::Value(33)).count() as i32;
    let countNull = votes.iter().filter(|&n| *n == &Vote::NullVote).count() as i32;
    println!("Votes: {:?}\n", votes);
    println!("\tVoteCount for {:?}: {:?}", Vote::Value(33), countValue);
    println!("\tVoteCount for NullVote: {:?}", countNull);

    if countValue > countNull {
        (Vote::Value(33), countValue)
    } else {
        (Vote::NullVote, countNull)
    }
}




