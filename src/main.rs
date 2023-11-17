use std::collections::{BTreeMap, BTreeSet};
use std::time::Instant;

// TODO
// improve next_words ranking heuristics (rare letters?)
// rank partial solutions somehow, and use a priority queue instead of BFS
// look for more trimming improvements
// include a filtered 5-letter dictionary as straight bytes
//   also index it ahead of time somehow?
// do real benchmark testing; maybe switch to snapshots for unit tests
// try preserving & updating ranking state/index instead of reranking

fn main() {
    let start = Instant::now();

    let words = load_dictionary();
    let input = include_str!("../files/puzzle-11-17-2023.txt");
    let columns = into_columns(input);
    let after_load = Instant::now();

    let typeshift = Typeshift::new(columns, words);
    let (solution, steps) = typeshift.find_best_solution();
    let after_solve = Instant::now();

    dbg!(solution);
    dbg!(steps);

    let load = after_load.duration_since(start);
    let solve = after_solve.duration_since(after_load);
    let total = after_solve.duration_since(start);

    dbg!(load);
    dbg!(solve);
    dbg!(total);
}

fn load_dictionary() -> Vec<&'static str> {
    let file = include_str!("../files/wordlist-20210729.txt");

    file.lines()
        .map(|l| l.strip_prefix('"').unwrap())
        .map(|l| l.strip_suffix('"').unwrap())
        .collect()
}

/// An unsolved Typeshift puzzle
#[derive(Debug)]
struct Typeshift {
    /// The rotated or inverted puzzle input columns
    /// eg, the first inner vec of this would be the leftmost column of the puzzle
    columns: Vec<Vec<char>>,
    /// A dictionary of usable words, reduced to match the input
    words: Vec<&'static str>,
}

impl Typeshift {
    /// Returns a new filtered dictionary from a puzzle input and the loaded full-size dictionary.
    /// Includes only (and all) words that can be made with the puzzle input columns.
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

    /// Returns the first minimal solution found using breadth-first-search & heuristics,
    /// and the number of intermediate partial solutions touched along the way.
    fn find_best_solution(&self) -> (BTreeSet<&'static str>, usize) {
        let optimal_solution_size = self.columns.iter().map(Vec::len).max().unwrap();

        let mut steps: usize = 0;
        let mut partial_solutions = vec![PartialSolution::new(self)];
        let mut complete_solutions: BTreeSet<BTreeSet<&'static str>> = BTreeSet::new();

        while let Some(mut partial_solution) = partial_solutions.pop() {
            steps += 1;

            if partial_solution.solved() {
                let words = partial_solution.used_words;
                if words.len() == optimal_solution_size {
                    // NOTE comment this out to find & log all minimal solutions
                    return (words, steps);
                }

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

        let minimum_size = complete_solutions
            .iter()
            .min_by_key(|set| set.len())
            .unwrap()
            .len();

        let mut all_smallest: BTreeSet<_> = complete_solutions
            .into_iter()
            .filter(|sol| sol.len() == minimum_size)
            .collect();

        dbg!(&all_smallest);

        (all_smallest.pop_first().unwrap(), steps)
    }
}

#[derive(Clone)]
struct PartialSolution<'a> {
    typeshift: &'a Typeshift,
    /// the words in the solution so far
    used_words: BTreeSet<&'static str>,
    /// the current total usages of a positional character from the input grid
    char_usages: Vec<BTreeMap<char, usize>>,
    /// the words we should ignore, because they're in a sibling solution
    trimmed_words: BTreeSet<&'static str>,
}

// deliberately omitting the word list just to make output shorter
impl<'a> std::fmt::Debug for PartialSolution<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PartialSolution")
            .field("used_words", &self.used_words)
            .field("char_usages", &self.char_usages)
            .field("trimmed_words", &self.trimmed_words)
            .finish()
    }
}

impl<'a> PartialSolution<'a> {
    fn new(typeshift: &'a Typeshift) -> Self {
        let char_usages: Vec<BTreeMap<char, usize>> = typeshift
            .columns
            .iter()
            .map(|_| Default::default())
            .collect();

        Self {
            typeshift,
            used_words: Default::default(),
            char_usages,
            trimmed_words: Default::default(),
        }
    }

    /// Ranks all untrimmed words, and returns all tied for best.
    /// Returns an emtpy Vec if this solution should be abandoned.
    fn next_words(&mut self) -> Vec<&'static str> {
        let mut ranked_words = Vec::new();
        for &word in &self.typeshift.words {
            let mut score: usize = 0;
            for (col, ch) in word.chars().enumerate() {
                let usages = *self.char_usages[col].get(&ch).unwrap_or(&0);
                if usages == 0 {
                    score += 1;
                }
            }

            ranked_words.push((word, score));
        }

        ranked_words.sort_by_key(|(_word, score)| *score);
        let max_score = ranked_words.last().unwrap().1;

        return ranked_words
            .into_iter()
            .rev()
            .take_while(|(_word, score)| *score == max_score)
            .map(|(word, _score)| word)
            // NOTE it's important to do this after scoring, instead of before;
            // this leads to better trimming by returning an empty Vec
            // if the sibling solutions are better than this one
            .filter(|word| !self.trimmed_words.contains(word))
            .collect();
    }

    fn add_word(&mut self, word: &'static str) {
        debug_assert!(!self.trimmed_words.contains(word));

        for (col, word_ch) in word.char_indices() {
            let entry = self.char_usages[col].entry(word_ch).or_default();
            *entry += 1;
        }

        self.used_words.insert(word);
    }

    fn solved(&mut self) -> bool {
        for (col, col_chars) in self.typeshift.columns.iter().enumerate() {
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

/// Converts an input file of a rotated/inverted typeshift
/// into a char table to be solved.
fn into_columns(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::BTreeSet;

    #[test]
    fn nov_16_2023() {
        let input = include_str!("../files/puzzle-11-16-2023.txt");
        let expected_solution = ["above", "basic", "steel", "study", "whups"];
        let max_steps = 201;

        test_input(input, expected_solution, max_steps);
    }

    #[test]
    fn nov_17_2023() {
        let input = include_str!("../files/puzzle-11-17-2023.txt");
        let expected_solution = ["again", "gater", "mouth", "quick", "woods"];
        let max_steps = 4567;

        test_input(input, expected_solution, max_steps);
    }

    fn test_input(
        input: &str,
        expected_solution: impl Into<BTreeSet<&'static str>>,
        max_steps: usize,
    ) {
        let words = load_dictionary();
        let columns = into_columns(input);

        let typeshift = Typeshift::new(columns, words);
        let (solution, steps) = typeshift.find_best_solution();

        assert_eq!(solution, expected_solution.into());
        assert!(steps <= max_steps);
    }
}
