use bevy::{
    prelude::*,
    window::WindowMode,
};
use bevy_embedded_assets::EmbeddedAssetPlugin;

mod menu;
mod game;
mod game_over;

#[derive(Resource, Clone)]
pub struct GameConfiguraiton {
    pub width: usize,
    pub height: usize,
    pub button_size: f32,
    pub button_padding: f32,
    pub font_size: f32,
    pub color_correct: Color,
    pub color_incorrect: Color,
    pub color_default: Color,
    pub color_text: Color,
    pub color_background: Color,
    pub timer_duration: f32,
    pub incorrect_penalty: u8,
}

impl Default for GameConfiguraiton {
    fn default() -> Self {
        GameConfiguraiton {
            width: 2,
            height: 2,
            button_size: 50.0,
            button_padding: 5.0,
            font_size: 30.0,
            color_correct: Color::srgb(0.2, 0.8, 0.2),
            color_incorrect: Color::srgb(0.8, 0.2, 0.2),
            color_default: Color::srgb(0.5, 0.5, 0.5),
            color_text: Color::WHITE,
            color_background: Color::srgb(0.9, 0.9, 0.9),
            timer_duration: 0.2,
            incorrect_penalty: 2,
        }
    }
}

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
                }),
            GamePlugin::default(),
        ))
        .run();
}

pub struct GamePlugin {
    configuration: GameConfiguraiton,
}

impl Default for GamePlugin {
    fn default() -> Self {
        GamePlugin {
            configuration: GameConfiguraiton::default(),
        }
    }
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(self.configuration.clone())
            .insert_resource(ClearColor(self.configuration.color_background))
            .init_state::<GameState>()
            .add_systems(Startup, setup)
            .add_plugins((menu::menu_plugin, game::game_plugin, game_over::game_over_plugin));
    }
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
