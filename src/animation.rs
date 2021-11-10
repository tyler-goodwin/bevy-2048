use std::collections::HashMap;

use crate::{events::BlocksMoved, ui_plugin::number_renderer::Block};

use bevy::prelude::*;

struct AnimationPlugin;

struct BlockAnimation {
    step: (f32, f32),
    total_iter: usize,
    current_iter: usize,
}

impl BlockAnimation {
    pub fn new(step: (f32, f32), total_iter: usize) -> Self {
        Self {
            step: step,
            total_iter: total_iter,
            current_iter: 0,
        }
    }
}

struct State {
    running: bool,
    timer: Timer,
    blocks: HashMap<i32, BlockAnimation>,
}

impl State {
    pub fn new() -> Self {
        State {
            running: false,
            timer: Timer::from_seconds(0.02, true),
            blocks: HashMap::new(),
        }
    }
}

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(State::new())
            .add_system(blocks_moved_listener.system())
            .add_system(run_animations.system());
    }
}

fn blocks_moved_listener(mut state: ResMut<State>, mut events: EventReader<BlocksMoved>) {
    for event in events.iter() {
        if !state.running {
            state.running = true;

            for (id, target) in event.moves.iter() {
                // Calculate target position
                // Calcuate step & total iterations required
                let animation = BlockAnimation::new((0.0, 0.0), 3);
                state.blocks.insert(*id, animation);
            }
        }
    }
}

fn run_animations(
    time: Res<Time>,
    mut state: ResMut<State>,
    mut query: Query<(&mut Transform, &Block)>,
) {
    if state.timer.tick(time.delta()).just_finished() {
        for (mut transform, block) in query.iter_mut() {
            if let Some(animation) = state.blocks.get_mut(&block.id) {
                animation.current_iter += 1;

                let (x, y) = animation.step;
                transform.translation.x += x;
                transform.translation.y += y;

                if animation.total_iter == animation.current_iter {
                    state.blocks.remove(&block.id);
                }
            }
        }

        if state.blocks.len() == 0 {
            // DONE Animation
        }
    }
}
