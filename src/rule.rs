use std::fmt::Display;

use crate::cell::Cell;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rule {
    pub num: u8,
    pub pat_111: Cell,
    pub pat_110: Cell,
    pub pat_101: Cell,
    pub pat_100: Cell,
    pub pat_011: Cell,
    pub pat_010: Cell,
    pub pat_001: Cell,
    pub pat_000: Cell,
}

impl Rule {
    pub fn new(n: u8) -> Rule {
        let mut patterns = [false; 8]
            .iter()
            .enumerate()
            .map(|(i, _v)| n & (1 << (7 - i)) != 0)
            .map(Cell::new);

        Self {
            num: n,
            pat_111: patterns.next().unwrap(),
            pat_110: patterns.next().unwrap(),
            pat_101: patterns.next().unwrap(),
            pat_100: patterns.next().unwrap(),
            pat_011: patterns.next().unwrap(),
            pat_010: patterns.next().unwrap(),
            pat_001: patterns.next().unwrap(),
            pat_000: patterns.next().unwrap(),
        }
    }

    pub fn apply(&self, source: &[Cell]) -> Cell {
        if source.len() != 3 {
            panic!("expected 3 cells, got {}", source.len());
        }

        let [Cell { is_alive: left }, Cell { is_alive: middle }, Cell { is_alive: right }] = source
        else {
            panic!("expected 3 cells, got {}", source.len());
        };

        match (left, middle, right) {
            (true, true, true) => self.pat_111,
            (true, true, false) => self.pat_110,
            (true, false, true) => self.pat_101,
            (true, false, false) => self.pat_100,
            (false, true, true) => self.pat_011,
            (false, true, false) => self.pat_010,
            (false, false, true) => self.pat_001,
            (false, false, false) => self.pat_000,
        }
    }
}

impl Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rule {}", self.num)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rule() {
        let t = Rule::new(30);
        assert_eq!(
            t,
            Rule {
                num: 30,
                pat_111: Cell::new(false),
                pat_110: Cell::new(false),
                pat_101: Cell::new(false),
                pat_100: Cell::new(true),
                pat_011: Cell::new(true),
                pat_010: Cell::new(true),
                pat_001: Cell::new(true),
                pat_000: Cell::new(false),
            }
        )
    }

    #[test]
    fn apply() {
        let t = Rule::new(30);
        assert_eq!(
            t.apply(&[Cell::new(true), Cell::new(true), Cell::new(true)]),
            Cell::new(false)
        );
        assert_eq!(
            t.apply(&[Cell::new(true), Cell::new(true), Cell::new(false)]),
            Cell::new(false)
        );
        assert_eq!(
            t.apply(&[Cell::new(true), Cell::new(false), Cell::new(true)]),
            Cell::new(false)
        );
        assert_eq!(
            t.apply(&[Cell::new(true), Cell::new(false), Cell::new(false)]),
            Cell::new(true)
        );
        assert_eq!(
            t.apply(&[Cell::new(false), Cell::new(true), Cell::new(true)]),
            Cell::new(true)
        );
        assert_eq!(
            t.apply(&[Cell::new(false), Cell::new(true), Cell::new(false)]),
            Cell::new(true)
        );
        assert_eq!(
            t.apply(&[Cell::new(false), Cell::new(false), Cell::new(true)]),
            Cell::new(true)
        );
        assert_eq!(
            t.apply(&[Cell::new(false), Cell::new(false), Cell::new(false)]),
            Cell::new(false)
        );
    }
}
