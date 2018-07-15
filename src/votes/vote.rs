
use std::hash::{ Hash, Hasher };
use std::collections::hash_map::DefaultHasher;


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

