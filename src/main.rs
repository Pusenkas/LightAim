mod camera;
mod geometry;
mod stats;
mod systems;
mod target_types;
mod task_1;
mod task_2;

use crate::systems::*;
use crate::task_1::TaskOnePlugin;
use bevy::color::palettes::css::BLUE;
use bevy::color::palettes::css::GREEN;
use bevy::color::palettes::css::RED;
use bevy::prelude::*;
use bevy::window::PresentMode;
use camera::CameraPlugin;
use iyes_perf_ui::prelude::*;
use target_types::{init_capsule_assets, init_sphere_assets};
use task_2::TaskTwoPLugin;

const OW_SENS: f32 = 0.0198;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        .init_state::<TaskStates>()
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(PerfUiPlugin)
        .add_systems(Startup, spawn_fps_overlay)
        .add_plugins(CameraPlugin)
        .add_systems(Startup, (init_capsule_assets, init_sphere_assets))
        .add_plugins(TaskOnePlugin)
        .add_plugins(TaskTwoPLugin)
        .add_systems(Update, start.run_if(in_state(TaskStates::Start)))
        .add_systems(Update, switch_main_state)
        .add_systems(Update, draw_axes)
        .add_systems(Update, (exit_app, set_fullsrenn))
        .run();
}

#[derive(Component)]
pub struct AxisX;

#[derive(Component)]
pub struct AxisY;

#[derive(Component)]
pub struct AxisZ;

#[derive(Component)]
pub struct Ray;

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum TaskStates {
    #[default]
    Start,
    One,
    Two,
}

fn start(
    mut next_task_state: ResMut<NextState<TaskStates>>,
    //mut next_task_one_state: ResMut<NextState<TaskOneStates>>,
) {
    next_task_state.set(TaskStates::One);
    //next_task_one_state.set(TaskOneStates::Prepare);
    println!("Switched state")
}

fn draw_axes(mut gizoms: Gizmos) {
    gizoms.arrow(Vec3::ZERO, Vec3::X * 10.0, RED);
    gizoms.arrow(Vec3::ZERO, Vec3::Y * 10.0, GREEN);
    gizoms.arrow(Vec3::ZERO, Vec3::Z * 10.0, BLUE);
}

fn spawn_fps_overlay(mut commands: Commands) {
    commands.spawn((
        PerfUiRoot {
            display_labels: false,
            layout_horizontal: true,
            ..default()
        },
        PerfUiEntryFPSWorst::default(),
        PerfUiEntryFPS::default(),
    ));

    // plane
    /*commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(20., 20.)),
            material: materials.add(Color::rgb(0.5, 0.5, 0.5)),
            ..default()
        },
        Geometry,
    ));*/

    // light
}

pub fn switch_main_state(
    keys: Res<ButtonInput<KeyCode>>,
    task_state: Res<State<TaskStates>>,
    mut next_task_state: ResMut<NextState<TaskStates>>,
) {
    if keys.just_pressed(KeyCode::Digit1) {
        if *task_state.get() == TaskStates::One {
            return;
        }
        next_task_state.set(TaskStates::One);
        // next_task_one_state.set(TaskOneStates::Prepare);
    } else if keys.just_pressed(KeyCode::Digit2) {
        if *task_state.get() == TaskStates::Two {
            return;
        }
        next_task_state.set(TaskStates::Two);
    }
}
