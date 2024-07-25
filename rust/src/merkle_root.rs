use crate::hash::Hash;

#[derive(Debug)]
pub struct MerkleRoot {
    data: Hash,
}

impl MerkleRoot {
    /// New zero initialized merkle root
    pub fn new() -> Self {
        MerkleRoot {
            data: Hash::new(),
        }
    }

    pub fn from_hash(hash: Hash) -> Self {
        MerkleRoot {
            data: hash,
        }
    }

    pub fn from_hex_string(hex: &str) -> Result<MerkleRoot, Box<dyn std::error::Error>> {
        let data = Hash::from_hex_string(hex)?;
        Ok(MerkleRoot { data, })
    }

    /// Compute MerkleRoot from list of hashes
    pub fn compute_merkle_root(hashes: &Vec<Hash>) -> MerkleRoot {
        let mut buffer = hashes.clone();

        if buffer.len() == 1 {
            buffer.push(buffer[0].clone());
        }

        while buffer.len() != 1 {
            buffer = merkle_parent_level(buffer);
        }
        MerkleRoot::from_hash(buffer[0].clone())
    }

    /// Transfer ownership of the internal buffer
    pub fn to_hash(self) -> Hash {
        self.data
    }

    /// Convert to String, big endian
    pub fn to_string(&self) -> String {
        self.data.to_string()
    }

    /// Convert to String, little endian
    pub fn to_le_string(&self) -> String {
        self.data.to_le_string()
    }

    /// Access the internal buffer
    pub fn as_slice(&self) -> &[u8] {
        &self.data.as_slice()
    }

}

// Compute merkle parent from two hashes
fn merkle_parent(left: &Hash, right: &Hash) -> Hash {
    let mut buffer: [u8; 64] = [0; 64];
    buffer[0..32].clone_from_slice(left.as_slice());
    buffer[32..64].clone_from_slice(right.as_slice());
    Hash::hash256(&buffer)
}

