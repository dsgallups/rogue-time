use bevy::prelude::*;

use crate::{
    gameplay::GameState,
    screens::Screen,
    theme::{interaction::OnPress, widgets},
};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Paused), spawn_pause_ui);
}

fn spawn_pause_ui(mut commands: Commands) {
    //we attach state scoped
    commands.spawn((
        StateScoped(GameState::Paused),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(10.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        BackgroundColor(Color::srgba(0., 0., 0., 0.24)),
        ZIndex(2),
        children![
            widgets::button("Resume", resume),
            widgets::button("Exit", exit_to_title)
        ],
    ));
}
fn resume(_trigger: Trigger<OnPress>, mut next_screen: ResMut<NextState<GameState>>) {
    next_screen.set(GameState::Playing);
}

fn exit_to_title(_trigger: Trigger<OnPress>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
