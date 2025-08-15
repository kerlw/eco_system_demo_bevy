use bevy::ecs::relationship::RelationshipSourceCollection;
use bevy::math::ops::floor;
use bevy::platform::collections::HashSet;
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::Material2d;
use bevy_egui::egui::ahash::{HashMap, HashMapExt};

use crate::core::components::EntityType;

/// 六边形网格坐标, x,y为奇行偏移坐标，q,r,s为立方体坐标
#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HexMapPosition {
    pub x: i32,
    pub y: i32,
    pub q: i32,
    pub r: i32,
    pub s: i32,
}

impl HexMapPosition {
    pub fn new(x: i32, y: i32) -> Self {
        let q = x - (y - (y & 1)) / 2; // 奇行偏移修正
        let r = y;
        Self {
            x,
            y,
            q,
            r,
            s: -q - r,
        }
    }

    pub fn to_vec2(&self) -> IVec2 {
        IVec2::new(self.x, self.y)
    }

    pub fn cube_coord(&self) -> IVec3 {
        IVec3::new(self.q, self.r, self.s)
    }

    pub fn add_cube_coord(&mut self, coord: &IVec3) -> HexMapPosition {
        self.q += coord.x;
        self.r += coord.y;
        self.s += coord.z;
        self.x = self.q + (self.r - (self.r & 1)) / 2;
        self.y = self.r;
        *self
    }

    pub fn move_towards(&mut self, target: &HexMapPosition, speed: f32, _config: &HexGridConfig) {
        // 基于六边形网格的移动逻辑
        let dx = (target.x - self.x).clamp(-1, 1);
        let dy = (target.y - self.y).clamp(-1, 1);

        self.x += dx * speed as i32;
        self.y += dy * speed as i32;
    }
}

impl From<IVec2> for HexMapPosition {
    fn from(pos: IVec2) -> Self {
        let q = pos.x - (pos.y - (pos.y & 1)) / 2; // 奇行偏移修正
        let r = pos.y;
        Self {
            x: pos.x as i32,
            y: pos.y as i32,
            q,
            r,
            s: -q - r,
        }
    }
}

/// 六边形网格配置
#[derive(Debug, Resource, Clone)]
pub struct HexGridConfig {
    pub size: f32,       // 六边形边长
    pub width: usize,    // 网格宽度(列数)
    pub height: usize,   // 网格高度(行数)
    pub move_speed: f32, // 默认移动速度
}

impl HexGridConfig {
    pub fn new(size: f32, width: usize, height: usize, move_speed: f32) -> Self {
        Self {
            size,
            width,
            height,
            move_speed,
        }
    }
}

pub fn world_to_grid(_pos: &Vec3, _hex_size: f32) -> HexMapPosition {
    HexMapPosition::default()
}

/// 计算两个六边形之间的距离
pub fn hex_distance(a: &HexMapPosition, b: &HexMapPosition) -> i32 {
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    (dx.abs() + (dx + dy).abs() + dy.abs()) / 2
}

// 立方体坐标的6个方向向量 [2,4](@ref)
pub const CUBE_DIRECTIONS: [IVec3; 6] = [
    IVec3::new(1, -1, 0), // 右 → 东北
    IVec3::new(1, 0, -1), // 右上 → 东
    IVec3::new(0, 1, -1), // 左上 → 西北
    IVec3::new(-1, 1, 0), // 左 → 西南
    IVec3::new(-1, 0, 1), // 左下 → 西
    IVec3::new(0, -1, 1), // 右下 → 东南
];

// pub fn get_neighbours(pos: &HexMapPosition) -> Vec<HexMapPosition> {
//     CUBE_DIRECTIONS
//         .iter()
//         .map(|dir| pos.clone().add_cube_coord(dir))
//         .collect()
// }

#[derive(Debug, Resource, Clone, Eq, PartialEq, Hash)]
pub struct EntityWithCoord {
    pub entity: Entity,
    pub pos: HexMapPosition,
}

