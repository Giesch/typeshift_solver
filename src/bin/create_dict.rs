use std::fs::File;
use std::io::{BufWriter, Write};

/// Writes a length-filtered wordnik dictionary as a rust module,
/// which avoids string processing and file io in the main binary.
fn main() {
    let dict = load_dictionary();

    let mut buf = String::new();
    buf.push_str(HEADER);
    buf.push_str("pub static DICT: [&str; ");
    buf.push_str(&dict.len().to_string());
    buf.push_str("] = [\n");
    for word in dict {
        buf.push_str("    \"");
        buf.push_str(word);
        buf.push_str("\",\n");
    }
    buf.push_str("];");

    let file = File::create("./src/dict.rs").unwrap();
    let mut file = BufWriter::new(file);

    file.write_all(buf.as_bytes()).unwrap();
}

const MIN_WORD_LEN: usize = 4;
const MAX_WORD_LEN: usize = 6;

const HEADER: &str = r#"//! THIS IS A GENERATED FILE
//! Do not edit it directly; see src/bin/create_dict.rs

/// The reduced wordnik dictionary
"#;

fn load_dictionary() -> Vec<&'static str> {
    let file = include_str!("../../files/wordlist-20210729.txt");

    file.lines()
        .map(|l| l.strip_prefix('"').unwrap())
        .map(|l| l.strip_suffix('"').unwrap())
        .filter(|w| w.len() >= MIN_WORD_LEN && w.len() <= MAX_WORD_LEN)
        .collect()
}
