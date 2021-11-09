use crate::logic::number::Number;
use ndarray::Array2;
use rand::Rng;
use std::collections::HashMap;

pub type Id = i32;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    LEFT,
    RIGHT,
    TOP,
    BOTTOM,
}

pub const WIDTH: usize = 4;
pub const HEIGHT: usize = 4;

pub struct PositionMap {
    positions: Array2<Option<Id>>,
    blocks: HashMap<Id, Number>,
}

impl PositionMap {
    pub fn new() -> Self {
        Self {
            positions: Array2::<Option<Id>>::from_elem((WIDTH, HEIGHT), None),
            blocks: HashMap::new(),
        }
    }

    pub fn set(&mut self, x: usize, y: usize, id: Option<Id>) {
        self.positions[[x, y]] = id
    }

    pub fn add_block(&mut self, id: Id, number: Number) {
        self.blocks.insert(id, number);
    }

    pub fn get(&self, x: usize, y: usize) -> Option<Id> {
        self.positions[[x, y]]
    }

    fn get_number(&self, x: i32, y: i32) -> Option<Number> {
        if x < 0 || y < 0 {
            return None;
        }

        if let Some(id) = self.get(x.try_into().unwrap(), y.try_into().unwrap()) {
            if let Some(number) = self.blocks.get(&id) {
                return Some(number.to_owned());
            }
        }
        None
    }

    pub fn get_random_free_position(&self) -> Option<Position> {
        let quantity: i32 = self.positions.iter().count() as i32;
        if quantity == 0 {
            return None;
        }

        let chosen = rand::thread_rng().gen_range(0..quantity);
        let mut current: i32 = -1;
        for ((x, y), value) in self.positions.indexed_iter() {
            if let None = value {
                current += 1;
                if current == chosen {
                    return Some(Position { x: x, y: y });
                }
            }
        }
        None
    }

    pub fn has_available_moves(&self) -> bool {
        for ((x, y), _) in self.positions.indexed_iter() {
            if self.has_adjacent_equal_position(x.try_into().unwrap(), y.try_into().unwrap()) {
                return true;
            }
        }
        return false;
    }

    fn has_adjacent_equal_position(&self, x: i32, y: i32) -> bool {
        let it = self.get_number(x, y);
        it == self.get_number(x - 1, y)
            || it == self.get_number(x + 1, y)
            || it == self.get_number(x, y - 1)
            || it == self.get_number(x, y + 1)
    }

    pub fn has_any_blocks(&self) -> bool {
        self.blocks.len() > 0
    }
}
