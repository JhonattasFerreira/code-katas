use std::process::{Command, Stdio};
use std::io::Write;

fn binary() -> Command {
    Command::new(env!("CARGO_BIN_EXE_ccwc"))
}

fn run_with_stdin(args: &[&str], input: &[u8]) -> std::process::Output {
    let mut child = binary()
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(input).unwrap();
    child.wait_with_output().unwrap()
}

// --- flag -c ---

#[test]
fn flag_c_counts_bytes_in_file() {
    let output = binary().args(["-c", "test.txt"]).output().unwrap();
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "342190 test.txt");
}

#[test]
fn flag_c_reads_from_stdin_when_no_file() {
    let output = run_with_stdin(&["-c"], b"hello");
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "5");
}

// --- flag -l ---

#[test]
fn flag_l_counts_lines_in_file() {
    let output = binary().args(["-l", "test.txt"]).output().unwrap();
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "7145 test.txt");
}

#[test]
fn flag_l_reads_from_stdin_when_no_file() {
    let output = run_with_stdin(&["-l"], b"hello\nworld\n");
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "2");
}

// --- flag -w ---

#[test]
fn flag_w_counts_words_in_file() {
    let output = binary().args(["-w", "test.txt"]).output().unwrap();
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "58164 test.txt");
}

#[test]
fn flag_w_reads_from_stdin_when_no_file() {
    let output = run_with_stdin(&["-w"], b"hello world\n");
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "2");
}

// --- flag -m ---

#[test]
fn flag_m_counts_chars_in_file() {
    let output = binary().args(["-m", "test.txt"]).output().unwrap();
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "339292 test.txt");
}

#[test]
fn flag_m_counts_multibyte_chars_from_stdin() {
    // "café" = 4 chars, 5 bytes
    let output = run_with_stdin(&["-m"], "café".as_bytes());
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "4");
}

// --- default mode (no flag) ---

#[test]
fn default_mode_outputs_lines_words_bytes_for_file() {
    let output = binary().arg("test.txt").output().unwrap();
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        "7145  58164 342190 test.txt"
    );
}

#[test]
fn default_mode_reads_from_stdin_when_no_args() {
    // "hello\n" -> 1 line, 1 word, 6 bytes
    let output = run_with_stdin(&[], b"hello\n");
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "1      1      6");
}

// --- múltiplas flags ---

#[test]
fn flags_l_and_w_with_file() {
    let output = binary().args(["-l", "-w", "test.txt"]).output().unwrap();
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        "7145  58164 test.txt"
    );
}

#[test]
fn flags_l_and_c_with_file() {
    let output = binary().args(["-l", "-c", "test.txt"]).output().unwrap();
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        "7145 342190 test.txt"
    );
}

#[test]
fn flags_l_and_m_with_file() {
    let output = binary().args(["-l", "-m", "test.txt"]).output().unwrap();
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        "7145 339292 test.txt"
    );
}

#[test]
fn flags_w_and_c_with_file() {
    let output = binary().args(["-w", "-c", "test.txt"]).output().unwrap();
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        "58164 342190 test.txt"
    );
}

#[test]
fn flags_order_does_not_affect_output() {
    // A ordem das flags não muda a ordem das colunas na saída
    let out_lw = binary().args(["-l", "-w", "test.txt"]).output().unwrap();
    let out_wl = binary().args(["-w", "-l", "test.txt"]).output().unwrap();
    assert_eq!(out_lw.stdout, out_wl.stdout);
}

#[test]
fn flags_l_and_w_from_stdin() {
    // "hello world\n" -> 1 linha, 2 palavras
    let output = run_with_stdin(&["-l", "-w"], b"hello world\n");
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "1      2");
}

// --- erros ---

#[test]
fn unknown_flag_exits_with_error() {
    let output = binary().args(["-z", "test.txt"]).output().unwrap();
    assert!(!output.status.success());
    assert!(!output.stderr.is_empty());
}

#[test]
fn nonexistent_file_exits_with_error() {
    let output = binary().args(["-c", "nope.txt"]).output().unwrap();
    assert!(!output.status.success());
    assert!(!output.stderr.is_empty());
}
