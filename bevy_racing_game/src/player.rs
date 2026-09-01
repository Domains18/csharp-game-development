use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
           .add_systems(Update, (player_input, apply_physics, update_wheels));
    }
}

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub max_speed: f32,
    pub acceleration: f32,
    pub turn_speed: f32,
    pub velocity: Vec3,
    pub on_ground: bool,
}

#[derive(Component)]
struct Wheel;

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Buggy body
    let body_mesh = meshes.add(Cuboid::new(1.4, 0.6, 2.2));
    let body_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.9, 0.3, 0.1), // Red/orange buggy
        metallic: 0.3,
        roughness: 0.4,
        ..default()
    });

    // Roll cage
    let cage_mesh = meshes.add(Cuboid::new(1.2, 0.5, 1.0));
    let cage_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.2, 0.2),
        metallic: 0.8,
        roughness: 0.2,
        ..default()
    });

    let wheel_mesh = meshes.add(Cylinder::new(0.3, 0.25));
    let wheel_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.1, 0.1, 0.1),
        roughness: 0.9,
        ..default()
    });

    commands.spawn((
        PbrBundle {
            mesh: body_mesh,
            material: body_material,
            transform: Transform::from_xyz(0.0, 0.8, 0.0),
            ..default()
        },
        Player {
            speed: 0.0,
            max_speed: 25.0,
            acceleration: 15.0,
            turn_speed: 2.5,
            velocity: Vec3::ZERO,
            on_ground: true,
        },
    ))
    .with_children(|parent| {
        // Roll cage
        parent.spawn(PbrBundle {
            mesh: cage_mesh,
            material: cage_material,
            transform: Transform::from_xyz(0.0, 0.55, -0.2),
            ..default()
        });

        // Wheels
        let wheel_positions = [
            (-0.8, 0.3, 0.7),   // Front left
            (0.8, 0.3, 0.7),    // Front right
            (-0.8, 0.3, -0.7),  // Rear left
            (0.8, 0.3, -0.7),   // Rear right
        ];

        for pos in wheel_positions {
            parent.spawn((
                PbrBundle {
                    mesh: wheel_mesh.clone(),
                    material: wheel_material.clone(),
                    transform: Transform::from_xyz(pos.0, pos.1, pos.2)
                        .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
                    ..default()
                },
                Wheel,
            ));
        }
    });
}

fn player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Player>,
    time: Res<Time>,
) {
    let Ok(mut player) = player_query.get_single_mut() else { return };

    let dt = time.delta_secs();
    let mut accel_input = 0.0;
    let mut turn_input = 0.0;

    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
        accel_input += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        accel_input -= 0.6;
    }
    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        turn_input += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        turn_input -= 1.0;
    }

    // Apply acceleration
    player.speed += accel_input * player.acceleration * dt;
    
    // Friction/drag
    player.speed *= 0.98;
    
    // Clamp speed
    player.speed = player.speed.clamp(-player.max_speed * 0.5, player.max_speed);

    // Turning (only when moving)
    if player.speed.abs() > 0.5 {
        let turn_factor = (player.speed / player.max_speed).clamp(0.3, 1.0);
        let rotation = turn_input * player.turn_speed * turn_factor * dt;
        
        // Rotate velocity direction
        let forward = Vec3::Z;
        let right = Vec3::X;
        let current_dir = forward * player.speed.signum();
        
        // Simple arcade steering: rotate the velocity vector
        let rotation_quat = Quat::from_rotation_y(rotation * player.speed.signum());
        player.velocity = rotation_quat * player.velocity;
    }

    // Calculate forward velocity
    let forward = Vec3::Z;
    player.velocity = forward * player.speed;
}

fn apply_physics(
    mut player_query: Query<(&mut Transform, &mut Player)>,
    time: Res<Time>,
) {
    let Ok((mut transform, mut player)) = player_query.get_single_mut() else { return };

    let dt = time.delta_secs();

    // Apply gravity if not on ground (simple)
    if !player.on_ground {
        player.velocity.y -= 9.8 * dt;
    } else {
        player.velocity.y = 0.0;
    }

    // Ground clamp (simple terrain following)
    let ground_height = get_ground_height(transform.translation.x, transform.translation.z);
    if transform.translation.y <= ground_height + 0.5 {
        transform.translation.y = ground_height + 0.5;
        player.on_ground = true;
    } else {
        player.on_ground = false;
    }

    // Move
    let movement = player.velocity * dt;
    transform.translation += movement;

    // Rotate body to face movement direction
    if player.speed.abs() > 0.1 {
        let target_rotation = Quat::from_rotation_y(
            f32::atan2(player.velocity.x, player.velocity.z)
        );
        transform.rotation = transform.rotation.slerp(target_rotation, 5.0 * dt);
    }
}

fn update_wheels(
    player_query: Query<&Player>,
    mut wheels: Query<&mut Transform, With<Wheel>>,
    time: Res<Time>,
) {
    let Ok(player) = player_query.get_single() else { return };
    
    for mut wheel_transform in &mut wheels {
        // Spin wheels based on speed
        let spin = player.speed * time.delta_secs() * 3.0;
        wheel_transform.rotate_x(spin);
    }
}

// Simple terrain height function
fn get_ground_height(x: f32, z: f32) -> f32 {
    let noise = (x * 0.1).sin() * (z * 0.1).cos() * 0.5;
    noise + 0.3 * ((x * 0.05).sin() + (z * 0.05).cos())
}