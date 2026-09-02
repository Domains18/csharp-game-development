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
    // Title
    commands.spawn((
        Text::new("Beach Buggy Racing"),
        TextFont {
            font_size: 40.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.9, 0.2)),
        // Style {
        //     position_type: PositionType::Absolute,
        //     top: Val::Px(10.0),
        //     left: Val::Px(10.0),
        //     ..default()
        // },
    ));

    // Speed Text
    commands.spawn((
        Text::new("Speed: 0 km/h"),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::WHITE),
        // Style {
        //     position_type: PositionType::Absolute,
        //     top: Val::Px(60.0),
        //     left: Val::Px(10.0),
        //     ..default()
        // },
        SpeedText,
    ));

    // Score Text
    commands.spawn((
        Text::new("Coins: 0"),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.84, 0.0)),
        // Style {
        //     position_type: PositionType::Absolute,
        //     top: Val::Px(90.0),
        //     left: Val::Px(10.0),
        //     ..default()
        // },
        ScoreText,
    ));

    // Time Text
    commands.spawn((
        Text::new("Time: 0.0s"),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::WHITE),
        // Style {
        //     position_type: PositionType::Absolute,
        //     top: Val::Px(120.0),
        //     left: Val::Px(10.0),
        //     ..default()
        // },
        TimeText,
    ));

    // Controls hint
    commands.spawn((
        Text::new("WASD / Arrows to drive"),
        TextFont {
            font_size: 16.0,
            ..default()
        },
        TextColor(Color::srgb(0.8, 0.8, 0.8)),
        // Style {
        //     position_type: PositionType::Absolute,
        //     bottom: Val::Px(10.0),
        //     left: Val::Px(10.0),
        //     ..default()
        // },
    ));
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
            // In Bevy 0.15+, Text is a tuple struct wrapping a String. Mutate .0 directly.
            text.0 = format!("Speed: {:.0} km/h", kmh);
        }
    }

    if let Ok(mut text) = score_text.get_single_mut() {
        text.0 = format!("Coins: {}", score.coins);
    }

    if let Ok(mut text) = time_text.get_single_mut() {
        text.0 = format!("Time: {:.1}s", score.time);
    }
}
