use sha2::{Digest, Sha256};

pub fn hash(raw_string: &String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(raw_string.as_bytes());
    let result = hasher.finalize();
    let prefix = "0x".to_string();
    prefix + &hex::encode(result)
}
