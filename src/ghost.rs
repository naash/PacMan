// Copyright (c) 2026 Nishant Sthalekar
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::sync::Mutex;

use mithya_engine::navigation::{grid_cell::GridCell, NavGrid};
use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::ai_pawn::AIController;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum GhostType {
    Blinky,
    Pinky,
    Inky,
    Clyde,
}

pub struct GhostState {
    pub entity_id: u32,
    #[allow(dead_code)]
    pub ghost_type: GhostType,
    pub controller: Box<dyn AIController>,
}

impl GhostState {
    pub fn new(
        entity_id: u32,
        ghost_type: GhostType,
        controller: Box<dyn AIController>,
    ) -> Self {
        Self { entity_id, ghost_type, controller }
    }
}

pub struct Ghosts(pub Vec<GhostState>);

/// Picks a random walkable tile anywhere in the maze; NavigationSystem A*s to it.
pub struct RandomWanderController {
    grid_cols: u32,
    grid_rows: u32,
    rng: Mutex<StdRng>,
}

impl RandomWanderController {
    pub fn new(grid_cols: u32, grid_rows: u32, seed: u64) -> Self {
        Self { grid_cols, grid_rows, rng: Mutex::new(StdRng::seed_from_u64(seed)) }
    }
}

impl AIController for RandomWanderController {
    fn target_tile(&self, _current_cell: GridCell, nav: &NavGrid) -> GridCell {
        let mut rng = self.rng.lock().unwrap();
        loop {
            let cell = GridCell::new(
                rng.gen_range(0..self.grid_cols) as i32,
                rng.gen_range(0..self.grid_rows) as i32,
            );
            if nav.is_walkable(cell) { return cell; }
        }
    }
}
