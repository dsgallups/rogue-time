use std::time::Duration;

use bevy::prelude::*;

use crate::{
    gameplay::{lives::LostLife, room::StartCountdown},
    screens::Screen,
};

use super::{
    GameSet, GameState,
    player::rewind::{EndRewind, StartRewind},
    room::RoomStarted,
};

pub(crate) const DEFAULT_DURATION: Duration = Duration::from_secs(5);

pub fn plugin(app: &mut App) {
    app.insert_resource(Stopwatch::default())
        .add_observer(start_countdown)
        .add_systems(
            Update,
            tick_stopwatch
                .in_set(GameSet::TickTimers)
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(PostUpdate, out_of_time.run_if(in_state(GameState::Playing)))
        .add_systems(OnExit(Screen::Gameplay), reset_stopwatch)
        .add_observer(start_timer_on_level)
        .add_observer(on_start_rewind)
        .add_observer(on_end_rewind);
}

#[derive(Resource)]
pub(crate) struct Stopwatch {
    // this is what's going to display on the screen
    active: Timer,
    initial_duration: Duration,
}

impl Default for Stopwatch {
    fn default() -> Self {
        let mut timer = Timer::new(DEFAULT_DURATION, TimerMode::Once);
        timer.pause();
        Self {
            active: timer,
            initial_duration: DEFAULT_DURATION,
        }
    }
}

#[allow(dead_code)]
impl Stopwatch {
    pub fn pause(&mut self) {
        self.active.pause();
    }
    pub fn unpause(&mut self) {
        self.active.unpause();
    }
    pub fn remaining(&self) -> Duration {
        self.active.remaining()
    }
    pub fn duration(&self) -> Duration {
        self.active.duration()
    }
    pub fn elapsed(&self) -> Duration {
        self.active.elapsed()
    }
    pub fn add_time(&mut self, time: Duration) {
        let current_duration = self.active.duration();

        let new_duration = current_duration + time;
        self.active.set_duration(new_duration);
    }
    pub fn set_duration(&mut self, new_duration: Duration) {
        self.active.set_duration(new_duration);
    }
    /// returns in millis
    pub fn duration_millis(&self) -> u64 {
        self.active.duration().as_millis() as u64
    }

    pub fn remaining_secs(&self) -> f32 {
        self.active.remaining_secs()
    }
    pub fn reset(&mut self) {
        self.active.reset();
    }
}

fn tick_stopwatch(mut stopwatch: ResMut<Stopwatch>, time: Res<Time>) {
    let delta = time.delta();
    stopwatch.active.tick(delta);
}
fn start_countdown(trigger: Trigger<StartCountdown>, mut stopwatch: ResMut<Stopwatch>) {
    let event = trigger.event();
    stopwatch.pause();
    stopwatch.reset();
    let initial_duration = Duration::from_millis(event.0);
    stopwatch.set_duration(initial_duration);
    stopwatch.initial_duration = initial_duration;
}

fn reset_stopwatch(mut stopwatch: ResMut<Stopwatch>) {
    *stopwatch = Stopwatch::default();
}

fn start_timer_on_level(_trigger: Trigger<RoomStarted>, mut stopwatch: ResMut<Stopwatch>) {
    info!("Starting stopwatch");
    stopwatch.unpause();
}

fn out_of_time(stopwatch: Res<Stopwatch>, mut commands: Commands) {
    if !stopwatch.active.finished() {
        return;
    }

    commands.trigger(LostLife);
    commands.trigger(StartCountdown(stopwatch.initial_duration.as_millis() as u64));
}

fn on_start_rewind(_trigger: Trigger<StartRewind>, mut stopwatch: ResMut<Stopwatch>) {
    stopwatch.pause();
}
fn on_end_rewind(_trigger: Trigger<EndRewind>, mut stopwatch: ResMut<Stopwatch>) {
    stopwatch.active = Timer::new(stopwatch.initial_duration, TimerMode::Once);
}
