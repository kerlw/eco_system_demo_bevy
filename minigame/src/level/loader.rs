use bevy::prelude::*;
use serde::Deserialize;
use std::fs;

use crate::level::config::LevelConfig;

/// 加载关卡系统
pub struct LevelLoaderPlugin;

impl Plugin for LevelLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_levels);
    }
}

/// 从文件加载关卡配置
pub fn load_levels(asset_server: Res<AssetServer>) {
    // TODO: 实现关卡加载逻辑
    // 从assets/levels目录加载JSON配置文件
    // 解析为LevelConfig结构
}