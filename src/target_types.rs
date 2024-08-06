use bevy::{color::palettes::css::ORANGE_RED, prelude::*};

pub const CAPSULE_RADIUS: f32 = 0.5;
pub const CAPSULE_LENGTH: f32 = 1.0;
pub const SPHERE_RADIUS: f32 = 0.8;

#[derive(Resource, Default)]
pub struct CapsuleAsset {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

pub fn init_capsule_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(CapsuleAsset {
        mesh: meshes.add(Capsule3d::new(CAPSULE_RADIUS, CAPSULE_LENGTH)),
        material: materials.add(StandardMaterial {
            base_color: ORANGE_RED.into(),
            metallic: 0.8,
            ..default()
        }),
    });
}

#[derive(Resource, Default)]
pub struct SphereAsset {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

pub fn init_sphere_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(SphereAsset {
        mesh: meshes.add(Sphere::new(SPHERE_RADIUS).mesh()),
        material: materials.add(StandardMaterial {
            base_color: ORANGE_RED.into(),
            metallic: 0.8,
            ..default()
        }),
    });
}

pub fn collision_ray_sphere(origin: Vec3, direction: Vec3, center: Vec3) -> bool {
    let radius = SPHERE_RADIUS;
    distance_squared_ray_point(origin, direction, center) < radius.powi(2)
}

pub fn collision_ray_capsule(origin: Vec3, direction: Vec3, center: Vec3) -> bool {
    let radius = CAPSULE_RADIUS;
    let length = CAPSULE_LENGTH;

    let point_a = center + Vec3::Y * (length - radius);
    if distance_squared_ray_point(origin, direction, point_a) < radius.powi(2) {
        return true;
    }
    let point_b = center - Vec3::Y * (length - radius);
    if distance_squared_ray_point(origin, direction, point_b) < radius.powi(2) {
        return true;
    }

    let direction_capsule = point_a - point_b;
    let n = direction.cross(direction_capsule).normalize();
    let d = n.dot(point_b - origin);
    if d.abs() < radius {
        let t = (n.cross(direction)).dot(origin - point_b);
        if t > 0.0 && t < 1.0 {
            return true;
        }
    }
    false
}

fn distance_squared_ray_point(origin: Vec3, direction: Vec3, point: Vec3) -> f32 {
    (point - origin).cross(direction).length_squared() / direction.length_squared()
}
