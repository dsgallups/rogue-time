use std::time::Duration;

use bevy::prelude::*;

use super::{
    GameSet, GameState,
    animation::AnimationPlayerAncestor,
    lives::Lives,
    room::{NewRoom, RoomStarted},
};

mod animation;

pub fn plugin(app: &mut App) {
    app.register_type::<Stopwatch>();

    app.add_plugins(animation::plugin);

    app.add_observer(on_stopwatch_spawn)
        .add_observer(reset_on_new_level)
        .add_systems(Update, tick_stopwatch.in_set(GameSet::TickTimers))
        .add_systems(PostUpdate, out_of_time.run_if(in_state(GameState::Playing)))
        .add_observer(start_timer_on_level);
}

const DEFAULT_DURATION: Duration = Duration::from_secs(5);

#[derive(Component)]
pub struct StopwatchTimer(pub Timer);

impl Default for StopwatchTimer {
    fn default() -> Self {
        let timer = Timer::new(DEFAULT_DURATION, TimerMode::Once);
        Self(timer)
    }
}

#[allow(dead_code)]
impl StopwatchTimer {
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
    pub fn reset_duration(&mut self) {
        self.0.set_duration(DEFAULT_DURATION);
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Stopwatch;

// this *probably* triggers before `OnAdd, SceneInstanceReady`
//
// TODO: we should add this to the glb directly with skein
fn on_stopwatch_spawn(trigger: Trigger<OnAdd, Stopwatch>, mut commands: Commands) {
    // this will then trigger the animation plugin
    //
    // which in turn triggers the `setup_stopwatch_animation` system in this
    // module's animation plugin
    commands
        .entity(trigger.target())
        .observe(animation::setup_stopwatch_animation)
        .insert(AnimationPlayerAncestor);
}

fn tick_stopwatch(mut stopwatch: Query<&mut StopwatchTimer>, time: Res<Time>) {
    let Ok(mut stopwatch) = stopwatch.single_mut() else {
        return;
    };
    stopwatch.0.tick(time.delta());
}
fn reset_on_new_level(_trigger: Trigger<NewRoom>, mut stopwatch: Query<&mut StopwatchTimer>) {
    let Ok(mut stopwatch) = stopwatch.single_mut() else {
        return;
    };
    stopwatch.pause();
    stopwatch.reset_duration();
}

fn start_timer_on_level(_trigger: Trigger<RoomStarted>, mut stopwatch: Query<&mut StopwatchTimer>) {
    let Ok(mut stopwatch) = stopwatch.single_mut() else {
        return;
    };
    info!("Starting stopwatch");
    stopwatch.unpause();
}

fn out_of_time(mut stopwatch: Query<&mut StopwatchTimer>, mut lives: Query<&mut Lives>) {
    let Ok(mut stopwatch) = stopwatch.single_mut() else {
        return;
    };
    if !stopwatch.0.finished() {
        return;
    }
    let mut lives = lives.single_mut().expect("Something should be living");
    lives.remove_life();
    stopwatch.reset_duration();
}
