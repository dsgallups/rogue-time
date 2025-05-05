use avian3d::prelude::*;
use bevy::prelude::*;

use crate::gameplay::{player::Player, portal::Portal};

pub fn plugin(app: &mut App) {
    app.add_observer(debug_portal_insert);
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
