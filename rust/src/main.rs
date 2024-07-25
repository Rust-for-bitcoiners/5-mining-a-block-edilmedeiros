use bitcoin::absolute::{Height, LockTime};
use bitcoin::consensus::Encodable;
use bitcoin::transaction::Version;
use week5_lib::hash::Hash;
use week5_lib::block_header::BlockHeader;

use std::fs::File;
use std::path::Path;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};
use std::str::FromStr;

use bitcoin::{Amount, OutPoint, ScriptBuf, Sequence, Transaction, TxIn, TxOut, Txid, Witness};


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
    //let tx: Transaction = serde_json::from_reader(tx_file)?;

    //println!("{tx:?}");

    // Decide which transactions will enter the block
    let candidate_txids: Vec<Hash> = Vec::new();

    ////////////////////////////////
    // Build coinbase transaction //
    ////////////////////////////////

    // 1. Build the coinbase transaction input

    // Specific "spending" outpoint for the coinbase transaction
    let outpoint = OutPoint {
        txid: Txid::from_str("0000000000000000000000000000000000000000000000000000000000000000").unwrap(),
        vout: 0xffff_ffff,
    };

    // Sequence should be 0xffff_ffff
    let coinbase_sequence = Sequence(0xffff_ffff);

    // TODO: script_sig data
    let coinbase_data = ScriptBuf::from_bytes("Mined by edilmedeiros".bytes().collect());

    // Witness should have a 'witness reserved value': BIP 141. Since the BIP
    // does not specify any values, using 32-byte array of zeros.
    let coinbase_witness_data: Vec<[u8; 32]> = vec![[0; 32]];
    let coinbase_witness = Witness::from_slice(&coinbase_witness_data);

    let txin = TxIn{
        previous_output: outpoint,
        script_sig: coinbase_data,
        sequence: coinbase_sequence,
        witness: coinbase_witness,
    };

    let coinbase_input = vec![txin];

    // 2. Build the coinbase transaction outputs
    // 2.a Output 0 will deposit the reward
    // TODO: compute reward + fees
    let coinbase_value_0 = Amount::from_sat(42);

    // TODO: create a locking script
    let output_script_0 = ScriptBuf::new();

    let txout_0 = TxOut {
        value: coinbase_value_0,
        script_pubkey: output_script_0,
    };

    //2.b Output 1 will have coinbase data
    // Value should be zero for arbitrary data
    let coinbase_value_1 = Amount::ZERO;

    // Commitment structure: BIP 141
    let mut commitment_structure = String::new();
    commitment_structure.push_str("6a24aa21a9ed");

    let commitment_hash = Hash::new();
    // TODO: Compute commitment hash
    commitment_structure.push_str(&commitment_hash.to_string());
    let output_script_1 = ScriptBuf::from_hex(&commitment_structure).unwrap();

    let txout_1 = TxOut {
        value: coinbase_value_1,
        script_pubkey: output_script_1,
    };

    // Build final output vector
    let coinbase_output = vec![txout_0, txout_1];


    // 3. Build the coinbase transaction final data structure
    let coinbase_lock_time = LockTime::Blocks(Height::MIN);
    let coinbase_version = Version(2);

    let coinbase = Transaction {
        version: coinbase_version,
        lock_time: coinbase_lock_time,
        input: coinbase_input,
        output: coinbase_output,
    };

    // Serialize coinbase transaction
    let mut coinbase_serialization: Vec<u8> = Vec::new();
    let _ = coinbase.consensus_encode(&mut coinbase_serialization);
    let coinbase_string = hex::encode(coinbase_serialization);


    ////////////////////////
    // Build block header //
    ////////////////////////
    let mut block_header = BlockHeader::empty();

    // Version should be at least 4
    block_header.version = 4;

    // Previous block hash can be any, using the genesis block =]
    //"000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f"
    block_header.prev_block_hash = Hash::from_hex_string("000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f").unwrap();

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

    /////////////////
    // Grind block //
    /////////////////
    let valid_block_header = block_header.grind().expect("Could not find valid nonce");
    println!("Found block: {:?}", valid_block_header);
    println!("Block hash: {}", valid_block_header.compute_hash().to_le_string());

    // Output solution data
    let mut output_file = File::create("out.txt")?;
    output_file.write(&valid_block_header.to_string().as_bytes())?;
    output_file.write(b"\n")?;
    output_file.write(coinbase_string.as_bytes())?;
    output_file.write(b"\n")?;
    candidate_txids.iter().for_each(|txid| {
        output_file.write(txid.to_string().as_bytes()).unwrap();
        output_file.write(b"\n").unwrap();
    });

    Ok(())
}
