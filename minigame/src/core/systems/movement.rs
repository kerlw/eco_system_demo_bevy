//! 实体移动系统实现
use super::super::components::{EnergyStore, MoveTo, VisionRange};
use super::super::hex_grid::{HexGridConfig, Position, hex_distance, is_valid_position};
use crate::core::state::GameState;
use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum MovementSystemSet {
    Pathfinding,
    Movement,
}

pub fn configure_movement_sets(app: &mut App) {
    app.configure_sets(
        Update,
        (MovementSystemSet::Pathfinding, MovementSystemSet::Movement)
            .chain()
            .run_if(in_state(GameState::Playing)),
    );
}

pub fn register_movement_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            pathfinding_system.in_set(MovementSystemSet::Pathfinding),
            update_paths.in_set(MovementSystemSet::Movement),
            movement_system.in_set(MovementSystemSet::Movement),
        ),
    );
}

/// 寻路系统
pub fn pathfinding_system(
    mut query: Query<(&Position, &mut MoveTo, Option<&VisionRange>)>,
    _config: Res<HexGridConfig>,
) {
    for (current_pos, mut move_to, vision_range) in &mut query {
        if move_to.path.is_empty() || hex_distance(*current_pos, move_to.target) > 1 {
            // 检查目标是否在视野范围内
            if let Some(vision_range) = vision_range {
                if hex_distance(*current_pos, move_to.target) > vision_range.radius {
                    continue;
                }
            }
            // 简单直线移动作为临时实现
            // TODO: 实现完整A*寻路算法
            move_to.path = vec![move_to.target];
        }
    }
}

/// 路径更新系统
pub fn update_paths(query: Query<(&Position, &MoveTo), Changed<Position>>) {
    for (_position, _move_to) in query.iter() {
        // 路径更新逻辑
        // 当位置变化时更新路径
    }
}

/// 移动执行系统
#[allow(unused_variables)]
pub fn movement_system(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut Position,
        &mut MoveTo,
        Option<&mut EnergyStore>,
        Option<&VisionRange>,
    )>,
    time: Res<Time>,
    config: Res<HexGridConfig>,
) {
    for (entity, mut position, mut move_to, mut energy, vision_range) in &mut query {
        if let Some(next_pos) = move_to.path.first() {
            // 检查能量是否足够（如果有能量组件）
            if let Some(ref mut energy) = energy {
                if energy.value <= 0.0 {
                    commands.entity(entity).remove::<MoveTo>();
                    continue;
                }

                // 消耗能量
                energy.value -= (0.5 * energy.max) * time.delta_secs();
            }

            // 检查位置有效性
            if !is_valid_position(*next_pos, &config) {
                commands.entity(entity).remove::<MoveTo>();
                continue;
            }

            // 检查目标是否在视野范围内
            if let Some(vision_range) = vision_range {
                if hex_distance(*position, move_to.target) > vision_range.radius {
                    commands.entity(entity).remove::<MoveTo>();
                    continue;
                }
            }

            // 更新位置
            *position = *next_pos;

            // 如果到达目标点，清除路径
            if *position == move_to.target {
                move_to.path.clear();
            }
        }
    }
}
