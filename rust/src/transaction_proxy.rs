// I found that the bitcoin rust crate can't deserialize the mempool json files
// straight away because the names exported by bitcoin core differ from the
// Transaction struct filed names. This module provides a translation layer to
// make it work.

use bitcoin::consensus::Decodable;
use bitcoin::Transaction;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TransactionProxy {
    hex: String,
    // fee: u64,
    // size: u64,
    // weight: u64,
}

impl TransactionProxy {
    pub fn transaction(self) -> Result<Transaction, bitcoin::consensus::encode::Error> {
        let buffer = hex::decode(self.hex)
            .map_err(|_| bitcoin::consensus::encode::Error::ParseFailed("got invalid hex string"))?;
        Transaction::consensus_decode(&mut buffer.as_slice())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    use std::fs::File;
    use std::io::Read;
    use std::path::PathBuf;

    use bitcoin::{Amount, Transaction};
    use bitcoin::transaction::{Version};
    use bitcoin::absolute::{LockTime};


    #[test]
    fn test_deserialize() {
        let mut filepath = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        filepath.push("../mempool/00000964b698b728022e6d180add7b2c060676e522ab2907f06198af7b2d0b99.json");
        let mut tx_file = File::open(filepath).unwrap();
        let mut tx_data = String::new();
        tx_file.read_to_string(&mut tx_data).unwrap();
        let tx: Transaction = serde_json::from_str::<TransactionProxy>(&tx_data)
            .expect("failed to parse json data")
            .transaction()
            .expect("failed to parse hex string");
        assert_eq!(tx.compute_txid().to_string(), "00000964b698b728022e6d180add7b2c060676e522ab2907f06198af7b2d0b99");
        assert_eq!(tx.version, Version(1));
        assert_eq!(tx.lock_time, LockTime::from_consensus(273));
        assert_eq!(tx.input.len(), 4);
        assert_eq!(tx.input[0].previous_output.txid.to_string(), "888888f6769c8b9c5a6be21a0232759104ecf4d69692bb3e20945fad4376223e");
        assert_eq!(tx.input[0].previous_output.vout, 1);
        assert_eq!(tx.input[0].sequence.to_consensus_u32(), 357913941);
        assert_eq!(tx.input[0].script_sig.to_hex_string(), "");
        assert_eq!(tx.output.len(), 1);
        assert_eq!(tx.output[0].value, Amount::from_sat(3624));
        assert_eq!(tx.output[0].script_pubkey.to_hex_string(), "5120a15e30586a58e86361659c3aa59f6f1441af61e969aa49b8195bd13e55edf759");
    }

}
