use bevy::prelude::*;
use super::game::{Penalty, GameDuration};

use super::{
    despawn_screen,
    GameState,
    GameConfiguraiton,
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

fn game_over_setup(
    mut commands: Commands,
    penalty: Res<Penalty>,
    game_duration: Res<GameDuration>,
    config: Res<GameConfiguraiton>,
) {
    let time = game_duration.time.elapsed().as_millis() as f32 / 1000.0 + **penalty as f32;
    let game_over_message = format!("Time: {:.2} s", time);
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
