use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use minigame::core::hex_grid::HexGridConfig;
use minigame::level::loader::LevelLoaderPlugin;
use minigame::ui::cards::EntityCardsPlugin;
use minigame::ui::hud::HudPlugin;

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
        .add_plugins((LevelLoaderPlugin, HudPlugin, EntityCardsPlugin))
        .insert_resource(HexGridConfig::new(50.0, 10, 10, 5.0))
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
