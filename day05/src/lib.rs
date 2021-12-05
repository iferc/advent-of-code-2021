pub mod first;
pub mod second;
pub mod vector;

use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Position {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Line {
    a: Position,
    b: Position,
}

#[derive(Clone, Debug, Default)]
pub struct Grid {
    pub map: HashMap<Position, usize>,
}

impl Grid {
    pub fn increment_at(&mut self, position: Position) {
        let counter = self.map.entry(position).or_insert(0);
        *counter += 1;
    }
}
