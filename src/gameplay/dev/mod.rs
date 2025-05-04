use avian3d::prelude::*;
use bevy::prelude::*;

use crate::screens::Screen;

use super::{GameState, player::Player, timebank::TimeBank};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), (make_timebank, spawn_test_cube))
        .add_observer(fk)
        .add_systems(
            Update,
            (
                bevy,
                print_player_transform.run_if(in_state(GameState::Playing)),
            ),
        );
}

fn make_timebank(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        //TimeBank { milliseconds: 5000 },
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("scenes/TimePickUp.glb"))),
        Transform::from_xyz(0., 1., -5.),
        //Sensor,
        RigidBody::Static,
        CollisionEventsEnabled,
        //CollisionLayers::new(CollisionLayer::Character, LayerMask::ALL),
    ));
}

#[derive(Component)]
pub struct TestCube;
fn spawn_test_cube(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let cube_mesh = meshes.add(Cuboid::default());
    commands.spawn((
        //TimeBank { milliseconds: 5000 },
        Mesh3d(cube_mesh.clone()),
        MeshMaterial3d(materials.add(Color::srgb(0.7, 0.7, 0.8))),
        Transform::from_xyz(3., 1., -5.),
        RigidBody::Dynamic,
        CollisionEventsEnabled,
        CollidingEntities::default(),
        //CollisionLayers::new(CollisionLayer::Character, LayerMask::ALL),
    ));
}

fn fk(trigger: Trigger<OnCollisionStart>) {
    error!("WEIOFUJQWOEIFJAIOEJF");
}
fn bevy(mut events: EventReader<CollisionStarted>) {
    for event in events.read() {
        error!("EV EDWHEOFJIA");
    }
}

fn print_player_transform(player: Query<&Transform, With<Player>>) {
    //error!("player trns: {:?}", player.single().unwrap().translation);
}
