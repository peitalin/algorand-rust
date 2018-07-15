

use ring::digest::{ digest, SHA256 };

pub fn signature(input: &str) -> String {
    let result = digest(&SHA256, input.as_bytes());
    result.as_ref().iter().map(|b| format!("{:x}", b)).collect()
}


