use crate::{cell::Cell, rule::Rule};

pub enum InitialState {
    None,
    SingleCentralCell,
    Random,
    Vec(Vec<Cell>),
}

impl InitialState {
    pub fn to_vec(self, width: usize) -> Vec<Cell> {
        match self {
            InitialState::None => vec![Cell::new(false); width],
            InitialState::SingleCentralCell => {
                let mut cells = vec![Cell::new(false); width];
                cells[width / 2] = Cell::new(true);
                cells
            }
            InitialState::Random => {
                let mut cells = Vec::with_capacity(width);
                for _ in 0..width {
                    cells.push(Cell::new(rand::random()));
                }
                cells
            }
            InitialState::Vec(cells) => cells,
        }
    }
    
}

pub struct World {
    cells: Vec<Cell>,
    rule: Rule,
    steps: usize,
    loop_edges: bool,
}

impl World {
    pub fn new(init_state: InitialState, width: usize, rule: Rule, loop_edges: bool) -> World {
        World {
            cells: init_state.to_vec(width),
            rule,
            steps: 0,
            loop_edges,
        }
    }

    pub fn cells(&self) -> &[Cell] {
        &self.cells
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
