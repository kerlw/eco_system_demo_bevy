use serde::Deserialize;
use crate::core::components::EntityType;

/// 关卡配置
#[derive(Debug, Deserialize)]
pub struct LevelConfig {
    pub name: String,
    pub width: i32,
    pub height: i32,
    pub entities: Vec<EntityConfig>,
}

/// 实体配置
#[derive(Debug, Deserialize, Clone)]
pub struct EntityConfig {
    #[serde(rename = "type")]
    pub entity_type: EntityType,
    pub x: i32,
    pub y: i32,
    pub health: Option<f32>,
    pub reproduction_rate: Option<f32>,
    pub growth_rate: Option<f32>,
    pub hunger_rate: Option<f32>,
    pub vision_range: Option<i32>,
    pub speed: Option<f32>,
}