use std::cmp::{Ordering, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap};

use crate::dict::DICT;

/// An unsolved Typeshift puzzle
#[derive(Debug)]
pub struct Typeshift {
    /// The rotated or inverted puzzle input columns
    /// eg, the first inner set of this would be the leftmost column of the puzzle
    columns: Vec<BTreeSet<char>>,

    /// A dictionary of usable words, reduced to only words spellable from the input
    words: Vec<&'static str>,

    /// The total frequencies of characters in the reduced problem dictionary
    char_freqs: BTreeMap<char, usize>,
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
            .filter(|word| word.len() == columns.len())
            .filter(|word| {
                word.chars()
                    .zip(columns.iter())
                    .all(|(ch, col)| col.contains(&ch))
            })
            .map(|word| *word)
            .collect();

        let mut char_freqs: BTreeMap<char, usize> = Default::default();
        let all_chars = words.iter().flat_map(|word| word.chars());
        for ch in all_chars {
            let entry = char_freqs.entry(ch).or_default();
            *entry += 1;
        }

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
            .expect("no solutions found")
            .len();

        let mut all_smallest: BTreeSet<_> = complete_solutions
            .into_iter()
            .filter(|sol| sol.len() == minimum_size)
            .collect();

        dbg!(&all_smallest);

        (all_smallest.pop_first().unwrap(), steps)
    }
}

/// A sortable wrapper for comparing the quality of partial solutions
struct RankedSolution<'a>(PartialSolution<'a>);

impl<'a> RankedSolution<'a> {
    /// Returns a tuple for sorting solutions by priority when solving
    /// for use in a max-heap; higher is better
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
    /// A count of 0 is guaranteed to be present for unused characters in the column.
    char_usages: Vec<BTreeMap<char, usize>>,
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
    fn new(typeshift: &'a Typeshift) -> Self {
        let char_usages: Vec<BTreeMap<char, usize>> = typeshift
            .columns
            .iter()
            .map(|col| BTreeMap::from_iter(col.iter().map(|ch| (*ch, 0))))
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
        let best_rank = ranked_words.first().unwrap().1;

        ranked_words
            .into_iter()
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
        word.chars()
            .zip(self.char_usages.iter())
            .map(|(ch, usages)| *usages.get(&ch).unwrap())
            .filter(|&usages| usages == 0)
            .count()
    }

    /// Returns the lowest dict frequency among the letters in the word
    fn min_char_freq(&self, word: &'static str) -> usize {
        *word
            .chars()
            .map(|ch| self.typeshift.char_freqs.get(&ch).unwrap())
            .min()
            .unwrap()
    }

    /// Add a word to the solution, updating used character counts
    fn add_word(&mut self, word: &'static str) {
        for (col, word_ch) in word.char_indices() {
            let entry = self.char_usages[col].entry(word_ch).or_default();
            *entry += 1;
        }

        self.used_words.insert(word);
    }

    /// Returns true if no characters are unused
    fn solved(&self) -> bool {
        self.char_usages
            .iter()
            .flat_map(|usages| usages.values())
            .all(|&count| count > 0)
    }

    /// Returns the total characters the solution uses more than once
    fn overlaps(&self) -> usize {
        self.char_usages
            .iter()
            .flat_map(|usages| usages.values())
            .filter(|&&count| count > 1)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::BTreeSet;

    use pretty_assertions::assert_eq;

    /// a small input that should stay fast
    #[test]
    fn small_example() {
        let input = include_str!("../files/puzzles/2023-11-16.txt");
        let solution = ["above", "basic", "study", "wheel", "whups"];
        let steps = 8;

        test_input(input, solution, steps);
    }

    /// the largest input and slowest puzzle so far
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
        let (solution, steps) = typeshift.find_best_solution();

        assert_eq!(steps, expected_steps);
        assert_eq!(solution, expected_solution.into());
    }
}
