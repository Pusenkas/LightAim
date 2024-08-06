use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowMode};

pub fn set_fullsrenn(
    keys: Res<ButtonInput<KeyCode>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    if keys.just_pressed(KeyCode::KeyF) {
        let window_mode = &mut windows.single_mut().mode;
        match window_mode {
            WindowMode::Fullscreen => *window_mode = WindowMode::Windowed,
            _ => *window_mode = WindowMode::Fullscreen,
        }
    }
}

pub fn exit_app(keys: Res<ButtonInput<KeyCode>>, mut app_exit_events: ResMut<Events<AppExit>>) {
    if keys.just_pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit::Success);
    }
}
