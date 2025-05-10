use animation::PortalAnimation;
use avian3d::prelude::{Collider, CollisionEventsEnabled, OnCollisionStart};
use bevy::prelude::*;

use crate::level::{Level, LevelOrigins};

use super::{
    blender::{BlenderObject, replace_blender_object},
    interact::Interact,
    lives::LostLife,
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
        .add_observer(insert_portal_key)
        .add_observer(reset_on_life_lost);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct BlenderPortal {
    level: Level,
    to: Level,
    wins: bool,
    initial_stopwatch_duration: u64,
}

impl BlenderObject for BlenderPortal {
    type BevyComponent = Portal;
    fn level(&self) -> Level {
        self.level
    }

    fn to_component(&self) -> Self::BevyComponent {
        Portal {
            to: self.to,
            wins: self.wins,
            initial_stopwatch_duration: self.initial_stopwatch_duration,
        }
    }
}

/// Need this because the door should stay opened if all portal keys have been clicked
#[derive(Component)]
pub struct Opened;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Portal {
    to: Level,
    wins: bool,
    initial_stopwatch_duration: u64,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship(relationship_target = PortalKeys)]
pub struct KeyFor(pub Entity);

#[derive(Component, Default)]
pub struct PortalKey {
    pub interacted: bool,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship_target(relationship=KeyFor, linked_spawn)]
pub struct PortalKeys(Vec<Entity>);

fn insert_portal(
    trigger: Trigger<OnAdd, Portal>,
    portals: Query<&Portal>,
    mut commands: Commands,
    levels: Query<&Level>,
    portal_keys: Query<(Entity, &Level), With<PortalKey>>,
) {
    let portal = portals.get(trigger.target()).unwrap();
    let mut ec = commands.entity(trigger.target());
    if portal.wins {
        ec.insert(GameWin);
    };
    ec.insert((CollisionEventsEnabled, Collider::cuboid(10., 15., 1.)))
        .observe(portal_me_elsewhere)
        .observe(interact_with_keys);
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
    portals: Query<(&Portal, Has<GameWin>), With<Opened>>,
    player: Query<&Player>,
    spawn_points: Res<LevelOrigins>,
) {
    let Ok((portal, wins_game)) = portals.get(trigger.target()) else {
        return;
    };

    let event = trigger.event();

    if player.get(event.collider).is_err() {
        warn!("collided, but not sure with what");
        return;
    };

    if wins_game {
        commands.trigger(GameWin);
        return;
    }

    info!("Moving to level {:?}", portal.to);

    let spawn_point = spawn_points.get_spawn_point(portal.to);

    commands.trigger(NewRoom {
        spawn_point,
        facing: Some(Dir3::NEG_Z),
    });
    warn!("Starting countdown from portal");
    commands.trigger(StartCountdown(portal.initial_stopwatch_duration));
}

// the user doesn't interact with the door directly. This is usually
// triggered by one of its keys.
fn interact_with_keys(
    trigger: Trigger<Interact>,
    mut commands: Commands,
    portals: Query<(Entity, &PortalKeys, Has<Opened>), With<Portal>>,
    keys: Query<&PortalKey>,
    animations_to_play: Query<&PortalAnimation>,
    children: Query<&Children>,
    mut players: Query<&mut AnimationPlayer>,
) {
    let (portal, portal_keys, is_open) = portals.get(trigger.target()).unwrap();
    //see if we can open this door
    let mut can_open = true;
    for key in keys.iter_many(&portal_keys.0) {
        if !key.interacted {
            can_open = false;
        }
    }

    //dont do anything, state didn't change
    if can_open == is_open {
        return;
    }

    if can_open {
        commands.entity(trigger.target()).insert(Opened);
    } else {
        commands.entity(trigger.target()).remove::<Opened>();
    }

    if let Ok(animation_to_play) = animations_to_play.get(portal) {
        for child in children.iter_descendants(portal) {
            if let Ok(mut player) = players.get_mut(child) {
                let animation = player.animation_mut(animation_to_play.index).unwrap();
                let seek = animation.seek_time();
                if animation.is_paused() {
                    animation.resume();
                }
                animation.replay();
                animation.seek_to(seek);
                // play the animation to the end
                if can_open {
                    animation.set_speed(1.);
                } else {
                    animation.set_speed(-1.);
                }
            }
        }
    }
}

fn reset_on_life_lost(_trigger: Trigger<LostLife>, mut keys: Query<&mut PortalKey>) {
    for mut key in &mut keys {
        key.interacted = false;
    }
}
