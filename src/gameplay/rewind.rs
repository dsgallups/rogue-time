use bevy::prelude::*;

use super::{player::Player, respawn::RespawnPoint};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(on_rewind);
}

#[derive(Component)]
pub struct CanRewind;

#[derive(Event)]
pub struct Rewind;

// may want to split this up into the seperate plugins' systems
fn on_rewind(
    _trigger: Trigger<Rewind>,
    mut player: Query<(&mut Transform, &RespawnPoint), With<Player>>,
) {
    //camera shake? idk. definitely move camera back
    let Ok((mut trns, respawn_point)) = player.single_mut() else {
        error!("Can't rewind player!");
        return;
    };
    trns.translation = respawn_point.0;
}
