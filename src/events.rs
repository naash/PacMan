// Copyright (c) 2026 Nishant Sthalekar
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::any::Any;
use mithya_engine::core::engine_events::EngineEvent;

#[derive(Debug, Clone)]
pub struct PowerPelletEatenEvent;

impl EngineEvent for PowerPelletEatenEvent {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, Clone)]
pub struct PlayerGhostCollisionEvent {
    pub is_frightened: bool,
}

impl EngineEvent for PlayerGhostCollisionEvent {
    fn as_any(&self) -> &dyn Any {
        self
    }
}


#[derive(Debug, Clone)]
pub struct PelletEatenEvent;

impl EngineEvent for PelletEatenEvent {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, Clone)]
pub struct LevelCompleteEvent {
    pub is_game_over: bool,
}

impl EngineEvent for LevelCompleteEvent {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
