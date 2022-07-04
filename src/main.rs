//! Here I follow the bevy tutorial by mwbryant
#![allow(clippy::redundant_field_names)]
use bevy::{
    prelude::*, 
    render::camera::ScalingMode, 
    window::PresentMode,
};

const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
const RESOLUTION: f32 = 16.0 / 9.0;
const WINDOWHEIGHT: f32 = 720.;

pub const TILE_SIZE: f32 = 0.1;
pub const PLAYERSPEED: f32 = 5.0;
pub const PLAYER_SIZE: f32 = 0.9;

mod player;
mod debug;
mod ascii;
mod tilemap;

use player::PlayerPlugin;
use debug::DebugPlugin;
use ascii::AsciiPlugin;
use tilemap::TileMapPlugin;

fn main() {
    let height: f32 = WINDOWHEIGHT;

    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: height * RESOLUTION,
            height: height,
            title: "Bevy Tutorial".to_string(),
            present_mode: PresentMode::Fifo,
            resizable: false,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_plugin(PlayerPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(AsciiPlugin)
        .add_plugin(TileMapPlugin)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;

    camera.orthographic_projection.right = 1.0 * RESOLUTION;
    camera.orthographic_projection.left = -1.0 * RESOLUTION;

    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
}