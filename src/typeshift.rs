use std::collections::{BTreeMap, BTreeSet, VecDeque};

/// An unsolved Typeshift puzzle
#[derive(Debug)]
pub struct Typeshift {
    /// The rotated or inverted puzzle input columns
    /// eg, the first inner set of this would be the leftmost column of the puzzle
    columns: Vec<BTreeSet<char>>,
    /// A dictionary of usable words, reduced to match the input
    words: Vec<&'static str>,
}

impl Typeshift {
    /// Returns a new filtered dictionary from a puzzle input and the loaded full-size dictionary.
    /// Includes only (and all) words that can be made with the puzzle input columns.
    pub fn new(columns: Vec<BTreeSet<char>>, words: Vec<&'static str>) -> Self {
        let words: Vec<&'static str> = words
            .iter()
            .filter(|&word| word.len() == columns.len())
            .filter_map(|word| {
                for (i, word_ch) in word.char_indices() {
                    if !columns[i].iter().any(|&ch| ch == word_ch) {
                        return None;
                    }
                }

                Some(*word)
            })
            .collect();

        Self { columns, words }
    }

    /// Returns the first minimal solution found,
    /// and the number of intermediate partial solutions touched along the way.
    /// Uses DFS while there are completely unused words remaining, and BFS afterwards.
    pub fn find_best_solution(&self) -> (BTreeSet<&'static str>, usize) {
        let optimal_solution_size = self.columns.iter().map(|c| c.len()).max().unwrap();

        let mut steps: usize = 0;
        let mut partial_solutions = VecDeque::from_iter([PartialSolution::new(self)]);
        let mut complete_solutions: BTreeSet<BTreeSet<&'static str>> = BTreeSet::new();

        while let Some(mut partial_solution) = partial_solutions.pop_front() {
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

            let (mut next_words, score) = partial_solution.next_words();
            let all_letters_used = score == self.columns.len();
            while let Some(next_word) = next_words.pop() {
                let mut partial_solution = partial_solution.clone();

                partial_solution.add_word(next_word);

                for remaining_word in &next_words {
                    partial_solution.trimmed_words.insert(remaining_word);
                }

                if all_letters_used {
                    partial_solutions.push_front(partial_solution);
                } else {
                    partial_solutions.push_back(partial_solution);
                }
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
    fn next_words(&mut self) -> (Vec<&'static str>, usize) {
        let ranked_words = self.rank_words();
        let max_score = ranked_words.last().unwrap().1;

        let max_score_words = ranked_words
            .into_iter()
            .rev()
            .take_while(|(_word, score)| *score == max_score)
            .map(|(word, _score)| word)
            // NOTE it's important to do this after scoring, instead of before;
            // this leads to better trimming by returning an empty vec
            // if the best next_words vec is in a sibling solution
            .filter(|word| !self.trimmed_words.contains(word))
            .collect();

        (max_score_words, max_score)
    }

    /// Score and sort all possible words in the typeshift
    /// by how many unused characters they would use,
    /// in ascending order (worst first)
    fn rank_words(&self) -> Vec<(&'static str, usize)> {
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

        ranked_words
    }

    fn add_word(&mut self, word: &'static str) {
        debug_assert!(!self.trimmed_words.contains(word));

        for (col, word_ch) in word.char_indices() {
            let entry = self.char_usages[col].entry(word_ch).or_default();
            *entry += 1;
        }

        self.used_words.insert(word);
    }

    fn solved(&self) -> bool {
        for (col, col_chars) in self.typeshift.columns.iter().enumerate() {
            let usages = &self.char_usages[col];
            for &ch in col_chars {
                let count = *usages.get(&ch).unwrap_or(&0);
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
pub fn into_columns(input: &str) -> Vec<BTreeSet<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::index;

    use std::collections::BTreeSet;

    #[test]
    fn nov_16_2023() {
        let input = include_str!("../files/puzzle-11-16-2023.txt");
        let expected_solution = ["above", "basic", "steel", "study", "whups"];
        let expected_steps = 274;

        test_input(input, expected_solution, expected_steps);
    }

    #[test]
    fn nov_17_2023() {
        let input = include_str!("../files/puzzle-11-17-2023.txt");
        let expected_solution = ["again", "gater", "mouth", "quick", "woods"];
        let expected_steps = 1019;

        test_input(input, expected_solution, expected_steps);
    }

    #[test]
    fn nov_18_2023() {
        let input = include_str!("../files/puzzle-11-18-2023.txt");
        let expected_solution = ["backup", "fridge", "heists", "lender"];
        let expected_steps = 150;

        test_input(input, expected_solution, expected_steps);
    }

    fn test_input(
        input: &str,
        expected_solution: impl Into<BTreeSet<&'static str>>,
        expected_steps: usize,
    ) {
        let words = unsafe { index::load() };
        let columns = into_columns(input);

        let typeshift = Typeshift::new(columns, words);
        let (solution, steps) = typeshift.find_best_solution();

        assert_eq!(steps, expected_steps);
        assert_eq!(solution, expected_solution.into());
    }
}
