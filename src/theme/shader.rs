use bevy::prelude::*;
use bevy_dog::settings::{DoGSettings, PassesSettings};

use crate::gameplay::player::camera::{PlayerCamera, WorldCamera};

// const DEFAULT_COLOR1: LinearRgba = LinearRgba::GREEN;

pub fn plugin(app: &mut App) {
    app.add_plugins((bevy_dog::plugin::DoGPlugin,))
        .add_systems(Update, insert_post_process);
}

fn insert_post_process(camera: Query<(Entity), (Added<WorldCamera>)>, mut commands: Commands) {
    for camera in camera {
        commands
            .entity(camera)
            .insert((DoGSettings::OUTLINE_DITHER, PassesSettings::default()));
    }
}
