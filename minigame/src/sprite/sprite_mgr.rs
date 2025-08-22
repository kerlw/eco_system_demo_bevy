use super::config::AtlasConfig;
use bevy::prelude::*;

#[derive(Resource)]
pub struct SpriteManager {
    pub texture: Handle<Image>,
    pub texture_atlas_layouts: Handle<TextureAtlasLayout>,
    pub config: AtlasConfig,
    pub stomach_icon: Handle<Image>,
}

pub struct SpriteManagerPlugin;

impl Plugin for SpriteManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_sprite_res);
    }
}

pub fn setup_sprite_res(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    info!("SpriteManager Plugin Loaded");
    let atlas_name = "sprite_sheet";
    let image_path = format!("textures/{}.png", atlas_name);

    let cfg_path = format!("assets/textures/{}.json", atlas_name);

    let config = match std::fs::read_to_string(&cfg_path) {
        Ok(cfg_str) => AtlasConfig::from(cfg_str.as_ref()),
        Err(err) => {
            error!("SpriteManager: {} {:?}", cfg_path, err);
            AtlasConfig::default()
        }
    };

    let texture = asset_server.load(image_path);
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(config.cell_width, config.cell_height),
        config.columns,
        config.rows,
        None,
        None,
    );

    commands.insert_resource(SpriteManager {
        texture: texture,
        texture_atlas_layouts: texture_atlas_layouts.add(layout),
        config,
        stomach_icon: asset_server.load("textures/stomach_icon.png"),
    });
}

impl SpriteManager {
    pub fn get_sprite_by_name(&self, name: &str) -> Sprite {
        return self.get_sprite_by_name_and_size(name, Vec2::new(64.0, 64.0));
    }

    pub fn get_sprite_by_name_and_size(&self, name: &str, size: Vec2) -> Sprite {
        let cfg = self.config.sprites_map.get(name).unwrap();
        Sprite {
            image: self.texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: self.texture_atlas_layouts.clone(),
                index: cfg.index,
            }),
            custom_size: Some(size),
            ..Default::default()
        }
    }

    pub fn create_image_node_by_name(&self, name: &str) -> ImageNode {
        let cfg = self.config.sprites_map.get(name).unwrap();
        ImageNode::from_atlas_image(
            self.texture.clone(),
            TextureAtlas {
                layout: self.texture_atlas_layouts.clone(),
                index: cfg.index,
            },
        )
    }
}
