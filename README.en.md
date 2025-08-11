

# Rust Bevy Ecosystem Simulation Game

This is a project for an ecosystem simulation game developed using Rust and the Bevy engine. The game visually demonstrates the interactions between living organisms and ecological balance, making it suitable for both educational and entertainment purposes.

## Project Features

- **Ecosystem Simulation**: Simulates food chain relationships and energy transfer among organisms.
- **Modular Architecture**: Utilizes Bevy's ECS architecture; modular design facilitates expansion and maintenance.
- **Data-driven**: Game content is defined via configuration files, supporting hot reloading.
- **Visual UI**: Provides an intuitive user interface to display organism states and game information.
- **Spatial Partitioning**: Optimizes performance and improves efficiency of large-scale entity interactions.

## Technology Stack

- **Rust**: A high-performance systems programming language.
- **Bevy**: An open-source ECS game engine that supports cross-platform development.
- **WGSL**: Used for writing shader code to achieve graphical rendering.

## Directory Structure

- `minigame/src/core`: Core game logic, including components, systems, and state machines.
- `minigame/src/level`: Level system responsible for loading and managing game levels.
- `minigame/src/ui`: User interface module, including HUD and entity cards.
- `minigame/src/sprite`: Sprite management module responsible for loading and managing game resources.
- `minigame/src/scenes`: Scene management module handling main menu and scene transitions.
- `minigame/src/ai`: AI behavior tree module controlling organism behavior logic.
- `minigame/assets`: Game asset files such as shaders and textures.

## Installation and Running

### System Requirements

- Rust 1.60+
- Bevy engine
- Cargo build tool

### Building the Project

```bash
git clone https://gitee.com/xeroin/rust_bevy_demo.git
cd rust_bevy_demo
cargo build --release
```

### Running the Game

```bash
cargo run
```

## Usage Instructions

1. **Start the Game**: After launching, you will enter the main menu. Select a level to begin playing.
2. **Interactions**:
   - Click on organisms with the mouse to select them and view detailed information.
   - Use the keyboard to control camera movement (WASD).
   - Press the `ESC` key to exit the game.
3. **Observe the Ecosystem**: The game simulates interactions between organisms, including behaviors such as foraging, movement, and reproduction.

## Contributing Guide

Contributions of code and suggestions are welcome! Please follow these steps:

1. Fork the project
2. Create a new branch (`git checkout -b feature/new-feature`)
3. Commit your changes (`git commit -am 'Add new feature'`)
4. Push your branch (`git push origin feature/new-feature`)
5. Create a Pull Request

## License

This project uses the MIT License. For more details, please refer to the [LICENSE](LICENSE) file.