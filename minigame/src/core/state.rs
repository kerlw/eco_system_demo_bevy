use bevy::prelude::*;

/// 游戏状态枚举
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    /// 关卡选择状态
    MainMenu,
    /// 数据加载，关卡创建状态
    GameLoading,
    /// 游戏进行中状态
    Playing,
    /// 游戏暂停状态
    Paused,
    /// 游戏结束状态
    GameOver,
}
