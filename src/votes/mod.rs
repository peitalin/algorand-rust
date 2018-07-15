
use std::fmt;
use std::fmt::Debug;
use std::hash::Hash;
use std::collections::HashMap;

// signatures.rs
mod signatures;
pub use self::signatures::signature;
// vote.rs
mod vote;
pub use self::vote::Vote;
// message_type.rs
mod message_type;
pub use self::message_type::MessageType;


#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub struct Sig<'a> {
    pub user: &'a str,
    pub vote: Vote,
    pub message: MessageType,
    pub signature: String,
}
impl<'a> Sig<'a> {
    pub fn new(i: &'a str, v: Vote, x: MessageType) -> Sig<'a> {
        Sig {
            user: i, // user i
            vote: v, // vote value
            message: x, // message
            signature: signature(i), // signature
        }
    }

    pub fn update_vote(&mut self, v: Vote) {
        self.vote = v.clone();
    }

    pub fn update_message_type(&mut self, message: MessageType) {
        self.message = message.clone();
    }

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n\t\t(User: {}, \n\t\tVote: {:?}, \n\t\tMessage: {:?}, \n\t\tSignature: {:?})",
           self.user,
           self.vote,
           self.message,
           self.signature,
       )
    }
}

pub fn gossip<'a>() -> Vec<Sig<'a>> {
    // let user_i = Sig {
    //     user: &String::from("i"),
    //     vote: Vote::Value(22),
    //     message: MessageType::NEXT,
    //     signature: signature(&String::from("idasdf"))
    // };
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
    let user_i = Sig::new(&"i", Vote::Value(22), MessageType::SOFT);
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
    let users: Vec<Sig> = vec![
        user_i, user_j, user_k, user_l, user_m, user_n,
        user_o, user_p, user_q, user_r, user_s,
    ];
    users
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

