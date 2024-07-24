use sha2::{Sha256, Digest};

/// Holds double sha256 hash data
#[derive(Debug, PartialEq, Clone)]
pub struct Hash {
    data: [u8; 32], // Store big endian
}

impl Hash {
    /// Zero initlized new Hash
    pub fn new() -> Self {
        Hash {
            data: [0; 32],
        }
    }

    /// Build Hash from byte array
    pub fn from_array(array: [u8; 32]) -> Self {
        Hash {
            data: array,
        }
    }

    /// Compute double sha256 hash from raw data
    pub fn hash256(slice: &[u8]) -> Self {
        Hash {
            data: Sha256::digest(Sha256::digest(slice)).into(),
        }
    }

    pub fn from_hex_string(hex: &str) -> Result<Hash, Box<dyn std::error::Error>> {
        let data = hex::decode(hex)?;
        let hash: [u8; 32] = <[u8; 32]>::try_from(data.as_slice())?;
        Ok(
            Hash {
                data: hash,
            })
    }

    /// Convert to String, big endian
    pub fn to_string(&self) -> String {
        hex::encode(self.data)
    }

    /// Convert to String, little endian
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

    /// Access the internal buffer
    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }

    /// Access the internal buffer for mutation
    pub fn as_mut_slice(&mut self) -> &[u8] {
        &self.data
    }

    pub fn reverse(&self) -> Hash {
        let mut buffer: [u8; 32] = [0; 32];
        for (i, &byte) in self.data.iter().rev().enumerate() {
            buffer[i] = byte;
        }
        Hash {
            data: buffer,
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        // Empty data vector
        let hash = Hash::hash256(b"abc");
        assert_eq!(hash.to_string(), "4f8b42c22dd3729b519ba6f68d2da7cc5b2d606d05daed5ad5128cc03e6c6358");

        // Block 853620
        let block_header = hex::decode("0000a324056ffff5fe093b3d797c70bdabf6562401d9ca2578b4020000000000000000003d4c22c742c8ba771df6c3be8f1785536e221d3b68d44eca716040c9999593142d1ea0663a6e03171d1e9a94").unwrap();
        let hash = Hash::hash256(&block_header);
        assert_eq!(hash.to_le_string(), "00000000000000000000d89e162692967cb3abc15715068d5b5d21937405ce37");
    }

    fn test_from_hex_string() {
        let expected = Hash {
            data: [0x4f, 0x8b, 0x42, 0xc2, 0x2d, 0xd3, 0x72, 0x9b,
                   0x51, 0x9b, 0xa6, 0xf6, 0x8d, 0x2d, 0xa7, 0xcc,
                   0x5b, 0x2d, 0x60, 0x6d, 0x05, 0xda, 0xed, 0x5a,
                   0xd5, 0x12, 0x8c, 0xc0, 0x3e, 0x6c, 0x63, 0x58]
        };
        assert_eq!(Hash::from_hex_string("4f8b42c22dd3729b519ba6f68d2da7cc5b2d606d05daed5ad5128cc03e6c6358").unwrap(), expected);
    }
}
