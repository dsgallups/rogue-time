use bevy::prelude::*;

use crate::gameplay::{GameSet, respawn::RespawnPoint};

use super::{Player, TeleportTo};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, rewind_input.in_set(GameSet::RecordInput));
}

#[derive(Event)]
pub struct RewindAnimation;

#[derive(Component)]
pub struct CanRewind;

// in theory, we could make this an observer attached to the player
// on the collect_timebank fn but fk it
fn rewind_input(
    mut commands: Commands,
    has_rewind: Query<Entity, (With<Player>, With<CanRewind>)>,
    respawn_point: Query<&RespawnPoint>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let Ok(entity) = has_rewind.single() else {
        //can't rewind
        return;
    };

    if !keys.just_pressed(KeyCode::KeyE) {
        return;
    }
    let respawn_point = respawn_point.single().unwrap();
    commands.entity(entity).remove::<CanRewind>();
    commands.trigger(RewindAnimation);
    commands.trigger_targets(TeleportTo(respawn_point.0), entity);
}
