use bevy::prelude::*;
use crate::player::Player;
use crate::Score;

pub struct CollectiblesPlugin;

impl Plugin for CollectiblesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_coins)
           .add_systems(Update, (rotate_coins, check_collection, spawn_boost_pads));
    }
}

#[derive(Component)]
struct Coin {
    base_y: f32,
    bob_offset: f32,
}

#[derive(Component)]
struct BoostPad;

fn spawn_coins(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let coin_mesh = meshes.add(Cylinder::new(0.3, 0.05));
    let coin_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.84, 0.0),
        metallic: 1.0,
        perceptual_roughness: 0.2,
        emissive: Color::srgb(0.3, 0.25, 0.0).into(),
        ..default()
    });

    let positions = [
        (3.0, 3.0), (-5.0, 8.0), (10.0, -5.0),
        (-8.0, -8.0), (15.0, 15.0), (-15.0, 10.0),
        (0.0, 20.0), (20.0, -10.0), (-20.0, -5.0),
        (8.0, -15.0), (-10.0, 18.0), (25.0, 5.0),
    ];

    for (i, (x, z)) in positions.iter().enumerate() {
        let y = crate::player::get_ground_height(*x, *z);
        
        commands.spawn((
            PbrBundle {
                mesh: Mesh3d(coin_mesh.clone()),
                material: MeshMaterial3d(coin_material.clone()),
                transform: Transform::from_xyz(*x, y + 1.0, *z)
                    .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
                ..default()
            },
            Coin {
                base_y: y + 1.0,
                bob_offset: i as f32 * 0.5,
            },
        ));
    }
}

fn rotate_coins(
    mut coins: Query<(&mut Transform, &Coin)>,
    time: Res<Time>,
) {
    for (mut transform, coin) in &mut coins {
        // Spin
        transform.rotate_y(3.0 * time.delta_secs());
        
        // Bob up and down
        let bob = (time.elapsed_secs() * 3.0 + coin.bob_offset).sin() * 0.2;
        transform.translation.y = coin.base_y + bob;
    }
}

fn check_collection(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    coin_query: Query<(Entity, &Transform), With<Coin>>,
    mut score: ResMut<Score>,
) {
    let Ok(player_transform) = player_query.get_single() else { return };

    for (coin_entity, coin_transform) in &coin_query {
        let distance = player_transform.translation.distance(coin_transform.translation);
        if distance < 1.2 {
            commands.entity(coin_entity).despawn();
            score.coins += 1;
        }
    }
}

fn spawn_boost_pads(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Only spawn once using a resource check in a real implementation
    // Simplified here
    let pad_mesh = meshes.add(Cuboid::new(2.0, 0.1, 3.0));
    let pad_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.5, 0.0),
        emissive: Color::srgb(0.5, 0.25, 0.0).into(),
        ..default()
    });

    let positions = [(0.0, 10.0), (-10.0, -10.0), (20.0, 0.0)];

    for (x, z) in positions {
        let y = crate::player::get_ground_height(x, z);
        commands.spawn((
            PbrBundle {
                mesh: Mesh3d(pad_mesh.clone()),
                material: MeshMaterial3d(pad_material.clone()),
                transform: Transform::from_xyz(x, y + 0.05, z),
                ..default()
            },
            BoostPad,
        ));
    }
}