//! 场景系统实现

use crate::core::components::{
    EnergyStore, EntityType, Hunger, MoveTo, Player, Position, Render, Species,
};
use bevy::prelude::Camera3d;
use bevy::prelude::*;

/// 初始化测试场景
pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 摄像机
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 20.0, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
        Visibility::default(),
    ));

    // 测试实体 - 可移动狐狸
    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::new(1.0, 1.0)),
            color: Color::srgb(1.0, 0.0, 0.0),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(5.0, 5.0, 0.0)),
        // 材质组件
        Position { x: 5, y: 5 },
        EntityType::Animal(Species::Fox),
        EnergyStore {
            value: 100.0,
            max: 100.0,
        },
        Hunger {
            value: 50.0,
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
