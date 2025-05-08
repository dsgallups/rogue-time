use std::time::Duration;

use bevy::prelude::*;

use crate::gameplay::{player::TeleportTo, room::StartCountdown};

use super::{GameSet, GameState, lives::Lives, respawn::RespawnPoint, room::RoomStarted};

const DEFAULT_DURATION: Duration = Duration::from_secs(5);

pub fn plugin(app: &mut App) {
    app.insert_resource(LevelTimer::default())
        .add_observer(reset_on_new_level)
        .add_systems(Update, tick_stopwatch.in_set(GameSet::TickTimers))
        .add_systems(PostUpdate, out_of_time.run_if(in_state(GameState::Playing)))
        .add_observer(start_timer_on_level);
}

#[derive(Resource)]
pub(crate) struct LevelTimer(pub Timer);

impl Default for LevelTimer {
    fn default() -> Self {
        let mut timer = Timer::new(DEFAULT_DURATION, TimerMode::Once);
        timer.pause();
        Self(timer)
    }
}

#[allow(dead_code)]
impl LevelTimer {
    pub fn pause(&mut self) {
        self.0.pause();
    }
    pub fn unpause(&mut self) {
        self.0.unpause();
    }
    pub fn add_time(&mut self, time: Duration) {
        let current_duration = self.0.duration();

        let new_duration = current_duration + time;
        self.0.set_duration(new_duration);
    }
    pub fn set_duration(&mut self, new_duration: Duration) {
        self.0.set_duration(new_duration);
    }
    /// returns in millis
    pub fn duration(&self) -> u64 {
        self.0.duration().as_millis() as u64
    }
    pub fn reset(&mut self) {
        self.0.reset();
    }
}

fn tick_stopwatch(mut stopwatch: ResMut<LevelTimer>, time: Res<Time>) {
    stopwatch.0.tick(time.delta());
}
fn reset_on_new_level(trigger: Trigger<StartCountdown>, mut stopwatch: ResMut<LevelTimer>) {
    let event = trigger.event();
    stopwatch.pause();
    stopwatch.reset();
    stopwatch.set_duration(Duration::from_millis(event.0));
}

fn start_timer_on_level(_trigger: Trigger<RoomStarted>, mut stopwatch: ResMut<LevelTimer>) {
    info!("Starting stopwatch");
    stopwatch.unpause();
}

fn out_of_time(
    stopwatch: Res<LevelTimer>,
    mut commands: Commands,
    current_respawn_point: Single<&RespawnPoint>,
    mut lives: Single<&mut Lives>,
) {
    if !stopwatch.0.finished() {
        return;
    }

    error!(
        "Stopwatch stuff: {} {}",
        stopwatch.0.duration().as_secs_f64(),
        stopwatch.0.elapsed_secs_f64()
    );

    lives.remove_life();
    commands.trigger(TeleportTo::new(current_respawn_point.0));
    commands.trigger(StartCountdown(stopwatch.duration()));
}
