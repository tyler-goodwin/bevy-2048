use bevy::prelude::*;
use std::path::Path;

use crate::events::ScoreChanged;

pub const WIDTH: f32 = 480.0;
pub const HEIGHT: f32 = 640.0;
const CELL_SIZE: f32 = WIDTH / 5.0;
const FIELD_SIZE: f32 = 50.0 + 4.0 * CELL_SIZE;
const LEFT_INDENT: f32 = (WIDTH - FIELD_SIZE) / 2.0;
const TOP_INDENT: f32 = 150.0;

pub struct UIPlugin;

struct ScoreText;
struct BestText;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(score_changed_listener.system());
        // .add_system(animate.system());
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(UiCameraBundle::default());

    let font: Handle<Font> = asset_server.load(Path::new("fonts").join("FiraSans-Bold.ttf"));
    let bg_color = materials.add(Color::rgb(0.725, 0.675, 0.627).into());
    let bg_cell_color = materials.add(Color::rgb(0.808, 0.753, 0.698).into());

    commands
        .spawn_bundle(root(&mut materials))
        .with_children(|parent| {
            build_header(parent, &mut materials, font);

            parent
                .spawn_bundle(background_field(bg_color))
                .with_children(|parent| {
                    for row in 0..4 {
                        for col in 0..4 {
                            parent.spawn_bundle(bg_cell(row, col, bg_cell_color.clone()));
                        }
                    }
                });
        });

    // commands.spawn_bundle(heading(font));
}

fn score_changed_listener(
    mut events: EventReader<ScoreChanged>,
    mut query: Query<&mut Text, With<ScoreText>>,
) {
    for event in events.iter() {
        for mut text in query.iter_mut() {
            text.sections[0].value = format!("{}", event.score);
        }
    }
}

// fn animate(time: Res<Time>, mut query: Query<&mut Transform, With<Text>>) {
//   for mut transform in query.iter_mut() {
//     transform.translation.x = 100.0 * time.seconds_since_startup().sin() as f32;
//     transform.translation.y = 100.0 * time.seconds_since_startup().cos() as f32;
//   }
// }

fn root(materials: &mut ResMut<Assets<ColorMaterial>>) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Px(WIDTH), Val::Px(HEIGHT)),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::FlexStart,
            ..Default::default()
        },
        material: materials.add(Color::rgb(0.992, 0.969, 0.941).into()),
        ..Default::default()
    }
}

fn build_header(
    parent: &mut ChildBuilder,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    font: Handle<Font>,
) {
    let no_color = materials.add(Color::NONE.into());
    let logo_color = materials.add(Color::rgb(0.929, 0.778, 0.012).into());
    let bg_color = materials.add(Color::rgb(0.725, 0.675, 0.627).into());

    parent
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Px(CELL_SIZE + 20.0)),
                padding: Rect {
                    left: Val::Px(LEFT_INDENT),
                    right: Val::Px(LEFT_INDENT),
                    top: Val::Px(LEFT_INDENT),
                    bottom: Val::Px(LEFT_INDENT),
                },
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(0.0),
                    top: Val::Px(LEFT_INDENT),
                    ..Default::default()
                },
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            material: no_color,
            ..Default::default()
        })
        .with_children(|wrapper| {
            wrapper
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(CELL_SIZE), Val::Px(CELL_SIZE)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    material: logo_color,
                    ..Default::default()
                })
                .with_children(|logo| {
                    logo.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "2048",
                            TextStyle {
                                font: font.clone(),
                                font_size: (CELL_SIZE * 0.5),
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
            wrapper
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(CELL_SIZE)),
                        align_items: AlignItems::FlexEnd,
                        justify_content: JustifyContent::FlexEnd,
                        ..Default::default()
                    },
                    material: materials.add(Color::NONE.into()),
                    ..Default::default()
                })
                .with_children(|right_wrapper| {
                    build_score(right_wrapper, bg_color.clone(), font.clone());
                    build_best(right_wrapper, bg_color, font.clone());
                });
        });
}

fn build_score(parent: &mut ChildBuilder, bg_color: Handle<ColorMaterial>, font: Handle<Font>) {
    parent
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(CELL_SIZE * 1.5), Val::Px(CELL_SIZE * 0.8)),
                padding: Rect::all(Val::Px(5.0)),
                margin: Rect {
                    left: Val::Px(5.0),
                    right: Val::Px(5.0),
                    ..Default::default()
                },
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: bg_color,
            ..Default::default()
        })
        .with_children(|bg| {
            bg.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "0",
                    TextStyle {
                        font: font.clone(),
                        font_size: CELL_SIZE * 0.5,
                        color: Color::WHITE,
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                ),
                ..Default::default()
            })
            .insert(ScoreText);

            bg.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Score",
                    TextStyle {
                        font: font.clone(),
                        font_size: CELL_SIZE * 0.25,
                        color: Color::WHITE,
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Top,
                        horizontal: HorizontalAlign::Center,
                    },
                ),
                ..Default::default()
            });
        });
}

fn build_best(parent: &mut ChildBuilder, bg_color: Handle<ColorMaterial>, font: Handle<Font>) {
    parent
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(CELL_SIZE * 1.5), Val::Px(CELL_SIZE * 0.8)),
                margin: Rect {
                    left: Val::Px(5.0),
                    ..Default::default()
                },
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: bg_color,
            ..Default::default()
        })
        .with_children(|bg| {
            bg.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "0",
                    TextStyle {
                        font: font.clone(),
                        font_size: CELL_SIZE * 0.5,
                        color: Color::WHITE,
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                ),
                ..Default::default()
            })
            .insert(BestText);

            bg.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Best",
                    TextStyle {
                        font: font.clone(),
                        font_size: CELL_SIZE * 0.25,
                        color: Color::WHITE,
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Top,
                        horizontal: HorizontalAlign::Center,
                    },
                ),
                ..Default::default()
            });
        });
}

fn background_field(color: Handle<ColorMaterial>) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Px(FIELD_SIZE), Val::Px(FIELD_SIZE)),
            border: Rect::all(Val::Px(1.0)),
            position_type: PositionType::Absolute,
            position: Rect {
                left: Val::Px(LEFT_INDENT),
                top: Val::Px(TOP_INDENT),
                ..Default::default()
            },
            ..Default::default()
        },
        material: color,
        ..Default::default()
    }
}

fn bg_cell(row: i32, col: i32, color: Handle<ColorMaterial>) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Px(CELL_SIZE), Val::Px(CELL_SIZE)),
            border: Rect::all(Val::Px(1.0)),
            position_type: PositionType::Absolute,
            position: Rect {
                left: Val::Px((10 + (10 + CELL_SIZE as i32) * row) as f32),
                top: Val::Px((10 + (10 + CELL_SIZE as i32) * col) as f32),
                ..Default::default()
            },
            ..Default::default()
        },
        material: color,
        ..Default::default()
    }
}
