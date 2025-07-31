use crate::core::components::EntityType;
use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

/// 关卡配置
#[derive(Debug, Deserialize, Clone, Default)]
pub struct AtlasConfig {
    pub name: String,
    pub cell_width: u32,
    pub cell_height: u32,
    pub rows: u32,
    pub columns: u32,
    pub sprites: Vec<EntitySpriteConfig>,
    #[serde(skip)]
    pub sprites_map: HashMap<String, EntitySpriteConfig>,
}

/// 实体配置
#[derive(Debug, Deserialize, Clone)]
pub struct EntitySpriteConfig {
    pub name: String,
    #[serde(rename = "type")]
    pub entity_type: Option<EntityType>,
    pub index: usize,
}

impl AtlasConfig {
    pub fn from(json: &str) -> Self {
        let mut config = serde_json::from_str::<Self>(json).unwrap();
        for cfg in config.sprites.iter() {
            config.sprites_map.insert(cfg.name.clone(), cfg.clone());
        }
        return config;
    }
}
