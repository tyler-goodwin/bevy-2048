use bevy::prelude::*;

mod ui_plugin;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(ui_plugin::UIPlugin)
        .run();
}