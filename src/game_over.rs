use bevy::prelude::*;
use super::game::Score;

use super::{
    despawn_screen,
    GameState,
    COLOR_TEXT,
};


pub fn game_over_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(GameState::GameOver), game_over_setup)
        .add_systems(Update, countdown.run_if(in_state(GameState::GameOver)))
        .add_systems(OnExit(GameState::GameOver), despawn_screen::<OnGameOverScreen>);
}

#[derive(Component)]
struct OnGameOverScreen;

#[derive(Resource, Deref, DerefMut)]
struct GameOverTimer(Timer);

fn game_over_setup(mut commands: Commands, score: Res<Score>) {
    let game_over_message = format!("Game Over, Score: {}", score.to_string());
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
                    game_over_message,
                    TextStyle {
                        font_size: 67.0,
                        color: COLOR_TEXT,
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
