// Copyright (c) 2026 Nishant Sthalekar
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::any::{Any, TypeId};

use mithya_engine::{
    Transform,
    core::{EngineEventListener, EngineActionQueue, EngineEventQueue},
    engine::{resources::Time, system::{System, SystemUpdateContext}, World},
    navigation::{grid_cell::GridCell, NavGrid},
};

use crate::config;
use crate::events::PlayerGhostCollisionEvent;
use crate::resources::GameStateResource;
use crate::player::{Direction, PlayerState};

pub struct GameStateSystem;

impl System for GameStateSystem {
    fn initialize(&mut self, _world: &mut World) {}

    fn update(&mut self, ctx: &mut SystemUpdateContext) {
        let delta = ctx.world.resources.get::<Time>().map(|t| t.delta).unwrap_or(0.0);

        if let Some(player) = ctx.world.resources.get_mut::<PlayerState>() {
            if player.invulnerability_timer > 0.0 {
                player.invulnerability_timer -= delta;
            }
        }

        if let Some(state) = ctx.world.resources.get::<GameStateResource>() {
            if state.game_over {
                println!("[GameState] Resetting for new game...");
            }
        }
    }

    fn as_event_listener_mut(&mut self) -> Option<&mut dyn EngineEventListener> {
        Some(self)
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl EngineEventListener for GameStateSystem {
    fn interested_events(&self) -> Vec<TypeId> {
        vec![TypeId::of::<PlayerGhostCollisionEvent>()]
    }

    fn on_events(&mut self, events: &EngineEventQueue, actions: &mut EngineActionQueue, _world: &World) {
        for event in events.iter_type::<PlayerGhostCollisionEvent>() {
            if !event.is_frightened {
                actions.push_anonymous(|world| {
                    println!("[Death] Player hit ghost!");
                    if let Some(state) = world.resources.get_mut::<GameStateResource>() {
                        state.lives = state.lives.saturating_sub(1);
                        println!("[Death] Lives remaining: {}", state.lives);

                        if state.lives == 0 {
                            state.game_over = true;
                            println!("[Death] GAME OVER!");
                            return;
                        }
                    }

                    let (pacman_id, spawn_cell) = {
                        let player = match world.resources.get::<PlayerState>() {
                            Some(p) => p,
                            None => return,
                        };
                        (player.entity_id, GridCell::new(config::spawn::PACMAN_COL as i32, config::spawn::PACMAN_ROW as i32))
                    };

                    if let Some(player) = world.resources.get_mut::<PlayerState>() {
                        player.current_cell = spawn_cell;
                        player.target_cell = None;
                        player.current_direction = Direction::None;
                        player.queued_direction = Direction::None;
                        player.invulnerability_timer = config::timing::INVULNERABILITY_AFTER_RESPAWN;
                    }

                    if let Some(nav_grid) = world.resources.get::<NavGrid>() {
                        let spawn_pos = nav_grid.cell_to_world(spawn_cell);
                        if let Some(transform) = world.entity_manager.get_component_mut::<Transform>(pacman_id) {
                            transform.position = spawn_pos;
                        }
                    }
                });
            }
        }
    }
}
