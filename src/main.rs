//! Here I follow the bevy tutorial by mwbryant
#![allow(clippy::redundant_field_names)]
use bevy::{
    prelude::*, 
    render::camera::ScalingMode, 
    window::PresentMode,
};

const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
const RESOLUTION: f32 = 16.0 / 9.0;
const WINDOWHEIGHT: f32 = 1080.;

pub const TILESIZE: f32 = 0.1;
pub const PLAYERSPEED: f32 = 2.5;
pub const PLAYERSIZE: f32 = 0.9;
pub const MINPROTECT: f32 = 1.0; // The duration in which the player is protected from encounters.
pub const MAXPROTECT: f32 = 7.0;

mod player;
mod debug;
mod ascii;
mod tilemap;
mod combat;
mod fadeout;

use player::PlayerPlugin;
use debug::DebugPlugin;
use ascii::AsciiPlugin;
use tilemap::TileMapPlugin;
use combat::CombatPlugin;
use fadeout::FadeoutPlugin;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum GameState {
    Overworld,
    Combat,
}

fn main() {
    let height: f32 = WINDOWHEIGHT;

    App::new()
        .add_state(GameState::Overworld)
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
        .add_plugin(CombatPlugin)
        .add_plugin(FadeoutPlugin)
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