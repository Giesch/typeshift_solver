use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::dict::DICT;

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
    /// Returns a new filtered dictionary from a puzzle input.
    /// Includes only (and all) words that can be made with the puzzle input columns.
    /// Expects input as a rotated or inverted set of lines:
    /// The leftmost column of the puzzle should be the first line of input.
    pub fn new(input: &str) -> Self {
        let columns: Vec<BTreeSet<char>> = input.lines().map(|l| l.chars().collect()).collect();

        let words: Vec<&'static str> = DICT
            .iter()
            .filter(|&word| word.len() == columns.len())
            .filter_map(|word| {
                for (i, word_ch) in word.char_indices() {
                    if !columns[i].contains(&word_ch) {
                        return None;
                    }
                }

                Some(*word)
            })
            .collect();

        Self { columns, words }
    }

    /// The number of possible words (and size of the solution space)
    pub fn size(&self) -> usize {
        self.words.len()
    }

    /// Returns the first minimal solution found,
    /// and the number of intermediate partial solutions touched along the way.
    /// Uses DFS while there are completely unused words remaining, and BFS afterwards.
    pub fn find_best_solution(&self) -> (BTreeSet<&'static str>, usize) {
        // the longest column determines the minimal solution
        let optimal_solution_size = self.columns.iter().map(|c| c.len()).max().unwrap();

        let mut steps: usize = 0;
        let mut partial_solutions = VecDeque::from_iter([PartialSolution::new(self)]);
        let mut complete_solutions: BTreeSet<BTreeSet<&'static str>> = Default::default();
        let mut attempted_solutions: BTreeSet<BTreeSet<&'static str>> = Default::default();

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
                if attempted_solutions.contains(&partial_solution.used_words) {
                    continue;
                }

                if partial_solution.solved() || all_letters_used {
                    partial_solutions.push_front(partial_solution);
                } else {
                    partial_solutions.push_back(partial_solution);
                }
            }

            attempted_solutions.insert(partial_solution.used_words);
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
}

// deliberately omitting the word list just to make output shorter
impl<'a> std::fmt::Debug for PartialSolution<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PartialSolution")
            .field("used_words", &self.used_words)
            .field("char_usages", &self.char_usages)
            .field("typeshift.size", &self.typeshift.size())
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
        }
    }

    /// Ranks all words, and returns all tied for best.
    /// Returns an emtpy Vec if this solution should be abandoned.
    fn next_words(&mut self) -> (Vec<&'static str>, usize) {
        let ranked_words = self.rank_words();
        let max_score = ranked_words.last().unwrap().1;

        let max_score_words = ranked_words
            .into_iter()
            .rev()
            .take_while(|(_word, score)| *score == max_score)
            .map(|(word, _score)| word)
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
        for (col, word_ch) in word.char_indices() {
            let entry = self.char_usages[col].entry(word_ch).or_default();
            *entry += 1;
        }

        self.used_words.insert(word);
    }

    fn solved(&self) -> bool {
        for (i, chars) in self.typeshift.columns.iter().enumerate() {
            let usages = &self.char_usages[i];
            for ch in chars {
                let count = *usages.get(ch).unwrap_or(&0);
                if count == 0 {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::BTreeSet;

    #[test]
    fn nov_16_2023() {
        let input = include_str!("../files/puzzles/2023-11-16.txt");
        let solution = ["above", "basic", "steel", "study", "whups"];
        let steps = 10;

        test_input(input, solution, steps);
    }

    #[test]
    fn nov_17_2023() {
        let input = include_str!("../files/puzzles/2023-11-17.txt");
        let solution = ["again", "gater", "mouth", "quick", "woods"];
        let steps = 113;

        test_input(input, solution, steps);
    }

    #[test]
    fn nov_18_2023() {
        let input = include_str!("../files/puzzles/2023-11-18.txt");
        let solution = ["backup", "fridge", "heists", "lender"];
        let steps = 100;

        test_input(input, solution, steps);
    }

    #[test]
    fn nov_19_2023() {
        let input = include_str!("../files/puzzles/2023-11-19.txt");
        let solution = ["chumps", "corves", "fifers", "granny", "poiser"];
        let steps = 456;

        test_input(input, solution, steps);
    }

    fn test_input(
        input: &str,
        expected_solution: impl Into<BTreeSet<&'static str>>,
        expected_steps: usize,
    ) {
        let typeshift = Typeshift::new(input);
        let (solution, steps) = typeshift.find_best_solution();

        assert_eq!(steps, expected_steps);
        assert_eq!(solution, expected_solution.into());
    }
}
