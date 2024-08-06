use super::timers::PlayTimer;
use super::timers::WaitTimer;
use super::Stats;
use bevy::prelude::*;
use bevy::text::BreakLineOn;

#[derive(Component)]
pub struct ShotUI;

#[derive(Component)]
pub struct HitUI;

#[derive(Component)]
pub struct DurationUI;

#[derive(Component)]
pub struct AccUI;

#[derive(Component)]
pub struct StatsUI;

#[derive(Component)]
pub struct FinalScoreUI;

#[derive(Component)]
pub struct WaitUI;

#[derive(Component)]
pub struct TimerUI;

pub fn spawn_stats_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    play_timer: Res<PlayTimer>,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    column_gap: Val::Px(256.0),
                    ..default()
                },
                ..default()
            },
            StatsUI,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            "Shot: 0",
                            TextStyle {
                                font: asset_server.load("fonts\\FiraSans-Bold.ttf"),
                                font_size: 64.0,
                                color: Color::WHITE,
                            },
                        )],
                        justify: JustifyText::Center,
                        linebreak_behavior: BreakLineOn::NoWrap,
                    },
                    ..default()
                },
                ShotUI,
            ));

            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            format!(
                                "Time: {}",
                                time_from_secs(play_timer.timer.remaining().as_secs())
                            ),
                            TextStyle {
                                font: asset_server.load("fonts\\FiraSans-Bold.ttf"),
                                font_size: 64.0,
                                color: Color::WHITE,
                            },
                        )],
                        justify: JustifyText::Center,
                        linebreak_behavior: BreakLineOn::NoWrap,
                    },
                    ..default()
                },
                DurationUI,
            ));

            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            "Acc: 100%",
                            TextStyle {
                                font: asset_server.load("fonts\\FiraSans-Bold.ttf"),
                                font_size: 64.0,
                                color: Color::WHITE,
                            },
                        )],
                        justify: JustifyText::Center,
                        linebreak_behavior: BreakLineOn::NoWrap,
                    },
                    ..default()
                },
                AccUI,
            ));

            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            "Hit: 0",
                            TextStyle {
                                font: asset_server.load("fonts\\FiraSans-Bold.ttf"),
                                font_size: 64.0,
                                color: Color::WHITE,
                            },
                        )],
                        justify: JustifyText::Center,
                        linebreak_behavior: BreakLineOn::NoWrap,
                    },
                    ..default()
                },
                HitUI,
            ));
        });
}

pub fn update_stats_ui(
    mut hit_ui: Query<
        &mut Text,
        (
            With<HitUI>,
            Without<DurationUI>,
            Without<ShotUI>,
            Without<AccUI>,
        ),
    >,
    mut shot_ui: Query<
        &mut Text,
        (
            With<ShotUI>,
            Without<DurationUI>,
            Without<HitUI>,
            Without<AccUI>,
        ),
    >,
    mut acc_ui: Query<
        &mut Text,
        (
            With<AccUI>,
            Without<DurationUI>,
            Without<ShotUI>,
            Without<HitUI>,
        ),
    >,
    mut time_ui: Query<
        &mut Text,
        (
            With<DurationUI>,
            Without<HitUI>,
            Without<ShotUI>,
            Without<AccUI>,
        ),
    >,
    play_timer: Res<PlayTimer>,
    stats: Res<Stats>,
) {
    hit_ui.single_mut().sections[0].value = format!("Hit: {}", stats.hit);
    shot_ui.single_mut().sections[0].value = format!("Shot: {}", stats.shot);
    time_ui.single_mut().sections[0].value = format!(
        "Time: {}",
        time_from_secs(play_timer.timer.remaining().as_secs())
    );
    acc_ui.single_mut().sections[0].value = format!("Acc: {}%", stats.get_acc());
}

pub fn despawn_stats_ui(mut commands: Commands, ui_query: Query<Entity, With<StatsUI>>) {
    if let Ok(entity) = ui_query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn spawn_final_score_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    stats: Res<Stats>,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            FinalScoreUI,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        format!(
                            "Final score: {}\nAcc: {}% \nHit percentage: {}%",
                            stats.hit,
                            stats.get_acc(),
                            stats.get_hit_precentage()
                        ),
                        TextStyle {
                            font: asset_server.load("fonts\\FiraSans-Bold.ttf"),
                            font_size: 256.0,
                            color: Color::WHITE,
                        },
                    )],
                    justify: JustifyText::Center,
                    linebreak_behavior: BreakLineOn::NoWrap,
                },
                ..default()
            });
        });
}

pub fn despawn_final_score_ui(mut commands: Commands, ui_query: Query<Entity, With<FinalScoreUI>>) {
    if let Ok(entity) = ui_query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn spawn_wait_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    wait_timer: Res<WaitTimer>,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            WaitUI,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            format!("{}", wait_timer.timer.remaining_secs() as i32),
                            TextStyle {
                                font: asset_server.load("fonts\\FiraSans-Bold.ttf"),
                                font_size: 128.0,
                                color: Color::WHITE,
                            },
                        )],
                        justify: JustifyText::Center,
                        linebreak_behavior: BreakLineOn::NoWrap,
                    },
                    ..default()
                },
                TimerUI,
            ));
        });
}

pub fn update_wait_ui(mut ui_query: Query<&mut Text, With<TimerUI>>, wait_timer: Res<WaitTimer>) {
    ui_query.single_mut().sections[0].value =
        format!("{}", wait_timer.timer.remaining_secs() as i32);
}

pub fn despawn_wait_ui(mut commands: Commands, ui_query: Query<Entity, With<WaitUI>>) {
    if let Ok(entity) = ui_query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}

fn time_from_secs(secs: u64) -> String {
    let res = format!("{}:{}", secs / 60, secs % 60);
    res
}
