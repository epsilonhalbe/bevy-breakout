use bevy::prelude::*;

use crate::{config::*, domains::collision::Collider};

#[derive(Component)]
pub struct Brick;

impl Brick {
    pub fn ui(brick_position: Vec2) -> (SpriteBundle, Brick, Collider) {
        (
            SpriteBundle {
                sprite: Sprite {
                    color: BRICK_COLOR,
                    ..default()
                },
                transform: Transform {
                    translation: brick_position.extend(0.0),
                    scale: Vec3::new(BRICK_SIZE.x, BRICK_SIZE.y, 1.0),
                    ..default()
                },
                ..default()
            },
            Brick,
            Collider,
        )
    }

    pub fn columns() -> usize {
        let total_width_of_bricks = (RIGHT_WALL - LEFT_WALL) - 2. * GAP_BETWEEN_BRICKS_AND_SIDES;
        assert!(total_width_of_bricks > 0.0);
        (total_width_of_bricks / (BRICK_SIZE.x + GAP_BETWEEN_BRICKS)).floor() as usize
    }
    pub fn rows() -> usize {
        let total_height_of_bricks = TOP_WALL
            - BOTTOM_WALL
            - CONFIG.paddle.margin_b
            - GAP_BETWEEN_PADDLE_AND_BRICKS
            - GAP_BETWEEN_BRICKS_AND_CEILING;

        (total_height_of_bricks / (BRICK_SIZE.y + GAP_BETWEEN_BRICKS)).floor() as usize
    }

    pub fn offset() -> Vec2 {
        Vec2::new(
            (LEFT_WALL + RIGHT_WALL) / 2.0
                // Space taken up by the bricks
                - (Brick::columns() as f32 / 2.0 * BRICK_SIZE.x)
                // Space taken up by the gaps
                - (Brick::columns() - 1) as f32 / 2.0 * GAP_BETWEEN_BRICKS
                + BRICK_SIZE.x / 2.0,
            BOTTOM_WALL
                + CONFIG.paddle.margin_b
                + GAP_BETWEEN_PADDLE_AND_BRICKS
                + BRICK_SIZE.y / 2.0,
        )
    }
}
