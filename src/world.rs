use crate::{cell::Cell, rule::Rule};

pub struct World {
    cells: Vec<Cell>,
    rule: Rule,
    steps: usize,
    loop_edges: bool,
}

impl World {
    pub fn new(width: usize, rule: Rule, loop_edges: bool) -> World {
        World {
            cells: vec![Cell::new(false); width],
            rule,
            steps: 0,
            loop_edges,
        }
    }

    pub fn cells(&self) -> &[Cell] {
        &self.cells
    }

    pub fn initial_central_live_cell(width: usize, rule: Rule, loop_edges: bool) -> World {
        let mut cells = Self::new(width, rule, loop_edges);
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
        self.steps > self.cells.len() / 2
    }

    pub fn forward(&mut self) -> &[Cell] {
        let mut next = self.cells.clone();

        for (i, next_cell) in next.iter_mut().enumerate() {
            let before = if self.loop_edges {
                [
                    self.cells[(i + self.cells.len() - 1) % self.cells.len()],
                    self.cells[i],
                    self.cells[(i + 1) % self.cells.len()],
                ]
            } else {
                match i {
                    0 => [Cell::new(false), self.cells[i], self.cells[i + 1]],
                    i if i == self.cells.len() - 1 => {
                        [self.cells[i - 1], self.cells[i], Cell::new(false)]
                    }
                    _ => [self.cells[i - 1], self.cells[i], self.cells[i + 1]],
                }
            };
            *next_cell = self.rule.apply(&before);
        }

        self.cells = next;
        self.steps += 1;
        &self.cells
    }
}
