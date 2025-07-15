//! 网格系统实现

use super::super::components::Position;
use super::super::hex_grid::{HexGridConfig, SpatialPartition, grid_to_world};
use bevy::prelude::*;

/// 初始化网格
pub fn setup_grid(mut commands: Commands, config: Res<HexGridConfig>) {
    commands.spawn((
        Name::new("HexGrid"),
        SpatialPartition::new(config.into_inner().clone()),
    ));
}

/// 渲染网格系统
pub fn render_grid_system(config: Res<HexGridConfig>, mut gizmos: Gizmos) {
    for x in 0..config.width as i32 {
        for y in 0..config.height as i32 {
            let pos = Position { x, y };
            let center = grid_to_world(pos, &config);

            // 绘制六边形边框（使用更显眼的颜色）
            gizmos.linestrip(
                (0..=6).map(|i| {
                    let angle = std::f32::consts::TAU * i as f32 / 6.0;
                    center + Vec3::new(config.size * angle.cos(), 0.0, config.size * angle.sin())
                }),
                Color::srgb(0.0, 1.0, 0.0), // 使用亮绿色
            );
        }
    }
}
