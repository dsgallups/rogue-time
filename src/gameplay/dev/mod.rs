use bevy::prelude::*;

mod test_objects;
mod timebank;

pub fn plugin(app: &mut App) {
    //make_timebank,
    app.add_plugins((test_objects::plugin, timebank::plugin));
}
