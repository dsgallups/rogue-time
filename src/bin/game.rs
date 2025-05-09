use bevy::prelude::*;
use rogue_time::*;
// this bin is used when reflected types have changed
fn main() {
    let mut app = App::new();

    app.add_plugins(AppPlugin::default());

    app.run();
}
