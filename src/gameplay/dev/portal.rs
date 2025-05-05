use avian3d::prelude::*;
use bevy::prelude::*;

use crate::gameplay::{GameSet, player::Player, portal::Portal};

pub fn plugin(app: &mut App) {
    app.add_observer(debug_portal_insert);
    app.add_systems(
        Update,
        determine_distance_from_portal.in_set(GameSet::Update),
    );
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

fn debug_portaling(trigger: Trigger<OnCollisionStart>, player: Query<&Player>) {
    let event = trigger.event();

    let is_player = player.get(event.collider).is_ok();

    if is_player {
        info!("Player collided with portal!");
    } else {
        info!("Player didn't collid with portal!");
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
