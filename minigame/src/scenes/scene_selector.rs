use bevy::{asset::AssetLoadFailedEvent, prelude::*};

use crate::{
    core::{GameState, render_grid_system, setup_grid},
    level::{
        config::{LevelConfigAsset, LevelConfigAssetLoader},
        loader::load_level_system,
    },
    scenes::{game_loading::*, *},
};

pub struct SceneSelectorPlugin;

#[derive(SystemSet, Clone, Hash, Debug, PartialEq, Eq)]
pub enum SceneSystemSet {
    MenuSystems,
    LoadingSystem,
    GameSystems,
}

impl Plugin for SceneSelectorPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            SceneSystemSet::MenuSystems.run_if(in_state(GameState::MainMenu)),
        )
        .configure_sets(
            Update,
            SceneSystemSet::LoadingSystem.run_if(in_state(GameState::GameLoading)),
        )
        .configure_sets(
            Update,
            SceneSystemSet::GameSystems.run_if(in_state(GameState::Playing)),
        );

        app.init_asset::<LevelConfigAsset>()
            .init_asset_loader::<LevelConfigAssetLoader>()
            .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(
                Update,
                handle_level_button_interaction.in_set(SceneSystemSet::MenuSystems),
            )
            .add_systems(OnExit(GameState::MainMenu), despawn_level_selection_ui)
            .add_systems(OnEnter(GameState::GameLoading), load_level_system)
            .add_systems(
                Update,
                (
                    on_level_config_load_event.run_if(on_event::<AssetEvent<LevelConfigAsset>>),
                    on_level_config_load_failed_event
                        .run_if(on_event::<AssetLoadFailedEvent<LevelConfigAsset>>),
                )
                    .in_set(SceneSystemSet::LoadingSystem),
            )
            .add_systems(
                OnEnter(GameState::Playing),
                (setup_game_scene, setup_grid, render_grid_system).chain(),
            )
            .add_systems(OnExit(GameState::Playing), despawn_scene);
    }
}
