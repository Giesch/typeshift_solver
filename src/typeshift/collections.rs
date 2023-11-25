//! Collections of ascii characters implemented with arrays

/// A set of lowercase alphabetic ascii characters
pub struct AlphaSet([bool; 26]);

impl AlphaSet {
    pub fn new() -> Self {
        Self([false; 26])
    }

    pub fn from_iter(chars: impl Iterator<Item = char>) -> Self {
        let mut set = Self::new();
        for ch in chars {
            set.add(ch)
        }

        set
    }

    pub fn add(&mut self, ch: char) {
        self.0[Self::index(ch)] = true;
    }

    pub fn contains(&self, ch: char) -> bool {
        self.0[Self::index(ch)]
    }

    /// Returns only the char counts that are included in the set
    pub fn filter_counts<'a>(
        &'a self,
        counts: &'a AlphaCounts,
    ) -> impl Iterator<Item = usize> + 'a {
        (0..26).filter(|&i| self.0[i]).map(|i| counts.get_raw(i))
    }

    #[inline(always)]
    fn index(ch: char) -> usize {
        ch as usize - b'a' as usize
    }
}

impl std::fmt::Debug for AlphaSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_set()
            .entries(
                self.0
                    .iter()
                    .enumerate()
                    .filter(|(_i, &include)| include)
                    .map(|(i, _include)| (i as u8 + b'a') as char),
            )
            .finish()
    }
}

/// A map of lowercase ascii characters to natural numbers
#[derive(Clone)]
pub struct AlphaCounts([usize; 26]);

impl AlphaCounts {
    pub fn new() -> Self {
        Self([0; 26])
    }

    pub fn from_iter(chars: impl Iterator<Item = char>) -> Self {
        let mut counts = Self::new();
        for ch in chars {
            counts.add(ch)
        }

        counts
    }

    pub fn add(&mut self, ch: char) {
        self.0[Self::index(ch)] += 1;
    }

    pub fn get(&self, ch: char) -> usize {
        self.0[Self::index(ch)]
    }

    fn get_raw(&self, i: usize) -> usize {
        self.0[i]
    }

    #[inline(always)]
    fn index(ch: char) -> usize {
        ch as usize - b'a' as usize
    }
}

impl std::fmt::Debug for AlphaCounts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map()
            .entries(
                self.0
                    .iter()
                    .enumerate()
                    .filter(|(_i, count)| **count > 0)
                    .map(|(i, count)| ((i as u8 + b'a') as char, count)),
            )
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn filter_counts_smoke() {
        let column = AlphaSet::from_iter("eiz".chars());
        let usages = AlphaCounts::from_iter("eeeii".chars());
        let result: Vec<_> = column.filter_counts(&usages).collect();

        // order is alphabetical, but that should be irrelevant to how its used
        assert_eq!(BTreeSet::from_iter([3, 2, 0]), BTreeSet::from_iter(result));
    }

    #[test]
    fn alpha_set_smoke() {
        let set = AlphaSet::from_iter("hi".chars());

        assert!(set.contains('h'));
        assert!(set.contains('i'));
        assert!(!set.contains('z'));
    }

    #[test]
    fn alpha_counts_smoke() {
        let counts = AlphaCounts::from_iter("heyyy".chars());

        assert_eq!(counts.get('h'), 1);
        assert_eq!(counts.get('e'), 1);
        assert_eq!(counts.get('y'), 3);
        assert_eq!(counts.get('z'), 0);
    }

    #[test]
    fn alpha_set_debug() {
        let set = AlphaSet::from_iter("hi".chars());
        let debug = format!("{set:?}");

        assert_eq!(debug, "{'h', 'i'}");
    }

    #[test]
    fn alpha_counts_debug() {
        let counts = AlphaCounts::from_iter("heyyy".chars());
        let debug = format!("{counts:?}");

        assert_eq!(debug, "{'e': 1, 'h': 1, 'y': 3}");
    }
}
