use bevy::{prelude::*, color::palettes::css::*};

#[derive(Component, Debug)]
pub(crate) struct AABBCollider {
    pub width: f32,
    pub height: f32,
}

impl AABBCollider {
    pub fn new(width: f32, height: f32) -> Self {
        Self {width, height}
    }
}

pub(crate) fn aabb_collision(a: &AABBCollider, b: &AABBCollider, transform_delta: &Vec3) -> bool {
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

pub(crate) fn find_collisions(
    mut gizmos: Gizmos,
    colliders: Query<(Entity, &AABBCollider, &Transform, &CollideType)>,
    mut collision_event_writer: EventWriter<CollisionEvent>,
) {
    // Draw Gizmos
    for (_, collider, transform, _) in colliders.iter() {
        gizmos.rect_2d(
            Isometry2d::new(transform.translation.truncate(),
            Rot2::default()),
            Vec2::new(collider.width, collider.height),
            RED
        );
    }

    let collisions = colliders.iter_combinations()
        .filter(|collider_pairs| match collider_pairs {
            [(_,_,_, CollideType::Dynamic), (_,_,_,_)] |
            [(_,_,_,_), (_,_,_, CollideType::Dynamic)] => true,
            _ => false,
        });
    for [
        (entity_a, collider_a, transform_a, _),
        (entity_b, collider_b, transform_b, _),
    ] in collisions {
        if aabb_collision(
            collider_a, collider_b,
            &(transform_a.translation - transform_b.translation),
        ) {
            collision_event_writer.write(CollisionEvent::new(entity_a, entity_b));
        };
    }
}

pub(crate) fn handle_collision (
    mut collision_event_reader: EventReader<CollisionEvent>,
    mut query: Query<(&mut Transform, &AABBCollider, &CollideType)>,
) {
    for CollisionEvent{entity, collided_entity} in collision_event_reader.read() {
        let Ok([
            (mut transform_a, AABBCollider{width: width_a, height: height_a}, type_a),
            (mut transform_b, AABBCollider{width: width_b, height: height_b}, type_b),
        ]) = query.get_many_mut([*entity, *collided_entity]) else {continue;};

        let delta = transform_b.translation - transform_a.translation;

        let overlap = Vec2::new(
            width_a/2. + width_b/2. - delta.x.abs(),
            height_a/2. + height_b/2. - delta.y.abs(),
        );

        let axis = overlap.x.abs() < overlap.y.abs() && overlap.x.abs() != 0.;

        match (type_a, type_b) {
            (CollideType::Dynamic, CollideType::Dynamic) => {
                if axis {
                    transform_a.translation.x += overlap.x/2.;
                    transform_b.translation.x -= overlap.x/2.;
                } else {
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
