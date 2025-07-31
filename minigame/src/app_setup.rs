//! 应用初始化和系统配置

use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy::sprite::Material2dPlugin;
use bevy::window::PrimaryWindow;
use bevy::winit::WinitSettings;
use minigame::core::camera::CameraControlPlugin;
use minigame::core::interaction::MapInteractionPlugin;
use minigame::core::state::GameState;
use minigame::core::systems::hex_grid::HexagonBorderMaterial;
use minigame::level::loader::LevelLoader;
use minigame::scenes::scene_selector::SceneSelectorPlugin;
use minigame::sprite::sprite_mgr::SpriteManagerPlugin;
use minigame::ui::cards::EntityCardsPlugin;
use minigame::ui::hud::HudPlugin;

fn close_window_on_esc(
    mut window_events: EventWriter<bevy::window::WindowCloseRequested>,
    window: Query<Entity, With<PrimaryWindow>>,
    state: Res<State<GameState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    // 主菜单界面时退出应用
    if state.get().eq(&GameState::MainMenu) {
        if let Ok(window_entity) = window.single() {
            window_events.write(bevy::window::WindowCloseRequested {
                window: window_entity,
            });
        }
    } else {
        // 非主菜单界面时退到主菜单界面
        game_state.set(GameState::MainMenu);
    }
}

/// 创建应用并配置系统
pub fn create_app() -> App {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::desktop_app())
        .add_plugins(Material2dPlugin::<HexagonBorderMaterial>::default())
        .insert_resource(LevelLoader::default())
        .add_plugins((SpriteManagerPlugin, SceneSelectorPlugin))
        .init_state::<GameState>()
        .add_plugins((
            CameraControlPlugin,
            HudPlugin,
            MapInteractionPlugin,
            EntityCardsPlugin,
        ))
        // .insert_resource(HexGridConfig::new(50.0, 50, 50, 5.0))
        // .add_systems(Startup, setup_sprite_res)
        .add_systems(
            Update,
            (
                minigame::core::systems::debug::debug_position_system,
                // minigame::core::systems::movement::movement_system,
                // minigame::core::systems::state_machine::state_machine_system,
                close_window_on_esc.run_if(input_just_pressed(KeyCode::Escape)),
            ),
        );

    app
}
