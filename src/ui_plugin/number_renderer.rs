use crate::logic::number::Number;
use crate::ui_plugin::CELL_SIZE;
use bevy::prelude::*;

pub struct Block {
    pub id: i32,
}

impl Number {
    pub fn color(&self) -> Color {
        match self {
            Number::ZERO => Color::rgb_u8(240, 228, 218),
            Number::ONE => Color::rgb_u8(236, 224, 201),
            Number::TWO => Color::rgb_u8(255, 178, 120),
            Number::THREE => Color::rgb_u8(254, 150, 92),
            Number::FOUR => Color::rgb_u8(247, 123, 97),
            Number::FIVE => Color::rgb_u8(235, 88, 55),
            Number::SIX => Color::rgb_u8(236, 220, 146),
            Number::SEVEN => Color::rgb_u8(240, 212, 121),
            Number::EIGHT => Color::rgb_u8(244, 206, 96),
            Number::NINE => Color::rgb_u8(248, 200, 71),
            Number::TEN => Color::rgb_u8(255, 194, 46),
            Number::ELEVEN => Color::rgb_u8(104, 130, 249),
            Number::TWELVE => Color::rgb_u8(51, 85, 247),
            Number::THIRTEEN => Color::rgb_u8(10, 47, 222),
            Number::FOURTEEN => Color::rgb_u8(9, 43, 202),
            Number::FIFTEEN => Color::rgb_u8(181, 37, 188),
            Number::SIXTEEN => Color::rgb_u8(166, 34, 172),
        }
    }

    pub fn text_color(&self) -> Color {
        match self {
            Number::ZERO | Number::ONE => Color::BLACK,
            _ => Color::WHITE,
        }
    }

    pub fn text_size(&self) -> f32 {
        use Number::*;
        let multiplier: f32 = match self {
            ZERO | ONE | TWO | THREE | FOUR | FIVE => 0.5,
            SIX | SEVEN | EIGHT => 4.0 / 9.0,
            NINE | TEN | ELEVEN | TWELVE => 2.0 / 5.0,
            THIRTEEN | FOURTEEN | FIFTEEN => 7.0 / 20.0,
            SIXTEEN => 3.0 / 10.0,
        };
        CELL_SIZE * multiplier
    }

    pub fn render(
        &self,
        id: i32,
        position: (f32, f32),
        parent: &mut ChildBuilder,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        font: Handle<Font>,
    ) {
        parent
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Px(CELL_SIZE), Val::Px(CELL_SIZE)),
                    border: Rect::all(Val::Px(1.0)),
                    position_type: PositionType::Absolute,
                    position: Rect {
                        left: Val::Px(position.0),
                        top: Val::Px(position.1),
                        ..Default::default()
                    },
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                material: materials.add(self.color().into()),
                ..Default::default()
            })
            .with_children(|cell| {
                cell.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        format!("{}", self.value()),
                        TextStyle {
                            font: font.clone(),
                            font_size: self.text_size(),
                            color: self.text_color(),
                        },
                        TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Center,
                        },
                    ),
                    ..Default::default()
                });
            })
            .insert(Block { id: id });
    }
}
