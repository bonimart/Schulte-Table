use bevy::{
    prelude::*,
    window::WindowMode,
};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use schulte_table::{
    GamePlugin,
    GameConfiguraiton,
};

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
