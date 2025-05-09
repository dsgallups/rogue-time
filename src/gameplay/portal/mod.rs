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
        .register_type::<PortalKeys>()
        .add_plugins(animation::plugin)
        .add_observer(insert_portal)
        .add_systems(PreUpdate, replace_blender_object::<BlenderPortal>)
        .add_observer(insert_portal_key);
}

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

#[derive(Component)]
pub struct Opened;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Portal {
    to: Level,
    initial_stopwatch_duration: u64,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship(relationship_target = PortalKeys)]
pub struct KeyFor(pub Entity);

#[derive(Component)]
pub struct PortalKey;

#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship_target(relationship=KeyFor, linked_spawn)]
pub struct PortalKeys(Vec<Entity>);

fn insert_portal(
    trigger: Trigger<OnAdd, Portal>,
    mut commands: Commands,
    levels: Query<&Level>,
    portal_keys: Query<(Entity, &Level), With<PortalKey>>,
) {
    commands
        .entity(trigger.target())
        .observe(portal_me_elsewhere);
    let portal_level = levels.get(trigger.target()).unwrap();

    for (entity, level) in portal_keys {
        if portal_level != level {
            continue;
        }
        commands.entity(entity).insert(KeyFor(trigger.target()));
    }
}

fn insert_portal_key(
    trigger: Trigger<OnAdd, PortalKey>,
    mut commands: Commands,
    levels: Query<&Level>,
    portals: Query<(Entity, &Level), With<Portal>>,
) {
    let key_level = levels.get(trigger.target()).unwrap();
    for (portal, portal_level) in portals {
        if key_level == portal_level {
            commands.entity(trigger.target()).insert(KeyFor(portal));
        }
    }
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
