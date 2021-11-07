use bevy::prelude::*;
use rand::Rng;

pub mod number;
pub mod position_map;

use number::Number;
use position_map::{Id, Position, PositionMap};

use crate::events::BlockAdded;

pub enum GenerateResult {
    GameOver,
    BlockAdded(Id, Number, Position),
}

pub struct LogicState {
    pub position_map: PositionMap,
    pub current_id: Id,
}

impl LogicState {
    pub fn new() -> Self {
        LogicState {
            position_map: PositionMap::new(),
            current_id: 0,
        }
    }

    pub fn generate_block(&mut self) -> GenerateResult {
        let id = self.current_id;
        self.current_id += 1;

        let random: f32 = rand::thread_rng().gen_range(0.0..1.0);
        let number = if random > 0.9 {
            Number::ZERO
        } else {
            Number::ONE
        };

        if let Some(position) = self.position_map.get_random_free_position() {
            self.position_map.add_block(id, number);
            self.position_map.set(position.x, position.y, Some(id));
            GenerateResult::BlockAdded(id, number, position)
        } else {
            GenerateResult::GameOver
        }
    }
}

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(LogicState::new())
            .add_startup_system(generate_starting_block.system());
        // app.add_startup_system(setup.system())
        //     .add_system(score_changed_listener.system())
        //     .add_system(best_changed_listener.system());
        // .add_system(animate.system());
    }
}

fn generate_starting_block(mut state: ResMut<LogicState>, mut events: EventWriter<BlockAdded>) {
    if let GenerateResult::BlockAdded(id, number, position) = state.generate_block() {
        println!("Block added!");
        events.send(BlockAdded {
            id: id,
            number: number,
            position: position,
        })
    }
}
