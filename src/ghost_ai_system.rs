// Copyright (c) 2026 Nishant Sthalekar
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::any::{Any, TypeId};

use mithya_engine::{
    MoveToEvent, NavAgent, NavGrid,
    core::{EngineActionQueue, EngineEventListener, EngineEventQueue},
    engine::{World, system::{System, SystemUpdateContext}},
};

use crate::ghost::Ghosts;

/// Fires a MoveToEvent for any ghost whose NavAgent has exhausted its path.
/// Runs after NavigationSystem so it observes fresh NavAgent state.
pub struct GhostAISystem;

impl System for GhostAISystem {
    fn initialize(&mut self, _world: &mut World) -> () {
    }

    fn update(&mut self, ctx: &mut SystemUpdateContext) {
        let nav = match ctx.world.resources.get::<NavGrid>() {
            Some(n) => n,
            None => return,
        };
        let ghosts = match ctx.world.resources.get::<Ghosts>() {
            Some(g) => g,
            None => return,
        };

        let mut events: Vec<MoveToEvent> = Vec::new();
        for ghost in &ghosts.0 {
            if let Some(agent) = ctx.world.entity_manager.get_component::<NavAgent>(ghost.entity_id) {
                if agent.is_idle() {
                    let target = ghost.controller.target_tile(agent.current_cell, nav);
                    events.push(MoveToEvent { entity_id: ghost.entity_id, target });
                }
            }
        }

        for event in events {
            ctx.events.push(event);
        }
    }

    fn as_event_listener_mut(&mut self) -> Option<&mut dyn EngineEventListener> {
        None
    }

    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}

impl EngineEventListener for GhostAISystem {
    fn interested_events(&self) -> Vec<TypeId> { vec![] }
    fn on_events(&mut self, _e: &EngineEventQueue, _a: &mut EngineActionQueue, _w: &World) {}
}
