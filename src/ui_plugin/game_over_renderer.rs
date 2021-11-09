use crate::ui_plugin::{HEIGHT, WIDTH};
use bevy::prelude::*;

pub struct GameOverRoot;

pub fn render(
    parent: &mut ChildBuilder,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    font: Handle<Font>,
) {
    parent
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(WIDTH), Val::Px(HEIGHT)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: materials.add(Color::rgba_u8(0, 0, 0, 180).into()),
            ..Default::default()
        })
        .insert(GameOverRoot)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Game Over!",
                    TextStyle {
                        font: font.clone(),
                        font_size: 60.0,
                        color: Color::WHITE,
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                ),
                ..Default::default()
            });
        });
}
