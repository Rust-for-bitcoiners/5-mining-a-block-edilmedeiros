use week5_lib::hash::Hash;
use week5_lib::block_header::BlockHeader;

use std::fs::File;
use std::path::Path;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::Deserialize;


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
    let mut mempool_spec = File::open(mempool_dir.join("mempool.json"))?;
    let mempool_files:Vec<String> = serde_json::from_reader(mempool_spec)?;

    // let mut mempool: Vec<Transaction> = Vec::new();
    // for file in mempool_files {
    //     let mut tx_file = File::open(mempool_dir.join(file).join(".json"))?;
    //     let tx: Transaction = serde_json::from_reader(tx_file)?;
    // }

    let tx_path = "mempool/00000964b698b728022e6d180add7b2c060676e522ab2907f06198af7b2d0b99.json";
    let tx_file = File::open(tx_path)?;
    let tx: Transaction = serde_json::from_reader(tx_file)?;

    println!("{tx:?}");

    // Decide which transactions will enter the block
    let candidate_txids: Vec<Hash> = Vec::new();

    // Build coinbase transaction
    let coinbase = 0;


    ////////////////////////
    // Build block header //
    ////////////////////////
    let mut block_header = BlockHeader::empty();

    // Version should be at least 4
    block_header.version = 4;

    // Previous block hash can be any, using the genesis block =]
    //"000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f"
    block_header.prev_block_hash = Hash::new();

    // Build merkle root from candidate transactions
    //block_header.merkle_root = MerkleRoot::compute_merkle_root(candidate_txs);
    block_header.merkle_root = Hash::new();

    // Timestamp with current time
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    block_header.timestamp = timestamp.as_secs() as u32; // 24/07/2024 23h59m59s

    // The difficulty target is `0000ffff00000000000000000000000000000000000000000000000000000000`
    block_header.target = 0x1f00ffff;

    // Grind block
    block_header.nonce = 0;



    //println!("{:?}", mempool);

    // let mut mempool: Vec<Transaction> = Vec::new();
    // for file in read_dir(INPUT_PATH)? {
    //     println!("Processing file {:?}", file);
    //     let transaction_data = File::open(file)?;
    //     let tx: Transaction = serde_json::from_reader(transaction_data)?;
    //     println!("{:?}", tx);

    // }

    // Output solution data
    let mut output_file = File::create("out.txt")?;
    output_file.write(&block_header.to_string().as_bytes())?;
    output_file.write(b"\n")?;
    output_file.write(coinbase.to_string().as_bytes())?;
    output_file.write(b"\n")?;
    candidate_txids.iter().for_each(|txid| {
        output_file.write(txid.to_string().as_bytes()).unwrap();
        output_file.write(b"\n").unwrap();
    });

    Ok(())
}
