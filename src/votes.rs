
use std::fmt;
use std::hash::{ Hash, Hasher };
use std::collections::hash_map::DefaultHasher;
use ring::digest::{ digest, SHA256, Digest };


#[derive(Debug, Clone, Copy, Eq)]
pub enum Vote {
    Value(i32),
    NullVote,
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
    fn hash<H: Hasher>(&self, state: &mut H) {
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

#[derive(Debug)]
pub enum MessageType {
    CERT,
    SOFT,
    NEXT,
}


#[derive(Debug)]
pub struct SIG<'a> {
    pub user: &'a str,
    pub vote: Vote,
    pub message: MessageType,
    pub signature: String,
}
impl<'a> SIG<'a> {
    pub fn new(i: &'a str, v: Vote, x: MessageType) -> SIG<'a> {
        SIG {
            user: i, // user i
            vote: v, // vote value
            message: x, // message
            signature: signature(i), // signature
        }
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


