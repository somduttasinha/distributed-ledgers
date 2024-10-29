use std::time::{SystemTime, UNIX_EPOCH};

use crate::primitives::{Block, BlockChain, Transaction};
use crate::util::{self, hash};
use crate::Hash;

/// Q1: Load the most recent hash from the blockchain
pub fn load_most_recent_hash(blocks: &Vec<Block>) -> &Hash {
    let most_recent_block = blocks
        .iter()
        .max_by_key(|b| b.block_header.timestamp)
        .expect("Expected block to exist");

    &most_recent_block.block_header.hash
}

/// Q2: Create a new block from the mempool
pub fn create_block(
    mempool: &Vec<Transaction>,
    blockchain: &Vec<Block>,
    addresses: &Vec<String>,
) -> Block {
    println!("Creating block");

    let most_recent_block = blockchain
        .iter()
        .max_by_key(|b| b.block_header.timestamp)
        .expect("Expected block to exist");

    let latest_timestamp = most_recent_block.block_header.timestamp;

    let new_timestamp = latest_timestamp + 10;

    let mut transactions: Vec<Transaction> = mempool
        .iter()
        .filter(|t| t.lock_time <= new_timestamp)
        .cloned()
        .collect();

    transactions.sort_by(|a, b| b.transaction_fee.cmp(&a.transaction_fee));

    transactions.truncate(100);

    let merkle_tree = Block::generate_merkle_tree(&transactions);

    let transactions_merkle_root = merkle_tree[0].clone();

    let now = SystemTime::now();

    let miner_idx = (now
        .duration_since(UNIX_EPOCH)
        .expect("Ok time conversion")
        .subsec_nanos())
        % (addresses.len() as u32);

    let miner = addresses[miner_idx as usize].clone();

    let height = most_recent_block.block_header.height + 1;

    let transactions_count = transactions.len() as u32;

    let previous_block_header_hash = most_recent_block.block_header.hash.clone();

    let difficulty = calculate_difficulty(most_recent_block.block_header.difficulty, height);

    println!(
        "Creating block with difficulty: {}, height: {}, miner: {}, previous block hash: {}, timestamp: {}, transactions count: {}",
        difficulty, height, miner, previous_block_header_hash, new_timestamp, transactions_count
    );

    let (nonce, hash): (usize, String) = calculate_hash(
        difficulty,
        height,
        &transactions_merkle_root,
        &miner,
        &previous_block_header_hash,
        new_timestamp,
        transactions_count,
    );

    Block {
        block_header: crate::primitives::BlockHeader {
            difficulty,
            hash,
            height,
            miner,
            nonce,
            previous_block_header_hash,
            timestamp: new_timestamp,
            transactions_count,
            transactions_merkle_root,
        },
        transactions: transactions.iter().cloned().collect(),
        merkle_tree,
    }
}

/// Q3: Write the current state of the blockchain to a file
pub fn write_to_file(blockchain: &BlockChain) {
    let serialized = serde_json::to_string(blockchain).unwrap();
    std::fs::write("static/data/current_blockchain.json", serialized)
        .expect("Unable to write file");
}

/// Q4: Show an inclusion proof the hash of a block
///
/// Outputs a vector of hashes
pub fn produce_inclusion_proof(block: &mut Block, transaction_number: usize) -> Vec<&String> {
    let mut inclusion_proof: Vec<&String> = Vec::new();

    if transaction_number > block.get_merkle_tree().len() {
        return inclusion_proof;
    }

    let merkle_tree = &block.merkle_tree;

    if transaction_number % 2 == 1 {
        inclusion_proof.push(&merkle_tree[transaction_number + 1])
    } else {
        inclusion_proof.push(&merkle_tree[transaction_number])
    }

    // get the indices required

    let height = (((merkle_tree.len() + 1) as f64).log2() as u32) - 1;

    let offset = (1 << height) - 1;

    let transaction_idx = offset + transaction_number - 1;

    println!("transaction idx: {}", transaction_idx);

    let mut current_idx = match transaction_idx % 2 == 0 {
        true => transaction_idx - 1,
        false => transaction_idx + 1,
    };

    loop {
        if false {
            break;
        }
        inclusion_proof.push(&merkle_tree[current_idx]);
        println!("Pushing index {} to vec", current_idx);
        let parent_idx = (current_idx - 1) / 2;
        if parent_idx == 0 {
            break;
        }
        if parent_idx % 2 == 1 {
            current_idx = parent_idx + 1;
        } else {
            current_idx = parent_idx - 1;
        }
    }

    println!("{:?}", inclusion_proof);

    inclusion_proof
}

// Q5: Verify inclusion proof
//
// Returns true if valid proof, else false

pub fn verify_inclusion_proof(
    root_hash: String,
    transaction_hash: String,
    proof: &Vec<String>,
) -> bool {
    let mut current_hash = transaction_hash; // hash of immediate parent
    for hash in proof {
        let raw_string;
        if hash < &current_hash {
            raw_string = format!("{}{}", hash, current_hash);
        } else {
            raw_string = format!("{}{}", current_hash, hash);
        }

        let new_hash = util::hash(&raw_string).clone();
        current_hash = new_hash
    }

    current_hash == root_hash
}

fn calculate_hash(
    difficulty: u8,
    height: u16,
    transactions_merkle_root: &String,
    miner: &String,
    previous_block_header_hash: &String,
    new_timestamp: u64,
    transactions_count: u32,
) -> (usize, String) {
    let mut nonce = 0;
    let mut current_hash = hash(&format!(
        "{},{},{},{},{}, {}, {}, {}",
        difficulty,
        height,
        miner,
        nonce,
        previous_block_header_hash,
        new_timestamp,
        transactions_count,
        transactions_merkle_root
    ));

    while !is_valid_hash(&current_hash, difficulty) {
        nonce += 1;
        let raw_string = format!(
            "{},{},{},{},{}, {}, {}, {}",
            difficulty,
            height,
            miner,
            nonce,
            previous_block_header_hash,
            new_timestamp,
            transactions_count,
            transactions_merkle_root
        );
        current_hash = hash(&raw_string);
    }

    (nonce, current_hash)
}

fn count_leading_zeros(hex: &String) -> usize {
    if let Some(stripped) = hex.strip_prefix("0x") {
        stripped.chars().take_while(|&c| c == '0').count()
    } else {
        0
    }
}

fn is_valid_hash(hash: &String, difficulty: u8) -> bool {
    count_leading_zeros(hash) >= (difficulty as usize)
}

fn calculate_difficulty(prev_difficulty: u8, height: u16) -> u8 {
    // if current difficulty is 6, return 6

    let new_height = height + 1;
    if prev_difficulty == 6 {
        prev_difficulty
    } else if new_height >= 300 {
        6
    } else if new_height % 50 == 0 {
        prev_difficulty + 1
    } else {
        prev_difficulty
    }
}
