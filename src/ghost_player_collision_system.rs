// Copyright (c) 2026 Nishant Sthalekar
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::any::Any;

use mithya_engine::{
    core::EngineEventListener,
    engine::{system::{System, SystemUpdateContext}, World},
    navigation::NavAgent,
};

use crate::ghost::{Ghost, GhostFrightened, PlayerGhostCollisionEvent, GameStateResource};
use crate::player::PlayerState;

pub struct GhostPlayerCollisionSystem;

impl System for GhostPlayerCollisionSystem {
    fn initialize(&mut self, _world: &mut World) {}

    fn update(&mut self, ctx: &mut SystemUpdateContext) {
        let game_over = ctx.world.resources.get::<GameStateResource>()
            .map(|s| s.game_over)
            .unwrap_or(false);

        if game_over {
            println!("[Collision] Game over - resetting. Collisions disabled.");
            if let Some(state) = ctx.world.resources.get_mut::<GameStateResource>() {
                state.game_over = false;
                state.lives = 3;
                println!("[Collision] Lives reset to 3. Game restarted!");
            }
            return;
        }

        let (player_cell, is_invulnerable) = match ctx.world.resources.get::<PlayerState>() {
            Some(p) => (p.current_cell, p.invulnerability_timer > 0.0),
            None => return,
        };

        if is_invulnerable {
            return;
        }

        let entities = ctx.world.entity_manager.query_component::<Ghost>();
        for entity_id in entities {
            let (nav, is_frightened) = {
                let nav = ctx.world.entity_manager.get_component::<NavAgent>(entity_id);
                let frightened = ctx.world.entity_manager.get_component::<GhostFrightened>(entity_id);
                match nav {
                    Some(n) => (n, frightened.is_some()),
                    None => continue,
                }
            };

            if nav.current_cell == player_cell {
                ctx.events.push(PlayerGhostCollisionEvent {
                    ghost_id: entity_id,
                    is_frightened,
                });
            }
        }
    }

    fn as_event_listener_mut(&mut self) -> Option<&mut dyn EngineEventListener> {
        None
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
