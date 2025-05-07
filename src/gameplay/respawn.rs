use bevy::prelude::*;

use super::{
    player::{Player, TeleportTo},
    room::NewRoom,
};

pub fn plugin(app: &mut App) {
    app.add_observer(init_respawn_point)
        .add_observer(set_respawn_point_on_new_room);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct RespawnPoint(pub Vec3);

fn init_respawn_point(trigger: Trigger<OnAdd, Player>, mut commands: Commands) {
    commands
        .entity(trigger.target())
        .insert(RespawnPoint(Vec3::ZERO));
}

fn set_respawn_point_on_new_room(
    trigger: Trigger<NewRoom>,
    mut commands: Commands,
    mut respawn_point: Query<&mut RespawnPoint>,
) {
    let mut respawn_point = respawn_point.single_mut().unwrap();

    respawn_point.0 = trigger.event().spawn_point;

    match trigger.facing {
        Some(facing) => {
            info!("I am facing {facing:?}");
            commands.trigger(TeleportTo::new_facing(trigger.spawn_point, facing));
        }
        None => {
            info!("I am not facing");
            commands.trigger(TeleportTo::new(trigger.spawn_point));
        }
    }
}
