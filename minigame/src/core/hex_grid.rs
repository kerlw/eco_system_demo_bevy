//! 六边形网格系统实现

use bevy::prelude::*;
use super::components::Position;

/// 六边形网格配置
#[derive(Resource, Clone)]
pub struct HexGridConfig {
    pub size: f32,      // 六边形边长
    pub width: usize,   // 网格宽度(列数)
    pub height: usize,  // 网格高度(行数)
    pub move_speed: f32, // 默认移动速度
}

impl HexGridConfig {
    pub fn new(size: f32, width: usize, height: usize, move_speed: f32) -> Self {
        Self {
            size,
            width,
            height,
            move_speed
        }
    }
}

/// 将网格坐标转换为世界坐标
pub fn grid_to_world(pos: Position, config: &HexGridConfig) -> Vec3 {
    let x = config.size * 1.5 * pos.x as f32;
    let z = config.size * f32::sqrt(3.0) * (pos.y as f32 + 0.5 * (pos.x as f32 % 2.0));
    Vec3::new(x, z, 0.0)
}

/// 计算两个六边形之间的距离
pub fn hex_distance(a: Position, b: Position) -> i32 {
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    (dx.abs() + (dx + dy).abs() + dy.abs()) / 2
}

/// 检查位置是否在网格范围内
pub fn is_valid_position(pos: Position, config: &HexGridConfig) -> bool {
    pos.x >= 0 && pos.x < config.width as i32 &&
    pos.y >= 0 && pos.y < config.height as i32
}

/// 空间分区系统
#[derive(Component)]
pub struct SpatialPartition {
    partitions: Vec<Vec<Entity>>,
    config: HexGridConfig,
}

impl SpatialPartition {
    pub fn new(config: HexGridConfig) -> Self {
        let capacity = config.width * config.height;
        let mut partitions = Vec::with_capacity(capacity);
        partitions.resize_with(capacity, Vec::new);
        
        Self { partitions, config }
    }

    /// 获取分区索引
    fn get_index(&self, pos: Position) -> usize {
        (pos.y as usize * self.config.width) + pos.x as usize
    }

    /// 添加实体到分区
    pub fn insert(&mut self, entity: Entity, pos: Position) {
        let index = self.get_index(pos);
        self.partitions[index].push(entity);
    }

    /// 查询附近实体
    pub fn query(&self, center: Position, radius: i32) -> Vec<Entity> {
        let mut results = Vec::new();
        
        for dy in -radius..=radius {
            for dx in -radius..=radius {
                if hex_distance(center, Position { x: center.x + dx, y: center.y + dy }) <= radius {
                    let pos = Position { x: center.x + dx, y: center.y + dy };
                    if pos.x >= 0 && pos.x < self.config.width as i32 
                        && pos.y >= 0 && pos.y < self.config.height as i32 {
                        
                        let index = self.get_index(pos);
                        results.extend(&self.partitions[index]);
                    }
                }
            }
        }
        
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::Entity;

    #[test]
    fn test_hex_distance() {
        let a = Position { x: 0, y: 0 };
        let b = Position { x: 3, y: 2 };
        assert_eq!(hex_distance(a, b), 3);
    }

    #[test]
    fn test_spatial_partition() {
        let config = HexGridConfig { size: 1.0, width: 10, height: 10, move_speed: 1.0 };
        let mut partition = SpatialPartition::new(config);
        let entity = Entity::from_raw(0);
        
        partition.insert(entity, Position { x: 5, y: 5 });
        let results = partition.query(Position { x: 5, y: 5 }, 2);
        
        assert!(results.contains(&entity));
    }
}