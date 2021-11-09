use bevy::input::keyboard::{KeyCode, KeyboardInput};
use bevy::input::ElementState;
use bevy::prelude::*;

use crate::events::{MoveRequested, RestartRequested};
use crate::logic::position_map::Direction;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(handle_keyboard_input.system());
    }
}

fn handle_keyboard_input(
    mut input_events: EventReader<KeyboardInput>,
    mut move_events: EventWriter<MoveRequested>,
    mut restart_events: EventWriter<RestartRequested>,
) {
    use KeyCode::*;

    for event in input_events.iter() {
        if let Some(keycode) = event.key_code {
            if event.state == ElementState::Pressed {
                match keycode {
                    Up => move_events.send(MoveRequested::new(Direction::TOP)),
                    Down => move_events.send(MoveRequested::new(Direction::BOTTOM)),
                    Left => move_events.send(MoveRequested::new(Direction::LEFT)),
                    Right => move_events.send(MoveRequested::new(Direction::RIGHT)),
                    Space => restart_events.send(RestartRequested),
                    _ => (),
                }
            }
        }
    }
}
