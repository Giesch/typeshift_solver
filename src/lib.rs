const MIN_WORD_LEN: usize = 4;
const MAX_WORD_LEN: usize = 6;

pub fn create_index() -> Vec<u8> {
    let dict = load_dictionary();
    let mut bytes = Vec::with_capacity(dict.len() * MAX_WORD_LEN);

    for word in dict {
        bytes.extend_from_slice(word.as_bytes());
        bytes.push(b'\n');
    }

    bytes
}

pub unsafe fn load_index() -> Vec<&'static str> {
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
