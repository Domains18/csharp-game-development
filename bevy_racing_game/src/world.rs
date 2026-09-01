use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_terrain, spawn_palm_trees, spawn_rocks));
    }
}

fn spawn_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Sand ground
    let sand_mesh = meshes.add(Plane3d::new(Vec3::Y, Vec2::new(100.0, 100.0)));
    let sand_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.93, 0.87, 0.65),
        perceptual_roughness: 1.0,
        metallic: 0.0,
        ..default()
    });

    commands.spawn((
        Mesh3d(sand_mesh),
        MeshMaterial3d(sand_material),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Water plane
    let water_mesh = meshes.add(Plane3d::new(Vec3::Y, Vec2::new(200.0, 200.0)));
    let water_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.0, 0.5, 0.8, 0.7),
        alpha_mode: AlphaMode::Blend,
        metallic: 0.1,
        perceptual_roughness: 0.1,
        ..default()
    });

    commands.spawn((
        Mesh3d(water_mesh),
        MeshMaterial3d(water_material),
        Transform::from_xyz(0.0, -0.5, 0.0),
    ));
}

fn spawn_palm_trees(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let trunk_mesh = meshes.add(Cylinder::new(0.15, 2.5));
    let trunk_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.55, 0.35, 0.15),
        perceptual_roughness: 0.9,
        ..default()
    });

    let leaf_mesh = meshes.add(Cone::new(1.2, 0.8));
    let leaf_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.7, 0.2),
        perceptual_roughness: 0.8,
        ..default()
    });

    let positions = [
        (8.0, 5.0), (-10.0, 8.0), (15.0, -12.0),
        (-8.0, -15.0), (20.0, 20.0), (-18.0, 5.0),
        (5.0, -20.0), (-15.0, -8.0), (25.0, 0.0),
        (0.0, 30.0), (-25.0, 15.0), (12.0, 25.0),
    ];

    for (x, z) in positions {
        let y = crate::player::get_ground_height(x, z);

        commands.spawn((
            Mesh3d(trunk_mesh.clone()),
            MeshMaterial3d(trunk_material.clone()),
            Transform::from_xyz(x, y + 1.25, z),
        ));

        // Leaves
        for i in 0..4 {
            let angle = (i as f32) * std::f32::consts::FRAC_PI_2;
            let offset = Vec3::new(angle.cos() * 0.5, 0.0, angle.sin() * 0.5);
            
            commands.spawn((
                Mesh3d(leaf_mesh.clone()),
                MeshMaterial3d(leaf_material.clone()),
                Transform::from_xyz(x + offset.x, y + 2.5, z + offset.z)
                    .with_rotation(Quat::from_rotation_x(0.3)),
            ));
        }
    }
}

fn spawn_rocks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let rock_mesh = meshes.add(Sphere::new(0.8));
    let rock_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.5, 0.5, 0.5),
        perceptual_roughness: 0.95,
        ..default()
    });

    let positions = [
        (5.0, 3.0), (-7.0, -4.0), (12.0, 8.0),
        (-3.0, 12.0), (18.0, -5.0), (-12.0, -10.0),
    ];

    for (x, z) in positions {
        let y = crate::player::get_ground_height(x, z);
        
        commands.spawn((
            Mesh3d(rock_mesh.clone()),
            MeshMaterial3d(rock_material.clone()),
            Transform::from_xyz(x, y + 0.4, z)
                .with_scale(Vec3::new(1.0 + ((x as i32 % 3) as f32 * 0.3), 0.8, 1.0)),
        ));
    }
}