/// 空间分区系统
#[derive(Debug, Resource)]
pub struct SpatialPartition {
    pub cell_entity: Vec<Entity>,
    pub ground_entities: Vec<HashSet<Entity>>, //在此格内的地表实体
    pub other_entities: Vec<HashSet<Entity>>,  //在此格内的实体
    pub entities_map: HashMap<EntityType, HashSet<EntityWithCoord>>,
    pub config: HexGridConfig,
}

const SQRT3: f32 = 1.7320508;

impl SpatialPartition {
    pub fn new(config: HexGridConfig) -> Self {
        let capacity = config.width * config.height;
        let mut partitions = Vec::with_capacity(capacity);
        partitions.resize_with(capacity, HashSet::new);

        Self {
            cell_entity: vec![Entity::PLACEHOLDER; capacity],
            ground_entities: partitions.clone(),
            other_entities: partitions,
            entities_map: HashMap::new(),
            config,
        }
    }

    /// 检查位置是否在网格范围内
    pub fn is_valid_position(&self, pos: &HexMapPosition) -> bool {
        pos.x >= 0
            && pos.x < self.config.width as i32
            && pos.y >= 0
            && pos.y < self.config.height as i32
    }

    pub fn is_obstacle(&self, _pos: &HexMapPosition) -> bool {
        //TODO : 实体检查
        return false;
    }

    pub fn check_entity_conflict_by_pos(
        &self,
        entity_type: EntityType,
        pos: &HexMapPosition,
    ) -> bool {
        let index = self.get_index(pos);
        match entity_type {
            EntityType::Cell => false,
            EntityType::Grass => self.ground_entities[index].is_empty(),
            _ => self.other_entities[index].is_empty(),
        }
    }

    pub fn get_cell_by_pos(&self, pos: &HexMapPosition) -> Entity {
        self.cell_entity[self.get_index(pos)]
    }

    pub fn get_valid_neighbours(&self, pos: &HexMapPosition) -> Vec<HexMapPosition> {
        CUBE_DIRECTIONS
            .iter()
            .map(|dir| pos.clone().add_cube_coord(dir))
            .filter(|pos| self.is_valid_position(pos) && !self.is_obstacle(pos))
            .collect()
    }

    pub fn remove_entity(&mut self, entity: Entity, pos: &HexMapPosition, entity_type: EntityType) {
        let index = self.get_index(pos);
        self.entities_map.get_mut(&entity_type).map(|entities| {
            entities.remove(&EntityWithCoord {
                entity,
                pos: pos.clone(),
            });
        });
        match entity_type {
            EntityType::Cell => {}
            EntityType::Grass => {
                self.ground_entities[index].remove(&entity);
            }
            _ => {
                self.other_entities[index].remove(&entity);
            }
        }
    }

    /// 获取分区索引
    fn get_index(&self, pos: &HexMapPosition) -> usize {
        (pos.y as usize * self.config.width) + pos.x as usize
    }

    /// 将网格坐标转换为世界坐标
    pub fn grid_to_world(&self, pos: &IVec2) -> Vec3 {
        let hex_size = self.config.size;
        let x = hex_size * SQRT3 * (pos.x as f32 + 0.5 * (pos.y as f32 % 2.0));
        let y = hex_size * 1.5 * pos.y as f32;
        Vec3::new(x, y, 0.0)
    }

    pub fn world_to_grid(&self, pos: &Vec2) -> HexMapPosition {
        let hex_size = self.config.size;
        let x = floor((pos.x + SQRT3 * hex_size * 0.5) / (hex_size * SQRT3)) as i32;
        let y = floor((pos.y + hex_size * 0.5) / (hex_size * 1.5)) as i32;
        let mut result = HexMapPosition::new(x, y);
        let mut distance = self
            .grid_to_world(&result.to_vec2())
            .xy()
            .distance(pos.clone());

        let center = result.clone();
        for offset in CUBE_DIRECTIONS {
            let neighbour = center.clone().add_cube_coord(&offset);
            let new_distance = self
                .grid_to_world(&neighbour.to_vec2())
                .xy()
                .distance(pos.clone());
            if new_distance < distance {
                distance = new_distance;
                result = neighbour;
            }
        }

        return result;
    }

