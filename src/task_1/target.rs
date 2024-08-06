use super::resources::Positions;
use super::TARGET_AMOUNT_TASK_ONE;
use crate::target_types::{collision_ray_sphere, SphereAsset};
use crate::stats::Stats;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct TaskOneTarget;

pub fn spawn_targets(
    mut commands: Commands,
    target_asset: Res<SphereAsset>,
    mut positions: ResMut<Positions>,
    target_queue: Query<(), With<TaskOneTarget>>,
) {
    for _ in 0..(TARGET_AMOUNT_TASK_ONE - target_queue.iter().len()) {
        let pos = positions.inf_pop();
        commands.spawn((
            PbrBundle {
                mesh: target_asset.mesh.clone_weak(),
                material: target_asset.material.clone_weak(),
                transform: Transform::from_xyz(pos.0, pos.1, pos.2),
                ..default()
            },
            TaskOneTarget,
        ));
    }
}

pub fn despawn_targets(mut commands: Commands, target_query: Query<Entity, With<TaskOneTarget>>) {
    for target_entity in target_query.iter() {
        commands.entity(target_entity).despawn();
    }
}

pub fn dynamic_spawn_targets(
    mut commands: Commands,
    target_queue: Query<(), With<TaskOneTarget>>,
    mut positions: ResMut<Positions>,
    target_asset: Res<SphereAsset>,
) {
    let shortage = TARGET_AMOUNT_TASK_ONE - target_queue.iter().len();
    if shortage <= 0 {
        return;
    }
    for _ in 0..shortage {
        let pos = positions.inf_pop();
        commands.spawn((
            PbrBundle {
                mesh: target_asset.mesh.clone_weak(),
                material: target_asset.material.clone_weak(),
                transform: Transform::from_xyz(pos.0, pos.1, pos.2),
                ..default()
            },
            TaskOneTarget,
        ));
    }
}

pub fn press_hit_target(
    mut commands: Commands,
    camera_query: Query<&Transform, With<Camera>>,
    target_queue: Query<(Entity, &Transform), With<TaskOneTarget>>,
    mut stats: ResMut<Stats>,
    keys: Res<ButtonInput<MouseButton>>,
) {
    if keys.just_pressed(MouseButton::Left) {
        let camera_transform = camera_query.single();
        let origin = camera_transform.translation;
        let direction = camera_transform.rotation.mul_vec3(Vec3::NEG_Z);

        let mut hit = false;
        for (target_entity, target_pos) in target_queue.iter() {
            if collision_ray_sphere(
                origin,
                direction,
                target_pos.translation,
            ) {
                hit = true;
                commands.entity(target_entity).despawn();
            }
        }
        stats.update(hit);
    }
}
