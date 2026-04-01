use std::io::{self, Read};

pub struct Counts {
    pub lines: u64,
    pub words: u64,
    pub bytes: u64,
    pub chars: u64,
}

fn read_chunks(mut reader: impl Read, mut process: impl FnMut(&[u8])) -> io::Result<()> {
    let mut buf = [0u8; 8192];
    loop {
        let n = reader.read(&mut buf)?;
        if n == 0 {
            break;
        }
        process(&buf[..n]);
    }
    Ok(())
}

pub fn count_all(reader: impl Read) -> io::Result<Counts> {
    let mut lines = 0u64;
    let mut words = 0u64;
    let mut bytes = 0u64;
    let mut chars = 0u64;
    let mut in_word = false;
    read_chunks(reader, |chunk| {
        bytes += chunk.len() as u64;
        chars += chunk.iter().filter(|&&b| !matches!(b, 0x80..=0xBF)).count() as u64;
        for &b in chunk {
            if b == b'\n' {
                lines += 1;
            }
            if in_word && b.is_ascii_whitespace() {
                in_word = false;
            } else if !in_word && !b.is_ascii_whitespace() {
                in_word = true;
                words += 1;
            }
        }
    })?;
    Ok(Counts { lines, words, bytes, chars })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn all(input: &[u8]) -> Counts {
        count_all(Cursor::new(input)).unwrap()
    }

    // --- bytes ---

    #[test]
    fn bytes_empty_input_returns_zero() {
        assert_eq!(all(b"").bytes, 0);
    }

    #[test]
    fn bytes_ascii_only_equals_char_count() {
        assert_eq!(all(b"hello").bytes, 5);
    }

    #[test]
    fn bytes_newline_counts_as_one_byte() {
        assert_eq!(all(b"hello\nworld").bytes, 11);
    }

    #[test]
    fn bytes_multibyte_utf8_counts_bytes_not_chars() {
        // "café" = 5 bytes (é is 2 bytes)
        assert_eq!(all("café".as_bytes()).bytes, 5);
    }

    #[test]
    fn bytes_counts_all_bytes_including_null() {
        assert_eq!(all(b"ab\x00cd").bytes, 5);
    }

    // --- lines ---

    #[test]
    fn lines_empty_input_returns_zero() {
        assert_eq!(all(b"").lines, 0);
    }

    #[test]
    fn lines_single_line_no_trailing_newline() {
        // wc counts newlines, not lines — "hello" has 0 newlines
        assert_eq!(all(b"hello").lines, 0);
    }

    #[test]
    fn lines_single_newline() {
        assert_eq!(all(b"hello\n").lines, 1);
    }

    #[test]
    fn lines_multiple_newlines() {
        assert_eq!(all(b"hello\nworld\n").lines, 2);
    }

    #[test]
    fn lines_no_trailing_newline_counts_only_internal() {
        // "hello\nworld" has 1 newline
        assert_eq!(all(b"hello\nworld").lines, 1);
    }

    // --- words ---

    #[test]
    fn words_empty_input_returns_zero() {
        assert_eq!(all(b"").words, 0);
    }

    #[test]
    fn words_single_word() {
        assert_eq!(all(b"hello").words, 1);
    }

    #[test]
    fn words_multiple_words_single_space() {
        assert_eq!(all(b"hello world").words, 2);
    }

    #[test]
    fn words_multiple_spaces_between_words() {
        assert_eq!(all(b"  hello   world  ").words, 2);
    }

    #[test]
    fn words_newlines_are_word_separators() {
        assert_eq!(all(b"hello\nworld\n").words, 2);
    }

    #[test]
    fn words_only_whitespace_returns_zero() {
        assert_eq!(all(b"   \n  \t  ").words, 0);
    }

    // --- chars ---

    #[test]
    fn chars_empty_input_returns_zero() {
        assert_eq!(all(b"").chars, 0);
    }

    #[test]
    fn chars_ascii_same_as_bytes() {
        assert_eq!(all(b"hello").chars, 5);
    }

    #[test]
    fn chars_multibyte_counts_chars_not_bytes() {
        // "café" = 4 chars, 5 bytes
        assert_eq!(all("café".as_bytes()).chars, 4);
    }

    #[test]
    fn chars_cjk_three_chars_nine_bytes() {
        // each CJK char is 3 bytes in UTF-8
        assert_eq!(all("日本語".as_bytes()).chars, 3);
    }
}
