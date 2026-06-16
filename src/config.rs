// Copyright (c) 2026 Nishant Sthalekar
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

// Grid & Maze Configuration
pub mod maze {
    pub const WIDTH: usize = 28;
    pub const HEIGHT: usize = 31;
    pub const TILE_SIZE: f32 = 16.0;
}

// Spawn Positions
pub mod spawn {
    pub const PACMAN_COL: usize = 13;
    pub const PACMAN_ROW: usize = 23;

    pub struct GhostSpawn {
        pub col: usize,
        pub row: usize,
        pub scatter_col: i32,
        pub scatter_row: i32,
    }

    pub const BLINKY: GhostSpawn = GhostSpawn {
        col: 13,
        row: 11,
        scatter_col: 25,
        scatter_row: 0,
    };

    pub const PINKY: GhostSpawn = GhostSpawn {
        col: 13,
        row: 13,
        scatter_col: 2,
        scatter_row: 0,
    };

    pub const INKY: GhostSpawn = GhostSpawn {
        col: 11,
        row: 13,
        scatter_col: 27,
        scatter_row: 30,
    };

    pub const CLYDE: GhostSpawn = GhostSpawn {
        col: 15,
        row: 13,
        scatter_col: 0,
        scatter_row: 30,
    };
}

// Scoring Configuration
pub mod scoring {
    pub const PELLET: u32 = 10;
    pub const POWER_PELLET: u32 = 50;
    pub const GHOST_COMBO: &[u32] = &[200, 400, 800, 1600];
}

// Game Timing Configuration
pub mod timing {
    pub const GHOST_START_DELAY: f32 = 3.0;
    pub const FRIGHTENED_DURATION: f32 = 6.0;
    pub const INVULNERABILITY_AFTER_RESPAWN: f32 = 1.0;
}

// Movement Speed Configuration (in tiles per second)
pub mod movement {
    use super::maze;

    pub const PACMAN_SPEED_MULTIPLIER: f32 = 8.0;
    pub const GHOST_SPEED_MULTIPLIER: f32 = 3.0;
    pub const GHOST_NAV_INTERPOLATION: f32 = 0.05;

    pub fn pacman_speed() -> f32 {
        PACMAN_SPEED_MULTIPLIER * maze::TILE_SIZE
    }

    pub fn ghost_speed() -> f32 {
        GHOST_SPEED_MULTIPLIER * maze::TILE_SIZE
    }
}

// Sprite Scale Configuration (width, height, depth)
pub mod sprites {
    pub const WALL: (f32, f32, f32) = (16.0, 16.0, 1.0);
    pub const PELLET: (f32, f32, f32) = (4.0, 4.0, 1.0);
    pub const POWER_PELLET: (f32, f32, f32) = (8.0, 8.0, 1.0);
    pub const PACMAN: (f32, f32, f32) = (16.0, 16.0, 1.0);
    pub const GHOST: (f32, f32, f32) = (16.0, 16.0, 1.0);
}

// Colors Configuration (RGB)
pub mod colors {
    pub const WALL: [f32; 3] = [0.1, 0.2, 0.9];
    pub const PELLET: [f32; 3] = [1.0, 1.0, 0.6];
    pub const POWER_PELLET: [f32; 3] = [1.0, 1.0, 1.0];
}

// Camera Configuration
pub mod camera {
    pub const ZOOM: f32 = 248.0;
}

// Power Pellet Positions (corner cells)
pub mod pellets {
    pub const POWER_PELLET_CELLS: &[(i32, i32)] = &[(1, 3), (26, 3), (1, 23), (26, 23)];
}

// Game State Configuration
pub mod game_state {
    pub const INITIAL_LIVES: u32 = 3;
    pub const INITIAL_LEVEL: u32 = 1;
}

// Screen Display Durations
pub mod screens {
    pub const LEVEL_COMPLETE_DURATION: f32 = 3.0;
    pub const GAME_OVER_DURATION: f32 = 5.0;
}
