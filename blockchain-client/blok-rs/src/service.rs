use std::time::{SystemTime, UNIX_EPOCH};

use log::info;

use crate::primitives::{Block, Transaction};
use crate::util::{generate_merkle_tree, hash};
use crate::Hash;

pub fn load_most_recent_hash(blocks: &Vec<Block>) -> &Hash {
    let most_recent_block = blocks
        .iter()
        .max_by_key(|b| b.block_header.timestamp)
        .expect("Expected block to exist");

    &most_recent_block.block_header.hash
}

pub fn create_block(
    mempool: &Vec<Transaction>,
    blockchain: &Vec<Block>,
    addresses: &Vec<String>,
) -> Block {
    println!("Creating block");
    /*
        let mempool = data_store::get_mempool_instance()
            .lock()
            .expect("Expected lock");

        println!("Got mempool");

        // get the latest time in the blockchain

        let blockchain = data_store::get_blockchain_instance()
            .lock()
            .expect("Expected lock");

        let addresses = data_store::get_addresses_instance()
            .lock()
            .expect("Expected lock");
    */

    let most_recent_block = blockchain
        .iter()
        .max_by_key(|b| b.block_header.timestamp)
        .expect("Expected block to exist");

    let latest_timestamp = most_recent_block.block_header.timestamp;

    let new_timestamp = latest_timestamp + 10;

    let mut transactions: Vec<&Transaction> = mempool
        .iter()
        .filter(|t| t.lock_time <= new_timestamp)
        .collect();

    transactions.sort_by(|a, b| b.transaction_fee.cmp(&a.transaction_fee));

    transactions.truncate(100);

    let merkle_tree = generate_merkle_tree(&transactions);

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

    info!(
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
        transactions: transactions.iter().cloned().cloned().collect(),
    }
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
    let mut current_hash = hash(format!(
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
        current_hash = hash(raw_string);
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
