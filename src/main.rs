use std::collections::{BTreeMap, BTreeSet};
use std::time::Instant;

// TODO
// include input from a file or read stdin
// use old puzzles as test cases; include max partial solutions touched as output
//
// improve next_words ranking heuristics (rare letters?)
// prepare a cached index for 5 letter words?
// rank partial solutions somehow, and use a priority queue instead of BFS
// try preserving & updating ranking state/index instead of reranking
// look for more trimming improvements
// include a filtered 5-letter dictionary as straight bytes
// do real benchmark testing

fn main() {
    let start = Instant::now();

    let words = load_dictionary();

    let after_dict_load = Instant::now();

    // let columns = vec![
    //     vec!['w', 's', 'a', 'b'],
    //     vec!['h', 'b', 't', 'a'],
    //     vec!['o', 'e', 's', 'u'],
    //     vec!['d', 'p', 'i', 'v', 'e'],
    //     vec!['l', 'c', 'e', 'y', 's'],
    // ];

    let columns = vec![
        vec!['q', 'a', 'g', 'w', 'm'],
        vec!['o', 'g', 'a', 'u'],
        vec!['u', 'a', 'i', 't', 'o'],
        vec!['i', 'd', 'e', 't', 'c'],
        vec!['h', 's', 'n', 'r', 'k'],
    ];

    let typeshift = Typeshift::new(columns, words);

    let after_dict_filter = Instant::now();

    let best_solution = typeshift.find_best_solution();

    let after_solve = Instant::now();

    println!("best solution: {best_solution:#?}");

    let after_print = Instant::now();

    let dict_load = after_dict_load.duration_since(start);
    let dict_filter = after_dict_filter.duration_since(after_dict_load);
    let solve = after_solve.duration_since(after_dict_filter);
    let print = after_print.duration_since(after_solve);
    let total = after_print.duration_since(start);

    dbg!(dict_load);
    dbg!(dict_filter);
    dbg!(solve);
    dbg!(print);
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
struct Typeshift {
    /// The 'rotated' or 'inverted' puzzle input columns
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

    /// Returns the first solution in the dictionary.
    /// The solution is probably not minimal, but will contain no fully unnecessary words.
    #[allow(unused)]
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
                let row = self.columns[col]
                    .iter()
                    .position(|&col_ch| col_ch == word_ch)
                    .unwrap(); // relies on filtering in Typeshift::new

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

    /// Returns the first minimal solution found using breadth-first-search & heuristics.
    /// If there is no minimal solution, then returns one of the best ones.
    fn find_best_solution(&self) -> Vec<&'static str> {
        let optimal_solution_size = self.columns.iter().map(Vec::len).max().unwrap();
        let mut partial_solutions = vec![PartialSolution::new(self)];
        let mut complete_solutions: BTreeSet<BTreeSet<&'static str>> = BTreeSet::new();

        let mut partial_solutions_touched = 0;

        while let Some(mut partial_solution) = partial_solutions.pop() {
            partial_solutions_touched += 1;

            if partial_solution.solved() {
                let words = partial_solution.used_words.into_iter();
                if words.len() == optimal_solution_size {
                    dbg!(partial_solutions_touched);
                    return Vec::from_iter(words);
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

        dbg!(partial_solutions_touched);

        let smallest = complete_solutions
            .iter()
            .min_by_key(|set| set.len())
            .unwrap();

        let minimum_size = smallest.len();

        let all_smallest: BTreeSet<_> = complete_solutions
            .into_iter()
            .filter(|sol| sol.len() == minimum_size)
            .collect();

        dbg!(&all_smallest);

        Vec::from_iter(all_smallest.first().unwrap().into_iter().map(|s| *s))
    }
}

#[derive(Clone)]
struct PartialSolution<'a> {
    dict: &'a Typeshift,
    /// the words in the solution so far
    used_words: BTreeSet<&'static str>,
    /// the current total usages of a positional character from the input grid
    char_usages: Vec<BTreeMap<char, usize>>,
    /// the words we should ignore, because they're in a sibling solution
    trimmed_words: BTreeSet<&'static str>,
}

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
    fn new(dict: &'a Typeshift) -> Self {
        let char_usages: Vec<BTreeMap<char, usize>> =
            dict.columns.iter().map(|_| Default::default()).collect();

        Self {
            dict,
            used_words: Default::default(),
            char_usages,
            trimmed_words: Default::default(),
        }
    }

    /// rank all untrimmed words in the dict, and return all tied for best
    fn next_words(&mut self) -> Vec<&'static str> {
        let mut ranked_words = Vec::new();
        for &word in &self.dict.words {
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
        for (col, col_chars) in self.dict.columns.iter().enumerate() {
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
