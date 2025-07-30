use bevy::prelude::*;
use bevy::sprite::Material2dPlugin;
use bevy::window::PrimaryWindow;
use minigame::core::camera::CameraControlPlugin;
use minigame::core::hex_grid::HexGridConfig;
use minigame::core::interaction::MapInteractionPlugin;
use minigame::core::systems::hex_grid::HexagonBorderMaterial;
use minigame::ui::cards::EntityCardsPlugin;
use minigame::ui::hud::HudPlugin;
use minigame::sprite::sprite_mgr::SpriteManagerPlugin;

fn close_window_on_esc(
    mut window_events: EventWriter<bevy::window::WindowCloseRequested>,
    mut keyboard_events: EventReader<bevy::input::keyboard::KeyboardInput>,
    window: Query<Entity, With<PrimaryWindow>>,
) {
    for event in keyboard_events.read() {
        if event.key_code == KeyCode::Escape {
            if event.state.is_pressed() {
                if let Ok(window_entity) = window.single() {
                    window_events.write(bevy::window::WindowCloseRequested {
                        window: window_entity,
                    });
                }
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Material2dPlugin::<HexagonBorderMaterial>::default())
        .add_plugins((
            CameraControlPlugin,
            HudPlugin,
            EntityCardsPlugin,
            MapInteractionPlugin,
            SpriteManagerPlugin,
        ))
        .insert_resource(HexGridConfig::new(50.0, 50, 50, 5.0))
        .add_systems(
            Startup,
            (
                minigame::core::systems::scene::setup_scene,
                minigame::core::systems::grid::setup_grid,
            ),
        )
        .add_systems(
            Update,
            (
                minigame::core::systems::debug::debug_position_system,
                minigame::core::systems::grid::render_grid_system,
                minigame::core::systems::movement::movement_system,
                minigame::core::systems::state_machine::state_machine_system,
                close_window_on_esc,
            ),
        )
        .run();
}
