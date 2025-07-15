use bevy::prelude::*;

/// 游戏状态枚举
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States)]
pub enum GameState {
    /// 主菜单状态
    Menu,
    /// 游戏进行中状态
    Playing,
    /// 游戏暂停状态
    Paused,
    /// 游戏结束状态
    GameOver,
}