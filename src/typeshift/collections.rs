//! Collections of ascii characters implemented with arrays

/// A set of lowercase alphabetic ascii characters
pub struct LetterSet(LetterMap<bool>);

impl LetterSet {
    pub fn new() -> Self {
        Self(LetterMap::new())
    }

    pub fn from_iter(chars: impl Iterator<Item = char>) -> Self {
        let mut set = Self::new();
        for ch in chars {
            set.add(ch)
        }

        set
    }

    pub fn add(&mut self, ch: char) {
        let entry = self.0.entry(ch);
        *entry = true;
    }

    pub fn contains(&self, ch: char) -> bool {
        self.0.get(ch)
    }

    /// Returns only the char counts that are included in the set
    pub fn filter_counts<'a>(
        &'a self,
        counts: &'a LetterCounts,
    ) -> impl Iterator<Item = usize> + 'a {
        self.iter().map(|ch| counts.get(ch))
    }

    fn iter(&self) -> impl Iterator<Item = char> + '_ {
        ('a'..='z').filter(|&ch| self.contains(ch))
    }
}

impl std::fmt::Debug for LetterSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

/// A map of lowercase ascii characters to natural numbers
#[derive(Clone)]
pub struct LetterCounts(LetterMap<usize>);

impl LetterCounts {
    pub fn new() -> Self {
        Self(LetterMap::new())
    }

    pub fn from_iter(chars: impl Iterator<Item = char>) -> Self {
        let mut counts = Self::new();
        for ch in chars {
            counts.add(ch)
        }

        counts
    }

    pub fn add(&mut self, ch: char) {
        let entry = self.0.entry(ch);
        *entry += 1;
    }

    pub fn get(&self, ch: char) -> usize {
        self.0.get(ch)
    }
}

impl std::fmt::Debug for LetterCounts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let entries = self.0.entries().filter(|(_ch, &count)| count > 0);
        f.debug_map().entries(entries).finish()
    }
}

/// An array-backed map of ascii characters to a value
#[derive(Clone, Copy, Default)]
struct LetterMap<T>([T; 26]);

impl<T: Copy + Default> LetterMap<T> {
    fn new() -> Self {
        Self([Default::default(); 26])
    }

    fn entry(&mut self, ch: char) -> &mut T {
        &mut self.0[Self::index(ch)]
    }

    fn get(&self, ch: char) -> T {
        self.0[Self::index(ch)]
    }

    fn index(ch: char) -> usize {
        ch as usize - b'a' as usize
    }

    fn entries(&self) -> impl Iterator<Item = (char, &T)> + '_ {
        ('a'..='z').zip(self.0.iter())
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn filter_counts_smoke() {
        let column = LetterSet::from_iter("eiz".chars());
        let usages = LetterCounts::from_iter("eeeii".chars());
        let result: BTreeSet<_> = column.filter_counts(&usages).collect();

        assert_eq!(BTreeSet::from_iter([3, 2, 0]), result);
    }

    #[test]
    fn letter_set_smoke() {
        let set = LetterSet::from_iter("hi".chars());

        assert!(set.contains('h'));
        assert!(set.contains('i'));
        assert!(!set.contains('z'));
    }

    #[test]
    fn letter_counts_smoke() {
        let counts = LetterCounts::from_iter("heyyy".chars());

        assert_eq!(counts.get('h'), 1);
        assert_eq!(counts.get('e'), 1);
        assert_eq!(counts.get('y'), 3);
        assert_eq!(counts.get('z'), 0);
    }

    #[test]
    fn letter_set_debug() {
        let set = LetterSet::from_iter("hi".chars());
        let debug = format!("{set:?}");

        assert_eq!(debug, "{'h', 'i'}");
    }

    #[test]
    fn letter_counts_debug() {
        let counts = LetterCounts::from_iter("heyyy".chars());
        let debug = format!("{counts:?}");

        assert_eq!(debug, "{'e': 1, 'h': 1, 'y': 3}");
    }
}
