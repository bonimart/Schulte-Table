use bevy::{
    prelude::*,
    window::WindowMode,
};
use schulte_table::GamePlugin;

fn main() {
    App::new()
        .add_plugins((
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
