use avian3d::prelude::*;
use bevy::prelude::*;

use crate::gameplay::{
    player::Player,
    portal::{Portal, PortalKey},
};

pub fn plugin(app: &mut App) {
    //app.add_observer(debug_portal_insert);
    // app.add_systems(
    //     Update,
    //     determine_distance_from_portal.in_set(GameSet::Update),
    // );
    //

    app.add_systems(Update, open_portals);
}
// set all portal keys to interacted
fn open_portals(mut portal_keys: Query<&mut PortalKey>, keys: Res<ButtonInput<KeyCode>>) {
    if !keys.just_pressed(KeyCode::KeyP) {
        return;
    }

    for mut key in &mut portal_keys {
        key.interacted = true;
    }
}

///Portals are sensors
fn debug_portal_insert(
    trigger: Trigger<OnAdd, Portal>,
    mut commands: Commands,
    transform: Query<&Transform>,
) {
    let transform = transform.get(trigger.target()).unwrap();
    info!("Spawning portal at {:?}", transform.translation);

    commands.add_observer(debug_portaling);
}

fn debug_portaling(
    trigger: Trigger<OnCollisionStart>,
    colliders: Query<&ColliderOf>,
    player: Query<&Player>,
) {
    let event = trigger.event();

    let root_collider_is_player = player.get(event.collider).is_ok();

    if root_collider_is_player {
        info!("Player collided with portal!");
    } else {
        info!("Player didn't collid with portal!");
    }
    let parent = colliders.get(event.collider).unwrap();
    let collider_of_player = player.get(parent.body).is_ok();
    if collider_of_player {
        info!("is collider of player!");
    } else {
        info!("isn't collider of player!");
    }
}

fn determine_distance_from_portal(
    player: Query<&Transform, With<Player>>,
    portals: Query<&Transform, With<Portal>>,
) {
    let Ok(player) = player.single() else { return };
    for portal in portals {
        let diff = player.translation - portal.translation;

        info!("\nDistance from portal: {}\nDiff: {}", diff.length(), diff);
    }
}
