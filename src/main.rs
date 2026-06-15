// Copyright (c) 2026 Nishant Sthalekar
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

mod config;
mod events;
mod resources;
mod game_state_system;
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
    Movement, NavAgent, NavMovementSystem, NavigationSystem, PlayerControlled, PlayerInputSystem,
    RandomMovement, RandomMovementSystem, Transform,
    asset::UniformValue,
    engine::{Engine, EngineConfig, EntityBuilder, GameLogic, World, system::SystemsManager},
    input::{InputMapping, mapping::{InputAction, InputBinding}},
    navigation::grid_cell::GridCell,
    rendering::{Camera, Mesh, Render},
};
use glam::Vec3;
use winit::keyboard::KeyCode;

use game_state_system::GameStateSystem;
use ghost::{Ghost, GhostType};
use resources::{GhostModeResource, GameStateResource, ScoreResource};
use scoring_system::ScoringSystem;
use ghost_chase_system::GhostChaseSystem;
use ghost_flee_system::GhostFleeSystem;
use ghost_mode_system::GhostModeSystem;
use ghost_player_collision_system::GhostPlayerCollisionSystem;
use pellet::Pellet;
use maze::{Maze, TileType, build_nav_grid};
use pacman_movement_system::PacmanMovementSystem;
use pellet_collection_system::PelletCollectionSystem;
use player::PlayerState;
use power_pellet::PowerPellet;
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
        }

        let wall_material_id = world
            .asset_manager
            .load_material("wall_color")
            .expect("Failed to create wall material");

        if let Some(mat) = world.asset_manager.get_material_mut(wall_material_id) {
            mat.uniforms.insert("u_color".to_string(), UniformValue::Vec3(config::colors::WALL));
        }

        EntityBuilder::new(&mut world.entity_manager)
            .with(Transform::default())
            .with(Camera::new(config::camera::ZOOM))
            .build();

        let maze = Maze::new();
        let nav_grid = build_nav_grid(&maze);

        for row in 0..config::maze::HEIGHT {
            for col in 0..config::maze::WIDTH {
                if maze.tiles[row][col] == TileType::Wall {
                    let pos = nav_grid.cell_to_world(GridCell::new(col as i32, row as i32));
                    let (w, h, d) = config::sprites::WALL;
                    EntityBuilder::new(&mut world.entity_manager)
                        .with(Transform {
                            position: pos,
                            scale: Vec3::new(w, h, d),
                            ..Default::default()
                        })
                        .with(Render {
                            mesh: Mesh::new_quad(),
                            material_id: Some(wall_material_id),
                            tint : None,
                            gpu_cache: None
                        })
                        .build();
                }
            }
        }

        // Spawn pellets on all floor tiles
        let pellet_material_id = world
            .asset_manager
            .load_material("pellet_color")
            .expect("Failed to create pellet material");

        if let Some(mat) = world.asset_manager.get_material_mut(pellet_material_id) {
            mat.uniforms.insert("u_color".to_string(), UniformValue::Vec3(config::colors::PELLET));
        }

        for row in 0..config::maze::HEIGHT {
            for col in 0..config::maze::WIDTH {
                if maze.tiles[row][col] == TileType::Floor {
                    let is_power_pellet = config::pellets::POWER_PELLET_CELLS.iter()
                        .any(|&(pc, pr)| pc == col as i32 && pr == row as i32);
                    if is_power_pellet {
                        continue;
                    }
                    let cell = GridCell::new(col as i32, row as i32);
                    let pos = nav_grid.cell_to_world(cell);
                    let (w, h, d) = config::sprites::PELLET;
                    EntityBuilder::new(&mut world.entity_manager)
                        .with(Transform {
                            position: pos,
                            scale: Vec3::new(w, h, d),
                            ..Default::default()
                        })
                        .with(Render {
                            mesh: Mesh::new_quad(),
                            material_id: Some(pellet_material_id),

                            tint : None,
                            gpu_cache: None
                        })
                        .with(Pellet::new(cell))
                        .build();
                }
            }
        }

        let power_pellet_material_id = world
            .asset_manager
            .load_material("power_pellet_color")
            .expect("Failed to create power pellet material");

        if let Some(mat) = world.asset_manager.get_material_mut(power_pellet_material_id) {
            mat.uniforms.insert("u_color".to_string(), UniformValue::Vec3(config::colors::POWER_PELLET));
        }

        for &(col, row) in config::pellets::POWER_PELLET_CELLS {
            let cell = GridCell::new(col, row);
            let pos = nav_grid.cell_to_world(cell);
            let (w, h, d) = config::sprites::POWER_PELLET;
            EntityBuilder::new(&mut world.entity_manager)
                .with(Transform {
                    position: pos,
                    scale: Vec3::new(w, h, d),
                    ..Default::default()
                })
                .with(Render {
                    mesh: Mesh::new_quad(),
                    material_id: Some(power_pellet_material_id),

                    tint : None,
                            gpu_cache: None
                })
                .with(PowerPellet::new(cell))
                .build();
        }

        let pacman_material_id = world.asset_manager.get_material_by_name("pacman");
        let spawn_pos = nav_grid.cell_to_world(GridCell::new(
            config::spawn::PACMAN_COL as i32,
            config::spawn::PACMAN_ROW as i32,
        ));
        let spawn_cell = GridCell::new(config::spawn::PACMAN_COL as i32, config::spawn::PACMAN_ROW as i32);
        let (w, h, d) = config::sprites::PACMAN;
        let pacman_id = EntityBuilder::new(&mut world.entity_manager)
            .with(Transform {
                position: spawn_pos,
                scale: Vec3::new(w, h, d),
                ..Default::default()
            })
            .with(Render {
                mesh: Mesh::new_quad_textured(),
                material_id: pacman_material_id,

                tint : None,
                            gpu_cache: None
            })
            .with(Movement::new(config::movement::pacman_speed()))
            .with(PlayerControlled)
            .build();

        // Ghost spawn configurations
        let ghost_spawn_data: &[(&str, &config::spawn::GhostSpawn, GhostType)] = &[
            ("blinky", &config::spawn::BLINKY, GhostType::Blinky),
            ("pinky", &config::spawn::PINKY, GhostType::Pinky),
            ("inky", &config::spawn::INKY, GhostType::Inky),
            ("clyde", &config::spawn::CLYDE, GhostType::Clyde),
        ];

        let (w, h, d) = config::sprites::GHOST;
        for &(texture_name, spawn_cfg, kind) in ghost_spawn_data {
            let material_id = world.asset_manager.get_material_by_name(texture_name);
            let spawn_cell = GridCell::new(spawn_cfg.col as i32, spawn_cfg.row as i32);
            let spawn_pos = nav_grid.cell_to_world(spawn_cell);
            let scatter_corner = GridCell::new(spawn_cfg.scatter_col, spawn_cfg.scatter_row);
            EntityBuilder::new(&mut world.entity_manager)
                .with(Transform {
                    position: spawn_pos,
                    scale: Vec3::new(w, h, d),
                    ..Default::default()
                })
                .with(Render {
                    mesh: Mesh::new_quad_textured(),
                    material_id,

                    tint : None,
                            gpu_cache: None
                })
                .with(NavAgent::new(spawn_cell, Some(config::movement::GHOST_NAV_INTERPOLATION)))
                .with(Movement::new(config::movement::ghost_speed()))
                .with(RandomMovement::new())
                .with(Ghost::new(kind, scatter_corner))
                .build();
        }

        world.resources.insert(maze);
        world.resources.insert(nav_grid);
        world.resources.insert(PlayerState::new(pacman_id, spawn_cell));
        world.resources.insert(GhostModeResource::new());
        world.resources.insert(GameStateResource::new());
        world.resources.insert(ScoreResource::new());

        systems_manager.add_system(GhostPlayerCollisionSystem, world);
        systems_manager.add_system(GameStateSystem, world);
        systems_manager.add_system(ScoringSystem, world);
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
