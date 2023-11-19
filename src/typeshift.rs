use std::cmp::{Ordering, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap};

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
    pub fn find_best_solution(&self) -> (BTreeSet<&'static str>, usize) {
        let minimum_words = self.columns.iter().map(|c| c.len()).max().unwrap();

        let mut steps: usize = 0;
        let mut ranked_solutions =
            BinaryHeap::from_iter([RankedSolution(PartialSolution::new(self))]);
        let mut complete_solutions: BTreeSet<BTreeSet<&'static str>> = Default::default();
        let mut attempted_solutions: BTreeSet<BTreeSet<&'static str>> = Default::default();

        while let Some(RankedSolution(mut partial_solution)) = ranked_solutions.pop() {
            steps += 1;

            if partial_solution.solved() {
                let words = partial_solution.used_words;
                if words.len() == minimum_words {
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
                if attempted_solutions.contains(&partial_solution.used_words) {
                    continue;
                }

                ranked_solutions.push(RankedSolution(partial_solution));
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

struct RankedSolution<'a>(PartialSolution<'a>);

impl<'a> RankedSolution<'a> {
    fn rank_tuple(&self) -> (bool, Reverse<usize>, usize) {
        (
            self.0.solved(),            // a finished solution comes first
            Reverse(self.0.overlaps()), // more efficient solutions rank more highly
            self.0.used_words.len(),    // efficient solutions closer to completion rank more highly
        )
    }
}

impl<'a> Ord for RankedSolution<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.rank_tuple().cmp(&other.rank_tuple())
    }
}

impl<'a> PartialOrd for RankedSolution<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> PartialEq for RankedSolution<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.rank_tuple().eq(&other.rank_tuple())
    }
}

impl<'a> Eq for RankedSolution<'a> {}

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
    fn next_words(&mut self) -> Vec<&'static str> {
        let ranked_words = self.rank_words();
        let max_score = ranked_words.last().unwrap().1;

        ranked_words
            .into_iter()
            .rev()
            .take_while(|(_word, score)| *score == max_score)
            .map(|(word, _score)| word)
            .collect()
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

    fn overlaps(&self) -> usize {
        let mut overlaps: usize = 0;

        for (i, chars) in self.typeshift.columns.iter().enumerate() {
            let usages = &self.char_usages[i];
            for ch in chars {
                if matches!(usages.get(ch), Some(&count) if count > 1) {
                    overlaps += 1
                }
            }
        }

        overlaps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::BTreeSet;

    /// a small input; this one was previously faster,
    /// but that might depend on brittle changes
    #[test]
    fn small_example() {
        let input = include_str!("../files/puzzles/2023-11-16.txt");
        let solution = ["above", "basic", "steel", "study", "whups"];
        let steps = 63;

        test_input(input, solution, steps);
    }

    /// the largest input so far
    #[test]
    fn large_example() {
        let input = include_str!("../files/puzzles/2023-11-19.txt");
        let solution = ["chumps", "corves", "fifers", "granny", "poiser"];
        let steps = 304;

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
