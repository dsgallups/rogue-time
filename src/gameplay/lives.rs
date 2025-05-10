use bevy::prelude::*;

use crate::screens::Screen;

use super::{GameState, player::Player, room::StartCountdown, stopwatch::Stopwatch};

pub fn plugin(app: &mut App) {
    app.add_observer(setup_lives)
        .add_observer(remove_life)
        .add_systems(
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

/// when out of time
#[derive(Event)]
pub struct LostLife;

// this observer is pretty terrible
// but it's 2am now
fn remove_life(
    _trigger: Trigger<LostLife>,
    stopwatch: Res<Stopwatch>,
    mut commands: Commands,
    mut lives: Query<&mut Lives>,
) {
    for mut life in &mut lives {
        life.remove_life();
        // must do this or else it'll trigger on the next play
        if life.0 != 0 {
            commands.trigger(StartCountdown(stopwatch.initial_duration.as_millis() as u64));
        }
    }
}

fn on_no_more_lives(lives: Query<&Lives>, mut next_state: ResMut<NextState<Screen>>) {
    let Ok(lives) = lives.single() else {
        error!("No more lives, but expected something to be living!");
        return;
    };
    if lives.0 != 0 {
        return;
    }
    next_state.set(Screen::Lose);
}
