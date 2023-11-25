mod raw_index;
use raw_index::*;

pub fn dict_with_counts(
    len: usize,
) -> Box<dyn Iterator<Item = &'static (&'static str, [usize; 26])>> {
    match len {
        // 1 => Box::new(INDEX_1.iter()),
        // 2 => Box::new(INDEX_2.iter()),
        // 3 => Box::new(INDEX_3.iter()),
        4 => Box::new(INDEX_4.iter()),
        5 => Box::new(INDEX_5.iter()),
        6 => Box::new(INDEX_6.iter()),
        7 => Box::new(INDEX_7.iter()),
        // 8 => Box::new(INDEX_8.iter()),
        // 9 => Box::new(INDEX_9.iter()),
        // 10 => Box::new(INDEX_10.iter()),
        // 11 => Box::new(INDEX_11.iter()),
        // 12 => Box::new(INDEX_12.iter()),
        // 13 => Box::new(INDEX_13.iter()),
        // 14 => Box::new(INDEX_14.iter()),
        // 15 => Box::new(INDEX_15.iter()),
        // 16 => Box::new(INDEX_16.iter()),
        // 17 => Box::new(INDEX_17.iter()),
        // 18 => Box::new(INDEX_18.iter()),
        // 19 => Box::new(INDEX_19.iter()),
        // 20 => Box::new(INDEX_20.iter()),
        // 21 => Box::new(INDEX_21.iter()),
        // 22 => Box::new(INDEX_22.iter()),
        // 23 => Box::new(INDEX_23.iter()),
        // 24 => Box::new(INDEX_24.iter()),
        // 25 => Box::new(INDEX_25.iter()),
        // // 26 => DELIBERATELY SKIPPED
        // 27 => Box::new(INDEX_27.iter()),
        // 28 => Box::new(INDEX_28.iter()),
        _ => panic!("unexpected word length: {len}"),
    }
}