    /// 添加实体到分区
    pub fn insert_cache_entity(
        &mut self,
        entity: Entity,
        pos: &HexMapPosition,
        entity_type: EntityType,
    ) {
        let index = self.get_index(pos);
        match entity_type {
            EntityType::Cell => {
                self.cell_entity[index] = entity;
            }
            EntityType::Grass => {
                self.ground_entities[index].insert(entity);
                self.entities_map
                    .entry(entity_type)
                    .or_insert_with(|| HashSet::new())
                    .insert(EntityWithCoord {
                        entity,
                        pos: pos.clone(),
                    });
            }
            _ => {
                self.other_entities[index].insert(entity);
                self.entities_map
                    .entry(entity_type)
                    .or_insert_with(|| HashSet::new())
                    .insert(EntityWithCoord {
                        entity,
                        pos: pos.clone(),
                    });
            }
        }
    }

    pub fn entities_at(&self, pos: &HexMapPosition) -> Vec<Entity> {
        let index = self.get_index(pos);
        let mut result = self.ground_entities[index].clone();
        result.extend(self.other_entities[index].clone());
        return result.into_iter().collect::<Vec<_>>();
    }

    pub fn entities_by_type(&self, entity_type: &EntityType) -> Vec<EntityWithCoord> {
        self.entities_map
            .get(entity_type)
            .map_or(Vec::new(), |e| e.clone().into_iter().collect::<Vec<_>>())
    }

    /// 查询附近实体
    pub fn query(&self, center: HexMapPosition, radius: i32) -> Vec<Entity> {
        let mut results = Vec::new();

        for dy in -radius..=radius {
            for dx in -radius..=radius {
                if hex_distance(&center, &HexMapPosition::new(center.x + dx, center.y + dy))
                    <= radius
                {
                    let pos = HexMapPosition::new(center.x + dx, center.y + dy);
                    if pos.x >= 0
                        && pos.x < self.config.width as i32
                        && pos.y >= 0
                        && pos.y < self.config.height as i32
                    {
                        let index = self.get_index(&pos);
                        results.extend(&self.other_entities[index]);
                    }
                }
            }
        }

        results
    }
}

// /// 六边形网格单元组件
// #[derive(Component)]
// pub struct HexCell {
//     pub hex: HexCoord,
// }

// 自定义边框着色器
#[derive(AsBindGroup, Debug, Clone, Asset, TypePath)]
pub struct HexagonBorderMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
    #[uniform(0)]
    pub border_color: LinearRgba,
    #[uniform(0)]
    pub border_width: f32,
}

impl Material2d for HexagonBorderMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/hexagon_border.wgsl".into()
    }

    // fn vertex_shader() -> ShaderRef {
    //     "shaders/hexagon_border.wgsl".into()
    // }
}

/// 创建可复用的六边形网格
pub fn create_hex_mesh(mut meshes: ResMut<Assets<Mesh>>, size: f32) -> Handle<Mesh> {
    let mesh = Mesh::from(RegularPolygon::new(size, 6));
    // mesh.rotate_by(Quat::from_rotation_z(std::f32::consts::FRAC_PI_6)); // 平顶布局需要旋转30度
    // 注意，旋转mesh会同时旋转了uv，所以计算时应当以尖顶六边形的uv来计算
    meshes.add(mesh)
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::Entity;

    #[test]
    fn test_hex_distance() {
        let a = HexMapPosition::new(0, 0);
        let b = HexMapPosition::new(3, 2);
        assert_eq!(hex_distance(&a, &b), 3);
    }

    #[test]
    fn test_spatial_partition() {
        let config = HexGridConfig {
            size: 1.0,
            width: 10,
            height: 10,
            move_speed: 1.0,
        };
        let mut partition = SpatialPartition::new(config);
        let entity = Entity::from_raw(0);

        partition.insert_cache_entity(entity, &HexMapPosition::new(5, 5), EntityType::Rabbit);
        let results = partition.query(HexMapPosition::new(5, 5), 2);

        assert!(results.contains(&entity));
    }
}
