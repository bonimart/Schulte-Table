use bevy::{
    prelude::*,
    window::WindowMode,
};
use bevy_embedded_assets::EmbeddedAssetPlugin;

mod menu;
mod game;
mod game_over;

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
const INCORRECT_PENALTY: u8 = 2;


#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Menu,
    Game,
    GameOver,
}

#[bevy_main]
fn main() {
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
        .insert_resource(ClearColor(COLOR_BACKGROUND))  // Background color
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins((menu::menu_plugin, game::game_plugin, game_over::game_over_plugin))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
