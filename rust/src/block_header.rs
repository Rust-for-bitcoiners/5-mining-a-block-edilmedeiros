use sha2::{Sha256, Digest};

/// Models a block header
#[derive(Debug)]
pub struct BlockHeader {
    pub version: u32,
    pub prev_block_hash: [u8; 32],
    pub merkle_root: [u8; 32],
    pub timestamp: u32,
    pub target: u32,
    pub nonce: u32
}

impl BlockHeader {
    /// Returns a fresh block header struct
    pub fn empty() -> Self {
        BlockHeader {
            version: 0,
            prev_block_hash: [0; 32], // Stored bigendian
            merkle_root: [0; 32],     // Stored big endian
            timestamp: 0,
            target: 0,
            nonce: 0,
        }
    }

    /// Serialize the block header
    pub fn serialize(&self) -> [u8; 80] {
        const VERSION_OFFSET: usize = 0;
        const PREV_BLOCK_HASH_OFFSET: usize = VERSION_OFFSET + 4;
        const MERKLE_ROOT_OFFSET: usize = PREV_BLOCK_HASH_OFFSET + 32;
        const TIMESTAMP_OFFSET: usize = MERKLE_ROOT_OFFSET + 32;
        const TARGET_OFFSET: usize = TIMESTAMP_OFFSET + 4;
        const NONCE_OFFSET: usize = TARGET_OFFSET + 4;

        // Result buffer
        let mut block_header: [u8; 80] = [0; 80];

        // Process version bytes
        for (i, &byte) in self.version.to_le_bytes().iter().enumerate() {
            block_header[i + VERSION_OFFSET] = byte;
        }

        // Process previous block hash
        for (i, &byte) in self.prev_block_hash.iter().rev().enumerate() {
            block_header[i + PREV_BLOCK_HASH_OFFSET] = byte;
        }

        // Process merkle root
        for (i, &byte) in self.merkle_root.iter().rev().enumerate() {
            block_header[i + MERKLE_ROOT_OFFSET] = byte;
        }

        // Process timestamp
        for (i, &byte) in self.timestamp.to_le_bytes().iter().enumerate() {
            block_header[i + TIMESTAMP_OFFSET] = byte;
        }

        // Process target
        for (i, &byte) in self.target.to_le_bytes().iter().enumerate() {
            block_header[i + TARGET_OFFSET] = byte;
        }

        // Process nonce
        for (i, &byte) in self.nonce.to_le_bytes().iter().enumerate() {
            block_header[i + NONCE_OFFSET] = byte;
        }

        block_header
    }

    pub fn to_string(&self) -> String {
        hex::encode(self.serialize())
    }

    /// Compute the block header hash
    pub fn compute_hash(&self) -> [u8; 32] {
        fn reverse_bytes(array: &[u8; 32]) -> [u8; 32] {
            let mut buffer: [u8; 32] = [0; 32];
            for (i, &byte) in array.iter().rev().enumerate() {
                buffer[i] = byte;
            }
            buffer
        }

        let block_hash: [u8; 32] = Sha256::digest(Sha256::digest(self.serialize())).into();
        // The resulting hash is to be interpreted little endian...
        reverse_bytes(&block_hash)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_hash() {
        // All zeros block
        let block_header = BlockHeader::empty();
        let block_hash = block_header.compute_hash();
        // Hash computed using openssl, bytes reversed
        assert_eq!(hex::encode(block_hash),
                   "14508459b221041eab257d2baaa7459775ba748246c8403609eb708f0e57e74b");

        // Real data from block 853620
        let prev_block_hash: [u8; 32] = <[u8; 32]>::try_from( hex::decode("00000000000000000002b47825cad9012456f6abbd707c793d3b09fef5ff6f05").unwrap().as_slice()).unwrap();
        let merkle_root: [u8; 32] = <[u8; 32]>::try_from( hex::decode("14939599c9406071ca4ed4683b1d226e5385178fbec3f61d77bac842c7224c3d").unwrap().as_slice()).unwrap();
        let block_header = BlockHeader {
            version: 0x24a30000,
            prev_block_hash: prev_block_hash,
            merkle_root: merkle_root,
            timestamp: 0x66a01e2d,
            target: 0x17036e3a,
            nonce: 0x949a1e1d,
        };

        // Check serialization
        assert_eq!("0000a324056ffff5fe093b3d797c70bdabf6562401d9ca2578b4020000000000000000003d4c22c742c8ba771df6c3be8f1785536e221d3b68d44eca716040c9999593142d1ea0663a6e03171d1e9a94", block_header.to_string());
        // Check block hash
        assert_eq!(hex::encode(block_header.compute_hash()), "00000000000000000000d89e162692967cb3abc15715068d5b5d21937405ce37")
    }
}

