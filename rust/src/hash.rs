use sha2::{Sha256, Digest};

/// Holds double sha256 hash data
#[derive(Debug)]
pub struct Hash {
    data: [u8; 32], // Store big endian
}

impl Hash {
    pub fn new() -> Self {
        Hash {
            data: [0; 32],
        }
    }

    pub fn from_slice(slice: &[u8]) -> Self {
        Hash {
            data: Sha256::digest(Sha256::digest(slice)).into(),
        }

    }

    pub fn to_string(&self) -> String {
        hex::encode(self.data)
    }

    pub fn to_le_string(&self) -> String {
        hex::encode(self.reverse_bytes())
    }

    fn reverse_bytes(&self) -> [u8; 32] {
        let mut buffer: [u8; 32] = [0; 32];
        for (i, &byte) in self.data.iter().rev().enumerate() {
            buffer[i] = byte;
        }
        buffer
    }

    pub fn to_slice(&self) -> &[u8] {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        // Empty data vector
        let hash = Hash::from_slice(b"abc");
        assert_eq!(hash.to_string(), "4f8b42c22dd3729b519ba6f68d2da7cc5b2d606d05daed5ad5128cc03e6c6358");

        // Block 853620
        let block_header = hex::decode("0000a324056ffff5fe093b3d797c70bdabf6562401d9ca2578b4020000000000000000003d4c22c742c8ba771df6c3be8f1785536e221d3b68d44eca716040c9999593142d1ea0663a6e03171d1e9a94").unwrap();
        let hash = Hash::from_slice(&block_header);
        assert_eq!(hash.to_le_string(), "00000000000000000000d89e162692967cb3abc15715068d5b5d21937405ce37");
    }
}
