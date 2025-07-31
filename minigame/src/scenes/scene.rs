//! 场景系统实现

use crate::{
    core::{
        camera::CameraController,
        components::{EnergyStore, EntityType, MoveTo, Player, Species},
        hex_grid::Position,
    },
    level::{config::LevelConfigAsset, loader::*},
    sprite::sprite_mgr::SpriteManager,
};
use bevy::prelude::*;

/// 初始化测试场景
pub fn setup_game_scene(
    mut commands: Commands,
    mut loader: ResMut<LevelLoader>,
    level_data: Res<Assets<LevelConfigAsset>>,
    _meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<StandardMaterial>>,
    sprite_manager: ResMut<SpriteManager>,
) {
    info!("Setup game scene");
    // 摄像机
    commands.spawn((
        // Camera3d::default(),
        // Transform::from_xyz(0.0, 40.0, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
        // Visibility::default(),
        Camera2d::default(),
        CameraController::default(),
    ));

    // 测试实体 - 可移动狐狸
    commands.spawn((
        sprite_manager.get_sprite_by_name("fox"),
        /* {
            custom_size: Some(Vec2::new(10.0, 10.0)),12
            color: Color::srgb(1.0, 0.0, 0.0),
            ..Default::default()
        }, */
        Transform::from_translation(Vec3::new(0.0, 0.0, 2.0)),
        // 材质组件
        Position { x: 5, y: 5 },
        EntityType::Animal(Species::Fox),
        EnergyStore {
            value: 100.0,
            max: 100.0,
        },
        MoveTo {
            target: Position { x: 5, y: 5 },
            path: Vec::new(),
            speed: 1.0,
        },
    ));
}

/// 生成玩家实体
pub fn spawn_player(mut commands: Commands) {
    commands.spawn((Position { x: 0, y: 0 }, Player));
}
