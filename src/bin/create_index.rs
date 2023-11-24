fn main() {
    // TODO
    // write an 'inner index' module with raw constants,
    // then write a nice-to-use wrapper for it
    let _index = load_index();
}

fn load_index() -> Vec<Vec<(&'static str, [usize; 26])>> {
    let file = include_str!("../../files/wordlist-20210729.txt");

    let dict: Vec<_> = file
        .lines()
        .map(|l| l.strip_prefix('"').unwrap())
        .map(|l| l.strip_suffix('"').unwrap())
        .map(|word| (word, count_chars(word)))
        .collect();

    // TODO
    // this 2d vec can't be converted to a static array,
    // because the inner vecs are of different lengths
    // is there a good way to normalize them?
    //   arrays of options of the max len?
    //   export all 28 individually and have a wrapper module/macro?
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
