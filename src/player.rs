use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_inspector_egui::Inspectable;

use crate::{
    PLAYERSPEED, TILE_SIZE, PLAYER_SIZE,
    ascii::{AsciiSheet, spawn_ascii_sprite},
    tilemap::TileCollider,
};

pub struct PlayerPlugin;

#[derive(Component, Inspectable)]
pub struct Player {
    speed: f32,
}


impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_player)
            .add_system(camera_follow.after("movement"))
            .add_system(player_movement.label("movement"));
    }
}

fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let player_transform: &Transform = player_query.single();
    let mut camera_transform: Mut<Transform> = camera_query.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}

fn player_movement(
    mut player_query: Query<(&Player, &mut Transform)>,
    wall_query: Query<&Transform, (With<TileCollider>, Without<Player>)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (player, mut transform): (&Player, Mut<'_, Transform>) = player_query.single_mut();

    let mut delta_y: f32 = 0.0;
    let mut delta_x: f32 = 0.0;

    if keyboard.pressed(KeyCode::W) {
        delta_y += player.speed * TILE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::S) {
        delta_y += -player.speed  * TILE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::A) {
        delta_x += -player.speed * TILE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::D) {
        delta_x += player.speed * TILE_SIZE * time.delta_seconds();
    }

    let target = transform.translation + Vec3::new(delta_x, 0.0, 0.0);
    if wall_collision_check(target, &wall_query) {
        transform.translation = target;
    }

    let target = transform.translation + Vec3::new(0.0, delta_y, 0.0);
    if wall_collision_check(target, &wall_query) {
        transform.translation = target;
    }
}

fn wall_collision_check(
    target_player_pos: Vec3,
    wall_query: &Query<&Transform, (With<TileCollider>, Without<Player>)>,
) -> bool {
    for wall_transform in wall_query.iter() {
        let collision = collide(
            target_player_pos,
            Vec2::splat(TILE_SIZE * PLAYER_SIZE),
            wall_transform.translation,
            Vec2::splat(TILE_SIZE)
        );

        if collision.is_some() {
            return false;
        }
    }
    true
}

fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    // Creates the player from a sprite.
    let player = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        1,
        Color::rgb(0.3, 0.3, 0.9),
        Vec3::new(2.0 * TILE_SIZE, -2.0 * TILE_SIZE, 900.0)
    );
    
    let player = commands
        .entity(player)
        .insert(Name::new("Player"))
        .insert(Player { speed: PLAYERSPEED })
        .id();

    // Creates a background for the player sprite.
    let background = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        0,
        Color::rgb(0.5, 0.5, 0.5),
        Vec3::new(0.0, 0.0, -1.0)
    );
    
    commands.entity(player).push_children(&[background]);
}