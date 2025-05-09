use bevy::prelude::*;

use crate::screens::Screen;

pub fn plugin(app: &mut App) {
    app.register_type::<GameWin>();
}

#[derive(Component, Reflect, Event)]
#[reflect(Component)]
pub struct GameWin;
//needs to be used
pub fn win_game(_trigger: Trigger<GameWin>, mut state: ResMut<NextState<Screen>>) {
    state.set(Screen::Win);
}
