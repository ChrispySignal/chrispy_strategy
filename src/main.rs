use bevy::{ prelude::*,
    window::close_on_esc,
};

use mini_world::{
    terrain::terrain,
    hero::HeroPlugin,
    movement::*,
    asset_loader::AssetLoaderPlugin,
    camera::MainCameraPlugin,
};
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((
                HeroPlugin,
                MovementPlugin,
                AssetLoaderPlugin,
                MainCameraPlugin,
                ))
        .add_systems(Startup, (terrain, close_on_esc))
        .run();
}


