use bevy::prelude::*;

// Terrain Tiles
pub const GRASS: &str = "MiniWorldSprites/Ground/TexturedGrass.png";
pub const DEAD_GRASS: &str = "MiniWorldSprites/Ground/DeadGrass.png"; 
// Animals 

// Hero Sprites
pub const BORG: &str = "MiniWorldSprites/Characters/Champions/borg.png";



#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub hero: Handle<Image>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>) {
    *scene_assets = SceneAssets {
        // Character Assets
        hero: asset_server.load(BORG),
    }
}
    

