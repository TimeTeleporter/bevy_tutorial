#![allow(unused_imports)]

use bevy::prelude::*;
use rand::Rng;

use crate::{GameState,
        ascii::{AsciiSheet, spawn_ascii_sprite}, 
        fadeout::{create_fadeout, FadeoutTimer}, 
        player::{Player, self}
    };

#[derive(Component)]
struct Enemy;

struct FightEvent {
    target: Entity,
    damage_amount: isize,
}

#[derive(Component)]
pub struct CombatStats {
    pub health: isize,
    pub max_health: isize,
    pub attack: isize,
    pub defense: isize,
}

struct CombatCooldown(Timer);

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CombatCooldown(Timer::from_seconds(0.5, false)))
            .add_event::<FightEvent>()
            .add_system_set(SystemSet::on_update(GameState::Combat)
                .with_system(damage_calculation)
                .with_system(combat_input)
                //.with_system(test_exit_combat)
                .with_system(combat_camera)
            )
            .add_system_set(SystemSet::on_enter(GameState::Combat).with_system(spawn_enemy))
            .add_system_set(SystemSet::on_exit(GameState::Combat).with_system(despawn_enemies));
    }
}

fn damage_calculation(
    mut commands: Commands,
    mut fight_event: EventReader<FightEvent>,
    mut target_query: Query<(&mut Name, &mut CombatStats)>,
    ascii: Res<AsciiSheet>,
    mut fade_timer: ResMut<FadeoutTimer>,
) {
    for event in fight_event.iter() {
        let (name, mut target_stats): (Mut<'_, Name>, Mut<'_, CombatStats>) = target_query
            .get_mut(event.target)
            .expect("Fighting target without stats!");
        
        target_stats.health = std::cmp::max(target_stats.health - (event.damage_amount-target_stats.defense), 0);
        println!("{} has {} hp left", name.to_owned(), target_stats.health);

        if target_stats.health == 0 {
            println!("{} has died.", name.to_owned());
            create_fadeout(&mut commands, GameState::Overworld, &ascii, &mut fade_timer);
        }
    }
}

fn combat_input(keyboard: ResMut<Input<KeyCode>>,
    mut fight_event: EventWriter<FightEvent>,
    player_query: Query<&CombatStats, With<Player>>,
    enemy_query: Query<Entity, With<Enemy>>,
    time: Res<Time>,
    mut combat_timer: ResMut<CombatCooldown>,
) {
    combat_timer.0.tick(time.delta());

    let target = enemy_query.single();
    let player_stats = player_query.single();

    if keyboard.just_pressed(KeyCode::Return) && combat_timer.0.finished() {
        fight_event.send(FightEvent {
            target,
            damage_amount: player_stats.attack,
        });
        combat_timer.0.reset();
    }
}

fn combat_camera(mut camera_query: Query<&mut Transform, With<Camera>>) {
    let mut camera_transform = camera_query.single_mut();
    camera_transform.translation.x = 0.0;
    camera_transform.translation.y = 0.0;
}

fn spawn_enemy(commands: Commands, asset_server: Res<AssetServer>) {
    const CHOICES: usize = 3;

    let choice = rand::thread_rng().gen_range(0..CHOICES);

    let (name, hp): (&str, isize) = match choice {
        0 => { spawn_rehu(commands, asset_server) },
        1 => { spawn_imi(commands, asset_server) },
        _ => { spawn_mibi(commands, asset_server)}
    };
    
        println!("A wild {} appears! It has {} hp.", name, hp);
}

fn spawn_mibi<'a>(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) -> (&'a str, isize) {
    let name = "Mibi";
    let hp = 1;

    commands.spawn_bundle(SpriteBundle {
            texture: asset_server.load("mibi.png"),
            transform: Transform {
                translation: Vec3::new(0.0, 0.2, 0.0),
                scale: Vec3::new(0.001, 0.001, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(Enemy)
        .insert(Name::new(name))
        .insert(CombatStats {
            health: hp,
            max_health: 3,
            attack: 2,
            defense: 1,
        });
    (name, hp)
}

fn spawn_imi<'a>(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) -> (&'a str, isize) {
    let name = "Imi";
    let hp = 3;

    commands.spawn_bundle(SpriteBundle {
            texture: asset_server.load("imi.png"),
            transform: Transform {
                translation: Vec3::new(0.0, 0.2, 0.0),
                scale: Vec3::new(0.001, 0.001, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(Enemy)
        .insert(Name::new(name))
        .insert(CombatStats {
            health: hp,
            max_health: 3,
            attack: 2,
            defense: 1,
        });
    (name, hp)
}

fn spawn_rehu<'a>(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) -> (&'a str, isize) {
    let name = "Rehu";
    let hp = 5;

    commands.spawn_bundle(SpriteBundle {
            texture: asset_server.load("rehu.png"),
            transform: Transform {
                translation: Vec3::new(0.0, 0.2, 0.0),
                scale: Vec3::new(0.001, 0.001, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(Enemy)
        .insert(Name::new(name))
        .insert(CombatStats {
            health: hp,
            max_health: 3,
            attack: 2,
            defense: 1,
        });
    (name, hp)
}


/*
fn spawn_enemy(mut commands: Commands, ascii:Res<AsciiSheet>) {
    let sprite = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        'N' as usize,
        Color::rgb(0.8, 0.8, 0.2),
        Vec3::new(0.0, 0.5, 100.0)
    );
    

    commands.entity(sprite)
        .insert(Enemy)
        .insert(Name::new("Bat"));
}
*/

fn despawn_enemies(mut commands: Commands, enemy_query: Query<Entity, With<Enemy>>) {
    for entity in enemy_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn _test_exit_combat(
    mut commands: Commands,
    keyboard: ResMut<Input<KeyCode>>,
    ascii: Res<AsciiSheet>,
    mut timer: ResMut<FadeoutTimer>
) {
    if keyboard.just_pressed(KeyCode::Space) {
        println!("Changing to overworld");
        create_fadeout(&mut commands, GameState::Overworld, &ascii, &mut timer);
    }
}