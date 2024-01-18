use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::config::*;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(Vec2);

impl Velocity {
    pub fn update(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
        for (mut transform, velocity) in &mut query {
            transform.translation.x += velocity.x * time.delta_seconds();
            transform.translation.y += velocity.y * time.delta_seconds();
        }
    }
}

#[derive(Component)]
pub struct Ball;

impl Ball {
    pub fn ui(
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) -> (
        MaterialMesh2dBundle<bevy::prelude::ColorMaterial>,
        Ball,
        Velocity,
    ) {
        (
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::default().into()).into(),
                material: materials.add(ColorMaterial::from(BALL_COLOR)),
                transform: Transform::from_translation(BALL_STARTING_POSITION)
                    .with_scale(BALL_SIZE),
                ..default()
            },
            Ball,
            Velocity(BALL_STARTING_DIRECTION.normalize() * BALL_SPEED),
        )
    }
}
