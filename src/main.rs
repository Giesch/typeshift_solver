use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;

fn main() {
    let words = load_dictionary();

    // TODO read this from a file or stdin
    let columns = vec![
        vec!['w', 's', 'a', 'b'],
        vec!['h', 'b', 't', 'a'],
        vec!['o', 'e', 's', 'u'],
        vec!['d', 'p', 'i', 'v', 'e'],
        vec!['l', 'c', 'e', 'y', 's'],
    ];

    let index = Index::new(columns, words);

    let first_solution = index.find_first_solution();
    println!("first solution: {first_solution:#?}");

    let best_solution = index.find_best_solution();
    println!("best solution: {best_solution:#?}");
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
    words: Vec<&'static str>,
}

impl Index {
    fn new(columns: Vec<Vec<char>>, words: Vec<&'static str>) -> Self {
        let words: Vec<&'static str> = words
            .iter()
            .filter(|&word| word.len() == columns.len())
            .filter_map(|word| {
                for (i, word_ch) in word.char_indices() {
                    if columns[i].iter().position(|&ch| ch == word_ch).is_none() {
                        return None;
                    }
                }

                Some(*word)
            })
            .collect();

        Self { columns, words }
    }

    // TODO make this better; actually index
    fn lookup_matches(&self, col: usize, ch: char) -> Vec<&'static str> {
        self.words
            .iter()
            .filter(|&word| word.chars().nth(col) == Some(ch))
            .map(|word| *word)
            .collect()
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
        for &word in &self.words {
            // fill in appropriate checkboxes
            let mut word_useful = false;
            for (col, word_ch) in word.char_indices() {
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
                solution.push(word);
            }

            // check if solved and exit early
            if checkboxes.iter().flatten().all(|&checked| checked) {
                break;
            }
        }

        solution
    }

    /// iteratively find the best solution using backtracking & heuristics
    fn find_best_solution(&self) -> Vec<&'static str> {
        let input_col_lens: Vec<_> = self.columns.iter().map(|c| c.len()).collect();
        let mut partial_solutions = vec![PartialSolution::new(self, &input_col_lens)];
        let mut complete_solutions: BTreeSet<BTreeSet<&'static str>> = BTreeSet::new();

        while let Some(mut partial_solution) = partial_solutions.pop() {
            if partial_solution.solved() {
                let words = partial_solution.used_words.into_iter();
                complete_solutions.insert(BTreeSet::from_iter(words));
                continue;
            }

            let mut next_words = partial_solution.next_words();

            while let Some(next_word) = next_words.pop() {
                let mut partial_solution = partial_solution.clone();

                partial_solution.add_word(next_word);

                for &remaining_word in &next_words {
                    partial_solution.trimmed_words.insert(remaining_word);
                }

                partial_solutions.push(partial_solution);
            }
        }

        let smallest = complete_solutions
            .into_iter()
            .min_by_key(|set| set.len())
            .unwrap();

        Vec::from_iter(smallest.into_iter())
    }
}

#[derive(Clone)]
struct PartialSolution<'a> {
    index: &'a Index,
    /// the words in the solution so far
    used_words: BTreeSet<&'static str>,
    /// the current total usages of a positional character from the input grid
    char_usages: Vec<BTreeMap<char, usize>>,
    /// the words we should ignore, because they're in a sibling solution
    trimmed_words: BTreeSet<&'static str>,
}

impl<'a> Debug for PartialSolution<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PartialSolution")
            .field("used_words", &self.used_words)
            .field("char_usages", &self.char_usages)
            .field("trimmed_words", &self.trimmed_words)
            .finish()
    }
}

impl<'a> PartialSolution<'a> {
    fn new(index: &'a Index, input_col_lens: &[usize]) -> Self {
        let input_cols = input_col_lens.len();
        let char_usages: Vec<BTreeMap<char, usize>> =
            (0..input_cols).map(|_| Default::default()).collect();

        Self {
            index,
            used_words: Default::default(),
            char_usages,
            trimmed_words: Default::default(),
        }
    }

    /// 'rank' all untrimmed words in the index, and return all tied for 'best'
    fn next_words(&mut self) -> Vec<&'static str> {
        // first, find the column with the least NONZERO unfilled usage rows (defaulting to the first one)
        // we want the 'narrowest' to reduce the size of the decision tree
        //   TODO it might be better to do this instead by using the size of the potential dictionary matches
        let mut col_with_min_unused = None;
        for (col, column) in self.index.columns.iter().enumerate() {
            let col_usages = &mut self.char_usages[col];

            let mut unfilled_row_count = 0;
            for &ch in column {
                let count = *col_usages.entry(ch).or_default();
                if count == 0 {
                    unfilled_row_count += 1;
                }
            }

            let was_nonzero = unfilled_row_count > 0;
            let lower_than_previous = col_with_min_unused
                .map(|(_, previous)| unfilled_row_count < previous)
                .unwrap_or(true);
            if was_nonzero && lower_than_previous {
                col_with_min_unused = Some((col, unfilled_row_count));
            }
        }

        // if there was no non-zero minimum, then this is a complete solution
        let Some((col, _)) = col_with_min_unused else {
            return vec![];
        };

        let mut lookups = Vec::new();
        let col_usages = &self.char_usages[col];
        let column = &self.index.columns[col];

        for ch in column {
            let usage_count = *col_usages.get(ch).unwrap_or(&0);
            if usage_count == 0 {
                lookups.push((col, ch));
            }
        }

        let matches: BTreeSet<&'static str> = lookups
            .into_iter()
            .map(|(col, ch)| self.index.lookup_matches(col, *ch))
            .flatten()
            .collect();

        matches
            .difference(&self.trimmed_words)
            .map(|p| *p)
            .collect()
    }

    fn add_word(&mut self, word: &'static str) {
        debug_assert!(!self.trimmed_words.contains(word));

        // update usages
        for (col, word_ch) in word.char_indices() {
            let entry = self.char_usages[col].entry(word_ch).or_default();
            *entry += 1;
        }

        self.used_words.insert(word);
    }

    fn solved(&mut self) -> bool {
        for (col, col_chars) in self.index.columns.iter().enumerate() {
            let usages = &mut self.char_usages[col];
            for &ch in col_chars {
                let count = *usages.entry(ch).or_default();
                if count == 0 {
                    return false;
                }
            }
        }

        true
    }
}
