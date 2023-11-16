fn main() {
    let words = load_dictionary();

    let columns = vec![
        vec!['w', 's', 'a', 'b'],
        vec!['h', 'b', 't', 'a'],
        vec!['o', 'e', 's', 'u'],
        vec!['d', 'p', 'i', 'v', 'e'],
        vec!['l', 'c', 'e', 'y', 's'],
    ];

    let index = Index::new(columns, words);

    let solution = index.find_first_solution();

    dbg!(solution);

    // TODO
    //
    // Find the best (most minimal) solution by brute force
    //
    // Find the best solution in a faster way
}

fn load_dictionary() -> Vec<&'static str> {
    let file = include_str!("../files/wordlist-20210729.txt");

    file.lines()
        .map(|l| l.strip_prefix('"').unwrap())
        .map(|l| l.strip_suffix('"').unwrap())
        .collect()
}

struct Index {
    columns: Vec<Vec<char>>,
    words: Vec<UsableWord>,
}

struct UsableWord {
    text: &'static str,
    used_indicies: Vec<usize>,
}

impl Index {
    fn new(columns: Vec<Vec<char>>, words: Vec<&'static str>) -> Self {
        let words: Vec<UsableWord> = words
            .iter()
            .filter(|&word| word.len() == columns.len())
            .filter_map(|word| {
                let mut used_indicies = Vec::new();
                for (i, word_ch) in word.char_indices() {
                    if !columns[i].contains(&word_ch) {
                        return None;
                    } else {
                        used_indicies.push(i);
                    }
                }

                Some(UsableWord {
                    text: &word,
                    used_indicies,
                })
            })
            .collect();

        Self { columns, words }
    }

    /// Returns the first solution in the dictionary.
    /// The solution is probably not minimal, but will contain no fully unnecessary words.
    fn find_first_solution(&self) -> Vec<&'static str> {
        // a bool grid matching the input shape,
        // tracking whether each character has been used
        let mut checkboxes: Vec<Vec<bool>> = self
            .columns
            .iter()
            .map(|c| c.iter().map(|_| false).collect())
            .collect();

        let mut solution = Vec::new();
        for word in &self.words {
            // fill in appropriate checkboxes
            let mut word_useful = false;
            for (col, word_ch) in word.text.char_indices() {
                // NOTE, this unwrap relies on the filtering above to only usable words
                let row = self.columns[col]
                    .iter()
                    .position(|&col_ch| col_ch == word_ch)
                    .unwrap();

                if !checkboxes[col][row] {
                    checkboxes[col][row] = true;
                    word_useful = true;
                }
            }

            if word_useful {
                solution.push(word.text);
            }

            // check if solved and exit early
            if checkboxes.iter().flatten().all(|&checked| checked) {
                break;
            }
        }

        solution
    }
}
