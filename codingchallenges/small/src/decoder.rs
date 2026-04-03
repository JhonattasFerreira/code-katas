use crate::{bits::BitReader, tree};

pub fn decode(compressed: &[u8]) -> Vec<u8> {
    // read original size (8 bytes)
    let original_size = u64::from_le_bytes(compressed[0..8].try_into().unwrap()) as usize;

    // read entry count (2 bytes)
    let n_entries = u16::from_le_bytes(compressed[8..10].try_into().unwrap()) as usize;

    // read (byte, freq) pairs and reconstruct frequency table
    let mut freq = [0u64; 256];
    for i in 0..n_entries {
        let offset = 10 + i * 9;
        let byte = compressed[offset] as usize;
        let f = u64::from_le_bytes(compressed[offset + 1..offset + 9].try_into().unwrap());
        freq[byte] = f;
    }

    let data_start = 10 + n_entries * 9;
    let data = &compressed[data_start..];

    // edge case: single distinct byte — no bits were written
    if n_entries == 1 {
        let byte = compressed[10];
        return vec![byte; original_size];
    }

    // rebuild tree and decode bits
    let root = tree::build_tree(&freq);
    let mut reader = BitReader::new(data);
    let mut out = Vec::with_capacity(original_size);
    let mut node = &root;

    while out.len() < original_size {
        match node {
            tree::HuffNode::Leaf { byte, .. } => {
                out.push(*byte);
                node = &root;
            }
            tree::HuffNode::Internal { left, right, .. } => {
                match reader.read_bit() {
                    Some(false) => node = left,
                    Some(true) => node = right,
                    None => break,
                }
            }
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encoder;

    // decode(encode(x)) == x for a short string
    #[test]
    fn test_round_trip_short_string() {
        let data = b"hello world";
        let compressed = encoder::encode(data);
        let result = decode(&compressed);
        assert_eq!(result, data);
    }

    // decode(encode(x)) == x for a single distinct byte repeated
    #[test]
    fn test_round_trip_single_byte() {
        let data = b"aaaaaaa";
        let compressed = encoder::encode(data);
        let result = decode(&compressed);
        assert_eq!(result, data);
    }

    // decode(encode(x)) == x for two distinct bytes
    #[test]
    fn test_round_trip_two_bytes() {
        let data = b"ababab";
        let compressed = encoder::encode(data);
        let result = decode(&compressed);
        assert_eq!(result, data);
    }

    // decode(encode(x)) == x for all 256 possible byte values
    #[test]
    fn test_round_trip_all_256_bytes() {
        let data: Vec<u8> = (0..=255u8).collect();
        let compressed = encoder::encode(&data);
        let result = decode(&compressed);
        assert_eq!(result, data);
    }

    // decode(encode(x)) == x for skewed data (many 'a', few 'b')
    #[test]
    fn test_round_trip_skewed_data() {
        let mut data = vec![b'a'; 1000];
        data.push(b'b');
        let compressed = encoder::encode(&data);
        let result = decode(&compressed);
        assert_eq!(result, data);
    }
}
