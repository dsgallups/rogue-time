use bevy::prelude::*;
use bevy_dog::settings::{DoGSettings, PassesSettings};

use crate::gameplay::time::DEFAULT_DURATION;

use super::palette::{FULL_TIME_COLOR, NO_TIME_COLOR};

use crate::gameplay::time::LevelTimer;

pub fn plugin(app: &mut App) {
    app.add_plugins((bevy_dog::plugin::DoGPlugin,))
        .add_systems(Update, (insert_post_process, change_palette_with_time));
}

fn insert_post_process(query: Query<(Entity), (Added<Camera3d>)>, mut commands: Commands) {
    for camera in query {
        commands.entity(camera).insert((
            DoGSettings {
                min_color: Vec3::ZERO,
                max_color: Vec3::ONE,
                ..DoGSettings::OUTLINE_DITHER
            },
            PassesSettings::default(),
        ));
    }
}
/// Lerps between current timer default and 0, changing scene color
fn change_palette_with_time(time: Res<LevelTimer>, query: Query<&mut DoGSettings>) {
    let color = NO_TIME_COLOR
        + (FULL_TIME_COLOR - NO_TIME_COLOR) * time.0.remaining().div_duration_f32(DEFAULT_DURATION);
    for (mut setting) in query {
        setting.min_color = color.to_vec3();
    }
}
