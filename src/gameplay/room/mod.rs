use std::time::Duration;

use bevy::prelude::*;

use crate::gameplay::player::rewind::MovementLog;

use super::GameSet;

pub fn plugin(app: &mut App) {
    app.add_observer(on_new_room);
    app.add_systems(
        Update,
        (update_room_countdown, remove_room_countdown)
            .chain()
            .in_set(GameSet::TickTimers)
            .run_if(resource_exists::<RoomCountdown>),
    );
}

#[derive(Event)]
pub struct StartCountdown(pub u64);

/// Sends a player to a new room with a respawn point.
///
/// DOES NOT start a countdown intentionally.
#[derive(Event)]
pub struct NewRoom {
    /// This is the spawn point of the room
    pub spawn_point: Vec3,
}

#[derive(Resource)]
pub struct RoomCountdown {
    pub secs_left: u8,
    timer: Timer,
}

impl RoomCountdown {
    pub fn complete(&self) -> bool {
        self.secs_left == 0
    }
}

impl Default for RoomCountdown {
    fn default() -> Self {
        Self {
            secs_left: 3,
            timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
        }
    }
}

fn on_new_room(_trigger: Trigger<StartCountdown>, mut commands: Commands) {
    commands.init_resource::<RoomCountdown>();
    info!("Level Countdown: 3");
}

//fk it it'll also do other things
fn update_room_countdown(mut countdown: ResMut<RoomCountdown>, time: Res<Time>) {
    countdown.timer.tick(time.delta());
    if countdown.timer.just_finished() {
        countdown.secs_left = countdown.secs_left.saturating_sub(1);
        info!("Level Countdown: {}", countdown.secs_left);
    }
}

#[derive(Event)]
pub struct RoomStarted;

fn remove_room_countdown(mut commands: Commands, countdown: Res<RoomCountdown>) {
    if countdown.complete() {
        info!("countdown complete");
        commands.remove_resource::<RoomCountdown>();
        commands.trigger(RoomStarted);
    }
}
