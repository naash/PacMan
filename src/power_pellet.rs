// Copyright (c) 2026 Nishant Sthalekar
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use mithya_engine::navigation::grid_cell::GridCell;

pub const POWER_PELLET_CELLS: &[(i32, i32)] = &[(1, 3), (26, 3), (1, 23), (26, 23)];

#[derive(Clone, Debug)]
pub struct PowerPellet {
    pub cell: GridCell,
}

impl PowerPellet {
    pub fn new(cell: GridCell) -> Self {
        Self { cell }
    }
}
