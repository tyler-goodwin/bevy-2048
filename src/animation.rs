use std::{collections::HashMap, ops::Add};

use crate::{
    events::{AnimationCompleted, BlocksMoved},
    ui_plugin::{self, number_renderer::Block},
};

use bevy::prelude::*;

pub struct AnimationPlugin;

struct BlockMoveAnimation {
    target: (f32, f32),
    step: (f32, f32),
    total_iter: usize,
    current_iter: usize,
}

impl BlockMoveAnimation {
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
}

impl State {
    pub fn new() -> Self {
        State {
            running: false,
            timer: Timer::from_seconds(0.01, true),
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

fn blocks_moved_listener(
    mut commands: Commands,
    query: Query<(Entity, &Block)>,
    mut events: EventReader<BlocksMoved>,
) {
    for event in events.iter() {
        for (entity, block) in query.iter() {
            for (id, target) in event.moves.iter() {
                if *id == block.id {
                    let target_x = ui_plugin::column_x(target.x.try_into().unwrap());
                    let target_y = ui_plugin::row_y(target.y.try_into().unwrap());

                    let animation = BlockMoveAnimation::new((target_x, target_y), 15);
                    commands.entity(entity).insert(animation);
                }
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
    mut commands: Commands,
    mut query: Query<(Entity, &mut Style, &mut BlockMoveAnimation)>,
    mut events: EventWriter<AnimationCompleted>,
) {
    if state.timer.tick(time.delta()).just_finished() {
        let mut count = 0;
        for (entity, mut style, mut animation) in query.iter_mut() {
            count += 1;
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
                count -= 1;
                commands.entity(entity).remove::<BlockMoveAnimation>();
            }
        }

        if count == 0 {
            state.running = false;
            events.send(AnimationCompleted);
        }
    }
}
