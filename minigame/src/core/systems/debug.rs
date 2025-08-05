//! 调试系统实现

use super::super::components::EntityType;
use super::hex_grid::HexMapPosition;
use bevy::prelude::*;

/// 调试位置输出系统
pub fn debug_position_system(
    query: Query<(&HexMapPosition, &EntityType)>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        for (pos, entity_type) in &query {
            println!("Entity at ({}, {}) - Type: {:?}", pos.x, pos.y, entity_type);
        }
    }
}
