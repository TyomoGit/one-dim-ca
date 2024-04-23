#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Cell {
    pub is_alive: bool,
}

impl Cell {
    pub fn new(is_alive: bool) -> Cell {
        Cell { is_alive }
    }
}
