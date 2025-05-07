use bevy::prelude::*;
use bevy_dog::settings::{DoGSettings, PassesSettings};

pub fn plugin(app: &mut App) {
    app.add_plugins((bevy_dog::plugin::DoGPlugin,))
        .add_systems(Update, insert_post_process);
}

fn insert_post_process(query: Query<(Entity), (Added<Camera3d>)>, mut commands: Commands) {
    for camera in query {
        commands
            .entity(camera)
            .insert((DoGSettings::OUTLINE_DITHER, PassesSettings::default()));
    }
}
