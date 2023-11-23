use std::fs::File;
use std::io::{BufWriter, Write};

/// Generates an organized/preprocessed version of the wordnik dictionary as a rust module.
/// This avoids both file IO and string processing in the main binary.
fn main() {
    let by_len = load_index();

    let mut buf = String::new();
    for row in by_len {
        if row.is_empty() {
            continue;
        }

        let export = format_index_row(&row);
        buf.push_str(&export);
    }

    let file = File::create("./src/index/raw_index.rs").unwrap();
    let mut file = BufWriter::new(file);

    file.write_all(buf.as_bytes()).unwrap();
}

fn load_index() -> Vec<Vec<(&'static str, [usize; 26])>> {
    let file = include_str!("../../files/wordlist-20210729.txt");

    let dict: Vec<_> = file
        .lines()
        .map(|l| l.strip_prefix('"').unwrap())
        .map(|l| l.strip_suffix('"').unwrap())
        .map(|word| (word, count_chars(word)))
        .collect();

    let mut by_len: Vec<Vec<(&'static str, CharCounts)>> = vec![vec![]; 28];
    for (word, counts) in dict {
        let row = &mut by_len[word.len() - 1];
        row.push((word, counts));
    }

    by_len
}

type CharCounts = [usize; 26];

fn count_chars(word: &'static str) -> CharCounts {
    let mut char_freqs = [0usize; 26];
    for ch in word.chars() {
        let i = (ch as u8 - b'a') as usize;
        let entry = &mut char_freqs[i];
        *entry += 1;
    }

    char_freqs
}

// NOTE panics on empty slice
fn format_index_row(row: &[(&'static str, CharCounts)]) -> String {
    let word_len = row[0].0.len();
    let size = row.len();

    let mut buf = String::new();
    let decl = format!("pub static INDEX_{word_len}: [(&'static str, [usize; 26]); {size}] = [\n");
    buf.push_str(&decl);

    for (word, counts) in row {
        let line = format!("    (\"{word}\", {counts:?}),\n");
        buf.push_str(&line);
    }

    buf.push_str("];\n\n");

    buf
}
