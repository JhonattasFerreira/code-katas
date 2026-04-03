use crate::{bits::BitWriter, frequency, table, tree};

pub fn encode(data: &[u8]) -> Vec<u8> {
    let freq = frequency::count(data);
    let root = tree::build_tree(&freq);
    let code_table = table::build_table(&root);

    // count distinct bytes (non-zero freq)
    let entries: Vec<(u8, u64)> = freq
        .iter()
        .enumerate()
        .filter(|&(_, f)| *f > 0)
        .map(|(b, &f)| (b as u8, f))
        .collect();

    let mut out = Vec::new();

    // 8 bytes: original size
    out.extend_from_slice(&(data.len() as u64).to_le_bytes());

    // 2 bytes: number of entries
    out.extend_from_slice(&(entries.len() as u16).to_le_bytes());

    // n * 9 bytes: (byte, freq) pairs
    for (byte, freq) in &entries {
        out.push(*byte);
        out.extend_from_slice(&freq.to_le_bytes());
    }

    // compressed data: one code per input byte, packed into bytes
    let mut writer = BitWriter::new();
    for &byte in data {
        if let Some(code) = &code_table[byte as usize] {
            for &bit in code {
                writer.write_bit(bit);
            }
        }
    }
    out.extend(writer.finish());

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    // first 8 bytes = original file size as u64 little-endian
    #[test]
    fn test_header_stores_original_size() {
        let data = b"hello";
        let result = encode(data);
        let size = u64::from_le_bytes(result[0..8].try_into().unwrap());
        assert_eq!(size, 5);
    }

    // bytes 8..10 = number of distinct bytes as u16 little-endian
    // "hello" has h, e, l, o => 4 distinct bytes
    #[test]
    fn test_header_stores_entry_count() {
        let data = b"hello";
        let result = encode(data);
        let n = u16::from_le_bytes(result[8..10].try_into().unwrap());
        assert_eq!(n, 4);
    }

    // each header entry is (byte: u8, freq: u64 LE) = 9 bytes
    // "aab" => a->2, b->1
    #[test]
    fn test_header_entries_correct() {
        let data = b"aab";
        let result = encode(data);
        let n = u16::from_le_bytes(result[8..10].try_into().unwrap()) as usize;
        assert_eq!(n, 2);

        let mut entries = std::collections::HashMap::new();
        for i in 0..n {
            let offset = 10 + i * 9;
            let byte = result[offset];
            let freq = u64::from_le_bytes(result[offset + 1..offset + 9].try_into().unwrap());
            entries.insert(byte, freq);
        }
        assert_eq!(entries[&b'a'], 2);
        assert_eq!(entries[&b'b'], 1);
    }

    // single distinct byte => code is empty (0 bits per symbol) => data section is empty
    #[test]
    fn test_single_byte_data_section_empty() {
        let data = b"aaaa";
        let result = encode(data);
        let n = u16::from_le_bytes(result[8..10].try_into().unwrap()) as usize;
        let header_size = 8 + 2 + n * 9;
        assert_eq!(result.len(), header_size);
    }

    // compressed output must be smaller than input for heavily skewed data
    // 'a' x1000, 'b' x1: 'a' gets ~1-bit code => ~125 bytes of data + small header
    #[test]
    fn test_compressed_smaller_than_input_for_skewed_data() {
        let mut data = vec![b'a'; 1000];
        data.push(b'b');
        let result = encode(&data);
        assert!(
            result.len() < data.len(),
            "expected compressed ({} bytes) < original ({} bytes)",
            result.len(),
            data.len()
        );
    }
}
