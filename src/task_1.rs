mod resources;
mod target;
mod timers;
mod ui;

use crate::camera::{position_camera, spawn_camera};
use crate::geometry::{despawn_geometry, spawn_geometry};
use crate::stats::Stats;
use crate::TaskStates;

use bevy::prelude::*;
use resources::Positions;
use target::*;
use timers::*;
use ui::*;

const TARGET_AMOUNT_TASK_ONE: usize = 3;
const GRID_SIZE: i32 = 4;

pub struct TaskOnePlugin;

#[derive(SubStates, Debug, Hash, PartialEq, Eq, Clone, Copy, Default)]
#[source(TaskStates = TaskStates::One)]
pub enum TaskOneStates {
    #[default]
    Prepare,
    Game,
    Finish,
}

#[derive(SubStates, Debug, Hash, PartialEq, Eq, Clone, Copy, Default)]
#[source(TaskOneStates = TaskOneStates::Game)]
pub enum TaskOneGameStates {
    #[default]
    Wait,
    Play,
}

#[derive(Component, Default)]
pub struct TaskOneGeometry;

impl Plugin for TaskOnePlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<TaskOneStates>()
            .add_sub_state::<TaskOneGameStates>()
            // State: One
            .add_systems(
                OnEnter(TaskStates::One),
                (
                    spawn_geometry::<TaskOneGeometry>,
                    position_camera.after(spawn_camera),
                    init_positions,
                    spawn_targets.after(init_positions),
                ),
            )
            .add_systems(Update, restart.run_if(in_state(TaskStates::One)))
            .add_systems(
                OnExit(TaskStates::One),
                (
                    despawn_geometry::<TaskOneGeometry>,
                    despawn_stats_ui,
                    despawn_targets,
                ),
            )
            // State: Prepare
            .add_systems(Update, begin.run_if(in_state(TaskOneStates::Prepare)))
            // State: Game
            .add_systems(
                OnEnter(TaskOneStates::Game),
                (init_score, spawn_play_timer, spawn_stats_ui).chain(),
            )
            .add_systems(OnExit(TaskOneStates::Game), despawn_stats_ui)
            // State: Game::Wait
            .add_systems(
                OnEnter(TaskOneGameStates::Wait),
                (spawn_wait_timer, spawn_wait_ui).chain(),
            )
            .add_systems(
                Update,
                (update_wait_timer, update_wait_ui).run_if(in_state(TaskOneGameStates::Wait)),
            )
            .add_systems(OnExit(TaskOneGameStates::Wait), despawn_wait_ui)
            // State: Game::Play
            .add_systems(
                Update,
                (
                    press_hit_target,
                    dynamic_spawn_targets,
                    update_stats_ui,
                    update_play_timer,
                )
                    .run_if(in_state(TaskOneGameStates::Play)),
            )
            // State: Finish
            .add_systems(OnEnter(TaskOneStates::Finish), spawn_final_score_ui)
            .add_systems(OnExit(TaskOneStates::Finish), despawn_final_score_ui);
    }
}

fn begin(keys: Res<ButtonInput<MouseButton>>, mut next_state: ResMut<NextState<TaskOneStates>>) {
    if keys.just_pressed(MouseButton::Left) {
        next_state.set(TaskOneStates::Game);
    }
}

fn init_score(mut commands: Commands) {
    commands.insert_resource(Stats::default());
}

fn init_positions(mut commands: Commands) {
    commands.insert_resource(Positions::default());
}

fn restart(keys: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<TaskOneStates>>) {
    if keys.just_pressed(KeyCode::KeyR) {
        // next_state.set(TaskOneStates::Finish);
        next_state.set(TaskOneStates::Prepare);
    }
}
