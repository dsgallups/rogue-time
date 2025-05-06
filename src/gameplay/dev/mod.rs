#![allow(dead_code)]
use bevy::prelude::*;

use super::player::camera::PlayerCamera;

mod inspector;
mod portal;
mod test_objects;
mod timebank;

pub fn plugin(app: &mut App) {
    //make_timebank,
    app.add_plugins((timebank::plugin, portal::plugin, inspector::gadget));
    //app.add_systems(Update, query_player_cam);
}

fn query_player_cam(query: Query<(&Transform, &PlayerCamera)>) {
    let Ok((trns, _c)) = query.single() else {
        info!("player camera: false");
        return;
    };
    info!("player_camera: {:?}", trns.translation);
}
