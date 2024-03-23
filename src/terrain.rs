use bevy::{
    prelude::*
};
use noise::{utils::*, BasicMulti, Perlin};
use rand::{thread_rng, Rng};
use crate::asset_loader::{
    GRASS,
    DEAD_GRASS,
};
fn generate_noise() -> NoiseMap {
   let mut rng = thread_rng();
   let seed: u32 = rng.gen();

   let basicmulti = BasicMulti::<Perlin>::new(seed);

   PlaneMapBuilder::<_, 2>::new(&basicmulti)
       .set_size(900, 900)
       .set_x_bounds(-5., 5.)
       .set_y_bounds(-5., 5.)
       .build()
}

fn get_tile(val: f64) -> String{
    let tile_result = match val.abs() {
        v if v < 0.2=> GRASS,
        v if v <= 1. => DEAD_GRASS,  
        _ => panic!("Bad Value")
    };
        tile_result.to_string()
}

pub fn terrain(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){  
    // Create map
    let map = generate_noise();
    let (grid_cols, grid_rows) = map.size();
    let tile_size = 16_f32;
    let start_x = -(grid_cols as f32) * tile_size / 2.;
    let start_y = -(grid_rows as f32) * tile_size /2.;

    commands.spawn(SpatialBundle::default())
        .with_children(|parent| {
            for cols in 0..grid_cols {
                for rows in 0..grid_rows{
                    let val = map.get_value(cols, rows);
                    let x = start_x + cols as f32 * tile_size;
                    let y = start_y + rows as f32 * tile_size;
                    parent.spawn(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(tile_size, tile_size)),
                            ..default()
                            },
                        texture: asset_server.load(get_tile(val)),
                        transform: Transform::from_translation(Vec3::new(x, y, 0.)),
                        ..default()
                    });
                }
            }
        });
}


