use avian3d::prelude::*;
use bevy::prelude::*;

use crate::gameplay::GameState;

use super::{Player, camera::PlayerCamera};

//todo: give lever a collider
pub fn plugin(app: &mut App) {
    //todo
    app.add_systems(Update, interact.run_if(in_state(GameState::Playing)));
}

fn interact(
    buttons: Res<ButtonInput<MouseButton>>,
    spatial_query: SpatialQuery,
    player: Single<Entity, With<Player>>,
    camera: Single<&Transform, With<PlayerCamera>>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

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

    info!("hit: {hit:?}");

    //todo
}
