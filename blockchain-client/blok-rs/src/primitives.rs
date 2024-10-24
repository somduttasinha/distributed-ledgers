use crate::util::hash;
use serde::{Deserialize, Serialize};

use crate::Hash;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
    pub receiver: String,
    pub lock_time: u64,
    pub amount: u64,
    pub sender: String,
    pub signature: String,
    pub transaction_fee: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BlockHeader {
    pub difficulty: u8,
    pub hash: Hash,
    pub height: u16,
    pub miner: String,
    pub nonce: usize,
    pub previous_block_header_hash: Hash,
    pub timestamp: u64,
    pub transactions_count: u32,
    pub transactions_merkle_root: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block {
    #[serde(rename(serialize = "header", deserialize = "header"))]
    pub block_header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

impl Transaction {
    /*
    * 1. Sort all the above fields in alphabetical order by their key.
      2. Produce a comma-separated string containing all the values, without any space. Numbers (amount,
         lock time, transaction fee) should be encoded as decimal value without any leading 0s. The signature
        and addresses (sender, receiver) should be hex-encoded.
      3. Hash the string produced in step 2 using the SHA-256 hash function (remember to ensure that the hex
        string starts with 0x).
    */
    pub fn generate_hash(&self) -> String {
        let raw_string = format!(
            "{},{},{},{},{},{}",
            self.amount,
            self.lock_time,
            self.receiver,
            self.sender,
            self.signature,
            self.transaction_fee
        );

        hash(raw_string)
    }
}
