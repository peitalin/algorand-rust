
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
        println!("\tUser updates Vote: {:?}", v);
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

pub fn gossip<'a>(gossip_type: &String) -> Vec<Sig<'a>> {

    let user_a = Sig::new(&"a", Vote::Value(11), MessageType::SOFT);
    let user_b = Sig::new(&"b", Vote::Value(11), MessageType::SOFT);
    let user_c = Sig::new(&"c", Vote::Value(11), MessageType::SOFT);
    let user_d = Sig::new(&"d", Vote::Value(11), MessageType::SOFT);
    let user_e = Sig::new(&"e", Vote::Value(11), MessageType::SOFT);
    let user_f = Sig::new(&"f", Vote::Value(22), MessageType::SOFT);
    let user_g = Sig::new(&"g", Vote::Value(22), MessageType::SOFT);
    let user_h = Sig::new(&"h", Vote::Value(22), MessageType::SOFT);
    let user_i = Sig::new(&"i", Vote::Value(22), MessageType::SOFT);
    // Duplicate votes, Value votes
    let user_j = Sig::new(&"j", Vote::Value(33), MessageType::SOFT);
    let user_k = Sig::new(&"k", Vote::Value(33), MessageType::SOFT);
    let user_l = Sig::new(&"l", Vote::Value(33), MessageType::SOFT);
    let user_m = Sig::new(&"m", Vote::Value(33), MessageType::SOFT);
    let user_n = Sig::new(&"n", Vote::Value(33), MessageType::SOFT);
    let user_o = Sig::new(&"o", Vote::Value(33), MessageType::SOFT);
    let user_p = Sig::new(&"p", Vote::Value(33), MessageType::SOFT);
    let user_q = Sig::new(&"q", Vote::Value(33), MessageType::SOFT);
    let user_r = Sig::new(&"r", Vote::Value(33), MessageType::SOFT);
    let user_s = Sig::new(&"s", Vote::Value(33), MessageType::SOFT);
    let user_t = Sig::new(&"t", Vote::Value(33), MessageType::NEXT);
    let user_u = Sig::new(&"u", Vote::Value(44), MessageType::NEXT);
    let user_v = Sig::new(&"v", Vote::Value(44), MessageType::NEXT);
    // Duplicate votes, NullVote
    let _user_j = Sig::new(&"j", Vote::NullVote, MessageType::SOFT);
    let _user_k = Sig::new(&"k", Vote::NullVote, MessageType::SOFT);
    let _user_l = Sig::new(&"l", Vote::NullVote, MessageType::SOFT);
    let _user_m = Sig::new(&"m", Vote::NullVote, MessageType::SOFT);
    let _user_n = Sig::new(&"n", Vote::NullVote, MessageType::SOFT);
    let _user_o = Sig::new(&"o", Vote::NullVote, MessageType::SOFT);
    let _user_p = Sig::new(&"p", Vote::NullVote, MessageType::SOFT);
    let _user_q = Sig::new(&"q", Vote::NullVote, MessageType::SOFT);
    let _user_r = Sig::new(&"r", Vote::NullVote, MessageType::SOFT);
    let _user_s = Sig::new(&"s", Vote::NullVote, MessageType::SOFT);
    let _user_t = Sig::new(&"t", Vote::NullVote, MessageType::NEXT);
    let _user_u = Sig::new(&"u", Vote::NullVote, MessageType::NEXT);
    let _user_v = Sig::new(&"v", Vote::NullVote, MessageType::NEXT);

    let user_w = Sig::new(&"w", Vote::NullVote, MessageType::NEXT);
    let user_x = Sig::new(&"x", Vote::NullVote, MessageType::NEXT);
    let user_y = Sig::new(&"y", Vote::NullVote, MessageType::NEXT);
    let user_z = Sig::new(&"z", Vote::NullVote, MessageType::NEXT);

    if *gossip_type == "values" {
        let users: Vec<Sig> = vec![
            user_a, user_b, user_c, user_d, user_e, user_f, user_g, user_h, user_i,
            user_j, user_k, user_l, user_m, user_n, user_o, user_p,
            user_q, user_r, user_s, user_t, user_u, user_v,
            user_w, user_x, user_y, user_z,
        ];
        users
    } else {
        let users: Vec<Sig> = vec![
            user_a, user_b, user_c, user_d, user_e, user_f, user_g, user_h, user_i,
            _user_j, _user_k, _user_l, _user_m, _user_n, _user_o, _user_p,
            _user_q, _user_r, _user_s, _user_t, _user_u, _user_v,
            user_w, user_x, user_y, user_z,
        ];
        users
    }
}



pub struct MajorityVote {
    pub message: MessageType,
    pub vote: Vote,
    pub count: u32,
}
impl MajorityVote {
    pub fn new(users: &Vec<Sig>) -> MajorityVote {
        let vote_message_dict: HashMap<MessageType, HashMap<Vote, u32>> = MajorityVote::vote_message_counter_hashmap(&users);
        let (majority_message, majority_vote, majority_count) = MajorityVote::calc_majority_vote(&vote_message_dict);
        MajorityVote {
            message: majority_message,
            vote: majority_vote,
            count: majority_count,
        }
    }

    fn vote_message_counter_hashmap<'a>(users: &Vec<Sig>) -> HashMap<MessageType, HashMap<Vote, u32>> {
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
}
