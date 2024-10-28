use crate::util::hash;
use serde::{Deserialize, Serialize};

use crate::Hash;

pub type BlockChain = Vec<Block>;

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
