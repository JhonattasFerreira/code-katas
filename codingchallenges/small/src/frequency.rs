pub fn count(data: &[u8]) -> [u64; 256] {
    let mut freq = [0u64; 256];
    for &byte in data {
        freq[byte as usize] += 1;
    }
    freq
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input_returns_all_zeros() {
        let freq = count(&[]);
        assert_eq!(freq, [0u64; 256]);
    }

    #[test]
    fn test_single_byte() {
        let freq = count(&[b'a']);
        assert_eq!(freq[b'a' as usize], 1);
        assert_eq!(freq.iter().sum::<u64>(), 1);
    }

    #[test]
    fn test_all_same_bytes() {
        let data = vec![b'z'; 1000];
        let freq = count(&data);
        assert_eq!(freq[b'z' as usize], 1000);
        assert_eq!(freq.iter().sum::<u64>(), 1000);
    }

    #[test]
    fn test_multiple_distinct_bytes() {
        let freq = count(b"aabbc");
        assert_eq!(freq[b'a' as usize], 2);
        assert_eq!(freq[b'b' as usize], 2);
        assert_eq!(freq[b'c' as usize], 1);
        assert_eq!(freq.iter().sum::<u64>(), 5);
    }

    #[test]
    fn test_all_256_byte_values_appear_once() {
        let data: Vec<u8> = (0u8..=255).collect();
        let freq = count(&data);
        for i in 0..=255 {
            assert_eq!(freq[i], 1, "byte {} should appear exactly once", i);
        }
    }

    #[test]
    fn test_high_bytes_non_ascii_are_counted() {
        // bytes acima de 127 são válidos (ex: UTF-8 multibyte)
        let data = vec![0xC3u8, 0xA9u8, 0xC3u8]; // "é" em UTF-8 + byte extra
        let freq = count(&data);
        assert_eq!(freq[0xC3], 2);
        assert_eq!(freq[0xA9], 1);
        assert_eq!(freq.iter().sum::<u64>(), 3);
    }

    #[test]
    fn test_null_byte_is_counted() {
        let data = vec![0u8, 0u8, 1u8];
        let freq = count(&data);
        assert_eq!(freq[0], 2);
        assert_eq!(freq[1], 1);
    }

    #[test]
    fn test_total_count_equals_input_length() {
        let data = b"Hello, World!";
        let freq = count(data);
        assert_eq!(freq.iter().sum::<u64>(), data.len() as u64);
    }

    #[test]
    fn test_only_bytes_present_in_input_are_nonzero() {
        let freq = count(b"abc");
        for i in 0..=255usize {
            if i != b'a' as usize && i != b'b' as usize && i != b'c' as usize {
                assert_eq!(freq[i], 0, "byte {} should be zero", i);
            }
        }
    }
}
