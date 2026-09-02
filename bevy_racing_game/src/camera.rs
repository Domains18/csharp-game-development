use bevy::prelude::*;
use crate::player::Player;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
           .add_systems(Update, camera_follow);
    }
}

#[derive(Component)]
struct GameCamera {
    offset: Vec3,
    target_offset: Vec3,
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, -8.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        GameCamera {
            offset: Vec3::new(0.0, 4.0, -7.0),
            target_offset: Vec3::new(0.0, 2.0, 3.0),
        },
    ));
}

fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<(&mut Transform, &GameCamera), Without<Player>>,
    time: Res<Time>,
) {
    let Ok(player_transform) = player_query.get_single() else { return };
    let Ok((mut camera_transform, camera)) = camera_query.get_single_mut() else { return };

    let dt = time.delta_secs();
    
    // Calculate desired camera position (behind and above player)
    let player_forward = player_transform.forward();
    let desired_pos = player_transform.translation 
        - player_forward * camera.offset.z 
        + Vec3::Y * camera.offset.y;

    // Smooth follow
    camera_transform.translation = camera_transform.translation.lerp(desired_pos, 5.0 * dt);

    // Look at player (slightly ahead)
    let look_target = player_transform.translation + player_forward * camera.target_offset.z;
    let target_rotation = Transform::from_translation(camera_transform.translation)
        .looking_at(look_target, Vec3::Y)
        .rotation;
    
    camera_transform.rotation = camera_transform.rotation.slerp(target_rotation, 3.0 * dt);
}
