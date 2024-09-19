use bevy::prelude::*;
use schulte_table::GamePlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            GamePlugin::default(),
        ))
        .run();
}
