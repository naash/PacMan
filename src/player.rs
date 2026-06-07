// Copyright (c) 2026 Nishant Sthalekar
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use glam::Vec2;
use mithya_engine::{ControllerBehavior, input::InputAction};
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
}

/// ControllerBehavior for Pacman: last direction key pressed wins each frame.
pub struct PacmanInputBehavior {
    pending: Option<Vec2>,
}

impl PacmanInputBehavior {
    pub fn new() -> Self {
        Self { pending: None }
    }
}

impl ControllerBehavior for PacmanInputBehavior {
    fn on_input_actions(&mut self, actions: &[InputAction]) {
        for action in actions {
            match action {
                InputAction::MoveLeft  => self.pending = Some(Vec2::new(-1.0,  0.0)),
                InputAction::MoveRight => self.pending = Some(Vec2::new( 1.0,  0.0)),
                InputAction::MoveUp    => self.pending = Some(Vec2::new( 0.0,  1.0)),
                InputAction::MoveDown  => self.pending = Some(Vec2::new( 0.0, -1.0)),
                _ => {}
            }
        }
    }

    fn compute_intent(&mut self) -> Vec2 {
        self.pending.take().unwrap_or(Vec2::ZERO)
    }
}

impl PlayerState {
    pub fn new(entity_id: u32, spawn_cell: GridCell) -> Self {
        Self {
            entity_id,
            current_direction: Direction::None,
            queued_direction: Direction::None,
            current_cell: spawn_cell,
            target_cell: None,
        }
    }
}
