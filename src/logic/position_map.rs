use crate::logic::number::Number;
use ndarray::Array2;
use rand::Rng;
use std::collections::HashMap;

pub type Id = i32;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn from(x: usize, y: usize) -> Self {
        Self {
            x: x.try_into().unwrap(),
            y: y.try_into().unwrap(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    LEFT,
    RIGHT,
    TOP,
    BOTTOM,
}

pub const WIDTH: i32 = 4;
pub const HEIGHT: i32 = 4;

#[derive(Debug)]
pub struct PositionMap {
    positions: Array2<Option<Id>>,
    blocks: HashMap<Id, Number>,
}

impl PositionMap {
    pub fn new() -> Self {
        Self {
            positions: Array2::<Option<Id>>::from_elem((WIDTH as usize, HEIGHT as usize), None),
            blocks: HashMap::new(),
        }
    }

    pub fn new_with_existing_blocks(&self) -> Self {
        Self {
            positions: Array2::<Option<Id>>::from_elem((WIDTH as usize, HEIGHT as usize), None),
            blocks: self.blocks.clone(),
        }
    }

    pub fn set(&mut self, x: i32, y: i32, id: Option<Id>) {
        if x < 0 || y < 0 || x >= WIDTH || y >= HEIGHT {
            panic!("Attempt to set out of bounds")
        }

        self.positions[[x as usize, y as usize]] = id
    }

    pub fn add_block(&mut self, id: Id, number: Number) {
        self.blocks.insert(id, number);
    }

    pub fn delete_block(&mut self, target: Id) {
        self.blocks.remove(&target);
        if let Some(p) = self.find_position(target) {
            self.set(p.x, p.y, None);
        }
    }

    pub fn find_position(&self, target: Id) -> Option<Position> {
        for ((x, y), value) in self.positions.indexed_iter() {
            if let Some(id) = value {
                if *id == target {
                    return Some(Position::from(x, y));
                }
            }
        }
        None
    }

    pub fn get(&self, x: i32, y: i32) -> Option<Id> {
        if x < 0 || y < 0 || x >= WIDTH || y >= HEIGHT {
            return None;
        }

        self.positions[[x.try_into().unwrap(), y.try_into().unwrap()]]
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

    pub fn get_number_with_id(&self, id: Id) -> Option<Number> {
        self.blocks
            .get(&id)
            .map_or(None, |number| Some(number.clone()))
    }

    pub fn get_random_free_position(&self) -> Option<Position> {
        let quantity: i32 = self
            .positions
            .iter()
            .filter(|i| i.is_none())
            .count()
            .try_into()
            .unwrap();

        if quantity == 0 {
            return None;
        }

        let chosen = rand::thread_rng().gen_range(0..quantity);

        let mut current: i32 = -1;
        for ((x, y), value) in self.positions.indexed_iter() {
            if let None = value {
                current += 1;
                if current == chosen {
                    return Some(Position::from(x, y));
                }
            }
        }
        println!("No free positions found");
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

    pub fn same_positions(&self, other: &PositionMap) -> bool {
        self.positions == other.positions
    }

    pub fn get_not_empty_position_from(&self, direction: Direction, line: i32) -> Option<Position> {
        match direction {
            Direction::LEFT => {
                for i in 0..=3 {
                    if let Some(_) = self.get(i, line) {
                        return Some(Position { x: i, y: line });
                    }
                }
            }
            Direction::RIGHT => {
                for i in (0..=3).rev() {
                    if let Some(_) = self.get(i, line) {
                        return Some(Position { x: i, y: line });
                    }
                }
            }
            Direction::TOP => {
                for i in 0..=3 {
                    if let Some(_) = self.get(line, i) {
                        return Some(Position { x: line, y: i });
                    }
                }
            }
            Direction::BOTTOM => {
                for i in (0..=3).rev() {
                    if let Some(_) = self.get(line, i) {
                        return Some(Position { x: line, y: i });
                    }
                }
            }
        }
        None
    }

    pub fn print_map(&self) {
        // println!("Map: {:?}", self.positions)
    }
}
