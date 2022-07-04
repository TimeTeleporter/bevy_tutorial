use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_inspector_egui::Inspectable;

use crate::{
    PLAYERSPEED, TILESIZE, PLAYERSIZE, GameState,
    ascii::{AsciiSheet, spawn_ascii_sprite},
    tilemap::{TileCollider, EncounterSpawner, Map},
};

#[derive(Component, Inspectable)]
pub struct Player {
    speed: f32,
    just_moved: bool,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(GameState::Overworld)
                .with_system(show_player)
                .with_system(show_map)
            )
            .add_system_set(SystemSet::on_enter(GameState::Combat)
                .with_system(hide_player)
                .with_system(hide_map)
            )
            .add_system_set(
                SystemSet::on_update(GameState::Overworld)
                .with_system(camera_follow.after("movement"))
                .with_system(player_movement.label("movement"))
                .with_system(player_encounter_checking)
            )
            .add_system_set(
                SystemSet::on_update(GameState::Combat)
                    .with_system(test_exit_combat)
            )
            .add_startup_system(spawn_player);
    }
}

fn test_exit_combat(mut keyboard: ResMut<Input<KeyCode>>, mut state: ResMut<State<GameState>>) {
    if keyboard.just_pressed(KeyCode::Space) {
        println!("Changing to overworld");
        state.set(GameState::Overworld).unwrap();
        keyboard.clear();
    }
}

fn hide_map(
    children_query: Query<&Children, With<Map>>,
    child_visibility_query: Query<&mut Visibility, Without<Map>>,
) {
    change_map_visability(children_query, child_visibility_query, false);
}

fn show_map(
    children_query: Query<&Children, With<Map>>,
    child_visibility_query: Query<&mut Visibility, Without<Map>>,
) {
    change_map_visability(children_query, child_visibility_query, true);
}

fn change_map_visability(
    children_query: Query<&Children, With<Map>>,
    mut child_visibility_query: Query<&mut Visibility, Without<Map>>,
    is_visible: bool,
) {
    if let Ok(children) = children_query.get_single() {
        for child in children.iter() {
            if let Ok(mut child_vis) = child_visibility_query.get_mut(*child) {
                child_vis.is_visible = is_visible;
            }
        }
    }
}

fn hide_player(
    player_query: Query<&mut Visibility, With<Player>>,
    children_query: Query<&Children, With<Player>>,
    child_visibility_query: Query<&mut Visibility, Without<Player>>,
) {
    change_player_visability(player_query, children_query, child_visibility_query, false);
}

fn show_player(
    player_query: Query<&mut Visibility, With<Player>>,
    children_query: Query<&Children, With<Player>>,
    child_visibility_query: Query<&mut Visibility, Without<Player>>,
) {
    change_player_visability(player_query, children_query, child_visibility_query, true);
}

fn change_player_visability(
    mut player_query: Query<&mut Visibility, With<Player>>,
    children_query: Query<&Children, With<Player>>,
    mut child_visibility_query: Query<&mut Visibility, Without<Player>>,
    is_visible: bool,
) {
    let mut player_vis: Mut<Visibility> = player_query.single_mut();
    player_vis.is_visible = is_visible;

    if let Ok(children) = children_query.get_single() {
        for child in children.iter() {
            if let Ok(mut child_vis) = child_visibility_query.get_mut(*child) {
                child_vis.is_visible = is_visible;
            }
        }
    }
}

fn player_encounter_checking(
    player_query: Query<(&Player, &Transform)>,
    encounter_query: Query<&Transform, (With<EncounterSpawner>, Without<Player>)>,
    mut state: ResMut<State<GameState>>,
) {
    let (player, player_transform) = player_query.single();
    if encounter_query.iter().any(|&transform| wall_collision_check(player_transform.translation, transform.translation))
        && player.just_moved {
        println!("Changing to combat!");
        state.set(GameState::Combat).expect("Failed ot change states");
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
    mut player_query: Query<(&mut Player, &mut Transform)>,
    wall_query: Query<&Transform, (With<TileCollider>, Without<Player>)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut player, mut transform): (Mut<'_, Player>, Mut<'_, Transform>) = player_query.single_mut();

    player.just_moved = false;

    let mut delta_y: f32 = 0.0;
    let mut delta_x: f32 = 0.0;

    if keyboard.pressed(KeyCode::W) {
        delta_y += player.speed * TILESIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::S) {
        delta_y += -player.speed  * TILESIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::A) {
        delta_x += -player.speed * TILESIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::D) {
        delta_x += player.speed * TILESIZE * time.delta_seconds();
    }

    let target = transform.translation + Vec3::new(delta_x, 0.0, 0.0);

    // We move the player only if the collision check was negative
    if !wall_query.iter().any(|&transform| wall_collision_check(target, transform.translation)) {
        transform.translation = target;

        if delta_x != 0.0 {
            player.just_moved = true;
        }
    }

    let target = transform.translation + Vec3::new(0.0, delta_y, 0.0);
    if !wall_query.iter().any(|&transform| wall_collision_check(target, transform.translation)) {
        transform.translation = target;

        if delta_x != 0.0 {
            player.just_moved = true;
        }
    }
}

fn wall_collision_check(target_player_pos: Vec3, wall_translation: Vec3) -> bool {
    let collision = collide(
        target_player_pos,
        Vec2::splat(TILESIZE * PLAYERSIZE),
        wall_translation,
        Vec2::splat(TILESIZE),
    );

    collision.is_some()
}

fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    // Creates the player from a sprite.
    let player = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        1,
        Color::rgb(0.3, 0.3, 0.9),
        Vec3::new(2.0 * TILESIZE, -2.0 * TILESIZE, 900.0)
    );
    
    let player = commands
        .entity(player)
        .insert(Name::new("Player"))
        .insert(Player {
            speed: PLAYERSPEED,
            just_moved: false,
        })
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