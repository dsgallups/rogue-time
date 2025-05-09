use bevy::prelude::*;

use super::time::LevelTimer;
/// Manages timeloop behaviour
pub fn plugin(app: &mut App) {
    app.add_systems(Update, trigger_each_loop);
}

/// Marker for logging the time the player interacted with something in previous loop
#[derive(Component)]
struct TriggeredAt(f32);

fn trigger_each_loop(time: Res<LevelTimer>, query: Query<(Entity, &TriggeredAt)>) {
    let time_elapsed = time.0.elapsed_secs();

    for (entity, deadline) in query {
        if (time_elapsed > deadline.0) {
            info!("{entity} trigged on loop")
        }
    }
}
