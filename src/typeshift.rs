use std::cmp::{Ordering, Reverse};
use std::collections::{BTreeSet, BinaryHeap};
use std::iter::zip;

use crate::dict::DICT;

mod collections;
use collections::*;

/// An unsolved Typeshift puzzle
#[derive(Debug)]
pub struct Typeshift {
    /// The rotated or inverted puzzle input columns
    /// The first inner set is the leftmost column of the puzzle.
    columns: Vec<LetterSet>,

    /// A dictionary of usable words, reduced to only words spellable from the input
    words: Vec<&'static str>,

    /// The total frequencies of characters in the reduced problem dictionary
    char_freqs: LetterCounts,
}

impl Typeshift {
    /// Returns a new filtered dictionary from a puzzle input.
    /// Includes only (and all) words that can be made with the puzzle input columns.
    /// Expects input as a rotated or inverted set of lines:
    /// The leftmost column of the puzzle should be the first line of input.
    pub fn new(input: &str) -> Self {
        let columns: Vec<_> = input
            .lines()
            .map(|l| LetterSet::from_iter(l.chars()))
            .collect();

        let words: Vec<&'static str> = DICT
            .iter()
            .filter(|word| word.len() == columns.len())
            .filter(|word| zip(word.chars(), columns.iter()).all(|(ch, col)| col.contains(ch)))
            .copied()
            .collect();

        let char_freqs = LetterCounts::from_iter(words.iter().flat_map(|word| word.chars()));

        Self {
            columns,
            words,
            char_freqs,
        }
    }

    /// The number of possible words (and size of the solution space)
    pub fn size(&self) -> usize {
        self.words.len()
    }

    /// Returns the first minimal solution found,
    /// and the number of intermediate partial solutions touched along the way.
    pub fn find_first_solution(&self) -> (BTreeSet<&'static str>, usize) {
        let (mut solutions, steps) = self.solve(SolveMode::FindFirst);
        (solutions.pop_first().unwrap(), steps)
    }

    /// Returns the set of all minimal solutions,
    /// and the number of intermediate partial solutions touched along the way.
    pub fn find_all_solutions(&self) -> (BTreeSet<BTreeSet<&'static str>>, usize) {
        self.solve(SolveMode::FindAll)
    }

    fn solve(&self, mode: SolveMode) -> (BTreeSet<BTreeSet<&'static str>>, usize) {
        let mut steps: usize = 0;
        let mut to_check = BinaryHeap::from_iter([RankedSolution(PartialSolution::empty(self))]);
        let mut complete: BTreeSet<BTreeSet<&'static str>> = Default::default();
        let mut attempted: BTreeSet<BTreeSet<&'static str>> = Default::default();

        while let Some(RankedSolution(mut partial_solution)) = to_check.pop() {
            steps += 1;

            if partial_solution.solved() {
                let words = partial_solution.used_words;

                match mode {
                    SolveMode::FindFirst => {
                        return (BTreeSet::from_iter([words]), steps);
                    }
                    SolveMode::FindAll => {
                        complete.insert(words);
                        continue;
                    }
                }
            }

            let mut next_words = partial_solution.next_words();
            while let Some(next_word) = next_words.pop() {
                let mut partial_solution = partial_solution.clone();

                partial_solution.add_word(next_word);
                if attempted.contains(&partial_solution.used_words) {
                    continue;
                }

                to_check.push(RankedSolution(partial_solution));
            }

            attempted.insert(partial_solution.used_words);
        }

        let minimum_size = complete
            .iter()
            .min_by_key(|set| set.len())
            .expect("no solutions found")
            .len();

        let all_smallest: BTreeSet<_> = complete
            .into_iter()
            .filter(|sol| sol.len() == minimum_size)
            .collect();

        (all_smallest, steps)
    }
}

/// Whether to find the first minimal solution or all minimal solutions
#[derive(Default, Debug, Clone, Copy)]
enum SolveMode {
    /// Find the first minimal solution
    #[default]
    FindFirst,
    /// Find all minimal solutions
    FindAll,
}

/// A sortable wrapper for comparing the quality of partial solutions
struct RankedSolution<'a>(PartialSolution<'a>);

impl<'a> RankedSolution<'a> {
    /// Returns a tuple for sorting solutions by priority when solving
    /// For use in a max-heap; higher is better
    fn rank(&self) -> impl Ord + Copy {
        (
            self.0.solved(),            // a finished solution comes first
            Reverse(self.0.overlaps()), // more efficient solutions rank more highly
            self.0.used_words.len(),    // efficient solutions closer to completion rank more highly
        )
    }
}

