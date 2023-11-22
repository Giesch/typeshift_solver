use std::fs::File;
use std::io::{BufWriter, Write};

/// Writes a length-filtered wordnik dictionary as a rust module,
/// which avoids string processing and file io in the main binary.
fn main() {
    let dict = load_dictionary();

    let mut buf = String::new();
    buf.push_str(MODULE_DOC);

    buf.push_str("/// The reduced wordnik dictionary\n");
    buf.push_str("pub static DICT: [&str; ");
    buf.push_str(&dict.len().to_string());
    buf.push_str("] = [\n");
    for word in dict {
        let line = format!("    \"{word}\",\n");
        buf.push_str(&line);
    }
    buf.push_str("];");

    let file = File::create("./src/dict.rs").unwrap();
    let mut file = BufWriter::new(file);

    file.write_all(buf.as_bytes()).unwrap();
}

const MIN_WORD_LEN: usize = 4;
const MAX_WORD_LEN: usize = 7;

const MODULE_DOC: &str = r#"//! THIS IS A GENERATED FILE
//! Do not edit it directly; see src/bin/create_dict.rs

"#;

fn load_dictionary() -> Vec<&'static str> {
    let file = include_str!("../../files/wordlist-20210729.txt");

    file.lines()
        .map(|l| l.strip_prefix('"').unwrap())
        .map(|l| l.strip_suffix('"').unwrap())
        .filter(|w| w.len() >= MIN_WORD_LEN && w.len() <= MAX_WORD_LEN)
        .collect()
}
