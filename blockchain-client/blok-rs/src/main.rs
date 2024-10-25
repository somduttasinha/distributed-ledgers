use std::fs;

mod data_store;
mod primitives;
mod service;
mod util;

pub type Hash = String;

fn main() {
    let mut mempool_instance = data_store::get_mempool_instance()
        .lock()
        .expect("Expected lock");

    println!("Reading mempool");

    let mempool_file = fs::File::open("static/data/mempool.json").expect("Expected file to exist");
    let mempool: Vec<primitives::Transaction> =
        serde_json::from_reader(&mempool_file).expect("Expected json");

    mempool.iter().for_each(|t| {
        mempool_instance.add_transaction(t.clone());
    });

    println!("Reading blocks");

    let mut blockchain_instance = data_store::get_blockchain_instance()
        .lock()
        .expect("Expected lock");

    let blockchain_file =
        fs::File::open("static/data/blockchain.json").expect("Expected file to exist");

    let blockcain: Vec<primitives::Block> =
        serde_json::from_reader(&blockchain_file).expect("Expected json");

    blockcain.iter().for_each(|b| {
        blockchain_instance.add_block(b.clone());
    });

    println!("Reading addresses");

    let mut addresses_instance = data_store::get_addresses_instance()
        .lock()
        .expect("Expected lock");

    let addresses_file =
        fs::File::open("static/data/miner_addresses.json").expect("Expected file to exist");

    let addresses: Vec<String> = serde_json::from_reader(&addresses_file).expect("Expected json");

    addresses.iter().for_each(|b| {
        addresses_instance.push(b.clone());
    });

    println!("Finished reading data from files");

    println!("Creating block");
    let block = service::create_block(&mempool, &blockcain, &addresses);
    println!("Block created");

    let block_json = serde_json::to_string(&block).expect("Expected json");

    println!("{}", block_json);
}
