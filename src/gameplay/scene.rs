use bevy::prelude::*;
pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_scene);
}
fn spawn_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SceneRoot(
        asset_server.load(GltfAssetLabel::Scene(0).from_asset("scenes/sandbox.glb")),
    ));

    commands.spawn((
        PointLight {
            intensity: 5000.,
            ..default()
        },
        Transform::default(),
    ));
}
