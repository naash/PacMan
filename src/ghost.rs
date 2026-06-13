// Copyright (c) 2026 Nishant Sthalekar
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::any::Any;

use mithya_engine::{core::engine_events::EngineEvent, navigation::grid_cell::GridCell};

pub const START_DURATION: f32 = 3.0;
pub const FRIGHTENED_DURATION: f32 = 6.0;

pub const MODE_SCHEDULE: &[(f32, GhostMode)] = &[
    (7.0,           GhostMode::Scatter),
    (20.0,          GhostMode::Chase),
    (7.0,           GhostMode::Scatter),
    (20.0,          GhostMode::Chase),
    (5.0,           GhostMode::Scatter),
    (20.0,          GhostMode::Chase),
    (5.0,           GhostMode::Scatter),
    (f32::INFINITY, GhostMode::Chase),
];

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
            if self.start_elapsed >= START_DURATION {
                self.in_start = false;
                self.phase_elapsed = 0.0;
            }
            return;
        }

        self.phase_elapsed += delta;
        while self.phase_index + 1 < MODE_SCHEDULE.len()
            && self.phase_elapsed >= MODE_SCHEDULE[self.phase_index].0
        {
            self.phase_elapsed -= MODE_SCHEDULE[self.phase_index].0;
            self.phase_index += 1;
        }
    }

    pub fn current_mode(&self) -> GhostMode {
        if self.in_start {
            GhostMode::Start
        } else {
            MODE_SCHEDULE[self.phase_index].1
        }
    }
}

#[derive(Debug, Clone)]
pub struct PowerPelletEatenEvent;

impl EngineEvent for PowerPelletEatenEvent {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, Clone)]
pub struct PlayerGhostCollisionEvent {
    pub ghost_id: u32,
    pub is_frightened: bool,
}

impl EngineEvent for PlayerGhostCollisionEvent {
    fn as_any(&self) -> &dyn Any {
        self
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
            lives: 3,
            game_over: false,
            level: 1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerDeathEvent;

impl EngineEvent for PlayerDeathEvent {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, Clone)]
pub struct GhostEatenEvent {
    pub ghost_id: u32,
}

impl EngineEvent for GhostEatenEvent {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
