// Copyright (c) 2026 Nishant Sthalekar
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::any::{Any, TypeId};

use glam::Vec3;
use mithya_engine::{
    Transform, NavAgent, Movement, RandomMovement, PlayerControlled, Render, Mesh,
    asset::UniformValue,
    core::{EngineEventListener, EngineActionQueue, EngineEventQueue},
    engine::{EntityBuilder, system::{System, SystemUpdateContext}, World},
    navigation::grid_cell::GridCell,
    rendering::Camera,
};

use crate::config;
use crate::events::LevelCompleteEvent;
use crate::ghost::{Ghost, GhostType};
use crate::maze::{Maze, TileType, build_nav_grid};
use crate::pellet::Pellet;
use crate::player::{PlayerState, Direction};
use crate::power_pellet::PowerPellet;
use crate::resources::{GameStateResource, ScoreResource, GhostModeResource};

fn spawn_level(world: &mut World, maze: Maze, nav_grid: mithya_engine::navigation::NavGrid) -> u32 {
    // Returns the Pac-Man entity ID
    // Also stores maze and nav_grid in world resources

    // 1. Respawn walls
    let wall_material_id = world
        .asset_manager
        .load_material("wall_color")
        .expect("Failed to create wall material");

    if let Some(mat) = world.asset_manager.get_material_mut(wall_material_id) {
        mat.uniforms.insert("u_color".to_string(), UniformValue::Vec3(config::colors::WALL));
    }

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
                        tint: None,
                        gpu_cache: None,
                    })
                    .build();
            }
        }
    }

    // 2. Respawn pellets
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
                        tint: None,
                        gpu_cache: None,
                    })
                    .with(Pellet::new(cell))
                    .build();
            }
        }
    }

    // 3. Respawn power pellets
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
                tint: None,
                gpu_cache: None,
            })
            .with(PowerPellet::new(cell))
            .build();
    }

    // 4. Respawn Pac-Man
    let pacman_material_id = world.asset_manager.get_material_by_name("pacman");
    let spawn_cell = GridCell::new(config::spawn::PACMAN_COL as i32, config::spawn::PACMAN_ROW as i32);
    let spawn_pos = nav_grid.cell_to_world(spawn_cell);
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
            tint: None,
            gpu_cache: None,
        })
        .with(Movement::new(config::movement::pacman_speed()))
        .with(PlayerControlled)
        .build();

    // 5. Respawn ghosts
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
                tint: None,
                gpu_cache: None,
            })
            .with(NavAgent::new(spawn_cell, Some(config::movement::GHOST_NAV_INTERPOLATION)))
            .with(Movement::new(config::movement::ghost_speed()))
            .with(RandomMovement::new())
            .with(Ghost::new(kind, scatter_corner))
            .build();
    }

    // Store maze and nav grid in resources
    world.resources.insert(maze);
    world.resources.insert(nav_grid);

    pacman_id
}

pub struct LevelProgressionSystem;

impl System for LevelProgressionSystem {
    fn initialize(&mut self, world: &mut World) {
        // Spawn initial level
        let maze = Maze::new();
        let nav_grid = build_nav_grid(&maze);
        let pacman_id = spawn_level(world, maze, nav_grid);

        // Set up player state
        let spawn_cell = GridCell::new(config::spawn::PACMAN_COL as i32, config::spawn::PACMAN_ROW as i32);
        world.resources.insert(PlayerState::new(pacman_id, spawn_cell));
    }

    fn update(&mut self, _ctx: &mut SystemUpdateContext) {}

    fn as_event_listener_mut(&mut self) -> Option<&mut dyn EngineEventListener> {
        Some(self)
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl EngineEventListener for LevelProgressionSystem {
    fn interested_events(&self) -> Vec<TypeId> {
        vec![TypeId::of::<LevelCompleteEvent>()]
    }

    fn on_events(&mut self, events: &EngineEventQueue, actions: &mut EngineActionQueue, _world: &World) {
        for event in events.iter_type::<LevelCompleteEvent>() {
            let is_game_over = event.is_game_over;

            actions.push_anonymous(move |world| {
                // 1. Handle game-over vs advance logic
                if is_game_over {
                    if let Some(state) = world.resources.get_mut::<GameStateResource>() {
                        state.lives = config::game_state::INITIAL_LIVES;
                        state.game_over = false;
                        println!("[LevelProgression] Lives reset to {}", state.lives);
                    }
                    if let Some(score) = world.resources.get_mut::<ScoreResource>() {
                        score.score = 0;
                        score.ghost_combo = 0;
                        println!("[LevelProgression] Score reset to 0");
                    }
                } else {
                    if let Some(state) = world.resources.get_mut::<GameStateResource>() {
                        state.level += 1;
                        println!("[LevelProgression] Advanced to level {}", state.level);
                    }
                    if let Some(score) = world.resources.get_mut::<ScoreResource>() {
                        score.ghost_combo = 0;
                    }
                }

                // 2. Destroy all game entities except camera
                world.entity_manager.destroy_all_entities_except(&[TypeId::of::<Camera>()]);

                // 3. Create new maze and nav grid
                let maze = Maze::new();
                let nav_grid = build_nav_grid(&maze);

                // 4. Spawn level
                let pacman_id = spawn_level(world, maze, nav_grid);

                // 5. Reset player state
                let spawn_cell = GridCell::new(config::spawn::PACMAN_COL as i32, config::spawn::PACMAN_ROW as i32);
                if let Some(player) = world.resources.get_mut::<PlayerState>() {
                    player.entity_id = pacman_id;
                    player.current_cell = spawn_cell;
                    player.target_cell = None;
                    player.current_direction = Direction::None;
                    player.queued_direction = Direction::None;
                    player.invulnerability_timer = 0.0;
                }

                // 6. Reset ghost mode resource
                if let Some(mode_res) = world.resources.get_mut::<GhostModeResource>() {
                    mode_res.start_elapsed = 0.0;
                    mode_res.phase_elapsed = 0.0;
                    mode_res.phase_index = 0;
                    mode_res.in_start = true;
                }

                println!("[LevelProgression] Level regenerated successfully!");
            });
        }
    }
}
