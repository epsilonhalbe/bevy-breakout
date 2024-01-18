use bevy::prelude::*;

use crate::config::CONFIG;

#[derive(Resource)]
pub struct Scoreboard {
    pub score: usize,
}

pub struct Config {
    pub font_size: f32,
    pub color: Color,
    pub padding: Val,
}

impl Scoreboard {
    pub fn new() -> Scoreboard {
        Scoreboard { score: 0 }
    }

    pub fn update(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
        let mut text = query.single_mut();
        text.sections[1].value = scoreboard.score.to_string();
    }

    pub fn ui() -> TextBundle {
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font_size: CONFIG.scoreboard.font_size,
                    color: CONFIG.scoreboard.color,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: CONFIG.scoreboard.font_size,
                color: CONFIG.scoreboard.color,
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: CONFIG.scoreboard.padding,
            left: CONFIG.scoreboard.padding,
            ..default()
        })
    }
}

impl Default for Scoreboard {
    fn default() -> Self {
        Self::new()
    }
}
