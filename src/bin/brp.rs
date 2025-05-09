use bevy::prelude::*;
use rogue_time::*;

// this bin is used when reflected types have changed. It won't actually instantiate levels, and not kick off
// other things that could be potentially erroneous
fn main() {
    let mut app = App::new();

    app.add_plugins(AppPlugin { load_level: false });

    app.run();
}
