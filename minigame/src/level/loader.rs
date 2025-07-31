use super::config::LevelConfigAsset;
use crate::core::GameState;
use bevy::asset::LoadState;
use bevy::prelude::*;

/// 关卡加载器资源
#[derive(Default, Resource)]
pub struct LevelLoader {
    pub current_level: Option<String>,
    pub loading: bool,
    pub level_data: Handle<LevelConfigAsset>,
}

/// 关卡加载系统
pub fn load_level_system(
    asset_server: Res<AssetServer>,
    mut level_loader: ResMut<LevelLoader>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if let Some(level_name) = level_loader.current_level.clone() {
        if level_loader.loading {
            return;
        }

        level_loader.loading = true;

        // 异步加载关卡文件
        let level_path = format!("levels/{}.lvc", level_name);
        level_loader.level_data = asset_server.load::<LevelConfigAsset>(&level_path);

        // 在主线程处理加载结果
        let load_state = asset_server.get_load_state(&level_loader.level_data);
        match load_state {
            Some(LoadState::Loaded) => {
                info!("Loaded level {}", level_name);
            }
            Some(LoadState::Failed(err)) => {
                error!("Failed to load level {}: {:?}", level_name, err);
                game_state.set(GameState::MainMenu);
            }
            _ => {}
        }

        level_loader.loading = false;
        level_loader.current_level = None;
    }
}
