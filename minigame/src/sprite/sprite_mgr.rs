use bevy::prelude::*;

#[derive(Resource)]
pub struct SpriteManager {
    pub texture: Handle<Image>,
    pub texture_atlas_layouts: TextureAtlasLayout,
}

pub struct SpriteManagerPlugin;

impl Plugin for SpriteManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_sprite_res);
    }
}

fn setup_sprite_res(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("textures/sprite_sheet.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(300), 8, 2, None, None);

    commands.insert_resource(SpriteManager {
        texture: texture,
        texture_atlas_layouts: layout,
    });
}
