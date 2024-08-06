use bevy::prelude::*;

pub fn spawn_geometry<T: Component + Default>(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(100., 100.)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.5, 0.5, 0.5),
                // metallic: 0.0,
                reflectance: 0.1,
                ..default()
            }),
            ..default()
        },
        T::default(),
    ));

    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                color: Color::WHITE,
                illuminance: 5000.0,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(50.0, 50.0, 50.0))
                .looking_at(Vec3::new(0.0, 10.0, -10.0), Vec3::Y),
            ..default()
        },
        T::default(),
    ));
}

pub fn despawn_geometry<T: Component>(
    mut commands: Commands,
    geometry_query: Query<Entity, With<T>>,
) {
    for geometry_entity in geometry_query.iter() {
        commands.entity(geometry_entity).despawn();
    }
}
