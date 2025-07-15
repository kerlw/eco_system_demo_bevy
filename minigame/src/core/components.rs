//! 核心ECS组件定义

use bevy::prelude::*;
use crate::core::hex_grid::HexGridConfig;

/// 六边形网格坐标
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn move_towards(&mut self, target: &Position, speed: f32, _config: &HexGridConfig) {
        // 基于六边形网格的移动逻辑
        let dx = (target.x - self.x).clamp(-1, 1);
        let dy = (target.y - self.y).clamp(-1, 1);
        
        self.x += dx * speed as i32;
        self.y += dy * speed as i32;
    }
}

/// 渲染信息
#[derive(Component)]
pub struct Render {
    pub mesh: Handle<Mesh>,
    pub material: Handle<ColorMaterial>,
}

/// 实体类型标记
#[derive(Component, Debug)]
pub enum EntityType {
    Animal(Species),
    Plant,
}

/// 动物种类
#[derive(Debug, Clone, Copy)]
pub enum Species {
    Fox,
    Rabbit,
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
}

/// 玩家标记
#[derive(Component, Debug)]
pub struct Player;

/// 移动目标
#[derive(Component, Debug)]
pub struct MoveTo {
    pub target: Position,
    pub path: Vec<Position>,
    pub speed: f32,
}

/// 空间分区
#[derive(Component, Debug)]
pub struct SpatialPartition {
    pub grid: Vec<Vec<Option<Entity>>>,
}