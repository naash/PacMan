// Copyright (c) 2026 Nishant Sthalekar
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

mod config;
mod events;
mod resources;
mod game_state_system;
mod level_complete_system;
mod level_progression_system;
mod ghost;
mod ghost_chase_system;
mod ghost_flee_system;
mod ghost_mode_system;
mod ghost_player_collision_system;
mod maze;
mod scoring_system;
mod pacman_movement_system;
mod pellet;
mod pellet_collection_system;
mod player;
mod power_pellet;
mod power_pellet_collection_system;

use mithya_engine::{
    NavMovementSystem, NavigationSystem, PlayerInputSystem,
    RandomMovementSystem, Transform,
    engine::{Engine, EngineConfig, EntityBuilder, GameLogic, World, system::SystemsManager},
    input::{InputMapping, mapping::{InputAction, InputBinding}},
    rendering::Camera,
    egui,
};
use winit::keyboard::KeyCode;

use game_state_system::GameStateSystem;
use level_complete_system::LevelCompleteSystem;
use level_progression_system::LevelProgressionSystem;
use resources::{GhostModeResource, GameStateResource, ScoreResource, LevelProgressionResource};
use scoring_system::ScoringSystem;
use ghost_chase_system::GhostChaseSystem;
use ghost_flee_system::GhostFleeSystem;
use ghost_mode_system::GhostModeSystem;
use ghost_player_collision_system::GhostPlayerCollisionSystem;
use pacman_movement_system::PacmanMovementSystem;
use pellet_collection_system::PelletCollectionSystem;
use power_pellet_collection_system::PowerPelletCollectionSystem;

struct PacmanGame;

impl GameLogic for PacmanGame {
    fn initialize(&mut self, world: &mut World, systems_manager: &mut SystemsManager) {

        if let Some(renderer) = systems_manager.get_rendering_system() {
            renderer.load_assets(&mut world.asset_manager, |assets, device, queue| {
                assets
                    .load_texture_for_material("pacman", "pacman.png", device, queue)
                    .expect("Failed to load pacman.png");
                assets
                    .load_texture_for_material("blinky", "blinky.png", device, queue)
                    .expect("Failed to load blinky.png");
                assets
                    .load_texture_for_material("pinky", "pinky.png", device, queue)
                    .expect("Failed to load pinky.png");
                assets
                    .load_texture_for_material("inky", "inky.png", device, queue)
                    .expect("Failed to load inky.png");
                assets
                    .load_texture_for_material("clyde", "clyde.png", device, queue)
                    .expect("Failed to load clyde.png");
            });

            renderer.ui_draw_fn = Some(Box::new(|ctx: &egui::Context, world: &World| {
                let score = world.resources.get::<ScoreResource>().map(|r| r.score).unwrap_or(0);
                let lives = world.resources.get::<GameStateResource>().map(|r| r.lives).unwrap_or(0);
                let level = world.resources.get::<GameStateResource>().map(|r| r.level).unwrap_or(1);

                egui::TopBottomPanel::top("hud")
                    .frame(
                        egui::Frame::new()
                            .fill(egui::Color32::WHITE)
                            .inner_margin(egui::Margin::symmetric(12, 6)),
                    )
                    .show(ctx, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new(format!("SCORE  {score}")).color(egui::Color32::BLACK).strong());
                            ui.add_space(24.0);
                            ui.label(egui::RichText::new(format!("LIVES  {lives}")).color(egui::Color32::BLACK).strong());
                            ui.add_space(24.0);
                            ui.label(egui::RichText::new(format!("LEVEL  {level}")).color(egui::Color32::BLACK).strong());
                        });
                    });
            }));
        }

        {
            EntityBuilder::new(&mut world.entity_manager)
                .with(Transform::default())
                .with(Camera::new(config::camera::ZOOM))
                .build();
        }

        world.resources.insert(GhostModeResource::new());
        world.resources.insert(GameStateResource::new());
        world.resources.insert(ScoreResource::new());
        world.resources.insert(LevelProgressionResource::new());

        systems_manager.add_system(GhostPlayerCollisionSystem, world);
        systems_manager.add_system(GameStateSystem, world);
        systems_manager.add_system(ScoringSystem, world);
        systems_manager.add_system(LevelCompleteSystem, world);
        systems_manager.add_system(LevelProgressionSystem, world);
        systems_manager.add_system(GhostModeSystem::new(), world);
        systems_manager.add_system(GhostChaseSystem, world);
        systems_manager.add_system(GhostFleeSystem, world);
        systems_manager.add_system(PelletCollectionSystem, world);
        systems_manager.add_system(PowerPelletCollectionSystem, world);
        systems_manager.add_system(NavigationSystem::new(), world);
        systems_manager.add_system(RandomMovementSystem, world);
        systems_manager.add_system(NavMovementSystem, world);
        systems_manager.add_system(PlayerInputSystem::new(), world);
        systems_manager.add_system(PacmanMovementSystem, world);
        
        
        if let Some(mapping) = world.resources.get_mut::<InputMapping>() {
            mapping
                .bind(KeyCode::ArrowLeft,  InputBinding::continuous(InputAction::MoveLeft))
                .bind(KeyCode::KeyA,       InputBinding::continuous(InputAction::MoveLeft))
                .bind(KeyCode::ArrowRight, InputBinding::continuous(InputAction::MoveRight))
                .bind(KeyCode::KeyD,       InputBinding::continuous(InputAction::MoveRight))
                .bind(KeyCode::ArrowUp,    InputBinding::continuous(InputAction::MoveUp))
                .bind(KeyCode::KeyW,       InputBinding::continuous(InputAction::MoveUp))
                .bind(KeyCode::ArrowDown,  InputBinding::continuous(InputAction::MoveDown))
                .bind(KeyCode::KeyS,       InputBinding::continuous(InputAction::MoveDown));
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = EngineConfig {
        window_title: "Pacman".to_string(),
        window_width: 448,
        window_height: 496,
        resizable: false,
        asset_root: std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")),
    };

    Engine::new(config, PacmanGame).run()
}
