use bevy::prelude::*;
use crate::physics::{
    PIXELS_PER_METER,
};

const FORCE_GRAVITY: f32 = PIXELS_PER_METER * 500.;

const ACCELERATION_MAXIMUM: f32 = PIXELS_PER_METER * 1000.;
const ACCELERATION_MINIMUM: f32 = PIXELS_PER_METER * 65.;
const ACCELERATION_DECAY: f32 = PIXELS_PER_METER * 100.;

const VELOCITY_MAXIMUM: f32 = PIXELS_PER_METER * 1000.;
const VELOCITY_MINIMUM: f32 = PIXELS_PER_METER * 65.;

#[derive(Component, Debug)]
pub struct Velocity(Vec2);

#[derive(Component, Debug)]
pub struct Acceleration(Vec2);

pub fn apply_forces(
    mut query: Query<(&mut Acceleration, &mut Velocity)>,
    time: Res<Time>,
) {
    for (mut acceleration, mut velocity) in query.iter_mut() {
        // acceleration.0.y -= FORCE_GRAVITY * time.delta_secs();

        acceleration.0.x +=
            if acceleration.0.x > 0. { -ACCELERATION_DECAY * time.delta_secs() }
            else if acceleration.0.x < 0. { ACCELERATION_DECAY * time.delta_secs()}
            else { 0.};
        acceleration.0 = acceleration.0.clamp(
            Vec2::splat(-ACCELERATION_MAXIMUM * time.delta_secs()),
            Vec2::splat( ACCELERATION_MAXIMUM * time.delta_secs()),
        );

        velocity.0 += acceleration.0 * time.delta_secs();
        velocity.0 = velocity.0.clamp(
            Vec2::splat(-VELOCITY_MAXIMUM * time.delta_secs()),
            Vec2::splat( VELOCITY_MAXIMUM * time.delta_secs()),
        );
    }
}

pub fn update_position(
    mut query: Query<(&Velocity, &mut Transform)>,
    time: Res<Time>,
) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.0.extend(0.) * time.delta_secs();
    }
}
