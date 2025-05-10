use bevy::prelude::*;
use bevy_dog::settings::{DoGSettings, PassesSettings, Thresholding};

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
                k: 0.5,
                tau: 32.,
                phi: 0.8,
                thresholding: Thresholding::Tanh as i32,
                thresholds: Vec4::from_array([12., 6., 3., 0.5]),
                ..DoGSettings::DEFAULT
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
