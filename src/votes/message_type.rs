
use std::hash::{ Hash, Hasher };
use std::collections::hash_map::DefaultHasher;

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
