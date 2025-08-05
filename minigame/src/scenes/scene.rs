//! 场景系统实现

use crate::{
    core::{
        HexGridConfig,
        camera::CameraController,
        components::Player,
        hex_grid::{Position, grid_to_world},
    },
    level::{config::LevelConfigAsset, loader::*},
};
use bevy::prelude::*;

#[derive(Component)]
pub struct GameSceneRoot;

/// 初始化测试场景
pub fn setup_game_scene(
    mut commands: Commands,
    loader: ResMut<LevelLoader>,
    level_data: Res<Assets<LevelConfigAsset>>,
    // sprite_manager: ResMut<SpriteManager>,
) {
    info!("Setup game scene");

    let cfg = level_data.get(&loader.level_data).unwrap();
    commands.insert_resource(HexGridConfig::new(
        50.0,
        cfg.size.x as usize,
        cfg.size.y as usize,
        0.0,
    ));

    let center = grid_to_world(cfg.startup_camera_pos.unwrap_or_default().into(), 50.0);

    // 摄像机
    commands.spawn((
        GameSceneRoot,
        Transform::default(),
        children![(
            Camera2d::default(),
            Camera {
                hdr: true,
                ..default()
            },
            bevy::core_pipeline::tonemapping::Tonemapping::TonyMcMapface,
            bevy::core_pipeline::bloom::Bloom::default(),
            Transform::from_translation(center),
            CameraController::default(),
        )],
    ));
}

/// 生成玩家实体
pub fn spawn_player(mut commands: Commands) {
    commands.spawn((Position::new(0, 0), Player));
}

pub fn despawn_scene(mut commands: Commands, query: Query<Entity, With<GameSceneRoot>>) {
    if let Ok(entity) = query.single() {
        commands.entity(entity).despawn();
    }
}
