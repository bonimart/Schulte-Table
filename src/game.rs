use bevy::prelude::*;
use rand::{
    seq::SliceRandom,
    thread_rng,
};

use super::{
    despawn_screen,
    GameState,
    FONT_SIZE,
    COLOR_TEXT,
    WIDTH,
    HEIGHT,
    BUTTON_SIZE,
    BUTTON_PADDING,
    COLOR_DEFAULT,
    COLOR_CORRECT,
    COLOR_INCORRECT,
    TIMER_DURATION,
};

pub fn game_plugin(app: &mut App) {
    app
        .init_resource::<NextExpected>()
        .init_resource::<Score>()
        .add_systems(OnEnter(GameState::Game), game_setup)
        .add_systems(Update, (game, blink_system).run_if(in_state(GameState::Game)))
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
struct Score(u8);

fn game_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_expected: ResMut<NextExpected>,
    mut score: ResMut<Score>,
) {
    *next_expected = NextExpected::default();
    *score = Score::default();
    // Generate random numbers for the grid
    let mut numbers: Vec<u8> = (1u8..=(WIDTH * HEIGHT) as u8).collect();
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
                            height: Val::Px((BUTTON_SIZE + BUTTON_PADDING * 2.0) * HEIGHT as f32), 
                            width: Val::Px((BUTTON_SIZE + BUTTON_PADDING * 2.0) * WIDTH as f32),
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
                                height: Val::Px(BUTTON_SIZE),
                                width: Val::Px(BUTTON_SIZE),
                                margin: UiRect::all(Val::Px(BUTTON_PADDING)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: BackgroundColor(COLOR_DEFAULT).into(),
                            ..default()
                        })
                        .insert(TileButton { number })
                            .with_children(|button| {
                                button.spawn(TextBundle {
                                    text: Text::from_section(
                                              number.to_string(),
                                              TextStyle {
                                                  font: asset_server.load("embedded://fonts/FiraSans-Bold.ttf"),
                                                  font_size: FONT_SIZE,
                                                  color: COLOR_TEXT,
                                              },
                                          ),
                                          ..default()
                                });
                            });
                    }
                });
        });
}

fn game(
    mut commands: Commands,
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor, Entity, &TileButton), (Changed<Interaction>, With<Button>)>,
    mut next_expected: ResMut<NextExpected>,
    mut score: ResMut<Score>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color, entity, tile_button) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                let new_color = if tile_button.number == **next_expected {
                    **next_expected += 1;
                    **score += 1;
                    COLOR_CORRECT
                } else {
                    COLOR_INCORRECT
                };
                *color = BackgroundColor(new_color.into());
                commands.entity(entity).insert(TileBlink {
                    timer: Timer::from_seconds(TIMER_DURATION, TimerMode::Once),
                });
            },
            _ => {}
        }
    }
    if **next_expected as usize > WIDTH * HEIGHT {
        game_state.set(GameState::GameOver);
    }
}

fn blink_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut TileBlink, &mut BackgroundColor)>,
) {
    for (entity, mut blink, mut color) in query.iter_mut() {
        blink.timer.tick(time.delta());
        if blink.timer.finished() {
            *color = BackgroundColor(COLOR_DEFAULT.into());
            commands.entity(entity).remove::<TileBlink>();
        }
    }
}
