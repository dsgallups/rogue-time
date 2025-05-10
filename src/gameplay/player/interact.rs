use avian3d::prelude::*;
use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::gameplay::{GameState, interact::Interact};

use super::{Player, camera::PlayerCamera};

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        interact.run_if(in_state(GameState::Playing).and(input_just_pressed(MouseButton::Left))),
    );
}

fn interact(
    mut commands: Commands,
    spatial_query: SpatialQuery,
    player: Single<Entity, With<Player>>,
    camera: Single<&Transform, With<PlayerCamera>>,
) {
    let origin = camera.translation;
    let Ok(direction) = Dir3::new(camera.rotation * -Vec3::Z) else {
        warn!("Couldn't determine direction of interaction!");
        return;
    };
    let max_distance = 5.;
    let solid = true;
    let filter = SpatialQueryFilter::from_excluded_entities([*player]);

    let Some(hit) = spatial_query.cast_ray(origin, direction, max_distance, solid, &filter) else {
        info!("Nothing hit!");
        return;
    };

    commands.entity(hit.entity).trigger(Interact::default());
}
