// Copyright (c) 2026 Nishant Sthalekar
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use mithya_engine::navigation::grid_cell::GridCell;

#[derive(Clone, Debug)]
pub struct Pellet {
    pub cell: GridCell,
}

impl Pellet {
    pub fn new(cell: GridCell) -> Self {
        Self { cell }
    }
}
