use avian3d::prelude::*;
use bevy::prelude::*;

use crate::level::{Level, LevelOrigins};

use super::{
    player::Player,
    room::{NewRoom, StartCountdown},
    win::GameWin,
};

pub fn plugin(app: &mut App) {
    app.register_type::<Portal>();
    app.add_observer(insert_portal);
}

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

///Portals are sensors
fn insert_portal(trigger: Trigger<OnAdd, Portal>, mut commands: Commands) {
    commands
        .entity(trigger.target())
        .insert((CollisionEventsEnabled, RigidBody::Static))
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
