use crate::core::components::EntityType;
use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// 关卡配置
#[derive(Asset, TypePath, Debug, Serialize, Deserialize)]
pub struct LevelConfigAsset {
    pub name: String,
    pub size: UVec2,
    pub startup_camera_pos: Option<UVec2>, // 初始相机位置，可选项
    pub entities: Vec<EntityConfig>,
}

#[derive(Default)]
pub struct LevelConfigAssetLoader;

/// Possible errors that can be produced by [`CustomAssetLoader`]
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum LevelConfigAssetLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    /// A [RON](ron) Error
    #[error("Could not parse RON: {0}")]
    RonSpannedError(#[from] ron::error::SpannedError),
}

/// 实体配置
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EntityConfig {
    #[serde(rename = "type")]
    pub entity_type: EntityType,
    pub pos: UVec2,
    pub health: Option<f32>,
    pub reproduction_rate: Option<f32>,
    pub growth_rate: Option<f32>,
    pub hunger_rate: Option<f32>,
    pub vision_range: Option<i32>,
    pub speed: Option<f32>,
}

impl AssetLoader for LevelConfigAssetLoader {
    type Asset = LevelConfigAsset;
    type Settings = ();
    type Error = LevelConfigAssetLoaderError;
    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let level_asset = ron::de::from_bytes::<LevelConfigAsset>(&bytes)?;
        Ok(level_asset)
    }

    fn extensions(&self) -> &[&str] {
        &["lvc"]
    }
}

#[cfg(test)]
mod tests {
    use crate::{core::components::EntityType, level::config::{EntityConfig, LevelConfigAsset}};
    use bevy::prelude::*;

    #[test]
    fn ron_ser() {
        let entity = EntityConfig {
            entity_type: EntityType::Grass,
            pos: UVec2::new(1, 1),
            health: None,
            reproduction_rate: None,
            growth_rate: None,
            hunger_rate: None,
            vision_range: None,
            speed: None,
        };
        let cfg = LevelConfigAsset {
            name: String::from("test"),
            size: UVec2::new(1, 1),
            startup_camera_pos: Some(UVec2 { x: 2, y: 2 }),
            entities: vec![entity.clone()],
        }; 
        println!("序列化字符串: {}", ron::ser::to_string(&cfg).unwrap());
        assert_eq!(
            "(type:(type:grass),pos:(1,1),health:None,reproduction_rate:None,growth_rate:None,hunger_rate:None,vision_range:None,speed:None)",
            ron::ser::to_string(&entity).unwrap()
        );
    }
}
