use bevy::prelude::*;

use crate::gameplay::{player::Player, timebank::TimeBank};

pub fn plugin(app: &mut App) {
    app.add_observer(on_timebank_insert);
}

fn on_timebank_insert(trigger: Trigger<OnAdd, TimeBank>, transform: Query<&Transform>) {
    let transform = transform.get(trigger.target()).unwrap();
    error!("Inserting timebank, {:?}", transform.translation);
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
