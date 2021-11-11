use std::{collections::HashMap, ops::Add};

use crate::{
    events::{AnimationCompleted, BlocksMoved},
    ui_plugin::{self, number_renderer::Block},
};

use bevy::prelude::*;

pub struct AnimationPlugin;

struct BlockAnimation {
    target: (f32, f32),
    step: (f32, f32),
    total_iter: usize,
    current_iter: usize,
}

impl BlockAnimation {
    pub fn new(target: (f32, f32), total_iter: usize) -> Self {
        Self {
            target: target,
            step: (0., 0.),
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
            timer: Timer::from_seconds(0.01, true),
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
                let target_x = ui_plugin::column_x(target.x.try_into().unwrap());
                let target_y = ui_plugin::row_y(target.y.try_into().unwrap());

                let animation = BlockAnimation::new((target_x, target_y), 15);
                state.blocks.insert(*id, animation);
            }
        }
    }
}

/**
 * Because everything was built out of UI elements, this means that
 * we don't have transforms for those elements, but instead Style
 * components define where elements are placed via their position.
 */
fn run_animations(
    time: Res<Time>,
    mut state: ResMut<State>,
    mut query: Query<(&mut Style, &Block), With<Block>>,
    mut events: EventWriter<AnimationCompleted>,
) {
    if state.timer.tick(time.delta()).just_finished() {
        for (mut style, block) in query.iter_mut() {
            if let Some(animation) = state.blocks.get_mut(&block.id) {
                if animation.current_iter == 0 {
                    let divisor: u16 = animation.total_iter.try_into().unwrap();
                    let divisor: f32 = divisor.try_into().unwrap();
                    // Calculate step size on first loop
                    let step_x: f32 = if let Val::Px(value) = style.position.left {
                        (animation.target.0 - value) / divisor
                    } else {
                        panic!("Was not a pixel value");
                    };

                    let step_y: f32 = if let Val::Px(value) = style.position.top {
                        (animation.target.1 - value) / divisor
                    } else {
                        panic!("Was not a pixel value");
                    };

                    animation.step = (step_x, step_y);
                }

                animation.current_iter += 1;

                let (x, y) = animation.step;
                style.position.left = style.position.left.add(x);
                style.position.top = style.position.top.add(y);

                if animation.total_iter == animation.current_iter {
                    state.blocks.remove(&block.id);
                }
            }
        }

        if state.blocks.len() == 0 {
            state.running = false;
            events.send(AnimationCompleted);
        }
    }
}
