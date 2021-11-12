use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum CustomStage {
    Before,
    After,
}

pub struct StagePlugin;

impl Plugin for StagePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_stage_before(
            CoreStage::Update,
            CustomStage::Before,
            SystemStage::parallel(),
        )
        .add_stage_after(
            CoreStage::Update,
            CustomStage::After,
            SystemStage::parallel(),
        );
    }
}
