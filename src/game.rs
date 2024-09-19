use bevy::{
    prelude::*,
    time::Stopwatch,
};
use rand::{
    seq::SliceRandom,
    thread_rng,
};

use super::{
    despawn_screen,
    GameState,
    GameConfiguraiton,
};

pub fn game_plugin(app: &mut App) {
    app
        .init_resource::<NextExpected>()
        .init_resource::<Penalty>()
        .add_systems(OnEnter(GameState::Game), game_setup)
        .add_systems(Update, (click_handler, blink_system, check_game_over, update_timer)
            .run_if(in_state(GameState::Game)))
        .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
}

#[derive(Component)]
struct OnGameScreen;

#[derive(Component)]
struct TileButton {
    number: u8,
}

#[derive(Component)]
struct TileBlink {
    timer: Timer,
}

#[derive(Resource, Deref, DerefMut)]
struct NextExpected(u8);

impl Default for NextExpected {
    fn default() -> Self {
        NextExpected(1)
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct Penalty(u8);

#[derive(Resource, Deref, DerefMut)]
pub struct GameDuration {
    pub time: Stopwatch
}

fn game_setup(
    mut commands: Commands,
    mut next_expected: ResMut<NextExpected>,
    mut penalty: ResMut<Penalty>,
    config: Res<GameConfiguraiton>,
) {
    *next_expected = NextExpected::default();
    *penalty = Penalty::default();
    // Generate random numbers for the grid
    let mut numbers: Vec<u8> = (1u8..=(config.width * config.height) as u8).collect();
    let mut rng = thread_rng();
    numbers.shuffle(&mut rng);

    // Root UI node
    commands
        .spawn((
                NodeBundle {
                    style: Style {
                        height: Val::Percent(100.0),
                        width: Val::Percent(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::NONE),
                    ..default()
                },
                OnGameScreen,
        ))
        .with_children(|parent| {
            // Create a grid of buttons for the Schulte table
            parent.
                spawn(
                    NodeBundle {
                        style: Style {
                            height: Val::Px((config.button_size + config.button_padding * 2.0) * config.height as f32), 
                            width: Val::Px((config.button_size + config.button_padding * 2.0) * config.width as f32),
                            flex_wrap: FlexWrap::Wrap,
                            justify_content: JustifyContent::SpaceAround,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ..default()
                    },
                )
                .with_children(|grid| {
                    for &number in numbers.iter() {
                        grid.spawn(ButtonBundle {
                            style: Style {
                                height: Val::Px(config.button_size),
                                width: Val::Px(config.button_size),
                                margin: UiRect::all(Val::Px(config.button_padding)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: BackgroundColor(config.color_default).into(),
                            ..default()
                        })
                        .insert(TileButton { number })
                            .with_children(|button| {
                                button.spawn(TextBundle {
                                    text: Text::from_section(
                                              number.to_string(),
                                              TextStyle {
                                                  font_size: config.font_size,
                                                  color: config.color_text,
                                                  ..default()
                                              },
                                          ),
                                          ..default()
                                });
                            });
                    }
                });
        });
    commands.insert_resource(GameDuration {
        time: Stopwatch::new(),
    });
}

fn click_handler(
    mut commands: Commands,
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor, Entity, &TileButton), (Changed<Interaction>, With<Button>)>,
    mut next_expected: ResMut<NextExpected>,
    mut penalty: ResMut<Penalty>,
    config: Res<GameConfiguraiton>,
) {
    for (interaction, mut color, entity, tile_button) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                let new_color = if tile_button.number == **next_expected {
                    **next_expected += 1;
                    config.color_correct
                } else {
                    **penalty += config.incorrect_penalty;
                    config.color_incorrect
                };
                *color = BackgroundColor(new_color.into());
                commands.entity(entity).insert(TileBlink {
                    timer: Timer::from_seconds(config.timer_duration, TimerMode::Once),
                });
            },
            _ => {}
        }
    }
}

fn blink_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut TileBlink, &mut BackgroundColor)>,
    config: Res<GameConfiguraiton>,
) {
    for (entity, mut blink, mut color) in query.iter_mut() {
        blink.timer.tick(time.delta());
        if blink.timer.finished() {
            *color = BackgroundColor(config.color_default.into());
            commands.entity(entity).remove::<TileBlink>();
        }
    }
}

fn check_game_over(
    next_expected: Res<NextExpected>,
    mut game_duration: ResMut<GameDuration>,
    mut game_state: ResMut<NextState<GameState>>,
    config: Res<GameConfiguraiton>,
) {
    if **next_expected as usize > config.width * config.height {
        game_duration.time.pause();
        game_state.set(GameState::GameOver);
    }
}

fn update_timer(
    mut game_duration: ResMut<GameDuration>,
    time: Res<Time>,
) {
    game_duration.time.tick(time.delta());
}
