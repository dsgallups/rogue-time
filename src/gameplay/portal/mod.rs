use avian3d::prelude::*;
use bevy::prelude::*;

use super::{level::NewLevel, player::Player};

pub fn plugin(app: &mut App) {
    app.register_type::<Portal>();
    app.add_observer(insert_portal);
}

/// Used in bevy skein
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
    mut player: Query<&mut Transform, With<Player>>,
) {
    let portal = portals.get(trigger.target()).unwrap();

    let event = trigger.event();

    let Ok(mut player) = player.get_mut(event.collider) else {
        return;
    };

    player.translation = portal.to;
    commands.trigger(NewLevel);
}
