use crate::{
    core::{HexGridConfig, components::EntityType, hex_grid::grid_to_world},
    level::{
        self,
        config::{EntityConfig, LevelConfigAsset},
        loader::LevelLoader,
    },
    sprite::sprite_mgr::SpriteManager,
};
use bevy::prelude::*;

/// 基础实体组件
#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

/// 生命状态组件
#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

/// 繁殖能力组件
#[derive(Component)]
pub struct Reproduction {
    pub rate: f32,
    pub cooldown: f32,
}

/// 草组件
#[derive(Component)]
pub struct Grass {
    pub growth_rate: f32,
    pub spread_range: i32,
}

/// 动物基础组件
#[derive(Component)]
pub struct Animal {
    pub hunger: f32,
    pub hunger_rate: f32,
    pub vision_range: i32,
    pub speed: f32,
}

/// 兔子特有组件
#[derive(Component)]
pub struct Rabbit;

/// 狐狸特有组件
#[derive(Component)]
pub struct Fox;

/// 实体生成配置
// pub struct EntityConfig {
//     pub entity_type: EntityType,
//     pub position: (i32, i32),
//     pub health: f32,
//     pub reproduction_rate: f32,
//     // 类型特定属性
//     pub growth_rate: Option<f32>,  // 草
//     pub hunger_rate: Option<f32>,  // 动物
//     pub vision_range: Option<i32>, // 动物
//     pub speed: Option<f32>,        // 动物
// }

/// 实体生成器
pub fn spawn_entity(
    commands: &mut Commands,
    config: &EntityConfig,
    sprite_manager: &ResMut<SpriteManager>,
    hex_config: &Res<HexGridConfig>,
) {
    let mut cmd = commands.spawn((
        sprite_manager.get_sprite_by_name("fox"),
        Transform::from_translation(grid_to_world(config.pos.into(), hex_config)),
    ));
    match (config.entity_type) {
        EntityType::Rabbit => {
            cmd.insert(Rabbit);
        }
        EntityType::Fox => {
            cmd.insert(Fox);
        }
        _ => {}
    };

    // // 添加基础组件
    // entity.insert_bundle((
    //     Position {
    //         x: config.position.0,
    //         y: config.position.1,
    //     },
    //     Health {
    //         current: config.health,
    //         max: config.health,
    //     },
    //     Reproduction {
    //         rate: config.reproduction_rate,
    //         cooldown: 0.0,
    //     },
    // ));

    // // 根据类型添加特定组件
    // match config.entity_type {
    //     EntityType::Grass => {
    //         entity.insert_bundle((Grass {
    //             growth_rate: config.growth_rate.unwrap_or(0.1),
    //             spread_range: 2,
    //         },));
    //     }
    //     EntityType::Rabbit => {
    //         entity.insert_bundle((
    //             Animal {
    //                 hunger: 0.0,
    //                 hunger_rate: config.hunger_rate.unwrap_or(0.1),
    //                 vision_range: config.vision_range.unwrap_or(5),
    //                 speed: config.speed.unwrap_or(1.0),
    //             },
    //             Rabbit,
    //             VisionRange {
    //                 radius: config.vision_range.unwrap_or(5),
    //             },
    //         ));
    //     }
    //     EntityType::Fox => {
    //         entity.insert_bundle((
    //             Animal {
    //                 hunger: 0.0,
    //                 hunger_rate: config.hunger_rate.unwrap_or(0.08),
    //                 vision_range: config.vision_range.unwrap_or(7),
    //                 speed: config.speed.unwrap_or(1.2),
    //             },
    //             Fox,
    //             VisionRange {
    //                 radius: config.vision_range.unwrap_or(7),
    //             },
    //         ));
    //     }
    // }

    // entity.id()
}

pub fn spawn_entities_system(
    mut commands: Commands,
    level_loader: Res<LevelLoader>,
    level_data: Res<Assets<LevelConfigAsset>>,
    sprite_manager: ResMut<SpriteManager>,
    hex_config: Res<HexGridConfig>,
) {
    let level_config = level_data.get(&level_loader.level_data).unwrap();
    for cfg in level_config.entities.iter() {
        spawn_entity(&mut commands, cfg, &sprite_manager, &hex_config);
    }
}
