use bevy::{
    prelude::*,
    input::touch::TouchPhase,
    ui::Val,
    window::{AppLifecycle, WindowMode},
};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use rand::{
    seq::SliceRandom,
    thread_rng,
};

const WIDTH: usize = 2;
const HEIGHT: usize = 2;
const BUTTON_SIZE: f32 = 50.0;
const BUTTON_PADDING: f32 = 5.0;
const FONT_SIZE: f32 = 30.0;
const COLOR_CORRECT: Color = Color::srgb(0.2, 0.8, 0.2);
const COLOR_INCORRECT: Color = Color::srgb(0.8, 0.2, 0.2);
const COLOR_DEFAULT: Color = Color::srgb(0.5, 0.5, 0.5);
const COLOR_TEXT: Color = Color::WHITE;
const COLOR_BACKGROUND: Color = Color::srgb(0.9, 0.9, 0.9);
const TIMER_DURATION: f32 = 0.2;

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

#[derive(Resource, Deref, DerefMut)]
struct Score(u8);

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Playing,
    GameOver,
}

#[bevy_main]
fn main() {
    run_game();
}

fn run_game() {
    App::new()
        .add_plugins((
            EmbeddedAssetPlugin::default(),
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: false,
                        mode: WindowMode::BorderlessFullscreen,
                        ..default()
                    }),
                    ..default()
                })
        ))
        .insert_state(AppState::Playing)
        .insert_resource(ClearColor(COLOR_BACKGROUND))  // Background color
        .insert_resource(NextExpected(1))  // Track the next expected tile
        .insert_resource(Score(0))  // Track the score
        .add_systems(Startup, setup)
        .add_systems(Update, button_click_handler)
        .add_systems(Update, blink_system)
        .add_systems(OnExit(AppState::Playing), game_over)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Generate random numbers for the grid
    let mut numbers: Vec<u8> = (1u8..=(WIDTH * HEIGHT) as u8).collect();
    let mut rng = thread_rng();
    numbers.shuffle(&mut rng);

    // Root UI node
    commands.spawn(NodeBundle {
        style: Style {
            height: Val::Percent(100.0),
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: BackgroundColor(Color::NONE),
        ..default()
    })
    .with_children(|parent| {
        // Create a grid of buttons for the Schulte table
        parent.spawn(NodeBundle {
            style: Style {
                height: Val::Px((BUTTON_SIZE + BUTTON_PADDING * 2.0) * HEIGHT as f32), 
                width: Val::Px((BUTTON_SIZE + BUTTON_PADDING * 2.0) * WIDTH as f32),
                flex_wrap: FlexWrap::Wrap,
                justify_content: JustifyContent::SpaceAround,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
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

fn button_click_handler(
    mut commands: Commands,
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor, Entity, &TileButton), (Changed<Interaction>, With<Button>)>,
    mut next_expected: ResMut<NextExpected>,
    mut score: ResMut<Score>,
    mut next_state: ResMut<NextState<AppState>>,    
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

                // Change the color and add the TileBlink component
                *color = BackgroundColor(new_color.into());
                commands.entity(entity).insert(TileBlink {
                    timer: Timer::from_seconds(TIMER_DURATION, TimerMode::Once),
                });

                if **next_expected as usize > WIDTH * HEIGHT {
                    // Game over
                    next_state.set(AppState::GameOver);
                }
            },
            _ => {}
        }
    }
}

fn blink_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut TileBlink, &mut BackgroundColor)>,
) {
    for (entity, mut blink, mut color) in query.iter_mut() {
        blink.timer.tick(time.delta());

        // Reset the color after 0.5s and remove the TileBlink component
        if blink.timer.finished() {
            *color = BackgroundColor(COLOR_DEFAULT.into());
            commands.entity(entity).remove::<TileBlink>();
        }
    }
}

fn game_over(mut commands: Commands, score: Res<Score>, asset_server: Res<AssetServer>) {
    commands.spawn(NodeBundle {
        style: Style {
            height: Val::Percent(100.0),
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: BackgroundColor(Color::NONE),
        ..default()
    });
    // show score on the screen
    commands.spawn(TextBundle {
        style: Style {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        text: Text::from_section(
            "Game Over!\nScore: ".to_string() + &score.to_string(),
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: FONT_SIZE,
                color: COLOR_TEXT,
            },
        ),
        ..default()
    });
}
