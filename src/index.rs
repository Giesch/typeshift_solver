const MIN_WORD_LEN: usize = 4;
const MAX_WORD_LEN: usize = 6;

/// Writes a smaller version of the wordnik dictionary to disk
/// to be included as bytes in the main program
pub fn create() -> Vec<u8> {
    let dict = load_dictionary();
    let mut bytes = Vec::with_capacity(dict.len() * MAX_WORD_LEN);

    for word in dict {
        bytes.extend_from_slice(word.as_bytes());
        bytes.push(b'\n');
    }

    bytes
}

/// Loads the previously written index at compile time
///
/// # Safety
/// Assumes that the previously written file is utf8
pub unsafe fn load() -> Vec<&'static str> {
    let bytes = include_bytes!("../files/index.dat");
    let words = std::str::from_utf8_unchecked(bytes);

    words.lines().collect()
}

fn load_dictionary() -> Vec<&'static str> {
    let file = include_str!("../files/wordlist-20210729.txt");

    file.lines()
        .map(|l| l.strip_prefix('"').unwrap())
        .map(|l| l.strip_suffix('"').unwrap())
        .filter(|w| w.len() >= MIN_WORD_LEN && w.len() <= MAX_WORD_LEN)
        .collect()
}
