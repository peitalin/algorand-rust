
use std::fmt;
use std::hash::{ Hash, Hasher };
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use ring::digest::{ digest, SHA256, Digest };


#[derive(Debug, Clone, Copy, Eq)]
pub enum Vote {
    Value(u32),
    NullVote,
}
impl Default for Vote {
    fn default() -> Vote {
        Vote::NullVote
    }
}
impl PartialEq for Vote {
    fn eq(&self, other: &Vote) -> bool {
        // println!("Eq: {:?} == {:?}", &self, other);
        match ( &self, other ) {
            (Vote::Value(x), Vote::Value(y)) => x == y,
            (Vote::NullVote, Vote::NullVote) => true,
            _ => false,
        }
    }
}
impl Hash for Vote {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        let mut hasher = DefaultHasher::new();
        match &self {
            Vote::Value(x) => Hash::hash_slice(x.to_string().as_bytes(), &mut hasher),
            Vote::NullVote => Hash::hash_slice(b"NullVote", &mut hasher),
        }
    }
}


pub fn signature(input: &str) -> String {
    let result = digest(&SHA256, input.as_bytes());
    result.as_ref().iter().map(|b| format!("{:x}", b)).collect()
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MessageType {
    CERT,
    SOFT,
    NEXT,
}
impl Hash for MessageType {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        let mut hasher = DefaultHasher::new();
        match &self {
            MessageType::CERT => Hash::hash_slice(b"MessageType::CERT", &mut hasher),
            MessageType::SOFT => Hash::hash_slice(b"MessageType::SOFT", &mut hasher),
            MessageType::NEXT => Hash::hash_slice(b"MessageType::NEXT", &mut hasher),
        }
    }
}


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

