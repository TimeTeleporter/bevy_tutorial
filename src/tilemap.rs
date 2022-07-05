use std::{fs::File, io::{BufReader, BufRead}};

use bevy::prelude::*;

use crate::{ascii::{AsciiSheet, spawn_ascii_sprite}, TILESIZE};

#[derive(Component)]
pub struct Map;

#[derive(Component)]
pub struct EncounterSpawner;


#[derive(Component)]
pub struct TileCollider;

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_simple_map);
    }
}

fn color_system(
    mut commands: Commands,
    mut fence_query: Query<&mut TextureAtlasSprite, With<TileCollider>>
) {
    for mut sprite in fence_query.iter_mut() {
        sprite.color = Color::RED;
    }
}


fn create_simple_map(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let file = File::open("assets/map.txt").expect("No map file found");
    let mut tiles = Vec::new();

    for (y, line) in BufReader::new(file).lines().enumerate() {
        if let Ok(line) = line {
            for (x, char) in line.chars().enumerate() {
                let tile = spawn_ascii_sprite(
                    &mut commands,
                    &ascii,
                    char as usize,
                    Color::rgb(0.9, 0.9, 0.9),
                    Vec3::new(x as f32 * TILESIZE, -(y as f32) * TILESIZE, 100.0)
                );
                if char == '#' {
                    commands.entity(tile)
                        .insert(TileCollider);
                }
                if char == '~' {
                    commands.entity(tile).insert(EncounterSpawner);
                }
                tiles.push(tile);
            }
        }
    }

    // Create map entity
    commands.spawn()
        .insert(Name::new("Map"))
        .insert(Map)
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .push_children(&tiles);
}