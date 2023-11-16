fn main() {
    let words = load_dictionary();

    let columns = vec![
        vec!['w', 's', 'a', 'b'],
        vec!['h', 'b', 't', 'a'],
        vec!['o', 'e', 's', 'u'],
        vec!['d', 'p', 'i', 'v', 'e'],
        vec!['l', 'c', 'e', 'y', 's'],
    ];

    let solution = first_solution(&columns, &words);

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

/// Returns the first solution in the dictionary.
/// The solution is probably not minimal, but will contain no fully unnecessary words.
///
/// * `columns` - the problem to solve
/// * `words` - the dictionary
fn first_solution<'a>(columns: &'a [Vec<char>], words: &'a [&'static str]) -> Vec<&'a str> {
    // filter the dictionary down to words that could possibly be used
    let words: Vec<&str> = words
        .iter()
        .map(|s| *s)
        .filter(|&word| word.len() == columns.len())
        .filter(|&word| {
            for (i, word_ch) in word.char_indices() {
                if !columns[i].contains(&word_ch) {
                    return false;
                }
            }

            true
        })
        .collect();

    // a grid matching the input tracking whether each character has been used
    let mut checkboxes: Vec<Vec<bool>> = columns
        .iter()
        .map(|c| c.iter().map(|_| false).collect())
        .collect();

    let mut solution = Vec::new();
    for word in words {
        // fill in appropriate checkboxes
        let mut word_useful = false;
        for (col, word_ch) in word.char_indices() {
            // NOTE, this unwrap relies on the filtering above to only valid words
            let row = columns[col]
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
        let solved = checkboxes.iter().flatten().all(|&checked| checked);
        if solved {
            break;
        }
    }

    solution
}
