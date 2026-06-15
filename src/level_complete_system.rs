// Copyright (c) 2026 Nishant Sthalekar
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::any::Any;

use mithya_engine::engine::{system::{System, SystemUpdateContext}, World};

use crate::events::LevelCompleteEvent;
use crate::pellet::Pellet;
use crate::resources::GameStateResource;

pub struct LevelCompleteSystem;

impl System for LevelCompleteSystem {
    fn initialize(&mut self, _world: &mut World) {}

    fn update(&mut self, ctx: &mut SystemUpdateContext) {
        let game_over = ctx.world.resources.get::<GameStateResource>()
            .map(|s| s.game_over)
            .unwrap_or(false);

        if game_over {
            println!("[LevelComplete] Game Over! Restarting level...");
            ctx.events.push(LevelCompleteEvent { is_game_over: true });
            return;
        }

        let remaining_pellets = ctx.world.entity_manager.query_component::<Pellet>().len();

        if remaining_pellets == 0 {
            println!("[LevelComplete] All pellets eaten! Advancing to next level!");
            ctx.events.push(LevelCompleteEvent { is_game_over: false });
        }
    }

    fn as_event_listener_mut(&mut self) -> Option<&mut dyn mithya_engine::core::EngineEventListener> {
        None
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
