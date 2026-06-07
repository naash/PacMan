// Copyright (c) 2026 Nishant Sthalekar
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use mithya_engine::navigation::{grid_cell::GridCell, NavGrid};

/// Send + Sync required: GhostState holds Box<dyn AIController> in world resources.
pub trait AIController: Send + Sync {
    fn target_tile(&self, current_cell: GridCell, nav: &NavGrid) -> GridCell;
}
