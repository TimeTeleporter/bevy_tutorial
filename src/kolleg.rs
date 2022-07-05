use bevy::prelude::*;

pub struct KollegPlugin;

pub struct RehuHandle(pub Handle<TextureAtlas>);
pub struct ImiHandle(pub Handle<TextureAtlas>);
pub struct MibiHandle(pub Handle<TextureAtlas>);

impl Plugin for KollegPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_kolleg);
    }
}

fn load_kolleg(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    let image = assets.load("rehu.png");
    let atlas = TextureAtlas::from_grid_with_padding(
        image,
        Vec2::splat(9.0),
        1,
        1,
        Vec2::splat(2.0)
    );

    let atlas_handle = texture_atlases.add(atlas);

    commands.insert_resource(RehuHandle(atlas_handle));

    let image = assets.load("imi.png");
    let atlas = TextureAtlas::from_grid_with_padding(
        image,
        Vec2::splat(9.0),
        1,
        1,
        Vec2::splat(2.0)
    );

    let atlas_handle = texture_atlases.add(atlas);

    commands.insert_resource(ImiHandle(atlas_handle));

    let image = assets.load("mibi.png");
    let atlas = TextureAtlas::from_grid_with_padding(
        image,
        Vec2::splat(9.0),
        1,
        1,
        Vec2::splat(2.0)
    );

    let atlas_handle = texture_atlases.add(atlas);

    commands.insert_resource(MibiHandle(atlas_handle));
}