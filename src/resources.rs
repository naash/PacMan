// Copyright (c) 2026 Nishant Sthalekar
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::config;

pub struct GhostModeResource {
    pub start_elapsed: f32,
    pub phase_elapsed: f32,
    pub phase_index: usize,
    pub in_start: bool,
}

impl GhostModeResource {
    pub fn new() -> Self {
        Self {
            start_elapsed: 0.0,
            phase_elapsed: 0.0,
            phase_index: 0,
            in_start: true,
        }
    }

    pub fn advance(&mut self, delta: f32) {
        if self.in_start {
            self.start_elapsed += delta;
            if self.start_elapsed >= config::timing::GHOST_START_DELAY {
                self.in_start = false;
                self.phase_elapsed = 0.0;
            }
            return;
        }

        self.phase_elapsed += delta;
        let schedule = &[
            (7.0, crate::ghost::GhostMode::Scatter),
            (20.0, crate::ghost::GhostMode::Chase),
            (7.0, crate::ghost::GhostMode::Scatter),
            (20.0, crate::ghost::GhostMode::Chase),
            (5.0, crate::ghost::GhostMode::Scatter),
            (20.0, crate::ghost::GhostMode::Chase),
            (5.0, crate::ghost::GhostMode::Scatter),
            (f32::INFINITY, crate::ghost::GhostMode::Chase),
        ];

        while self.phase_index + 1 < schedule.len()
            && self.phase_elapsed >= schedule[self.phase_index].0
        {
            self.phase_elapsed -= schedule[self.phase_index].0;
            self.phase_index += 1;
        }
    }

    pub fn current_mode(&self) -> crate::ghost::GhostMode {
        if self.in_start {
            crate::ghost::GhostMode::Start
        } else {
            let schedule = &[
                (7.0, crate::ghost::GhostMode::Scatter),
                (20.0, crate::ghost::GhostMode::Chase),
                (7.0, crate::ghost::GhostMode::Scatter),
                (20.0, crate::ghost::GhostMode::Chase),
                (5.0, crate::ghost::GhostMode::Scatter),
                (20.0, crate::ghost::GhostMode::Chase),
                (5.0, crate::ghost::GhostMode::Scatter),
                (f32::INFINITY, crate::ghost::GhostMode::Chase),
            ];
            schedule[self.phase_index].1
        }
    }
}

pub struct GameStateResource {
    pub lives: u32,
    pub game_over: bool,
    pub level: u32,
}

impl GameStateResource {
    pub fn new() -> Self {
        Self {
            lives: config::game_state::INITIAL_LIVES,
            game_over: false,
            level: config::game_state::INITIAL_LEVEL,
        }
    }
}

pub struct ScoreResource {
    pub score: u32,
    pub ghost_combo: u32,
}

impl ScoreResource {
    pub fn new() -> Self {
        Self {
            score: 0,
            ghost_combo: 0,
        }
    }
}
