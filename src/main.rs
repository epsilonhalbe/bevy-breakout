use crate::{
    config::*,
    domains::{arena::*, ball::Ball, brick::Brick, paddle::Paddle},
};
use bevy::prelude::*;
use domains::{
    ball::Velocity,
    collision::{self, Collider},
    scoreboard::Scoreboard,
};

pub mod config;
pub mod domains;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Scoreboard::new())
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_event::<collision::Event>()
        .add_systems(Startup, setup)
        // Add our gameplay simulation systems to the fixed timestep schedule
        // which runs at 64 Hz by default
        .add_systems(
            FixedUpdate,
            (
                Velocity::update,
                Paddle::move_,
                collision::check,
                collision::play_sound,
            )
                // `chain`ing systems together runs them in order
                .chain(),
        )
        .add_systems(Update, (Scoreboard::update, bevy::window::close_on_esc))
        .run();
}

// Add the game's entities to our world
fn setup(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());
    // Sound
    commands.insert_resource(collision::Sound(
        asset_server.load("sounds/breakout_collision.ogg"),
    ));

    commands.spawn((Paddle::ui(), Paddle, Collider));
    // Ball
    commands.spawn(Ball::ui(meshes, materials));
    // Scoreboard
    commands.spawn(Scoreboard::ui());
    // Walls
    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
    commands.spawn(WallBundle::new(WallLocation::Top));

    for row in 0..Brick::rows() {
        for column in 0..Brick::columns() {
            let brick_position = Vec2::new(
                Brick::offset().x + column as f32 * (BRICK_SIZE.x + GAP_BETWEEN_BRICKS),
                Brick::offset().y + row as f32 * (BRICK_SIZE.y + GAP_BETWEEN_BRICKS),
            );

            // brick
            commands.spawn(Brick::ui(brick_position));
        }
    }
}
