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

    #[serde(skip)]
    pub merkle_tree: Vec<String>,
}

impl Block {
    pub fn get_merkle_tree(&mut self) -> Vec<String> {
        if self.merkle_tree.len() == 0 {
            self.merkle_tree = Block::generate_merkle_tree(&self.transactions);
        }
        self.merkle_tree.clone()
    }

    pub fn generate_merkle_tree(transactions: &Vec<Transaction>) -> Vec<String> {
        println!("Generating Merkle tree");

        let n = transactions.len() as u64;

        let height = 64 - n.leading_zeros();

        let null_string = format!("0x{:0>64}", ""); // Fill with 64 zeros

        let size = (1 << (height + 1)) - 1;

        let mut tree: Vec<String> = vec![null_string; size];

        // iteratively populate the tree by starting from the index 2^height - 2 and going back

        let mut idx = (1 << height) - 1;

        for transaction in transactions {
            tree[idx] = transaction.generate_hash();
            idx += 1;
        }

        let end_idx = 1 << height - 2;

        for i in (0..end_idx).rev() {
            let left_child = &tree[(2 * i) + 1];
            let right_child = &tree[(2 * i) + 2];

            if left_child < right_child {
                tree[i] = hash(&format!("{}{}", left_child, right_child));
            } else {
                tree[i] = hash(&format!("{}{}", right_child, left_child));
            }
        }

        tree
    }
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

        hash(&raw_string)
    }
}
