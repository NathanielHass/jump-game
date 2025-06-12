use bevy::prelude::*;
use crate::{
    Player,
    physics::{motion::Velocity, collision::Collider},
};

#[derive(Component, Debug)]
pub struct GroundState{pub is_grounded:bool}

pub fn update_ground_state(
    mut actor_query: Query<(&Transform, &Velocity, &mut GroundState, &Collider), With<Player>>,
    mut terrain_query: Query<(&Transform, &Collider), Without<Player>>,
) {
    for (
        transform_a,
        velocity_a,
        mut ground_state,
        &Collider{ width: width_a, height: height_a}
    ) in actor_query.iter_mut() {
        if velocity_a.value.y > 0. {continue;}
        ground_state.is_grounded = false;
        for (
            transform_b,
            &Collider{width: width_b, height: height_b}
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
