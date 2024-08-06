use bevy::prelude::*;

use super::TaskOneGameStates;
use super::TaskOneStates;

#[derive(Resource)]
pub struct WaitTimer {
    pub timer: Timer,
}

#[derive(Resource)]
pub struct PlayTimer {
    pub timer: Timer,
}

pub fn spawn_wait_timer(mut commands: Commands) {
    commands.insert_resource(WaitTimer {
        timer: Timer::from_seconds(3.0, TimerMode::Once),
    });
}

pub fn spawn_play_timer(mut commands: Commands) {
    commands.insert_resource(PlayTimer {
        timer: Timer::from_seconds(30.0, TimerMode::Once),
    });
}

pub fn update_wait_timer(
    mut timer: ResMut<WaitTimer>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<TaskOneGameStates>>,
) {
    timer.timer.tick(time.delta());

    if timer.timer.finished() {
        next_state.set(TaskOneGameStates::Play);
    }
}

pub fn update_play_timer(
    mut timer: ResMut<PlayTimer>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<TaskOneStates>>,
) {
    timer.timer.tick(time.delta());

    if timer.timer.finished() {
        next_state.set(TaskOneStates::Finish);
    }
}
