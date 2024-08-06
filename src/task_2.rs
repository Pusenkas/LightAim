mod target;
mod timers;
mod ui;

use std::time::Duration;

use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;

use crate::camera::{position_camera, spawn_camera};
use crate::geometry::{despawn_geometry, spawn_geometry};
use crate::stats::Stats;
use crate::TaskStates;
use target::*;
use timers::*;
use ui::*;

#[derive(SubStates, Debug, Hash, PartialEq, Eq, Clone, Copy, Default)]
#[source(TaskStates = TaskStates::Two)]
pub enum TaskTwoStates {
    #[default]
    Prepare,
    Game,
    Finish,
}

#[derive(SubStates, Debug, Hash, PartialEq, Eq, Clone, Copy, Default)]
#[source(TaskTwoStates = TaskTwoStates::Game)]
pub enum TaskTwoGameStates {
    #[default]
    Wait,
    Play,
}

// #[derive(Component)]
// struct TaskTwoTarget;

#[derive(Component, Default)]
struct TaskTwoGeometry;

pub struct TaskTwoPLugin;

impl Plugin for TaskTwoPLugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<TaskTwoStates>()
            .add_sub_state::<TaskTwoGameStates>()
            // State: Two
            .add_systems(
                OnEnter(TaskStates::Two),
                (
                    spawn_geometry::<TaskTwoGeometry>,
                    position_camera.after(spawn_camera),
                    init_score,
                    spawn_target,
                ),
            )
            .add_systems(Update, restart.run_if(in_state(TaskStates::Two)))
            .add_systems(
                OnExit(TaskStates::Two),
                (despawn_geometry::<TaskTwoGeometry>, despawn_targets),
            )
            // State: Prepare
            .add_systems(Update, begin.run_if(in_state(TaskTwoStates::Prepare)))
            // State: Game
            .add_systems(
                OnEnter(TaskTwoStates::Game),
                (spawn_play_timer, spawn_stats_ui).chain(),
            )
            .add_systems(OnExit(TaskTwoStates::Game), despawn_stats_ui)
            // State: Game::Wait
            .add_systems(
                OnEnter(TaskTwoGameStates::Wait),
                (spawn_wait_timer, spawn_wait_ui).chain(),
            )
            .add_systems(
                Update,
                (update_wait_timer, update_wait_ui).run_if(in_state(TaskTwoGameStates::Wait)),
            )
            .add_systems(OnExit(TaskTwoGameStates::Wait), despawn_wait_ui)
            // State: Game::Play
            .add_systems(
                Update,
                (
                    hold_hit_target,
                    update_stats_ui,
                    update_play_timer,
                    movement_target,
                    change_target_strafe.run_if(on_timer(Duration::from_secs(1))),
                )
                    .run_if(in_state(TaskTwoGameStates::Play)),
            )
            // State: Finish
            .add_systems(OnEnter(TaskTwoStates::Finish), spawn_final_score_ui)
            .add_systems(OnExit(TaskTwoStates::Finish), despawn_final_score_ui);
        //.add_systems(OnExit(TaskStates::One), despawn_geometry)
        //.add_systems(Update, get_pos);
    }
}

fn init_score(mut commands: Commands) {
    commands.insert_resource(Stats::default());
}

fn begin(keys: Res<ButtonInput<MouseButton>>, mut next_state: ResMut<NextState<TaskTwoStates>>) {
    if keys.just_pressed(MouseButton::Left) {
        next_state.set(TaskTwoStates::Game);
    }
}

fn restart(keys: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<TaskTwoStates>>) {
    if keys.just_pressed(KeyCode::KeyR) {
        // next_state.set(TaskOneStates::Finish);
        next_state.set(TaskTwoStates::Prepare);
    }
}
