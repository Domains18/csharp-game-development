mod player;
mod world;
mod camera;
mod collectibles;
mod ui;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Beach Buggy Racing".into(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.52, 0.8, 0.92))) // Sky blue
        .insert_resource(Score { coins: 0, time: 0.0 })
        .add_systems(Startup, setup)
        .add_plugins((
            player::PlayerPlugin,
            world::WorldPlugin,
            camera::CameraPlugin,
            collectibles::CollectiblesPlugin,
            ui::UiPlugin,
        ))
        .run();
}

#[derive(Resource)]
pub struct Score {
    pub coins: u32,
    pub time: f32,
}

fn setup(mut commands: Commands) {
    // Ambient light
    commands.insert_resource(AmbientLight {
        color: Color::srgb(1.0, 0.95, 0.9),
        brightness: 0.6,
    });

    // Sun
    commands.spawn((
        DirectionalLight {
            illuminance: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(50.0, 80.0, 30.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}