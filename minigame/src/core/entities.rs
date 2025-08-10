use std::time::Duration;

use crate::{
    ai::{AnimalActorBoard, EdibleEntity, FrameCounter, get_ai_behave_tree},
    core::{
        components::EntityType,
        hex_grid::{HexMapPosition, SpatialPartition},
    },
    level::{
        config::{EntityConfig, LevelConfigAsset},
        loader::LevelLoader,
    },
    scenes::GameSceneRoot,
    sprite::sprite_mgr::SpriteManager,
};
use bevy::prelude::*;
use bevy_behave::prelude::BehaveTree;

#[derive(Component)]
#[require(Visibility::default())]
pub struct OnMapEntitiesRoot;

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

// /// 草组件
// #[derive(Component)]
// #[require(Visibility)]
// pub struct Grass {
//     pub growth_rate: f32,
//     pub spread_range: i32,
// }

// /// 动物基础组件
// #[derive(Component)]
// pub struct Animal {
//     pub hunger: f32,
//     pub hunger_rate: f32,
//     pub vision_range: i32,
//     pub speed: f32,
// }

// /// 兔子特有组件
// #[derive(Component)]
// #[require(Visibility)]
// pub struct Rabbit;

// /// 狐狸特有组件
// #[derive(Component)]
// #[require(Visibility)]
// pub struct Fox;

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
    partition: &mut ResMut<SpatialPartition>,
    parent: &Entity,
) {
    let mut center = partition.grid_to_world(&config.pos);
    center.z = 2.0;

    info!(
        "spawn {:?} at ({:?}, {:?})",
        &config.entity_type, config.pos, center
    );

    let mut cmd = commands.spawn((
        sprite_manager.get_sprite_by_name(config.entity_type.to_string().to_lowercase().as_str()),
        Transform::from_translation(center),
        EdibleEntity::default(),
    ));

    // 以下代码给精灵添加头顶ui，但更新ui可能存在性能问题，后面再研究
    // children![(
    //         Sprite::from_color(Color::srgb(0.25, 0.25, 0.55), Vec2::new(100.0, 30.0)),
    //         Transform::from_translation(Vec3::Y * 50.0 + Vec3::Z * 3.0),
    //         children![(
    //             Text2d::new("animal_ui"),
    //             TextLayout::new(JustifyText::Left, LineBreak::AnyCharacter),
    //             // Wrap text in the rectangle
    //             TextBounds::from(Vec2::new(100.0, 30.0)),
    //             TextFont {
    //                 font_size: 16.0,
    //                 ..Default::default()
    //             },
    //             TextColor(WHITE.into()),
    //         )],
    //     )],

    match config.entity_type {
        EntityType::Rabbit => {
            // info!("spawn rabbit behave tree");
            let mut timer = Timer::from_seconds(1.0, TimerMode::Repeating);
            timer.tick(Duration::from_millis(1500));
            cmd.insert((
                AnimalActorBoard {
                    current_pos: HexMapPosition::from(config.pos),
                    move_cd_timer: timer,
                    entity_type: EntityType::Rabbit,
                    satiety: 5500,
                    decay_faction: 1.1, // TODO根据不同的动物类型配置不同的饱食度衰减
                    ..Default::default()
                },
                children![(
                    Name::new("rabbit behave_tree"),
                    BehaveTree::new(get_ai_behave_tree(EntityType::Rabbit)).with_logging(true),
                )],
            ));
        }
        EntityType::Fox => {
            // cmd.insert(Fox);
        }
        _ => {}
    };

    cmd.insert(ChildOf(*parent));

    partition.insert_cache_entity(cmd.id(), &config.pos.into(), config.entity_type.clone());

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
    mut partition: ResMut<SpatialPartition>,
    root: Query<Entity, With<GameSceneRoot>>,
) {
    commands.insert_resource(FrameCounter::default());
    let level_config = level_data.get(&level_loader.level_data).unwrap();

    let root_parent = root.single().unwrap();

    let parent = commands
        .spawn((OnMapEntitiesRoot, Transform::from_xyz(0.0, 0.0, 2.0)))
        .insert(ChildOf(root_parent))
        .id();

    for cfg in level_config.entities.iter() {
        spawn_entity(&mut commands, cfg, &sprite_manager, &mut partition, &parent);
    }
}
