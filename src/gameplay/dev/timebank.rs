use avian3d::prelude::OnCollisionStart;
use bevy::prelude::*;

use crate::gameplay::{player::Player, timebank::TimeBank};

pub fn plugin(app: &mut App) {
    app.add_observer(on_timebank_insert);
}

fn on_timebank_insert(
    trigger: Trigger<OnAdd, TimeBank>,
    mut commands: Commands,
    transform: Query<&Transform>,
) {
    let transform = transform.get(trigger.target()).unwrap();
    error!("Inserting timebank, {:?}", transform.translation);
    commands
        .entity(trigger.target())
        .observe(on_timebank_collision);
}

fn on_timebank_collision(
    trigger: Trigger<OnCollisionStart>,
    transform: Query<&Transform>,

    player: Query<&Player>,
) {
    let loc = transform.get(trigger.target()).unwrap().translation;
    let is_player = player.get(trigger.event().collider).is_ok();
    if is_player {
        error!("Collided with player at {loc:?}");
    } else {
        error!("Collided with not player at {loc:?}");
    }
}

#[allow(dead_code)]
fn print_player_transform(
    player: Query<&Transform, With<Player>>,
    timebank: Query<&Transform, With<TimeBank>>,
) {
    let player = player.single().unwrap();
    let Ok(timebank) = timebank.single() else {
        error!("No timebank!");
        return;
    };

    let diff = (player.translation - timebank.translation);
    warn!("distance: {}\ndiff: {:?} ", diff.length(), diff);
    //error!("player trns: {:?}", player.single().unwrap().translation);
}
