use bevy::prelude::*;

use crate::screens::Screen;

use super::{GameState, player::Player};

pub fn plugin(app: &mut App) {
    app.add_observer(setup_lives).add_systems(
        PreUpdate,
        on_no_more_lives.run_if(in_state(GameState::Playing)),
    );
}

#[derive(Component)]
pub struct Lives(u8);
impl Lives {
    pub fn remove_life(&mut self) {
        self.0 = self.0.saturating_sub(1);
    }
    pub fn count(&self) -> u8 {
        self.0
    }
}

impl Default for Lives {
    fn default() -> Self {
        Self(3)
    }
}

fn setup_lives(trigger: Trigger<OnAdd, Player>, mut commands: Commands) {
    commands.entity(trigger.target()).insert(Lives::default());
}

fn on_no_more_lives(lives: Query<&Lives>, mut next_state: ResMut<NextState<Screen>>) {
    let lives = lives.single().expect("A single living thing");
    if lives.0 != 0 {
        return;
    }
    next_state.set(Screen::Title);
    //todo
}
