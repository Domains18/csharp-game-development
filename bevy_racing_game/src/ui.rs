use bevy::prelude::*;
use crate::Score;
use crate::player::Player;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
           .add_systems(Update, update_ui);
    }
}

#[derive(Component)]
struct SpeedText;

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct TimeText;

fn setup_ui(mut commands: Commands) {
    commands.spawn(
        TextBundle::from_section(
            "Beach Buggy Racing",
            TextStyle {
                font_size: 40.0,
                color: Color::srgb(1.0, 0.9, 0.2),
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
    );

    commands.spawn((
        TextBundle::from_section(
            "Speed: 0 km/h",
            TextStyle {
                font_size: 24.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(60.0),
            left: Val::Px(10.0),
            ..default()
        }),
        SpeedText,
    ));

    commands.spawn((
        TextBundle::from_section(
            "Coins: 0",
            TextStyle {
                font_size: 24.0,
                color: Color::srgb(1.0, 0.84, 0.0),
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(90.0),
            left: Val::Px(10.0),
            ..default()
        }),
        ScoreText,
    ));

    commands.spawn((
        TextBundle::from_section(
            "Time: 0.0s",
            TextStyle {
                font_size: 24.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(120.0),
            left: Val::Px(10.0),
            ..default()
        }),
        TimeText,
    ));

    // Controls hint
    commands.spawn(
        TextBundle::from_section(
            "WASD / Arrows to drive",
            TextStyle {
                font_size: 16.0,
                color: Color::srgb(0.8, 0.8, 0.8),
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
    );
}

fn update_ui(
    mut speed_text: Query<&mut Text, With<SpeedText>>,
    mut score_text: Query<&mut Text, (With<ScoreText>, Without<SpeedText>)>,
    mut time_text: Query<&mut Text, (With<TimeText>, Without<SpeedText>, Without<ScoreText>)>,
    player_query: Query<&Player>,
    mut score: ResMut<Score>,
    time: Res<Time>,
) {
    score.time += time.delta_secs();

    if let Ok(player) = player_query.get_single() {
        if let Ok(mut text) = speed_text.get_single_mut() {
            let kmh = (player.speed * 3.6).abs();
            text.sections[0].value = format!("Speed: {:.0} km/h", kmh);
        }
    }

    if let Ok(mut text) = score_text.get_single_mut() {
        text.sections[0].value = format!("Coins: {}", score.coins);
    }

    if let Ok(mut text) = time_text.get_single_mut() {
        text.sections[0].value = format!("Time: {:.1}s", score.time);
    }
}