impl<'a> Ord for RankedSolution<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.rank().cmp(&other.rank())
    }
}

impl<'a> PartialOrd for RankedSolution<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> PartialEq for RankedSolution<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.rank().eq(&other.rank())
    }
}

impl<'a> Eq for RankedSolution<'a> {}

#[derive(Clone)]
struct PartialSolution<'a> {
    typeshift: &'a Typeshift,

    /// The words in the solution so far
    used_words: BTreeSet<&'static str>,

    /// The current total usages of a positional character from the input grid
    char_usages: Vec<LetterCounts>,
}

// deliberately omitting the word list just to make output shorter
impl<'a> std::fmt::Debug for PartialSolution<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PartialSolution")
            .field("used_words", &self.used_words)
            .field("char_usages", &self.char_usages)
            .finish()
    }
}

impl<'a> PartialSolution<'a> {
    fn empty(typeshift: &'a Typeshift) -> Self {
        Self {
            typeshift,
            used_words: Default::default(),
            char_usages: vec![LetterCounts::new(); typeshift.columns.len()],
        }
    }

    /// Ranks all words, and returns all tied for best.
    fn next_words(&mut self) -> Vec<&'static str> {
        let ranked_words = self.rank_words();
        let best_rank = ranked_words.first().unwrap().1;

        ranked_words
            .into_iter()
            // TODO this overtrims and can fail to find all possible solutions
            .take_while(|(_word, rank)| *rank == best_rank)
            .map(|(word, _rank)| word)
            .collect()
    }

    /// Rank all possible words for usage as the next word in the solution (best first),
    /// by how many unused characters they would use,
    /// and the rarity of their rarest letter.
    fn rank_words(&self) -> Vec<(&'static str, impl Ord + Copy)> {
        let mut ranked_words = Vec::new();
        for &word in &self.typeshift.words {
            // for sorting; lower is better
            let rank = (
                // using more new letters is better
                Reverse(self.new_letters(word)),
                // a rarest letter with fewer usages is better
                self.min_char_freq(word),
            );

            ranked_words.push((word, rank));
        }

        ranked_words.sort_by_key(|(_word, rank)| *rank);

        ranked_words
    }

    /// Returns the number of unused letters the word would use
    fn new_letters(&self, word: &'static str) -> usize {
        zip(word.chars(), self.char_usages.iter())
            .map(|(ch, counts)| counts.get(ch))
            .filter(|&count| count == 0)
            .count()
    }

    /// Returns the lowest dict frequency among the letters in the word
    fn min_char_freq(&self, word: &'static str) -> usize {
        word.chars()
            .map(|ch| self.typeshift.char_freqs.get(ch))
            .min()
            .unwrap()
    }

    /// Add a word to the solution, updating used character counts
    fn add_word(&mut self, word: &'static str) {
        for (col, word_ch) in word.char_indices() {
            self.char_usages[col].add(word_ch);
        }

        self.used_words.insert(word);
    }

    /// Returns true if all characters are used at least once
    fn solved(&self) -> bool {
        self.included_char_counts().all(|c| c > 0)
    }

    /// Returns the total number of characters the solution uses more than once
    fn overlaps(&self) -> usize {
        self.included_char_counts().filter(|&c| c > 1).count()
    }

    /// Iterates over all char usage counts included in the input problem
    fn included_char_counts(&self) -> impl Iterator<Item = usize> + '_ {
        zip(self.typeshift.columns.iter(), self.char_usages.iter())
            .flat_map(|(col, counts)| col.filter_counts(counts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::BTreeSet;

    use pretty_assertions::assert_eq;

    /// A small input that should stay fast
    #[test]
    fn small_example() {
        let input = include_str!("../files/puzzles/2023-11-16.txt");
        let solution = ["above", "basic", "study", "wheel", "whups"];
        let steps = 8;

        test_input(input, solution, steps);
    }

    /// The largest input with a single solution (by this dictionary and algorithm);
    /// The slowest puzzle so far
    #[test]
    fn large_example() {
        let input = include_str!("../files/puzzles/2023-11-19.txt");
        let solution = ["chumps", "corves", "fifers", "granny", "poiser"];
        let steps = 67;

        test_input(input, solution, steps);
    }

    fn test_input(
        input: &str,
        expected_solution: impl Into<BTreeSet<&'static str>>,
        expected_steps: usize,
    ) {
        let typeshift = Typeshift::new(input);
        let (solution, steps) = typeshift.find_first_solution();

        assert_eq!(steps, expected_steps);
        assert_eq!(solution, expected_solution.into());
    }
}
