use std::fs;
use std::io::Read;
use std::path::Path;

use serde::Deserialize;

use bitcoin::*;

use week5_lib::block_header::BlockHeader;


#[derive(Debug, PartialEq, Deserialize)]
struct Outpoint {
    txid: String,
    vout: u64,
}

#[derive(Debug, PartialEq, Deserialize)]
struct Transaction {
    txid: String,
    vin: Vec<Outpoint>,
    size: u64,
    weight: u64,
    fee: u64,
}

// #[derive(Debug, PartialEq)]
// struct CandidateTx {
//     tx: Transaction,
//     fee_rate: f64,
//     parents: Vec<String>,
// }

// impl CandidateTx {
//     fn new(tx: Transaction) -> Self {
//         let fee_rate = tx.fee as f64 / tx.size as f64;
//         let mut parents: Vec<String> = Vec::new();

//     }
// }


fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Load mempool into memory
    let mempool_dir = Path::new("mempool");
    let mut mempool_spec = fs::File::open(mempool_dir.join("mempool.json"))?;
    let mempool_files:Vec<String> = serde_json::from_reader(mempool_spec)?;

    // let mut mempool: Vec<Transaction> = Vec::new();
    // for file in mempool_files {
    //     let mut tx_file = fs::File::open(mempool_dir.join(file).join(".json"))?;
    //     let tx: Transaction = serde_json::from_reader(tx_file)?;
    // }

    let tx_path = "mempool/00000964b698b728022e6d180add7b2c060676e522ab2907f06198af7b2d0b99.json";
    let tx_file = fs::File::open(tx_path)?;
    let tx: Transaction = serde_json::from_reader(tx_file)?;

    println!("{tx:?}");


    let mut block_header = BlockHeader::empty();
    block_header.version = 2;
    println!("{block_header:?}");
    println!("{:?}", block_header.serialize());
    println!("{}", block_header.to_string());

    //println!("{:?}", mempool);

    // let mut mempool: Vec<Transaction> = Vec::new();
    // for file in fs::read_dir(INPUT_PATH)? {
    //     println!("Processing file {:?}", file);
    //     let transaction_data = fs::File::open(file)?;
    //     let tx: Transaction = serde_json::from_reader(transaction_data)?;
    //     println!("{:?}", tx);

    // }

    let data: [u8; 80] = [0x31; 80];
    fs::write("out.txt", data)?;
    Ok(())
}
