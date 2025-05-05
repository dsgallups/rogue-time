#![allow(dead_code)]
use bevy::prelude::*;

mod portal;
mod test_objects;
mod timebank;

pub fn plugin(app: &mut App) {
    //make_timebank,
    app.add_plugins((timebank::plugin, portal::plugin));
}