fn merkle_parent_level(hashes: Vec<Hash>) -> Vec<Hash> {
    // Assume slice won't yeild zero elements. Will always be true in our case
    // since every block will have at least the coinbase transaction.
    let mut buffer: Vec<Hash> = Vec::new();
    let pairs = hashes.chunks(2);

    for pair in pairs {
        match pair.len() {
            // Because chunks(2) will always yield exactly one or two elements
            // slices, the default case will implement the duplication step of
            // the merkle parent computation.
            2 => { buffer.push(merkle_parent(&pair[0], &pair[1])) },
            _ => { buffer.push(merkle_parent(&pair[0], &pair[0])) },
        }
    }
    buffer
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merkle_parent() {
        let left = Hash::new();
        let right = Hash::new();
        assert_eq!(left.to_string(), "0000000000000000000000000000000000000000000000000000000000000000");
        assert_eq!(right.to_string(), "0000000000000000000000000000000000000000000000000000000000000000");
        assert_eq!(merkle_parent(&left, &right).to_string(),
                   "e2f61c3f71d1defd3fa999dfa36953755c690689799962b48bebd836974e8cf9");

        // Programming Bitcoin example
        let left = Hash::from_hex_string("c117ea8ec828342f4dfb0ad6bd140e03a50720ece40169ee38bdc15d9eb64cf5").unwrap();
        let right = Hash::from_hex_string("c131474164b412e3406696da1ee20ab0fc9bf41c8f05fa8ceea7a08d672d7cc5").unwrap();
        assert_eq!(merkle_parent(&left, &right).to_string(), "8b30c5ba100f6f2e5ad1e2a742e5020491240f8eb514fe97c713c31718ad7ecd");
    }

    #[test]
    fn test_merkle_parent_level() {
        // Empty input produces empty output
        let hashes: Vec<Hash> = Vec::new();
        let expected: Vec<Hash> = Vec::new();
        assert_eq!(merkle_parent_level(hashes), expected);

        // Single hash has to be duplicated
        let mut hashes: Vec<Hash> = Vec::new();
        hashes.push(Hash::new());
        let expected: Vec<Hash> = vec![
            "e2f61c3f71d1defd3fa999dfa36953755c690689799962b48bebd836974e8cf9",
        ].iter().map(|h| {
            Hash::from_hex_string(h).unwrap()
        }).collect();
        assert_eq!(merkle_parent_level(hashes), expected);

        // Double hashes are not duplicated. This will yield the same merkle root as above.
        let mut hashes: Vec<Hash> = Vec::new();
        hashes.push(Hash::new());
        hashes.push(Hash::new());
        let expected: Vec<Hash> = vec![
            "e2f61c3f71d1defd3fa999dfa36953755c690689799962b48bebd836974e8cf9",
        ].iter().map(|h| {
            Hash::from_hex_string(h).unwrap()
        }).collect();
        assert_eq!(merkle_parent_level(hashes), expected);

        // Odd number of input hashes
        let hashes: Vec<Hash> = vec![
            "c117ea8ec828342f4dfb0ad6bd140e03a50720ece40169ee38bdc15d9eb64cf5",
            "c131474164b412e3406696da1ee20ab0fc9bf41c8f05fa8ceea7a08d672d7cc5",
            "f391da6ecfeed1814efae39e7fcb3838ae0b02c02ae7d0a5848a66947c0727b0",
            "3d238a92a94532b946c90e19c49351c763696cff3db400485b813aecb8a13181",
            "10092f2633be5f3ce349bf9ddbde36caa3dd10dfa0ec8106bce23acbff637dae",
        ].iter().map(|h| {
            Hash::from_hex_string(h).unwrap()
        }).collect();

        let result: Vec<Hash> = vec![
            "8b30c5ba100f6f2e5ad1e2a742e5020491240f8eb514fe97c713c31718ad7ecd",
            "7f4e6f9e224e20fda0ae4c44114237f97cd35aca38d83081c9bfd41feb907800",
            "3ecf6115380c77e8aae56660f5634982ee897351ba906a6837d15ebc3a225df0",
        ].iter().map(|h| {
            Hash::from_hex_string(h).unwrap()
        }).collect();

        assert_eq!(merkle_parent_level(hashes), result);

        // Even number of input hashes
        let hashes: Vec<Hash> = vec![
            "c117ea8ec828342f4dfb0ad6bd140e03a50720ece40169ee38bdc15d9eb64cf5",
            "c131474164b412e3406696da1ee20ab0fc9bf41c8f05fa8ceea7a08d672d7cc5",
            "f391da6ecfeed1814efae39e7fcb3838ae0b02c02ae7d0a5848a66947c0727b0",
            "3d238a92a94532b946c90e19c49351c763696cff3db400485b813aecb8a13181",
            "10092f2633be5f3ce349bf9ddbde36caa3dd10dfa0ec8106bce23acbff637dae",
            "10092f2633be5f3ce349bf9ddbde36caa3dd10dfa0ec8106bce23acbff637dae",
        ].iter().map(|h| {
            Hash::from_hex_string(h).unwrap()
        }).collect();

        let result: Vec<Hash> = vec![
            "8b30c5ba100f6f2e5ad1e2a742e5020491240f8eb514fe97c713c31718ad7ecd",
            "7f4e6f9e224e20fda0ae4c44114237f97cd35aca38d83081c9bfd41feb907800",
            "3ecf6115380c77e8aae56660f5634982ee897351ba906a6837d15ebc3a225df0",
        ].iter().map(|h| {
            Hash::from_hex_string(h).unwrap()
        }).collect();

        assert_eq!(merkle_parent_level(hashes), result);

    }

    #[test]
    fn test_merkle_root() {
        // Single hash
        let mut hashes: Vec<Hash> = Vec::new();
        hashes.push(Hash::new());
        assert_eq!(MerkleRoot::compute_merkle_root(&hashes).to_string(), "e2f61c3f71d1defd3fa999dfa36953755c690689799962b48bebd836974e8cf9");

        // Double hashes are not duplicated. This will yield the same merkle root as above.
        let mut hashes: Vec<Hash> = Vec::new();
        hashes.push(Hash::new());
        hashes.push(Hash::new());
        assert_eq!(MerkleRoot::compute_merkle_root(&hashes).to_string(), "e2f61c3f71d1defd3fa999dfa36953755c690689799962b48bebd836974e8cf9");

        // Programming Bitcoin example
        let hashes: Vec<Hash> = vec![
            "c117ea8ec828342f4dfb0ad6bd140e03a50720ece40169ee38bdc15d9eb64cf5",
            "c131474164b412e3406696da1ee20ab0fc9bf41c8f05fa8ceea7a08d672d7cc5",
            "f391da6ecfeed1814efae39e7fcb3838ae0b02c02ae7d0a5848a66947c0727b0",
            "3d238a92a94532b946c90e19c49351c763696cff3db400485b813aecb8a13181",
            "10092f2633be5f3ce349bf9ddbde36caa3dd10dfa0ec8106bce23acbff637dae",
            "7d37b3d54fa6a64869084bfd2e831309118b9e833610e6228adacdbd1b4ba161",
            "8118a77e542892fe15ae3fc771a4abfd2f5d5d5997544c3487ac36b5c85170fc",
            "dff6879848c2c9b62fe652720b8df5272093acfaa45a43cdb3696fe2466a3877",
            "b825c0745f46ac58f7d3759e6dc535a1fec7820377f24d4c2c6ad2cc55c0cb59",
            "95513952a04bd8992721e9b7e2937f1c04ba31e0469fbe615a78197f68f52b7c",
            "2e6d722e5e4dbdf2447ddecc9f7dabb8e299bae921c99ad5b0184cd9eb8e5908",
            "b13a750047bc0bdceb2473e5fe488c2596d7a7124b4e716fdd29b046ef99bbf0",
        ].iter().map(|h| {
            Hash::from_hex_string(h).unwrap()
        }).collect();
        assert_eq!(MerkleRoot::compute_merkle_root(&hashes).to_string(), "acbcab8bcc1af95d8d563b77d24c3d19b18f1486383d75a5085c4e86c86beed6");

        // Data from block 170 (first bitcoin transaction)
        let hashes: Vec<Hash> = vec![
            "b1fea52486ce0c62bb442b530a3f0132b826c74e473d1f2c220bfa78111c5082",
            "f4184fc596403b9d638783cf57adfe4c75c605f6356fbc91338530e9831e9e16",
        ].iter().map(|h| {
            Hash::from_hex_string(h)
                .unwrap()
                .reverse() // Blockchain data is expected to be little endien
        }).collect();
        assert_eq!(MerkleRoot::compute_merkle_root(&hashes).to_le_string(), "7dac2c5666815c17a3b36427de37bb9d2e2c5ccec3f8633eb91a4205cb4c10ff");
    }
}
