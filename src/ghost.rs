// Copyright (c) 2026 Nishant Sthalekar
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use mithya_engine::navigation::grid_cell::GridCell;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum GhostMode {
    Start,
    Scatter,
    Chase,
    Frightened,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum GhostType {
    Blinky,
    Pinky,
    Inky,
    Clyde,
}

// Always present on ghost entities. Carries static identity data.
#[derive(Clone, Debug)]
pub struct Ghost {
    pub ghost_type: GhostType,
    pub scatter_corner: GridCell,
}

impl Ghost {
    pub fn new(ghost_type: GhostType, scatter_corner: GridCell) -> Self {
        Self { ghost_type, scatter_corner }
    }
}

// Mode marker components — exactly one present at a time (or none = Start).
#[derive(Clone)]
pub struct GhostChase;

#[derive(Clone)]
pub struct GhostScatter;

#[derive(Clone)]
pub struct GhostFrightened {
    pub timer: f32,
}

