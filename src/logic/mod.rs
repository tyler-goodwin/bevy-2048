use bevy::prelude::*;
use rand::Rng;

pub mod number;
pub mod position_map;

use number::Number;
use position_map::{Direction, Id, Position, PositionMap};

use crate::events::{
    AnimationCompleted, BlockAdded, BlocksDeleted, BlocksMoved, GameOver, GameRestarted,
    MoveRequested, RestartRequested,
};

pub enum GenerateResult {
    GameOver,
    BlockAdded(Id, Number, Position),
}

pub enum MoveBlockResult {
    None,
    GameOver,
    Success(BlocksMoved),
}

#[derive(Debug)]
pub struct LogicState {
    pub position_map: PositionMap,
    pub current_id: Id,
    pub is_game_over: bool,
    pub ready_for_next_move: bool,
    pub merges: Vec<(i32, i32, Position)>,
}

impl LogicState {
    pub fn new() -> Self {
        LogicState {
            position_map: PositionMap::new(),
            current_id: 0,
            is_game_over: false,
            ready_for_next_move: true,
            merges: vec![],
        }
    }

    pub fn restart(&mut self) {
        self.position_map = PositionMap::new();
        self.current_id = 0;
        self.is_game_over = false;
        self.ready_for_next_move = true;
    }

    pub fn add_block(&mut self, number: Number, position: Position) -> i32 {
        let id = self.current_id;
        self.current_id += 1;

        self.position_map.add_block(id, number);
        self.position_map.set(position.x, position.y, Some(id));
        id
    }

    pub fn generate_block(&mut self) -> GenerateResult {
        let random: f32 = rand::thread_rng().gen_range(0.0..1.0);
        let number = if random > 0.9 {
            Number::ONE
        } else {
            Number::ZERO
        };

        if let Some(position) = self.position_map.get_random_free_position() {
            let id = self.add_block(number, position);
            GenerateResult::BlockAdded(id, number, position)
        } else {
            GenerateResult::GameOver
        }
    }

    pub fn move_blocks_to(&mut self, direction: Direction) -> MoveBlockResult {
        if !self.ready_for_next_move {
            println!("Ignoring requested move");
            return MoveBlockResult::None;
        }

        if !self.position_map.has_available_moves() {
            if !self.is_game_over {
                self.is_game_over = true;
                return MoveBlockResult::GameOver;
            }
            return MoveBlockResult::None;
        }

        // Move blocks
        let mut moves: Vec<(i32, Position)> = vec![];
        let mut merges: Vec<(i32, i32, Position)> = vec![];

        let new_map = self.calculate_new_map(direction, &mut moves, &mut merges);
        new_map.print_map();

        if !new_map.same_positions(&self.position_map) {
            // Wait for dependent plugins to be ready for next move, i.e animation
            self.ready_for_next_move = false;

            // This may need to go after animations completed
            self.position_map = new_map;
            let mut merged_moves = moves.clone();
            for (id1, id2, pos) in merges.iter() {
                merged_moves.push((*id1, pos.clone()));
                merged_moves.push((*id2, *pos));
            }

            self.merges = merges.clone();

            MoveBlockResult::Success(BlocksMoved {
                moves: merged_moves,
            })
        } else {
            MoveBlockResult::None
        }
    }

    pub fn calculate_new_map(
        &mut self,
        direction: Direction,
        moves: &mut Vec<(i32, Position)>,
        merges: &mut Vec<(i32, i32, Position)>,
    ) -> PositionMap {
        let mut new_map = self.position_map.new_with_existing_blocks();
        let start_index = match direction {
            Direction::LEFT | Direction::TOP => 0,
            _ => 3,
        };

        #[allow(unused_assignments)]
        let mut column_row = start_index;

        let max: usize = 3;
        for line in 0..=max {
            let mut cur_pos = self
                .position_map
                .get_not_empty_position_from(direction, line);
            column_row = start_index;

            loop {
                if let Some(cur_pos) = cur_pos {
                    let new_pos = new_position(line, &mut column_row, direction);
                    let current_id = self.position_map.get(cur_pos.x, cur_pos.y);
                    self.position_map.set(cur_pos.x, cur_pos.y, None);

                    let next_pos = self
                        .position_map
                        .get_not_empty_position_from(direction, line);

                    let next_id = next_pos.map_or(None, |p| self.position_map.get(p.x, p.y));

                    let current_number =
                        current_id.map_or(None, |id| self.position_map.get_number_with_id(id));
                    let next_number =
                        next_id.map_or(None, |id| self.position_map.get_number_with_id(id));

                    if next_id.is_some() && current_id.is_some() && current_number == next_number {
                        // merge these blocks
                        self.position_map.set(cur_pos.x, cur_pos.y, None);
                        new_map.set(cur_pos.x, cur_pos.y, current_id);
                        merges.push((current_id.unwrap(), next_id.unwrap(), new_pos));
                    } else {
                        new_map.set(new_pos.x, new_pos.y, current_id);
                        moves.push((current_id.unwrap(), new_pos))
                    }
                } else {
                    break;
                }

                cur_pos = self
                    .position_map
                    .get_not_empty_position_from(direction, line);
            }
        }

        new_map
    }
}

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(LogicState::new())
            .add_startup_system(generate_starting_block.system())
            .add_system(move_requested_listener.system())
            .add_system(restart_request_listener.system())
            .add_system(game_restarted_listener.system())
            .add_system(animation_completed.system());
    }
}

