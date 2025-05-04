use avian3d::prelude::*;
use bevy::prelude::*;

use crate::screens::Screen;

use super::timebank::TimeBank;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), make_timebank);
}

fn make_timebank(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            //TimeBank { milliseconds: 5000 },
            SceneRoot(
                asset_server.load(GltfAssetLabel::Scene(0).from_asset("scenes/TimePickUp.glb")),
            ),
            Transform::from_xyz(0., 0., -5.),
            Sensor,
            RigidBody::Static,
            CollisionEventsEnabled,
        ))
        .observe(|trigger: Trigger<OnCollisionStart>| {
            error!("WEIOFUJQWOEIFJAIOEJF");
        });
}
