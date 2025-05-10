use animation::ElevatorAnimation;
use avian3d::prelude::{Collider, CollisionEventsEnabled, OnCollisionStart, RigidBody};
use bevy::prelude::*;

use crate::level::{Level, LevelOrigins};

use super::{
    blender::{BlenderObject, replace_blender_object},
    interact::Interact,
    lives::LostLife,
};

mod animation;

pub fn plugin(app: &mut App) {
    app.register_type::<BlenderElevator>()
        .register_type::<KeyFor>()
        .register_type::<ElevatorKeys>()
        .add_plugins(animation::plugin)
        .add_observer(add_elevator_collider)
        .add_systems(PreUpdate, replace_blender_object::<BlenderElevator>)
        .add_observer(insert_elevator_key)
        .add_observer(reset_on_life_lost);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct BlenderElevator {
    level: Level,
}

impl BlenderObject for BlenderElevator {
    type BevyComponent = Elevator;
    fn level(&self) -> Level {
        self.level
    }

    fn to_component(&self) -> Self::BevyComponent {
        Elevator { level: self.level }
    }
}

/// Need this because the door should stay opened if all portal keys have been clicked
#[derive(Component)]
pub struct Activated;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Elevator {
    level: Level,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship(relationship_target = ElevatorKeys)]
pub struct KeyFor(pub Entity);

#[derive(Component, Default)]
pub struct ElevatorKey {
    pub interacted: bool,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship_target(relationship=KeyFor, linked_spawn)]
pub struct ElevatorKeys(Vec<Entity>);

///Elevator Collider with reference to it's mesh
#[derive(Component)]
struct ElevatorCollider(Entity);

fn add_elevator_collider(
    trigger: Trigger<OnAdd, Elevator>,
    elevators: Query<(Entity, &Transform), With<Elevator>>,
    mut commands: Commands,
    levels: Query<&Level>,
    elevator_keys: Query<(Entity, &Level), With<ElevatorKey>>,
) {
    let (elevator, transform) = elevators.get(trigger.target()).unwrap();
    let mut ec = commands.entity(trigger.target());

    // I hate this but we're spawning an elevator collider
    // seperate from the mesh.
    commands.spawn((
        ElevatorCollider(trigger.target()),
        RigidBody::Kinematic,
        CollisionEventsEnabled,
        Collider::cylinder(1., 0.5),
        *transform,
    ));
    // ec.observe(elevator_me_elsewhere)
    //     .observe(interact_with_keys)
    // ;
    // let elevator_level = levels.get(trigger.target()).unwrap();

    // for (entity, level) in elevator_keys {
    //     if elevator_level != level {
    //         continue;
    //     }
    //     commands.entity(entity).insert(KeyFor(trigger.target()));
    // }
}

fn insert_elevator_key(
    trigger: Trigger<OnAdd, ElevatorKey>,
    mut commands: Commands,
    levels: Query<&Level>,
    elevators: Query<(Entity, &Level), With<Elevator>>,
) {
    let key_level = levels.get(trigger.target()).unwrap();
    for (elevator, elevator_level) in elevators {
        if key_level == elevator_level {
            commands.entity(trigger.target()).insert(KeyFor(elevator));
        }
    }
}

// the user doesn't interact with the door directly. This is usually
// triggered by one of its keys.
fn interact_with_keys(
    trigger: Trigger<Interact>,
    mut commands: Commands,
    elevators: Query<(Entity, &ElevatorKeys, Has<Activated>)>,
    keys: Query<&ElevatorKey>,
    animations_to_play: Query<&ElevatorAnimation>,
    children: Query<&Children>,
    mut players: Query<&mut AnimationPlayer>,
) {
    let (elevator, elevator_keys, is_rising) = elevators.get(trigger.target()).unwrap();
    //see if we can open this activated elevator
    let mut can_rise = true;
    for key in keys.iter_many(&elevator_keys.0) {
        if !key.interacted {
            can_rise = false;
        }
    }

    //dont do anything, state didn't change
    if can_rise == is_rising {
        return;
    }

    if can_rise {
        commands.entity(trigger.target()).insert(Activated);
    } else {
        commands.entity(trigger.target()).remove::<Activated>();
    }

    if let Ok(animation_to_play) = animations_to_play.get(elevator) {
        for child in children.iter_descendants(elevator) {
            if let Ok(mut player) = players.get_mut(child) {
                let animation = player.animation_mut(animation_to_play.index).unwrap();
                let seek = animation.seek_time();
                if animation.is_paused() {
                    animation.resume();
                }
                animation.replay();
                animation.seek_to(seek);
                // play the animation to the end
                if can_rise {
                    animation.set_speed(1.);
                } else {
                    animation.set_speed(-1.);
                }
            }
        }
    }
}

//Interpolate collider position

fn reset_on_life_lost(_trigger: Trigger<LostLife>, mut keys: Query<&mut ElevatorKey>) {
    for mut key in &mut keys {
        key.interacted = false;
    }
}
