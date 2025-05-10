use std::time::Duration;

use bevy::prelude::*;

use crate::gameplay::{lives::LostLife, room::StartCountdown};

use super::{GameSet, GameState, room::RoomStarted};

pub(crate) const DEFAULT_DURATION: Duration = Duration::from_secs(5);

pub fn plugin(app: &mut App) {
    app.insert_resource(LevelTimer::default())
        .add_observer(start_countdown)
        .add_systems(
            Update,
            tick_stopwatch
                .in_set(GameSet::TickTimers)
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(PostUpdate, out_of_time.run_if(in_state(GameState::Playing)))
        .add_observer(start_timer_on_level)
        .add_observer(on_lost_life);
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
fn start_countdown(trigger: Trigger<StartCountdown>, mut stopwatch: ResMut<LevelTimer>) {
    let event = trigger.event();
    stopwatch.pause();
    stopwatch.reset();
    stopwatch.set_duration(Duration::from_millis(event.0));
}

fn on_lost_life(_trigger: Trigger<LostLife>, mut commands: Commands, timer: Res<LevelTimer>) {
    commands.trigger(StartCountdown(timer.duration()));
}

fn start_timer_on_level(_trigger: Trigger<RoomStarted>, mut stopwatch: ResMut<LevelTimer>) {
    info!("Starting stopwatch");
    stopwatch.unpause();
}

fn out_of_time(stopwatch: Res<LevelTimer>, mut commands: Commands) {
    if !stopwatch.0.finished() {
        return;
    }

    error!(
        "Stopwatch stuff: {} {}",
        stopwatch.0.duration().as_secs_f64(),
        stopwatch.0.elapsed_secs_f64()
    );

    commands.trigger(LostLife);
}
