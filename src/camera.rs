use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::OW_SENS;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_camera, hide_cursor, setup_crosshair_ui))
            .add_systems(Update, camera_movement);
    }
}

pub fn position_camera(mut camera_query: Query<&mut Transform, With<Camera>>) {
    let transform = &mut camera_query.single_mut();
    let transform_new =
        Transform::from_xyz(0.0, 10.0, 0.0).looking_at(Vec3::new(0.0, 10.0, -10.0), Vec3::Y);
    transform.translation = transform_new.translation;
}

pub fn spawn_camera(mut commands: Commands) {
    // camera
    commands.spawn(Camera3dBundle {
        projection: Projection::Perspective(PerspectiveProjection {
            fov: (71.0_f32).to_radians(),
            ..default()
        }),
        ..default()
    });
}

fn hide_cursor(mut windows: Query<&mut Window, With<PrimaryWindow>>) {
    windows.single_mut().cursor.visible = false;
}

pub fn camera_movement(
    mut mouse_movement_events: EventReader<MouseMotion>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    let transform = &mut camera_query.single_mut();

    let mut delta = Vec2::ZERO;
    for mouse_motion in mouse_movement_events.read() {
        delta += mouse_motion.delta;
    }
    // println!("Mouse moution {:?}", delta);
    let delta_x = (OW_SENS * delta.x).to_radians();
    let delta_y = (OW_SENS * delta.y).to_radians();
    transform.rotation =
        Quat::from_rotation_y(-delta_x) * transform.rotation * Quat::from_rotation_x(-delta_y);
    // transform.rotation *=  Quat::from_rotation_y(delta_x) * Quat::from_rotation_x(delta_y); WHY NOT WORKING???
}

fn setup_crosshair_ui(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: UiImage {
                    texture: asset_server.load("crosshair.png"),
                    ..default()
                },
                style: Style {
                    width: Val::Px(12.),
                    height: Val::Px(12.),
                    ..default()
                },
                ..default()
            });
        });
}
