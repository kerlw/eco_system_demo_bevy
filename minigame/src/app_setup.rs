//! 应用初始化和系统配置

use bevy::prelude::*;
use crate::core::{
    components::Player,
    systems::{
        debug, grid, movement, 
        scene, state_machine
    },
};

/// 创建应用并配置系统
pub fn create_app() -> App {
    let mut app = App::new();
    
    // 基础插件
    app.add_plugins(DefaultPlugins)
        .init_resource::<grid::HexGridConfig>()
        .insert_resource(grid::HexGridConfig::new(1.0, 10, 10, 5.0))
        .add_systems(Startup, |config: Res<grid::HexGridConfig>| {
            debug!("HexGridConfig initialized: width={}, height={}, size={}", 
                config.width, config.height, config.size);
        });

    // 初始场景
    app.add_systems(Startup, (
        scene::spawn_camera,
        scene::spawn_grid_entities,
        scene::spawn_player,
    ));

    // 主游戏循环
    app.add_systems(Update, (
        debug::debug_position_system,
        grid::render_grid_system,
        movement::movement_systems(),
        state_machine::state_machine_systems(),
    ));

    app
}