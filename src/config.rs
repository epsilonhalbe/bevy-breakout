use bevy::{
    math::{Vec2, Vec3},
    render::color::Color,
    ui::Val,
};

use crate::domains::{paddle, scoreboard};

pub struct Configuration {
    pub scoreboard: scoreboard::Config,
    pub paddle: paddle::Config,
    // ball: ball::Config,
}

pub const CONFIG: Configuration = Configuration {
    scoreboard: scoreboard::Config {
        font_size: 40.0,
        color: Color::rgb(1.0, 0.5, 0.5),
        padding: Val::Px(5.0),
    },
    paddle: paddle::Config {
        size: Vec3::new(120.0, 20.0, 0.0),
        speed: 500.0,
        color: Color::rgb(0.3, 0.3, 0.7),
        margin_x: 10.0,
        margin_b: 60.0,
    },
};

// We set the z-value of the ball to 1 so it renders on top in the case of overlapping sprites.
pub const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
pub const BALL_SIZE: Vec3 = Vec3::new(30.0, 30.0, 0.0);
pub const BALL_SPEED: f32 = 400.0;
pub const BALL_STARTING_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);

pub const WALL_THICKNESS: f32 = 10.0;
// x coordinates
pub const LEFT_WALL: f32 = -450.0;
pub const RIGHT_WALL: f32 = 450.0;
// y coordinates
pub const BOTTOM_WALL: f32 = -300.0;
pub const TOP_WALL: f32 = 300.0;

pub const BRICK_SIZE: Vec2 = Vec2::new(100.0, 30.0);
// These values are exact
pub const GAP_BETWEEN_PADDLE_AND_BRICKS: f32 = 270.0;
pub const GAP_BETWEEN_BRICKS: f32 = 5.0;
// These values are lower bounds, as the number of bricks is computed
pub const GAP_BETWEEN_BRICKS_AND_CEILING: f32 = 20.0;
pub const GAP_BETWEEN_BRICKS_AND_SIDES: f32 = 20.0;

pub const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
pub const BALL_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
pub const BRICK_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
pub const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
pub const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
