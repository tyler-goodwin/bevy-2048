use bevy::prelude::*;
use std::path::{Path};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
      app.add_startup_system(setup.system())
        .add_system(animate.system());
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut windows: ResMut<Windows>) {
  commands.spawn_bundle(OrthographicCameraBundle::new_2d());
  let font = asset_server.load(Path::new("fonts").join("FiraSans-Bold.ttf"));
  commands.spawn_bundle(heading(font));

  let primary_window = windows.get_primary_mut().unwrap();
  setup_window(primary_window);
}

fn animate(time: Res<Time>, mut query: Query<&mut Transform, With<Text>>) {
  for mut transform in query.iter_mut() {
    transform.translation.x = 100.0 * time.seconds_since_startup().sin() as f32;
    transform.translation.y = 100.0 * time.seconds_since_startup().cos() as f32;
  }
}

fn setup_window(w: &mut Window) {
  w.set_title("Hello Bevy!".to_string());
  w.set_resolution(480.0, 640.0);
}

fn heading(font: Handle<Font>) -> Text2dBundle {
  Text2dBundle {
    text: Text::with_section(
      "Hello UI!",
      TextStyle {
        font: font,
        font_size: 60.0,
        color: Color::WHITE,
      },
      TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
      },
    ),
    ..Default::default()
  }
}