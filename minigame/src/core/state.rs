use bevy::prelude::*;

/// 游戏状态枚举
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    /// 应用启动状态，加载全局配置
    PrepareApp,
    #[default]
    /// 关卡选择状态
    MainMenu,
    /// 数据加载，关卡创建状态
    LevelLoading,
    /// 游戏进行中状态
    Playing,
    /// 游戏暂停状态
    Paused,
    /// 游戏结束状态
    GameOver,
}
