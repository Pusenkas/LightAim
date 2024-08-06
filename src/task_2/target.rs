use std::f32::consts::PI;

use super::PlayTimer;
use crate::target_types::{collision_ray_capsule, CapsuleAsset};
use crate::stats::Stats;
use bevy::prelude::*;
use rand::distributions::{Bernoulli, Distribution};
use rand::thread_rng;

const TARGET_DISTANCE: f32 = 15.0;

enum StrafeDirection {
    Right,
    Left,
}

#[derive(Component)]
pub struct TaskTwoTarget {
    strafe_direction: StrafeDirection,
    phi: f32, // from 0 to 1
}

pub fn spawn_target(mut commands: Commands, target_asset: Res<CapsuleAsset>, target_query: Query<(), With<TaskTwoTarget>>) {
    if !target_query.is_empty() {
        return;
    }
    commands.spawn((
        PbrBundle {
            mesh: target_asset.mesh.clone_weak(),
            material: target_asset.material.clone_weak(),
            transform: Transform::from_xyz(0.0, 10.0, -TARGET_DISTANCE),
            ..default()
        },
        TaskTwoTarget {
            strafe_direction: StrafeDirection::Right,
            phi: 0.0,
        },
    ));
    // println!("Spawned");
}

pub fn despawn_targets(mut commands: Commands, target_query: Query<Entity, With<TaskTwoTarget>>) {
    for target_entity in target_query.iter() {
        commands.entity(target_entity).despawn();
    }
    // println!("Despawned");
}

pub fn hold_hit_target(
    camera_query: Query<&Transform, With<Camera>>,
    target_queue: Query<&Transform, With<TaskTwoTarget>>,
    mut stats: ResMut<Stats>,
    keys: Res<ButtonInput<MouseButton>>,
) {
    stats.add_possible();
    if keys.pressed(MouseButton::Left) {
        let camera_transform = camera_query.single();
        let origin = camera_transform.translation;
        let direction = camera_transform.rotation.mul_vec3(Vec3::NEG_Z);

        let mut hit = false;
        for target_pos in target_queue.iter() {
            if collision_ray_capsule(
                origin,
                direction,
                target_pos.translation,
            ) {
                hit = true;
            }
        }
        stats.update(hit);
    }
}

pub fn movement_target(
    mut target_query: Query<(&mut Transform, &mut TaskTwoTarget), With<TaskTwoTarget>>,
    time: Res<Time>,
    play_timer: Res<PlayTimer>,
) {
    let (mut transform, mut target) = target_query.single_mut();
    let dir = match target.strafe_direction {
        StrafeDirection::Right => 1.0,
        StrafeDirection::Left => -1.0,
    };
    let speed_coef = 10.0;
    let change = Vec3::new(
        dir * TARGET_DISTANCE * speed_coef * PI * (target.phi * speed_coef * PI).cos() * time.delta_seconds()
            / play_timer.timer.duration().as_secs_f32(),
        0.0,
        dir * TARGET_DISTANCE * speed_coef * PI * (target.phi * speed_coef * PI).sin() * time.delta_seconds()
            / play_timer.timer.duration().as_secs_f32(),
    );
    target.phi += dir * time.delta_seconds() / play_timer.timer.duration().as_secs_f32();
    transform.translation += change;
}

pub fn change_target_strafe(mut target_query: Query<&mut TaskTwoTarget, With<TaskTwoTarget>>) {
    let bernoulli = Bernoulli::new(0.5).unwrap();
    if bernoulli.sample(&mut thread_rng()) {
        let mut target = target_query.single_mut();
        match target.strafe_direction {
            StrafeDirection::Right => target.strafe_direction = StrafeDirection::Left,
            StrafeDirection::Left => target.strafe_direction = StrafeDirection::Right,
        }
    }
}
