//! 场景系统实现

use crate::{
    core::{
        HexGridConfig,
        camera::CameraController,
        components::Player,
        hex_grid::{HexMapPosition, SpatialPartition},
    },
    level::{config::LevelConfigAsset, loader::*},
};
use bevy::prelude::*;
use bevy_egui::PrimaryEguiContext;

#[derive(Component)]
#[require(Visibility::default())]
pub struct GameSceneRoot;

#[derive(Component)]
pub struct GameSceneUIRoot;

/// 初始化测试场景
pub fn setup_game_scene(
    mut commands: Commands,
    loader: ResMut<LevelLoader>,
    level_data: Res<Assets<LevelConfigAsset>>,
    // sprite_manager: ResMut<SpriteManager>,
) {
    info!("Setup game scene");

    let cfg = level_data.get(&loader.level_data).unwrap();
    let config = HexGridConfig::new(50.0, cfg.size.x as usize, cfg.size.y as usize, 0.0);
    let partition = SpatialPartition::new(config.clone());
    let center = partition.grid_to_world(&cfg.startup_camera_pos.unwrap_or_default());

    commands.insert_resource(config);
    commands.insert_resource(partition);

    // 摄像机
    commands.spawn((
        GameSceneRoot,
        PrimaryEguiContext,
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

    commands.spawn((
        GameSceneUIRoot,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::Flex,
            ..Default::default()
        },
        GlobalZIndex(1),
    ));
}

/// 生成玩家实体
pub fn spawn_player(mut commands: Commands) {
    commands.spawn((HexMapPosition::new(0, 0), Player));
}

pub fn despawn_scene(
    mut commands: Commands,
    query: Query<Entity, With<GameSceneRoot>>,
    ui_query: Query<Entity, With<GameSceneUIRoot>>,
) {
    if let Ok(entity) = query.single() {
        commands.entity(entity).despawn();
    }

    if let Ok(entity) = ui_query.single() {
        commands.entity(entity).despawn();
    }
}
