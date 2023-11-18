use std::fs::File;
use std::io::{BufWriter, Write};

use typeshift_solver::*;

fn main() {
    let bytes = create_index();

    let file = File::create("./files/index.dat").unwrap();
    let mut file = BufWriter::new(file);

    file.write_all(&bytes).unwrap();
}
