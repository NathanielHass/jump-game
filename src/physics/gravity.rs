use bevy::prelude::*;
use crate::{
    physics::{motion::Velocity, collision::AABBCollider},
};

#[derive(Component, Debug)]
pub struct Ground;

#[derive(Component, Debug)]
pub enum GroundState{
    Grounded,
    Jumping,
    Falling,
}

pub fn update_ground_state(
    mut actor_query: Query<(&Transform, &Velocity, &mut GroundState, &AABBCollider)>,
    mut terrain_query: Query<(&Transform, &AABBCollider), With<Ground>>,
) {
    for (
        transform_a, velocity_a, mut ground_state,
        &AABBCollider{ width: width_a, height: height_a}
    ) in actor_query.iter_mut() {
        match ground_state {
            GroundState::Grounded => (),
            GroundState::Jumping => (),
            GroundState::Falling => (),
        }
        if velocity_a.value.y > 0. {continue;}
        ground_state.is_grounded = false;
        for (
            transform_b,
            &AABBCollider{width: width_b, height: height_b}
        ) in terrain_query.iter_mut() {
            if ground_state.is_grounded {continue;};

            let check = transform_a.translation - Vec3::new(0., height_a/2., 0.);
            let floor = transform_b.translation + Vec3::new(0., height_b/2., 0.);

            let delta = floor - check;

            if delta.y == 0. &&
               delta.x.abs() <= width_b/2. + width_a/2. {
                     ground_state.is_grounded = true;
            } else { ground_state.is_grounded = false; }
        }
    }
}
