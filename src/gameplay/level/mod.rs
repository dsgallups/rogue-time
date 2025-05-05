use std::time::Duration;

use bevy::prelude::*;

use super::GameSet;

pub fn plugin(app: &mut App) {
    app.add_observer(on_new_level);
    app.add_systems(
        Update,
        (update_level_countdown, remove_level_countdown)
            .chain()
            .in_set(GameSet::TickTimers)
            .run_if(resource_exists::<LevelCountdown>),
    );
}

#[derive(Event)]
pub struct NewLevel;

#[derive(Resource)]
pub struct LevelCountdown {
    secs_left: u8,
    timer: Timer,
}

impl LevelCountdown {
    pub fn complete(&self) -> bool {
        self.secs_left == 0
    }
}

impl Default for LevelCountdown {
    fn default() -> Self {
        Self {
            secs_left: 3,
            timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
        }
    }
}

fn on_new_level(_trigger: Trigger<NewLevel>, mut commands: Commands) {
    commands.init_resource::<LevelCountdown>();
}

//fk it it'll also do other things
fn update_level_countdown(mut countdown: ResMut<LevelCountdown>, time: Res<Time>) {
    countdown.timer.tick(time.delta());
    if countdown.timer.just_finished() {
        countdown.secs_left = countdown.secs_left.saturating_sub(1);
    }
}

#[derive(Event)]
pub struct LevelStarted;

fn remove_level_countdown(mut commands: Commands, countdown: Res<LevelCountdown>) {
    info!("conutdowdnfiawedof");
    if countdown.complete() {
        info!("countdown complete");
        commands.remove_resource::<LevelCountdown>();
        commands.trigger(LevelStarted);
    }
}
