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
                    .filter_map(|(i, &include)| include.then_some((i as u8 + b'a') as char)),
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

    pub fn sum(mut self, other: [usize; 26]) -> Self {
        for (i, count) in other.iter().enumerate() {
            self.0[i] += count;
        }

        self
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
                    .filter(|(_, count)| **count > 0)
                    .map(|(i, count)| ((i as u8 + b'a') as char, count)),
            )
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alpha_set_debug() {
        let set = AlphaSet::from_iter("hi".chars());
        let debug = format!("{set:?}");

        assert_eq!(debug, "{'h', 'i'}");
    }

    #[test]
    fn alpha_counts_debug() {
        let mut counts = AlphaCounts::new();
        for ch in "heyyy".chars() {
            counts.add(ch);
        }

        let debug = format!("{counts:?}");

        assert_eq!(debug, "{'e': 1, 'h': 1, 'y': 3}");
    }
}
