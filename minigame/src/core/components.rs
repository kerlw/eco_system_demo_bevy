//! 核心ECS组件定义

use std::fmt;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::core::systems::hex_grid::HexMapPosition;

/// 渲染信息
#[derive(Component)]
pub struct Render {
    pub mesh: Handle<Mesh>,
    pub material: Handle<ColorMaterial>,
}

/// 实体类型标记
#[derive(Component, Debug, Deserialize, Serialize, Clone, Default, Eq, PartialEq, Hash)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type", content = "species")]
pub enum EntityType {
    Cell,
    #[default]
    Grass,
    Rabbit,
    Fox,
}

impl fmt::Display for EntityType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            EntityType::Cell => "Map_cell",
            EntityType::Grass => "Grass_normal",
            EntityType::Rabbit => "Rabbit",
            EntityType::Fox => "Fox",
        };
        write!(f, "{}", s)
    }
}

/// 能量存储
#[derive(Component, Debug)]
pub struct EnergyStore {
    pub value: f32,
    pub max: f32,
}

/// 饥饿状态
#[derive(Component, Debug)]
pub struct Hunger {
    pub value: f32,
    pub max: f32,
    pub is_searching: bool,
}

/// 生命值组件
#[derive(Component, Debug, Default)]
pub struct Health {
    pub value: Option<f32>,
}

/// 玩家标记
#[derive(Component, Debug)]
pub struct Player;

/// 移动目标
#[derive(Component, Debug)]
pub struct MoveTo {
    pub target: HexMapPosition,
    pub path: Vec<HexMapPosition>,
    pub speed: f32,
}

/// 视野范围
#[derive(Component, Debug)]
pub struct VisionRange {
    pub radius: i32,
}
