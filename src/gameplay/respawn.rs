use bevy::prelude::*;

use crate::gameplay::room::NewRoom;

use super::player::Player;

pub fn plugin(app: &mut App) {
    app.add_observer(init_respawn_point)
        .add_observer(update_respawn_point);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct RespawnPoint(pub Vec3);

fn init_respawn_point(trigger: Trigger<OnAdd, Player>, mut commands: Commands) {
    commands
        .entity(trigger.target())
        .insert(RespawnPoint(Vec3::ZERO));
}

fn update_respawn_point(trigger: Trigger<NewRoom>, mut respawn_point: Query<&mut RespawnPoint>) {
    let mut respawn_point = respawn_point.single_mut().unwrap();

    respawn_point.0 = trigger.event().respawn_point;
}
