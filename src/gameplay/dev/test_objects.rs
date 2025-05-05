use avian3d::prelude::*;
use bevy::prelude::*;

use crate::screens::Screen;
pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(Screen::Gameplay),
        (spawn_test_cube, spawn_test_timebank),
    )
    .add_systems(Update, test_collision_events);
}

#[derive(Component)]
pub struct TestTimeBank;

fn spawn_test_timebank(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TestTimeBank,
        Collider::cylinder(1., 2.),
        Transform::from_xyz(0., 1., -5.),
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("scenes/TimePickUp.glb"))),
        //Sensor,
        RigidBody::Static,
        Sensor,
        CollisionEventsEnabled,
        //CollisionLayers::new(CollisionLayer::Character, LayerMask::ALL),
        // children![(

        //     Transform::from_xyz(0., -1., 0.)
        // )],
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
        TestCube,
        Mesh3d(cube_mesh.clone()),
        MeshMaterial3d(materials.add(Color::srgb(0.7, 0.7, 0.8))),
        Transform::from_xyz(-10., 1., 0.),
        Collider::cuboid(1., 1., 1.),
        RigidBody::Static,
        Sensor,
        CollisionEventsEnabled,
        CollidingEntities::default(),
        //CollisionLayers::new(CollisionLayer::Character, LayerMask::ALL),
    ));
}

fn test_collision_events(
    mut events: EventReader<CollisionStarted>,
    test_cube: Query<&TestCube>,
    timebank: Query<&TestTimeBank>,
) {
    for event in events.read() {
        let with_tc = test_cube.get(event.0).is_ok() || test_cube.get(event.1).is_ok();
        let with_timebank = timebank.get(event.0).is_ok() || timebank.get(event.1).is_ok();
        error!("Collided, with test cube: {with_tc}, with timebank: {with_timebank}");
    }
}
