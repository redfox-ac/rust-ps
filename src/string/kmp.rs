pub struct KMP<'a, T: PartialEq> {
    pattern: &'a Vec<T>,
    table: Vec<usize>,
}

impl<'a, T: PartialEq> KMP<'a, T> {
    pub fn new(pattern: &'a Vec<T>) -> Self {
        let mut table = vec![0; pattern.len()];
        let mut idx = 0;
        for begin in 1..pattern.len() {
            idx = Self::advance(&pattern, &table, idx, &pattern[begin]);
            table[begin] = idx;
        }

        KMP { pattern, table }
    }

    pub fn next_match(&self, text: &[T], i: usize) -> Option<usize> {
        let mut idx = 0;
        for begin in i..text.len() {
            idx = Self::advance(&self.pattern, &self.table, idx, &text[begin]);
            if idx == self.pattern.len() {
                return Some(begin + 1 - self.pattern.len());
            }
        }
        None
    }

    pub fn matched_len(&self, text: &[T], i: usize) -> usize {
        let mut idx = 0;
        for begin in (i + 1).saturating_sub(self.pattern.len())..=i {
            idx = Self::advance(&self.pattern, &self.table, idx, &text[begin]);
        }
        idx
    }


    fn advance(pattern: &[T], table: &[usize], mut idx: usize, c: &T) -> usize {
        if idx == pattern.len() {
            idx = table[idx - 1];
        }
        while idx > 0 && *c != pattern[idx] {
            idx = table[idx - 1];
        }
        if *c == pattern[idx] { idx + 1 } else { idx }
    }
}