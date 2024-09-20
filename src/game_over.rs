use bevy::prelude::*;
use super::game::{Penalty, GameDuration};
use std::fs::File;
use std::fs;
use std::path;
use std::io::Write;
#[cfg(target_os = "android")]
use android_activity::AndroidApp;

use super::{
    despawn_screen,
    GameState,
    GameConfiguraiton,
};

pub fn game_over_plugin(app: &mut App) {
    app
        .init_resource::<Score>()
        .add_systems(OnEnter(GameState::GameOver), (calculate_score, game_over_setup, save_score).chain())
        .add_systems(Update, countdown.run_if(in_state(GameState::GameOver)))
        .add_systems(OnExit(GameState::GameOver), despawn_screen::<OnGameOverScreen>);
}

#[derive(Component)]
struct OnGameOverScreen;

#[derive(Resource, Deref, DerefMut)]
struct GameOverTimer(Timer);

#[derive(Resource, Default, Deref, DerefMut)]
struct Score(f32);

fn calculate_score(
    mut score: ResMut<Score>,
    penalty: Res<Penalty>,
    game_duration: Res<GameDuration>,
) {
    let time = game_duration.time.elapsed().as_millis() as f32 / 1000.0 + **penalty as f32;
    **score = time;
}

fn save_score(
    score: Res<Score>,
    config: Res<GameConfiguraiton>,
) {
    let time_str = format!("{:.2} s", **score);

    #[cfg(target_os = "android")]
    let score_dir = AndroidApp::internal_data_dir();


    let score_dir = path::Path::new(&config.score_file_path)
        .parent()
        .expect("Unable to resolve score directory");
    fs::create_dir_all(score_dir)
        .expect("Unable to create score directory");
    let mut score_file = File::options().append(true).create(true).open(&config.score_file_path)
        .expect("Unable to open/create score file");
    write!(score_file, "{}\n", time_str)
        .expect("Unable to write score file");

}

fn game_over_setup(
    mut commands: Commands,
    score: Res<Score>,
    config: Res<GameConfiguraiton>,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            OnGameOverScreen,
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    format!("Time: {:.2} s", **score),
                    TextStyle {
                        font_size: 67.0,
                        color: config.color_text,
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(50.0)),
                    ..default()
                }),
            );
        });
    commands.insert_resource(GameOverTimer(Timer::from_seconds(2.0, TimerMode::Once)));
}

fn countdown(
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<GameOverTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GameState::Menu);
    }
}
