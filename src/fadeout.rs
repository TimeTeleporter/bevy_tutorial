use bevy::prelude::*;

use crate::{GameState, ascii::AsciiSheet};

pub struct FadeoutTimer(Timer);

pub struct FadeoutPlugin;

#[derive(Component)]
pub struct  ScreenFade {
    alpha: f32,
    sent: bool,
    next_state: GameState
}

impl Plugin for FadeoutPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FadeoutTimer(Timer::from_seconds(1.0, false)))
            .add_system(fadeout);
    }
}

fn fadeout(
    mut commands: Commands,
    mut fade_query: Query<(Entity, &mut ScreenFade, &mut TextureAtlasSprite)>,
    mut state: ResMut<State<GameState>>,
    mut timer: ResMut<FadeoutTimer>,
    time: Res<Time>,
) {
    for (entity, mut fade, mut sprite) in fade_query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.percent() < 0.5 {
            fade.alpha = timer.0.percent() * 2.0;
        } else {
            fade.alpha = timer.0.percent_left() * 2.0;
        }
        sprite.color.set_a(fade.alpha);

        if timer.0.percent() > 0.5 && !fade.sent {
            state.set(fade.next_state).unwrap();
            fade.sent = true;
        }
        if timer.0.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn create_fadeout<'a>(
    commands: &mut Commands,
    next_state: GameState,
    ascii: &Res<AsciiSheet>,
    timer: &'a mut ResMut<FadeoutTimer>
) -> Option<&'a ResMut<'a, FadeoutTimer>> {
    timer.0.reset();

    let mut sprite = TextureAtlasSprite::new(0);
    sprite.color = Color::rgba(0.1, 0.1, 0.15, 0.0);
    sprite.custom_size = Some(Vec2::splat(10000.0));

    commands.spawn_bundle(SpriteSheetBundle {
            sprite,
            texture_atlas: ascii.0.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 999.0),
                ..default()
            },
            ..default()
        })
        .insert(ScreenFade {
            alpha: 0.0,
            sent: false,
            next_state: next_state,
        })
        .insert(Name::new("Fadeout"));
    
    Some(timer)
}