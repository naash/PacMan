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

pub struct PlayerState {
    pub entity_id: u32,
    pub current_direction: Direction,
    pub queued_direction: Direction,
    pub current_cell: GridCell,
    pub target_cell: Option<GridCell>,
    pub invulnerability_timer: f32,
}

impl PlayerState {
    pub fn new(entity_id: u32, spawn_cell: GridCell) -> Self {
        Self {
            entity_id,
            current_direction: Direction::None,
            queued_direction: Direction::None,
            current_cell: spawn_cell,
            target_cell: None,
            invulnerability_timer: 0.0,
        }
    }
}
