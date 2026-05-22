// Copyright (c) 2026 Nishant Sthalekar
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

mod maze;
mod player;
mod pacman_movement_system;

use mithya_engine::{
    Transform,
    asset::UniformValue,
    engine::{Engine, EngineConfig, EntityBuilder, GameLogic, World, system::SystemsManager},
    input::{InputMapping, mapping::{InputAction, InputBinding}},
    navigation::{grid_cell::GridCell},
    rendering::{Camera, Mesh, Render},
};
use glam::Vec3;
use winit::keyboard::KeyCode;

use maze::{Maze, TileType, GRID_HEIGHT, GRID_WIDTH, build_nav_grid};
use player::PlayerState;
use pacman_movement_system::PacmanMovementSystem;

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
                        })
                        .build();
                }
            }
        }

        let pacman_material_id = world.asset_manager.get_material_by_name("pacman");
        let spawn_pos = nav_grid.cell_to_world(GridCell::new(
            PACMAN_SPAWN_COL as i32,
            PACMAN_SPAWN_ROW as i32,
        ));
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
            })
            .build();

        world.resources.insert(maze);
        world.resources.insert(nav_grid);
        world.resources.insert(PlayerState::new(pacman_id, PACMAN_SPAWN_COL, PACMAN_SPAWN_ROW));

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

    fn update(&mut self, _world: &mut World) {}
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
