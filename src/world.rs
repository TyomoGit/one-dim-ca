use crate::{cell::Cell, rule::Rule};

pub struct World {
    cells: Vec<Cell>,
    rule: Rule,
}

impl World {
    pub fn new(width: usize, rule: Rule) -> World {
        World {
            cells: vec![Cell::new(false); width],
            rule,
        }
    }

    pub fn initial_central_live_cell(width: usize, rule: Rule) -> World {
        let mut cells = Self::new(width, rule);
        cells.cells[width / 2] = Cell::new(true);
        cells
    }

    pub fn forward_until_glowed(&mut self) {
        loop {
            self.forward();
            if self.is_glowed() {
                break;
            }
        }
    }

    pub fn is_glowed(&self) -> bool {
        self.cells[0..2].iter().any(|x| x.is_alive)
        || self.cells[self.cells.len() - 2..].iter().any(|x| x.is_alive)
    }

    pub fn forward(&mut self) -> &[Cell] {
        let mut next = self.cells.clone();

        for (i, next_cell) in next.iter_mut().enumerate() {
            let before  = match i {
                0 => [&[Cell::new(false)], &self.cells[i..=i+1]].concat(),
                x if x == self.cells.len() - 1 => [&self.cells[i-1..=i], &[Cell::new(false)]].concat(),
                _ => self.cells[i-1..=i+1].into(),
            };
            *next_cell = self.rule.apply(&before);
        }

        self.cells = next;
        &self.cells
    }
}
