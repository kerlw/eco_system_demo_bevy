use crate::{
    core::GameState,
    level::{config::LevelConfigAsset, loader::LevelLoader},
};
use bevy::{asset::AssetLoadFailedEvent, prelude::*};

// 接收到LevelConfigAsset类型资源被加载事件时触发
pub fn on_level_config_load_event(
    mut loader: ResMut<LevelLoader>,
    mut events: EventReader<AssetEvent<LevelConfigAsset>>,
    // level_data: Res<Assets<LevelConfigAsset>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for event in events.read() {
        info!("Got level data event: {:?}", event);
        // if let AssetEvent::Added { id: _ } = event.is_added(asset_id)
        //     && check_level_data(&loader, &level_data) {
        // 这样写更简洁，效果是一样的
        if event.is_added(loader.level_data.id()) {
            loader.loading = false;
            game_state.set(GameState::Playing);
            break;
        }
    }
}

// 接收到LevelConfigAsset类型资源加载失败事件时触发
pub fn on_level_config_load_failed_event(
    mut events: EventReader<AssetLoadFailedEvent<LevelConfigAsset>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut loader: ResMut<LevelLoader>,
) {
    for event in events.read() {
        info!("Got level data event: {:?}", event);
        if event.id.eq(&loader.level_data.id()) {
            loader.loading = false;

            game_state.set(GameState::MainMenu);
            break;
        }
    }
}

// // 检查loader中指向的关卡数据是否被加载成功
// fn check_level_data(
//     loader: &ResMut<LevelLoader>,
//     level_data: &Res<Assets<LevelConfigAsset>>,
// ) -> bool {
//     let data = level_data.get(&loader.level_data);
//     return data.is_some();
// }
