pub struct Flags {
    pub lines: bool,
    pub words: bool,
    pub bytes: bool,
    pub chars: bool,
}

pub struct Args {
    pub flags: Flags,
    pub filename: Option<String>,
}

pub fn parse_args(args: &[String]) -> Result<Args, String> {
    let mut flags = Flags { lines: false, words: false, bytes: false, chars: false };
    let mut filename = None;
    let mut any_flag = false;

    for arg in args {
        if arg.starts_with('-') {
            any_flag = true;
            match arg.as_str() {
                "-c" => flags.bytes = true,
                "-l" => flags.lines = true,
                "-w" => flags.words = true,
                "-m" => flags.chars = true,
                other => return Err(format!("unknown flag: {other}")),
            }
        } else {
            filename = Some(arg.clone());
        }
    }

    if !any_flag {
        flags.lines = true;
        flags.words = true;
        flags.bytes = true;
    }

    Ok(Args { flags, filename })
}

pub fn format_counts(
    flags: &Flags,
    lines: u64,
    words: u64,
    chars: u64,
    bytes: u64,
    filename: Option<&str>,
) -> String {
    let mut nums = String::new();
    if flags.lines { nums.push_str(&format!("{lines:>7}")); }
    if flags.words { nums.push_str(&format!("{words:>7}")); }
    if flags.chars { nums.push_str(&format!("{chars:>7}")); }
    if flags.bytes { nums.push_str(&format!("{bytes:>7}")); }
    match filename {
        Some(name) => format!("{nums} {name}"),
        None => nums,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(val: &str) -> String {
        val.to_string()
    }

    // --- parse_args: single flag ---

    #[test]
    fn parses_flag_c_with_filename() {
        let args = vec![s("-c"), s("test.txt")];
        let result = parse_args(&args).unwrap();
        assert!(result.flags.bytes);
        assert!(!result.flags.lines);
        assert!(!result.flags.words);
        assert!(!result.flags.chars);
        assert_eq!(result.filename, Some(s("test.txt")));
    }

    #[test]
    fn parses_flag_c_without_filename_means_stdin() {
        let args = vec![s("-c")];
        let result = parse_args(&args).unwrap();
        assert!(result.flags.bytes);
        assert_eq!(result.filename, None);
    }

    #[test]
    fn parses_flag_l_with_filename() {
        let args = vec![s("-l"), s("test.txt")];
        let result = parse_args(&args).unwrap();
        assert!(result.flags.lines);
        assert!(!result.flags.bytes);
        assert_eq!(result.filename, Some(s("test.txt")));
    }

    #[test]
    fn parses_flag_w_with_filename() {
        let args = vec![s("-w"), s("test.txt")];
        let result = parse_args(&args).unwrap();
        assert!(result.flags.words);
        assert!(!result.flags.bytes);
        assert_eq!(result.filename, Some(s("test.txt")));
    }

    #[test]
    fn parses_flag_m_with_filename() {
        let args = vec![s("-m"), s("test.txt")];
        let result = parse_args(&args).unwrap();
        assert!(result.flags.chars);
        assert!(!result.flags.bytes);
        assert_eq!(result.filename, Some(s("test.txt")));
    }

    #[test]
    fn parses_filename_only_means_default_mode() {
        let args = vec![s("test.txt")];
        let result = parse_args(&args).unwrap();
        assert!(result.flags.lines);
        assert!(result.flags.words);
        assert!(result.flags.bytes);
        assert!(!result.flags.chars);
        assert_eq!(result.filename, Some(s("test.txt")));
    }

    #[test]
    fn parses_no_args_means_default_stdin() {
        let args: Vec<String> = vec![];
        let result = parse_args(&args).unwrap();
        assert!(result.flags.lines);
        assert!(result.flags.words);
        assert!(result.flags.bytes);
        assert!(!result.flags.chars);
        assert_eq!(result.filename, None);
    }

    #[test]
    fn returns_error_on_unknown_flag() {
        let args = vec![s("-z"), s("test.txt")];
        assert!(parse_args(&args).is_err());
    }

    // --- parse_args: multiple flags ---

    #[test]
    fn parses_flags_l_and_w_with_filename() {
        let args = vec![s("-l"), s("-w"), s("test.txt")];
        let result = parse_args(&args).unwrap();
        assert!(result.flags.lines);
        assert!(result.flags.words);
        assert!(!result.flags.bytes);
        assert!(!result.flags.chars);
        assert_eq!(result.filename, Some(s("test.txt")));
    }

    #[test]
    fn parses_flags_l_and_c_with_filename() {
        let args = vec![s("-l"), s("-c"), s("test.txt")];
        let result = parse_args(&args).unwrap();
        assert!(result.flags.lines);
        assert!(result.flags.bytes);
        assert!(!result.flags.words);
        assert_eq!(result.filename, Some(s("test.txt")));
    }

    #[test]
    fn parses_flags_l_and_m_with_filename() {
        let args = vec![s("-l"), s("-m"), s("test.txt")];
        let result = parse_args(&args).unwrap();
        assert!(result.flags.lines);
        assert!(result.flags.chars);
        assert!(!result.flags.bytes);
        assert_eq!(result.filename, Some(s("test.txt")));
    }

    #[test]
    fn parses_flags_in_any_order_same_result() {
        let args_a = vec![s("-l"), s("-w"), s("test.txt")];
        let args_b = vec![s("-w"), s("-l"), s("test.txt")];
        let a = parse_args(&args_a).unwrap();
        let b = parse_args(&args_b).unwrap();
        assert_eq!(a.flags.lines, b.flags.lines);
        assert_eq!(a.flags.words, b.flags.words);
        assert_eq!(a.flags.bytes, b.flags.bytes);
        assert_eq!(a.flags.chars, b.flags.chars);
    }

    #[test]
    fn parses_multiple_flags_without_filename() {
        let args = vec![s("-l"), s("-w")];
        let result = parse_args(&args).unwrap();
        assert!(result.flags.lines);
        assert!(result.flags.words);
        assert_eq!(result.filename, None);
    }

    // --- format_counts ---

    fn flags(lines: bool, words: bool, chars: bool, bytes: bool) -> Flags {
        Flags { lines, words, chars, bytes }
    }

    #[test]
    fn format_single_bytes_with_filename() {
        let output = format_counts(&flags(false, false, false, true), 0, 0, 0, 342190, Some("test.txt"));
        assert_eq!(output, " 342190 test.txt");
    }

    #[test]
    fn format_single_lines_without_filename() {
        let output = format_counts(&flags(true, false, false, false), 7145, 0, 0, 0, None);
        assert_eq!(output, "   7145");
    }

    #[test]
    fn format_zero_count() {
        let output = format_counts(&flags(false, false, false, true), 0, 0, 0, 0, Some("empty.txt"));
        assert_eq!(output, "      0 empty.txt");
    }

    #[test]
    fn format_default_mode_with_filename() {
        // default: lines + words + bytes
        let output = format_counts(&flags(true, true, false, true), 7145, 58164, 0, 342190, Some("test.txt"));
        assert_eq!(output, "   7145  58164 342190 test.txt");
    }

    #[test]
    fn format_default_mode_without_filename() {
        let output = format_counts(&flags(true, true, false, true), 0, 0, 0, 0, None);
        assert_eq!(output, "      0      0      0");
    }

    #[test]
    fn format_fixed_order_lines_words_chars_bytes() {
        // ordem sempre fixa: lines → words → chars → bytes
        let output = format_counts(&flags(true, true, true, true), 1, 2, 3, 4, None);
        assert_eq!(output, "      1      2      3      4");
    }
}
