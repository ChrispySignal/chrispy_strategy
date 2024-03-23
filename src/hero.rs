use bevy::{
    prelude::*,
    input::mouse::MouseButtonInput,
};

use crate::{
    movement::*,
    asset_loader::SceneAssets,
    camera::WorldCoords,
};
pub struct HeroPlugin;

impl Plugin for HeroPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_hero);
    }
}
#[derive(Component)]
pub struct Hero;


const STARTING_TRANSLATION: Vec3 = Vec3::new(0., -20., 1.);
const HERO_SPEED: f32 = 25.;

pub fn spawn_hero(
    mut commands: Commands,
    mut texture_atlas: ResMut<Assets<TextureAtlasLayout>>,
    scene_assets: Res<SceneAssets>,
) {
    let texture = scene_assets.hero.clone();
    let layout = TextureAtlasLayout::from_grid(Vec2::new(16., 16.), 5, 1, None, None);
    let texture_atlas_layout = texture_atlas.add(layout);
    let animation_indices = AnimationIndices { first: 1, last: 4 };
    commands.spawn((MovementBundle {
        velocity: Velocity::new(Vec3::ZERO),
        acceleration: Acceleration::new(Vec3::ZERO),
    },
            SpriteSheetBundle {
                texture,
                atlas: TextureAtlas {
                    layout: texture_atlas_layout,
                    ..Default::default()
                },
                transform: Transform::from_translation(STARTING_TRANSLATION),
                ..default()
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
            Hero,
            ));
}

