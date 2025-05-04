use bevy::prelude::*;

use crate::{screens::Screen, theme::Containers};

mod pause;

pub fn plugin(app: &mut App) {
    //decided to always show game ui in playing gamestate
    app.add_plugins((pause::plugin));
    app.add_systems(OnEnter(Screen::Gameplay), spawn_game_ui);
}

#[derive(Component)]
pub struct InteractionText;

#[derive(Component)]
pub struct HealthBar;

#[derive(Component)]
pub struct GameUi;

fn spawn_game_ui(mut commands: Commands) {
    // this does nothing essentially
    commands
        .ui_root()
        .insert((GameUi, Name::new("Game UI"), StateScoped(Screen::Gameplay)));
}
