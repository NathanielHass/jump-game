use bevy::prelude::*;

#[derive(Component, Debug)]
pub(crate) struct Collider {
    pub width: f32,
    pub height: f32,
    pub collision_type: CollideType,
}

impl Collider {
    pub fn new(width: f32, height: f32, collision_type: CollideType) -> Self {
        Self {width, height, collision_type}
    }
}

pub(crate) fn is_touching(a: &Collider, b: &Collider, transform_delta: &Vec3) -> bool {
    ( a.width/2.  + b.width/2.  >= transform_delta.x.abs()) &&
    ( a.height/2. + b.height/2. >= transform_delta.y.abs())
}

#[derive(Component, Debug, PartialEq)]
pub(crate) enum CollideType {
    Dynamic,  // For Entities that move
    Static,   // For Entities that stay in place
    Trigger,  // Non-Physics Interactions
}

#[derive(Event, Debug)]
pub(crate) struct CollisionEvent {
    pub entity: Entity,
    pub collided_entity: Entity,
}

impl CollisionEvent {
    pub fn new(entity: Entity, collided_entity: Entity) -> Self {
        Self { entity, collided_entity}
    }
}

pub(crate) fn find_collisions_dynamic (
    colliders: Query<(Entity, &Collider, &Transform)>,
    mut collision_event_writer: EventWriter<CollisionEvent>,
) {
    let collisions = colliders.iter_combinations()
        .filter(|[ (_, collider_a, _), (_, collider_b, _)]|
            (collider_a.collision_type == CollideType::Trigger) !=
            (collider_b.collision_type == CollideType::Trigger)
        )
        .filter(|[ (_, collider_a, _), (_, collider_b, _)]|
            (collider_a.collision_type == CollideType::Static) !=
            (collider_b.collision_type == CollideType::Static)
        )
        .filter(|[ (_, collider_a, _), (_, collider_b, _)]|
            (
                (collider_a.collision_type == CollideType::Trigger) !=
                (collider_b.collision_type == CollideType::Static)
            ) || (
                (collider_a.collision_type == CollideType::Static) !=
                (collider_b.collision_type == CollideType::Trigger)
            )
        );
    for [
        (entity_a, collider_a, transform_a),
        (entity_b, collider_b, transform_b),
    ] in collisions {
        if is_touching(
            collider_a,
            collider_b,
            &(transform_a.translation - transform_b.translation),
        ) {
            collision_event_writer.write(CollisionEvent::new(entity_a, entity_b));
        };
    }
}

pub(crate) fn handle_collision (
    mut collision_event_reader: ResMut<Events<CollisionEvent>>,
    mut query: Query<(&mut Transform, &Collider)>,
) {
    for CollisionEvent{entity, collided_entity} in collision_event_reader.drain() {
        let Ok([
               (mut transform_a, Collider{width: width_a, height: height_a, collision_type: type_a}),
               (mut transform_b, Collider{width: width_b, height: height_b, collision_type: type_b}),
        ]) = query.get_many_mut([entity, collided_entity]) else {continue;};

        let delta = transform_b.translation - transform_a.translation;

        let overlap = Vec2::new(
            width_a/2. + width_b/2. - delta.x.abs(),
            height_a/2. + height_b/2. - delta.y.abs(),
        );

        let axis: bool = overlap.x.abs() < overlap.y.abs() && overlap.x.abs() != 0.;

        match (type_a, type_b) {
            (CollideType::Dynamic, CollideType::Dynamic) => {
                if axis {
                    transform_a.translation.x += overlap.x/2.;
                    transform_b.translation.x -= overlap.x/2.;
                }
                else {
                    transform_a.translation.y += overlap.y/2.;
                    transform_b.translation.y -= overlap.y/2.;
                }
            },
            (CollideType::Dynamic, CollideType::Static) => {
                if axis { transform_a.translation.x += overlap.x; }
                else { transform_a.translation.y += overlap.y; }
            },
            (CollideType::Static, CollideType::Dynamic) => {
                if axis { transform_b.translation.x -= overlap.x; }
                else { transform_b.translation.y -= overlap.y; }
            },
            (CollideType::Trigger, CollideType::Dynamic) |
            (CollideType::Dynamic, CollideType::Trigger) => {
                 todo!("Handle Trigger Volumes");
            },
            _ => (),
        }
    }
}
