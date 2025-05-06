use avian3d::prelude::*;
use bevy::prelude::*;

use super::{
    player::Player,
    room::{NewRoom, StartCountdown},
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
    to: Vec3,
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
    portals: Query<&Portal>,
    player: Query<&Player>,
) {
    let portal = portals.get(trigger.target()).unwrap();

    let event = trigger.event();

    if player.get(event.collider).is_err() {
        return;
    };

    commands.trigger(NewRoom {
        spawn_point: portal.to,
    });
    commands.trigger(StartCountdown(5000));
}
