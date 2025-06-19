use bevy::prelude::*;
use collision::CollisionEvent;

pub mod collision;
pub mod motion;

const PIXELS_PER_METER: f32 = 10.;

pub(super) fn plugin(app: &mut App) {
    app.add_event::<CollisionEvent>();
    app.add_systems(Update, (collision::find_collisions, collision::handle_collision).chain());
}
