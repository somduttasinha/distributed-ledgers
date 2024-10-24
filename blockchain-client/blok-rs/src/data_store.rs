use std::sync::{Mutex, OnceLock};

use crate::primitives::{Block, Transaction};

pub struct Mempool {
    txs: Vec<Transaction>,
}

pub struct Blockchain {
    blocks: Vec<Block>,
}

impl Mempool {
    fn new() -> Self {
        Mempool { txs: Vec::new() }
    }

    pub fn add_transaction(&mut self, tx: Transaction) {
        self.txs.push(tx);
    }

    pub fn get_transactions(&self) -> &Vec<Transaction> {
        &self.txs
    }
}

impl Blockchain {
    fn new() -> Self {
        Blockchain { blocks: Vec::new() }
    }

    pub fn add_block(&mut self, bl: Block) {
        self.blocks.push(bl);
    }

    pub fn get_blocks(&self) -> &Vec<Block> {
        &self.blocks
    }
}


static MEMPOOL_INSTANCE: OnceLock<Mutex<Mempool>> = OnceLock::new();
static BLOCKCHAIN_INSTANCE: OnceLock<Mutex<Blockchain>> = OnceLock::new();
static ADDRESSES_INSTANCE: OnceLock<Mutex<Vec<String>>> = OnceLock::new();

pub fn get_addresses_instance() -> &'static Mutex<Vec<String>> {
    ADDRESSES_INSTANCE.get_or_init(|| Mutex::new(Vec::new()))
}


pub fn get_mempool_instance() -> &'static Mutex<Mempool> {
    println!("Getting mempool instance");
    let mempool =MEMPOOL_INSTANCE.get_or_init(|| Mutex::new(Mempool::new()));
    println!("Got mempool instance");
    mempool
}

pub fn get_blockchain_instance() -> &'static Mutex<Blockchain> {
    BLOCKCHAIN_INSTANCE.get_or_init(|| Mutex::new(Blockchain::new()))
}