fn generate_starting_block(mut state: ResMut<LogicState>, mut events: EventWriter<BlockAdded>) {
    if let GenerateResult::BlockAdded(id, number, position) = state.generate_block() {
        println!("Block added!");
        events.send(BlockAdded {
            id: id,
            number: number,
            position: position,
        });
    } else {
        panic!("Unexpected result during first block generation")
    }
}

fn move_requested_listener(
    mut state: ResMut<LogicState>,
    mut move_events: EventReader<MoveRequested>,
    mut game_over: EventWriter<GameOver>,
    mut blocks_moved: EventWriter<BlocksMoved>,
) {
    for move_event in move_events.iter() {
        println!("State: {:?}", *state);
        match state.move_blocks_to(move_event.direction) {
            MoveBlockResult::None => (),
            MoveBlockResult::GameOver => game_over.send(GameOver),
            MoveBlockResult::Success(event) => blocks_moved.send(event),
        }
    }
}

fn restart_request_listener(
    mut state: ResMut<LogicState>,
    mut events: EventReader<RestartRequested>,
    mut restarted: EventWriter<GameRestarted>,
) {
    for _ in events.iter() {
        if state.is_game_over {
            state.restart();
            restarted.send(GameRestarted);
        }
    }
}

fn game_restarted_listener(
    mut state: ResMut<LogicState>,
    mut events: EventReader<GameRestarted>,
    mut block_added: EventWriter<BlockAdded>,
) {
    for _ in events.iter() {
        if !state.position_map.has_any_blocks() {
            if let GenerateResult::BlockAdded(id, number, position) = state.generate_block() {
                println!("Block added!");
                block_added.send(BlockAdded {
                    id: id,
                    number: number,
                    position: position,
                });
            } else {
                panic!("Unexpected result during first block generation")
            }
        }
    }
}

fn animation_completed(
    mut state: ResMut<LogicState>,
    mut events: EventReader<AnimationCompleted>,
    mut block_added: EventWriter<BlockAdded>,
    mut deleted_blocks: EventWriter<BlocksDeleted>,
) {
    for _ in events.iter() {
        if !state.ready_for_next_move {
            state.ready_for_next_move = true;

            // Deal with merges
            let mut deleted: Vec<i32> = vec![];
            let mut added: Vec<(i32, Number, Position)> = vec![];

            let length = state.merges.len();
            for i in 0..length {
                let (id1, id2, position) = state.merges[i];
                let next_number = state.position_map.get_number_with_id(id1).unwrap().next();
                state.position_map.delete_block(id1);
                state.position_map.delete_block(id2);

                deleted.push(id1);
                deleted.push(id2);

                let id = state.add_block(next_number, position);
                added.push((id, next_number, position));
            }

            if deleted.len() > 0 {
                deleted_blocks.send(BlocksDeleted { deleted: deleted })
            }

            for (id, number, position) in added.iter() {
                block_added.send(BlockAdded {
                    id: *id,
                    number: *number,
                    position: *position,
                });
            }

            state.merges = vec![];

            if let GenerateResult::BlockAdded(id, number, position) = state.generate_block() {
                println!("Block added!");
                block_added.send(BlockAdded {
                    id: id,
                    number: number,
                    position: position,
                });
            } else {
                panic!("Unexpected result during first block generation")
            }
        }
    }
}

fn new_position(line: usize, column_row: &mut usize, direction: Direction) -> Position {
    let tmp = *column_row;
    match direction {
        Direction::LEFT => {
            *column_row += 1;
            Position { x: tmp, y: line }
        }
        Direction::RIGHT => {
            *column_row -= 1;
            Position { x: tmp, y: line }
        }
        Direction::TOP => {
            *column_row += 1;
            Position { x: line, y: tmp }
        }
        Direction::BOTTOM => {
            *column_row -= 1;
            Position { x: line, y: tmp }
        }
    }
}
