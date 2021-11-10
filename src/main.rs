use bevy::prelude::*;
use events::{EventRegistrationPlugin, MoveRequested, ScoreChanged};

mod animation;
mod events;
mod input;
mod logic;
mod ui_plugin;

struct State {
    pub timer: Timer,
    pub score: i32,
}

impl Default for State {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1.0, true),
            score: 0,
        }
    }
}

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Hello Bevy!".to_string(),
            width: ui_plugin::WIDTH,
            height: ui_plugin::HEIGHT,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(State::default())
        .add_plugin(EventRegistrationPlugin)
        .add_plugin(logic::LogicPlugin)
        .add_plugin(input::InputPlugin)
        .add_plugin(ui_plugin::UIPlugin)
        .add_plugin(animation::AnimationPlugin)
        .add_system(score_changer.system())
        .add_system(move_listener.system())
        .run();
}

fn score_changer(time: Res<Time>, mut state: ResMut<State>, mut events: EventWriter<ScoreChanged>) {
    if state.timer.tick(time.delta()).just_finished() {
        state.score += 1;

        events.send(ScoreChanged { score: state.score });
    }
}

fn move_listener(mut events: EventReader<MoveRequested>) {
    for event in events.iter() {
        println!("Move requested towards {:?}", event.direction)
    }
}
