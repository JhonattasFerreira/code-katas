use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[allow(dead_code)]
pub enum HuffNode {
    Leaf {
        byte: u8,
        freq: u64,
    },
    Internal {
        freq: u64,
        left: Box<HuffNode>,
        right: Box<HuffNode>,
    },
}

impl HuffNode {
    #[cfg(test)]
    pub fn freq(&self) -> u64 {
        match self {
            HuffNode::Leaf { freq, .. } => *freq,
            HuffNode::Internal { freq, .. } => *freq,
        }
    }
}

struct HeapEntry {
    freq: u64,
    node: HuffNode,
}

impl PartialEq for HeapEntry {
    fn eq(&self, other: &Self) -> bool {
        self.freq == other.freq
    }
}

impl Eq for HeapEntry {}

impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.freq.cmp(&other.freq)
    }
}

pub fn build_tree(freq: &[u64; 256]) -> HuffNode {
    let mut heap: BinaryHeap<Reverse<HeapEntry>> = BinaryHeap::new();

    for (byte, &count) in freq.iter().enumerate() {
        if count > 0 {
            heap.push(Reverse(HeapEntry {
                freq: count,
                node: HuffNode::Leaf {
                    byte: byte as u8,
                    freq: count,
                },
            }));
        }
    }

    while heap.len() > 1 {
        let Reverse(left) = heap.pop().unwrap();
        let Reverse(right) = heap.pop().unwrap();
        let combined = left.freq + right.freq;
        heap.push(Reverse(HeapEntry {
            freq: combined,
            node: HuffNode::Internal {
                freq: combined,
                left: Box::new(left.node),
                right: Box::new(right.node),
            },
        }));
    }

    heap.pop().unwrap().0.node
}

#[cfg(test)]
mod tests {
    use super::*;

    fn opendsa_freq() -> [u64; 256] {
        let mut freq = [0u64; 256];
        freq[b'C' as usize] = 32;
        freq[b'D' as usize] = 42;
        freq[b'E' as usize] = 120;
        freq[b'K' as usize] = 7;
        freq[b'L' as usize] = 42;
        freq[b'M' as usize] = 24;
        freq[b'U' as usize] = 37;
        freq[b'Z' as usize] = 2;
        freq
    }

    fn depth_of(node: &HuffNode, target: u8) -> Option<usize> {
        match node {
            HuffNode::Leaf { byte, .. } => {
                if *byte == target {
                    Some(0)
                } else {
                    None
                }
            }
            HuffNode::Internal { left, right, .. } => depth_of(left, target)
                .map(|d| d + 1)
                .or_else(|| depth_of(right, target).map(|d| d + 1)),
        }
    }

    #[test]
    fn test_single_byte_produces_leaf_root() {
        let mut freq = [0u64; 256];
        freq[b'a' as usize] = 5;
        let root = build_tree(&freq);
        assert!(matches!(
            root,
            HuffNode::Leaf {
                byte: b'a',
                freq: 5
            }
        ));
    }

    #[test]
    fn test_two_bytes_produces_internal_root() {
        let mut freq = [0u64; 256];
        freq[b'a' as usize] = 3;
        freq[b'b' as usize] = 7;
        let root = build_tree(&freq);
        assert!(matches!(root, HuffNode::Internal { freq: 10, .. }));
    }

    #[test]
    fn test_root_freq_equals_sum_of_all_freqs() {
        let root = build_tree(&opendsa_freq());
        assert_eq!(root.freq(), 306);
    }

    #[test]
    fn test_zero_freq_bytes_are_ignored() {
        let mut freq = [0u64; 256];
        freq[b'x' as usize] = 10;
        let root = build_tree(&freq);
        assert!(matches!(root, HuffNode::Leaf { byte: b'x', .. }));
    }

    #[test]
    fn test_high_freq_has_lower_depth_than_low_freq() {
        let root = build_tree(&opendsa_freq());
        let depth_e = depth_of(&root, b'E').expect("E should be in the tree");
        let depth_z = depth_of(&root, b'Z').expect("Z should be in the tree");
        assert!(
            depth_e < depth_z,
            "E (freq=120) should be shallower than Z (freq=2)"
        );
    }

    #[test]
    fn test_all_nonzero_bytes_are_reachable() {
        let root = build_tree(&opendsa_freq());
        for byte in [b'C', b'D', b'E', b'K', b'L', b'M', b'U', b'Z'] {
            assert!(
                depth_of(&root, byte).is_some(),
                "byte '{}' should be reachable in the tree",
                byte as char
            );
        }
    }
}
