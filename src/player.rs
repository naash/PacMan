// Copyright (c) 2026 Nishant Sthalekar
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use mithya_engine::navigation::grid_cell::GridCell;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    None,
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    /// Returns (delta_col, delta_row). Row increases downward.
    pub fn delta(self) -> (i32, i32) {
        match self {
            Direction::Up    => ( 0, -1),
            Direction::Down  => ( 0,  1),
            Direction::Left  => (-1,  0),
            Direction::Right => ( 1,  0),
            Direction::None  => ( 0,  0),
        }
    }
}

/// Stored as a world resource. Tracks all tile-based movement state for Pac-Man.
pub struct PlayerState {
    pub entity_id: u32,
    pub tile: GridCell,
    pub target: GridCell,
    pub current_direction: Direction,
    pub queued_direction: Direction,
    /// Fraction of the way to the next tile: 0.0 = at tile center, 1.0 = arrived.
    pub move_progress: f32,
    /// Movement speed in tiles per second.
    pub speed: f32,
}

impl PlayerState {
    pub fn new(entity_id: u32, spawn_col: usize, spawn_row: usize) -> Self {
        let spawn = GridCell::new(spawn_col as i32, spawn_row as i32);
        Self {
            entity_id,
            tile: spawn,
            target: spawn,
            current_direction: Direction::Left,
            queued_direction: Direction::None,
            move_progress: 0.0,
            speed: 8.0,
        }
    }
}
