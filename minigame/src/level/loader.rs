use super::config::LevelConfig;
use crate::level::systems::spawn_entity;
use bevy::asset::LoadState;
use bevy::prelude::*;

#[derive(Asset, TypePath, Debug)]
pub struct LevelData {
    pub config: LevelConfig,
}

/// 关卡加载器资源
#[derive(Default, Resource)]
pub struct LevelLoader {
    pub current_level: Option<String>,
    pub loading: bool,
}

/// 关卡加载系统
pub fn load_level_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut level_loader: ResMut<LevelLoader>,
) {
    if let Some(level_name) = level_loader.current_level.clone() {
        if level_loader.loading {
            return;
        }

        level_loader.loading = true;

        // 异步加载关卡文件
        let level_path = format!("levels/{}.json", level_name);
        let handle = asset_server.load::<LevelData>(&level_path);

        // 在主线程处理加载结果
        let load_state = asset_server.get_load_state(&handle);
        match load_state {
            Some(LoadState::Loaded) => {
                // 直接加载并解析关卡配置文件
                if let Ok(level_str) = std::fs::read_to_string(&level_path) {
                    if let Ok(level_config) = serde_json::de::from_str::<LevelConfig>(&level_str) {
                        // 生成关卡实体
                        for entity_config in &level_config.entities {
                            // 实体生成逻辑
                            spawn_entity(&mut commands, entity_config);
                        }
                    }
                }
            }
            Some(LoadState::Failed(err)) => {
                error!("Failed to load level {}: {:?}", level_name, err);
            }
            _ => {}
        }

        level_loader.loading = false;
        level_loader.current_level = None;
    }
}
