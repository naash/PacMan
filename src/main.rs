use mithya_engine::engine::{Engine, EngineConfig, GameLogic};
use mithya_engine::engine::system::SystemsManager;
use mithya_engine::engine::world::World;

struct PacmanGame;

impl GameLogic for PacmanGame {
    fn initialize(&mut self, _world: &mut World, _systems_manager: &mut SystemsManager) {
        // TODO: insert resources, register systems, spawn entities
    }

    fn update(&mut self, _world: &mut World) {
        // TODO: per-frame game logic
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = EngineConfig {
        window_title: "Pacman".to_string(),
        window_width: 448,   // 28 tiles * 16px
        window_height: 496,  // 31 tiles * 16px
        resizable: false,
        asset_root: std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")),
    };

    Engine::new(config, PacmanGame).run()
}
