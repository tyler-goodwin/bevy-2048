use crate::logic::{number::Number, position_map::Direction, position_map::Position};
use bevy::prelude::*;

pub struct EventRegistrationPlugin;

impl Plugin for EventRegistrationPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<ScoreChanged>()
            .add_event::<BestChanged>()
            .add_event::<BlockAdded>()
            .add_event::<MoveRequested>()
            .add_event::<BlocksMoved>()
            .add_event::<BlocksDeleted>()
            .add_event::<AnimationCompleted>()
            .add_event::<RestartRequested>()
            .add_event::<GameRestarted>()
            .add_event::<GameOver>();
    }
}

pub struct ScoreChanged {
    pub score: i32,
}

pub struct BestChanged {
    pub best: i32,
}

pub struct BlockAdded {
    pub id: i32,
    pub number: Number,
    pub position: Position,
}

pub struct GameOver;

pub struct MoveRequested {
    pub direction: Direction,
}

impl MoveRequested {
    pub fn new(direction: Direction) -> Self {
        Self {
            direction: direction,
        }
    }
}

pub struct BlocksMoved {
    pub moves: Vec<(i32, Position)>,
}

pub struct BlocksDeleted {
    pub deleted: Vec<i32>,
}

pub struct AnimationCompleted;

pub struct RestartRequested;
pub struct GameRestarted;
