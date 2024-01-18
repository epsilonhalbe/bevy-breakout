use bevy::prelude::*;

use crate::config::*;

#[derive(Component)]
pub struct Paddle;

pub struct Config {
    pub size: Vec3,
    pub speed: f32,
    pub color: Color,
    pub margin_x: f32, // horizontal margin
    pub margin_b: f32,
}

impl Paddle {
    pub fn move_(
        keyboard_input: Res<Input<KeyCode>>,
        mut query: Query<&mut Transform, With<Paddle>>,
        time: Res<Time>,
    ) {
        let mut paddle_transform = query.single_mut();
        let mut direction = 0.0;

        if keyboard_input.pressed(KeyCode::Left) {
            direction -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            direction += 1.0;
        }

        // Calculate the new horizontal paddle position based on player input
        let new_paddle_position =
            paddle_transform.translation.x + direction * CONFIG.paddle.speed * time.delta_seconds();

        // Update the paddle position,
        // making sure it doesn't cause the paddle to leave the arena
        let left_bound =
            LEFT_WALL + WALL_THICKNESS / 2.0 + CONFIG.paddle.size.x / 2.0 + CONFIG.paddle.margin_x;
        let right_bound =
            RIGHT_WALL - WALL_THICKNESS / 2.0 - CONFIG.paddle.size.x / 2.0 - CONFIG.paddle.margin_x;

        paddle_transform.translation.x = new_paddle_position.clamp(left_bound, right_bound);
    }

    pub fn ui() -> SpriteBundle {
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, BOTTOM_WALL + CONFIG.paddle.margin_b, 0.0),
                scale: CONFIG.paddle.size,
                ..default()
            },
            sprite: Sprite {
                color: CONFIG.paddle.color,
                ..default()
            },
            ..default()
        }
    }
}
