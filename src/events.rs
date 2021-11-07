use bevy::prelude::*;

pub struct EventRegistrationPlugin;

impl Plugin for EventRegistrationPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<ScoreChanged>().add_event::<BestChanged>();
    }
}

pub struct ScoreChanged {
    pub score: i32,
}

pub struct BestChanged {
    pub best: i32,
}
