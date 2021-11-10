use bevy::prelude::*;
use rand::Rng;

pub mod number;
pub mod position_map;

use number::Number;
use position_map::{Direction, Id, Position, PositionMap};

use crate::events::{
    BlockAdded, BlocksMoved, GameOver, GameRestarted, MoveRequested, RestartRequested,
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

pub struct LogicState {
    pub position_map: PositionMap,
    pub current_id: Id,
    pub is_game_over: bool,
    pub ready_for_next_move: bool,
}

impl LogicState {
    pub fn new() -> Self {
        LogicState {
            position_map: PositionMap::new(),
            current_id: 0,
            is_game_over: false,
            ready_for_next_move: true,
        }
    }

    pub fn restart(&mut self) {
        self.position_map = PositionMap::new();
        self.current_id = 0;
        self.is_game_over = false;
        self.ready_for_next_move = true;
    }

    pub fn generate_block(&mut self) -> GenerateResult {
        let id = self.current_id;
        self.current_id += 1;

        let random: f32 = rand::thread_rng().gen_range(0.0..1.0);
        let number = if random > 0.9 {
            Number::ONE
        } else {
            Number::ZERO
        };

        if let Some(position) = self.position_map.get_random_free_position() {
            self.position_map.add_block(id, number);
            self.position_map.set(position.x, position.y, Some(id));
            GenerateResult::BlockAdded(id, number, position)
        } else {
            GenerateResult::GameOver
        }
    }

    pub fn move_blocks_to(&mut self, direction: Direction) -> MoveBlockResult {
        if !self.ready_for_next_move {
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
        println!("Moving blocks to {:?}", direction);
        let mut moves: Vec<(i32, Position)> = vec![];
        let mut merges: Vec<(i32, i32, Position)> = vec![];

        let newMap = self.calculateNewMap(direction, &mut moves, &mut merges);

        if !newMap.same_positions(&self.position_map) {
            // Wait for dependent plugins to be ready for next move, i.e animation
            self.ready_for_next_move = false;

            // This may need to go after animations completed
            self.position_map = newMap;
            MoveBlockResult::Success(BlocksMoved {
                moves: moves,
                merges: merges,
            })
        } else {
            MoveBlockResult::None
        }
    }

    pub fn calculateNewMap(
        &mut self,
        direction: Direction,
        moves: &mut Vec<(i32, Position)>,
        merges: &mut Vec<(i32, i32, Position)>,
    ) -> PositionMap {
        todo!();
        self.position_map.clone()
    }
}

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(LogicState::new())
            .add_startup_system(generate_starting_block.system())
            .add_system(move_requested_listener.system())
            .add_system(restart_request_listener.system())
            .add_system(game_restarted_listener.system());
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
