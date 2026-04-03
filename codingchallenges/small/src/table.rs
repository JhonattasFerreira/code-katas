use crate::tree::HuffNode;

pub fn build_table(root: &HuffNode) -> [Option<Vec<bool>>; 256] {
    let mut table: [Option<Vec<bool>>; 256] = std::array::from_fn(|_| None);
    traverse(root, &mut table, Vec::new());
    table
}

fn traverse(node: &HuffNode, table: &mut [Option<Vec<bool>>; 256], path: Vec<bool>) {
    match node {
        HuffNode::Leaf { byte, .. } => {
            table[*byte as usize] = Some(path);
        }
        HuffNode::Internal { left, right, .. } => {
            let mut left_path = path.clone();
            left_path.push(false);
            traverse(left, table, left_path);

            let mut right_path = path;
            right_path.push(true);
            traverse(right, table, right_path);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree;

    fn opendsa_tree() -> HuffNode {
        let mut freq = [0u64; 256];
        freq[b'C' as usize] = 32;
        freq[b'D' as usize] = 42;
        freq[b'E' as usize] = 120;
        freq[b'K' as usize] = 7;
        freq[b'L' as usize] = 42;
        freq[b'M' as usize] = 24;
        freq[b'U' as usize] = 37;
        freq[b'Z' as usize] = 2;
        tree::build_tree(&freq)
    }

    // Single symbol: no ambiguity possible, so the code is empty []
    #[test]
    fn test_single_byte_code_is_empty() {
        let mut freq = [0u64; 256];
        freq[b'a' as usize] = 5;
        let root = tree::build_tree(&freq);
        let table = build_table(&root);
        assert_eq!(table[b'a' as usize], Some(vec![]));
    }

    // Two symbols: one gets [false] (left/0) and the other [true] (right/1)
    #[test]
    fn test_two_bytes_get_single_bit_codes() {
        let mut freq = [0u64; 256];
        freq[b'a' as usize] = 3;
        freq[b'b' as usize] = 7;
        let root = tree::build_tree(&freq);
        let table = build_table(&root);

        let code_a = table[b'a' as usize].as_ref().expect("'a' must have a code");
        let code_b = table[b'b' as usize].as_ref().expect("'b' must have a code");

        assert_eq!(code_a.len(), 1);
        assert_eq!(code_b.len(), 1);
        assert_ne!(code_a, code_b);
    }

    // E (freq=120) must have a shorter code than Z (freq=2)
    #[test]
    fn test_high_freq_gets_shorter_code() {
        let root = opendsa_tree();
        let table = build_table(&root);

        let len_e = table[b'E' as usize].as_ref().expect("E must have a code").len();
        let len_z = table[b'Z' as usize].as_ref().expect("Z must have a code").len();

        assert!(len_e < len_z, "E (freq=120) must have a shorter code than Z (freq=2)");
    }

    // All 8 OpenDSA bytes must have Some(...)
    #[test]
    fn test_all_present_bytes_have_some() {
        let root = opendsa_tree();
        let table = build_table(&root);

        for byte in [b'C', b'D', b'E', b'K', b'L', b'M', b'U', b'Z'] {
            assert!(
                table[byte as usize].is_some(),
                "byte '{}' must have a code in the table",
                byte as char
            );
        }
    }

    // Bytes absent from the tree must return None
    #[test]
    fn test_absent_bytes_have_none() {
        let root = opendsa_tree();
        let table = build_table(&root);

        // 'A' is not in the OpenDSA set
        assert!(table[b'A' as usize].is_none());
        assert!(table[b'X' as usize].is_none());
        assert!(table[0].is_none()); // null byte
    }

    // No code may be a prefix of another (prefix-free property)
    #[test]
    fn test_prefix_free_property() {
        let root = opendsa_tree();
        let table = build_table(&root);

        let codes: Vec<&Vec<bool>> = table.iter().filter_map(|e| e.as_ref()).collect();

        for (i, a) in codes.iter().enumerate() {
            for (j, b) in codes.iter().enumerate() {
                if i == j {
                    continue;
                }
                let is_prefix = a.len() <= b.len() && b.starts_with(a.as_slice());
                assert!(
                    !is_prefix,
                    "code {:?} is a prefix of {:?} — prefix-free property violated",
                    a, b
                );
            }
        }
    }
}
