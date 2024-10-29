use std::fs;

use primitives::Transaction;

mod primitives;
mod service;
mod util;

use primitives::BlockChain;

pub type Hash = String;

fn main() {
    let mut mempool_instance: Vec<Transaction> = Vec::new();

    println!("Reading mempool");

    let mempool_file = fs::File::open("static/data/mempool.json").expect("Expected file to exist");
    let mempool: Vec<primitives::Transaction> =
        serde_json::from_reader(&mempool_file).expect("Expected json");

    mempool.iter().for_each(|t| {
        mempool_instance.push(t.clone());
    });

    println!("Reading blocks");

    let mut blockchain_instance: BlockChain = Vec::new();

    let blockchain_file =
        fs::File::open("static/data/blockchain.json").expect("Expected file to exist");

    let blockchain: Vec<primitives::Block> =
        serde_json::from_reader(&blockchain_file).expect("Expected json");

    blockchain.iter().for_each(|b| {
        blockchain_instance.push(b.clone());
    });

    println!("Reading addresses");

    let mut addresses_instance: Vec<String> = Vec::new();

    let addresses_file =
        fs::File::open("static/data/miner_addresses.json").expect("Expected file to exist");

    let addresses: Vec<String> = serde_json::from_reader(&addresses_file).expect("Expected json");

    addresses.iter().for_each(|b| {
        addresses_instance.push(b.clone());
    });

    println!("Finished reading data from files");

    println!("Creating block");
    let mut block = service::create_block(&mempool, &blockchain, &addresses);
    blockchain_instance.push(block.clone());
    println!("Block created");

    let block_json = serde_json::to_string(&block).expect("Expected json");

    service::write_to_file(&blockchain_instance);

    let inclusion_proof = service::produce_inclusion_proof(&mut block, 64);

    println!("{:?}", inclusion_proof);
}
