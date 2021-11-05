use bevy::prelude::*;

pub struct EventRegistrationPlugin;

impl Plugin for EventRegistrationPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<ScoreChanged>();
    }
}

pub struct ScoreChanged {
    pub score: i32,
}
