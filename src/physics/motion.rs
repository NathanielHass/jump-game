use bevy::prelude::*;
use crate::physics::{
    PIXELS_PER_METER,
    collision::CollisionType,
    gravity::GroundState,
};

const FORCE_GRAVITY: f32 = PIXELS_PER_METER * 500.;

const ACCELERATION_MAXIMUM: f32 = PIXELS_PER_METER * 1000.;
const ACCELERATION_MINIMUM: f32 = PIXELS_PER_METER * 65.;
const ACCELERATION_DECAY: f32 = PIXELS_PER_METER * 100.;

const VELOCITY_MAXIMUM: f32 = PIXELS_PER_METER * 1000.;
const VELOCITY_MINIMUM: f32 = PIXELS_PER_METER * 65.;

#[derive(Component, Debug)]
pub struct Velocity{pub value: Vec2}

#[derive(Component, Debug)]
pub struct Acceleration{pub value: Vec2}

pub fn apply_forces(
    mut query: Query<(&mut Acceleration, &mut Velocity, &GroundState, &CollisionType)>,
    time: Res<Time>,
) {
    for (mut acceleration, mut velocity, ground_state, collision_type) in query.iter_mut() {
        match collision_type {
            CollisionType::Dynamic => (),
            _ => continue,
        };

        if ground_state.is_grounded {
            acceleration.value.y = 0.;
            velocity.value.y = 0.;
        } else {
            acceleration.value.y -= FORCE_GRAVITY * time.delta_seconds();
        }

        acceleration.value.x +=
            if acceleration.value.x > 0. { -ACCELERATION_DECAY * time.delta_seconds() }
            else if acceleration.value.x < 0. { ACCELERATION_DECAY * time.delta_seconds()}
            else { 0.};
        acceleration.value = acceleration.value.clamp(
            Vec2::splat(-ACCELERATION_MAXIMUM * time.delta_seconds()),
            Vec2::splat( ACCELERATION_MAXIMUM * time.delta_seconds()),
        );

        velocity.value += acceleration.value * time.delta_seconds();
        velocity.value = velocity.value.clamp(
            Vec2::splat(-VELOCITY_MAXIMUM * time.delta_seconds()),
            Vec2::splat( VELOCITY_MAXIMUM * time.delta_seconds()),
        );
    }
}

pub fn update_position(
    mut query: Query<(&Velocity, &mut Transform)>,
    time: Res<Time>,
) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.value.extend(0.) * time.delta_seconds();
    }
}
