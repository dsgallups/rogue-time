use avian3d::prelude::OnCollisionStart;
use bevy::prelude::*;

use crate::level::{Level, LevelOrigins};

use super::{
    blender::{BlenderObject, replace_blender_object},
    player::Player,
    room::{NewRoom, StartCountdown},
    win::GameWin,
};

mod animation;

pub fn plugin(app: &mut App) {
    app.register_type::<BlenderPortal>()
        .register_type::<KeyFor>()
        .register_type::<Unlockers>()
        .add_plugins(animation::plugin)
        .add_observer(insert_portal)
        .add_systems(PreUpdate, replace_blender_object::<BlenderPortal>);
}

#[derive(Component)]
pub struct Opened;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct BlenderPortal {
    level: Level,
    to: Level,
    initial_stopwatch_duration: u64,
}

impl BlenderObject for BlenderPortal {
    type BevyComponent = Portal;
    fn level(&self) -> Level {
        self.level
    }

    fn to_component(&self) -> Self::BevyComponent {
        Portal {
            to: self.level,
            initial_stopwatch_duration: self.initial_stopwatch_duration,
        }
    }
}

// fn on_add_blender_portal(
//     mut commands: Commands,
//     blender_door: Query<(Entity, &Transform, &BlenderPortal)>,
//     level_origins: Res<LevelOrigins>,
// ) {
//     /*
//     commands
//         .entity(trigger.target())
//         .insert((CollisionEventsEnabled, RigidBody::Static))
//         .observe(portal_me_elsewhere);

//     */
//     for (entity, transform, door) in blender_door {
//         // we are going to take this thing,
//         // remove it from the scene entirely,
//         // and then construct it ourselves.
//         commands.entity(entity).despawn();
//         //commands.spawn((Door(door.0), *transform));
//         info!("Spawned Door")
//     }
// }

/// Used in bevy skein
///
///
/// TODO: need to give an initial time for the next room
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Portal {
    to: Level,
    initial_stopwatch_duration: u64,
}

fn insert_portal(trigger: Trigger<OnAdd, Portal>, mut commands: Commands) {
    commands
        .entity(trigger.target())
        .observe(portal_me_elsewhere);
}

fn portal_me_elsewhere(
    trigger: Trigger<OnCollisionStart>,
    mut commands: Commands,
    portals: Query<(&Portal, Has<GameWin>)>,
    player: Query<&Player>,
    spawn_points: Res<LevelOrigins>,
) {
    let (portal, wins_game) = portals.get(trigger.target()).unwrap();

    let event = trigger.event();

    if player.get(event.collider).is_err() {
        return;
    };

    if wins_game {
        commands.trigger(GameWin);
        return;
    }

    let spawn_point = spawn_points.get_spawn_point(portal.to);

    commands.trigger(NewRoom {
        spawn_point,
        facing: Some(Dir3::NEG_Z),
    });
    commands.trigger(StartCountdown(portal.initial_stopwatch_duration));
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship(relationship_target = Unlockers )]
pub struct KeyFor(pub Entity);

#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship_target(relationship=KeyFor, linked_spawn)]
pub struct Unlockers(Vec<Entity>);

// Should open the door with the captured entity
#[derive(Event)]
struct OpenDoor(Entity);

fn portal_lever_connection() {}
