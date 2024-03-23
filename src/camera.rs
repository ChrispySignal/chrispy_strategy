use bevy::{
    prelude::*,
    window::PrimaryWindow,
};

#[derive(Resource, Default)]
pub struct WorldCoords(Vec2);

#[derive(Component)]
pub struct MainCamera;

pub struct MainCameraPlugin;

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldCoords>()
        .add_systems(Startup, camera)
        .add_systems(Update, cursor_coords);
    }
}

pub fn camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

pub fn cursor_coords(
    mut coords: ResMut<WorldCoords>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
        {
            coords.0 = world_position;
            eprintln!("World Coords: {}/{}", world_position.x, world_position.y);
        }
}

