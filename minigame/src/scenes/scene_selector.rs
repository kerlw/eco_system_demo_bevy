use bevy::{asset::AssetLoadFailedEvent, prelude::*};
use bevy_behave::prelude::BehaveCtx;

use crate::{
    ai::*,
    core::{
        GameState,
        entities::{spawn_entities_system, spawn_satiety_pbar_onadd},
        render_grid_system, setup_grid,
    },
    level::{
        config::{LevelConfigAsset, LevelConfigAssetLoader},
        loader::load_level_system,
    },
    scenes::{game_loading::*, *},
    ui::spawn_card_ui,
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
            SceneSystemSet::LoadingSystem.run_if(in_state(GameState::LevelLoading)),
        )
        .configure_sets(
            Update,
            SceneSystemSet::GameSystems.run_if(in_state(GameState::Playing)),
        )
        .configure_sets(
            FixedUpdate,
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
            .add_systems(OnEnter(GameState::LevelLoading), load_level_system)
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
                (
                    setup_game_scene,
                    setup_grid,
                    render_grid_system,
                    // pre_spawn_entities_system,
                    spawn_entities_system,
                    spawn_card_ui,
                    spawn_satiety_pbar_onadd,
                )
                    .chain(),
            )
            //以下是AI控制部分的系统注册
            .add_systems(
                FixedUpdate,
                (
                    udpate_board_state_system,
                    idle_action_system,
                    forage_action_system,
                )
                    .chain()
                    .in_set(SceneSystemSet::GameSystems),
            )
            .add_systems(Update, render_gizmos.in_set(SceneSystemSet::GameSystems))
            // .add_observer(onadd_idle_action)
            // .add_observer(on_new_behaviour)
            // 退出Playing状态的系统注册
            .add_systems(OnExit(GameState::Playing), despawn_scene);
    }
}

#[allow(unused)]
fn on_new_behaviour(
    trigger: Trigger<OnAdd, BehaveCtx>,
    q: Query<(Entity, Option<&Name>, &BehaveCtx)>,
) {
    if let Ok((entity, name, ctx)) = q.get(trigger.target()) {
        info!("New behaviour spawned {entity} {ctx} = {name:?}");
    }
}
