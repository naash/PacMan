// Copyright (c) 2026 Nishant Sthalekar
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use glam::Vec3;
use mithya_engine::navigation::{grid_cell::GridCell, nav_grid::CellType, NavGrid};

pub const GRID_WIDTH: usize = 28;
pub const GRID_HEIGHT: usize = 31;
pub const TILE_SIZE: f32 = 16.0;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Maze {
    pub tiles: [[TileType; GRID_WIDTH]; GRID_HEIGHT],
}

impl Maze {
    pub fn new() -> Self {
        //TODO - Procedurally generated mazes
        let mut tiles = [[TileType::Floor; GRID_WIDTH]; GRID_HEIGHT];
        for row in 0..GRID_HEIGHT {
            for col in 0..GRID_WIDTH {
                let on_border = row == 0 || row == GRID_HEIGHT - 1 || col == 0 || col == GRID_WIDTH - 1;
                let is_post = row % 2 == 0 && col % 2 == 0;
                if on_border || is_post {
                    tiles[row][col] = TileType::Wall;
                }
            }
        }
        Maze { tiles }
    }
}

pub fn build_nav_grid(maze: &Maze) -> NavGrid {
    let origin = Vec3::new(
        -(GRID_WIDTH as f32 * TILE_SIZE / 2.0),
        GRID_HEIGHT as f32 * TILE_SIZE / 2.0,
        0.0,
    );
    let mut grid = NavGrid::new(GRID_WIDTH as u32, GRID_HEIGHT as u32, TILE_SIZE, origin);
    for row in 0..GRID_HEIGHT {
        for col in 0..GRID_WIDTH {
            if maze.tiles[row][col] == TileType::Wall {
                grid.set_cell(GridCell::new(col as i32, row as i32), CellType::Wall);
            }
        }
    }
    grid
}
