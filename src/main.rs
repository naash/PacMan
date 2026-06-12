// Copyright (c) 2026 Nishant Sthalekar
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

mod ghost;
mod ghost_chase_system;
mod ghost_flee_system;
mod ghost_mode_system;
mod maze;
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

use ghost::{Ghost, GhostType, GhostModeResource};
use ghost_chase_system::GhostChaseSystem;
use ghost_flee_system::GhostFleeSystem;
use ghost_mode_system::GhostModeSystem;
use pellet::Pellet;
use maze::{Maze, TileType, GRID_HEIGHT, GRID_WIDTH, TILE_SIZE, build_nav_grid};
use pacman_movement_system::PacmanMovementSystem;
use pellet_collection_system::PelletCollectionSystem;
use player::PlayerState;
use power_pellet::{PowerPellet, POWER_PELLET_CELLS};
use power_pellet_collection_system::PowerPelletCollectionSystem;

struct PacmanGame;

const PACMAN_SPAWN_COL: usize = 13;
const PACMAN_SPAWN_ROW: usize = 23;

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
            mat.uniforms.insert("u_color".to_string(), UniformValue::Vec3([0.1, 0.2, 0.9]));
        }

        EntityBuilder::new(&mut world.entity_manager)
            .with(Transform::default())
            .with(Camera::new(248.0))
            .build();

        let maze = Maze::new();
        let nav_grid = build_nav_grid(&maze);

        for row in 0..GRID_HEIGHT {
            for col in 0..GRID_WIDTH {
                if maze.tiles[row][col] == TileType::Wall {
                    let pos = nav_grid.cell_to_world(GridCell::new(col as i32, row as i32));
                    EntityBuilder::new(&mut world.entity_manager)
                        .with(Transform {
                            position: pos,
                            scale: Vec3::new(16.0, 16.0, 1.0),
                            ..Default::default()
                        })
                        .with(Render {
                            mesh: Mesh::new_quad(),
                            material_id: Some(wall_material_id),
                            gpu_cache: None,
                            tint : None
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
            mat.uniforms.insert("u_color".to_string(), UniformValue::Vec3([1.0, 1.0, 0.6]));
        }

        for row in 0..GRID_HEIGHT {
            for col in 0..GRID_WIDTH {
                if maze.tiles[row][col] == TileType::Floor {
                    let is_power_pellet = POWER_PELLET_CELLS.iter()
                        .any(|&(pc, pr)| pc == col as i32 && pr == row as i32);
                    if is_power_pellet {
                        continue;
                    }
                    let cell = GridCell::new(col as i32, row as i32);
                    let pos = nav_grid.cell_to_world(cell);
                    EntityBuilder::new(&mut world.entity_manager)
                        .with(Transform {
                            position: pos,
                            scale: Vec3::new(4.0, 4.0, 1.0),
                            ..Default::default()
                        })
                        .with(Render {
                            mesh: Mesh::new_quad(),
                            material_id: Some(pellet_material_id),
                            gpu_cache: None,
                            tint : None
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
            mat.uniforms.insert("u_color".to_string(), UniformValue::Vec3([1.0, 1.0, 1.0]));
        }

        for &(col, row) in POWER_PELLET_CELLS {
            let cell = GridCell::new(col, row);
            let pos = nav_grid.cell_to_world(cell);
            EntityBuilder::new(&mut world.entity_manager)
                .with(Transform {
                    position: pos,
                    scale: Vec3::new(8.0, 8.0, 1.0),
                    ..Default::default()
                })
                .with(Render {
                    mesh: Mesh::new_quad(),
                    material_id: Some(power_pellet_material_id),
                    gpu_cache: None,
                    tint : None
                })
                .with(PowerPellet::new(cell))
                .build();
        }

        let pacman_material_id = world.asset_manager.get_material_by_name("pacman");
        let spawn_pos = nav_grid.cell_to_world(GridCell::new(
            PACMAN_SPAWN_COL as i32,
            PACMAN_SPAWN_ROW as i32,
        ));
        let spawn_cell = GridCell::new(PACMAN_SPAWN_COL as i32, PACMAN_SPAWN_ROW as i32);
        let pacman_id = EntityBuilder::new(&mut world.entity_manager)
            .with(Transform {
                position: spawn_pos,
                scale: Vec3::new(16.0, 16.0, 1.0),
                ..Default::default()
            })
            .with(Render {
                mesh: Mesh::new_quad_textured(),
                material_id: pacman_material_id,
                gpu_cache: None,
                tint : None
            })
            .with(Movement::new(8.0 * TILE_SIZE))
            .with(PlayerControlled)
            .build();

        // (texture, spawn_col, spawn_row, scatter_col, scatter_row, type)
        let ghost_spawn_data: &[(&str, usize, usize, i32, i32, GhostType)] = &[
            ("blinky", 13, 11, 25,  0, GhostType::Blinky),
            ("pinky",  13, 13,  2,  0, GhostType::Pinky),
            ("inky",   11, 13, 27, 30, GhostType::Inky),
            ("clyde",  15, 13,  0, 30, GhostType::Clyde),
        ];

        for &(texture_name, col, row, sc_col, sc_row, kind) in ghost_spawn_data {
            let material_id = world.asset_manager.get_material_by_name(texture_name);
            let spawn_cell = GridCell::new(col as i32, row as i32);
            let spawn_pos = nav_grid.cell_to_world(spawn_cell);
            let scatter_corner = GridCell::new(sc_col, sc_row);
            EntityBuilder::new(&mut world.entity_manager)
                .with(Transform {
                    position: spawn_pos,
                    scale: Vec3::new(16.0, 16.0, 1.0),
                    ..Default::default()
                })
                .with(Render {
                    mesh: Mesh::new_quad_textured(),
                    material_id,
                    gpu_cache: None,
                    tint : None
                })
                .with(NavAgent::new(spawn_cell, Some(0.05)))
                .with(Movement::new(3.0 * TILE_SIZE))
                .with(RandomMovement::new())
                .with(Ghost::new(kind, scatter_corner))
                .build();
        }

        world.resources.insert(maze);
        world.resources.insert(nav_grid);
        world.resources.insert(PlayerState::new(pacman_id, spawn_cell));
        world.resources.insert(GhostModeResource::new());

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
