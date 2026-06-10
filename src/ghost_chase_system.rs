// Copyright (c) 2026 Nishant Sthalekar
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::any::Any;

use mithya_engine::{
    MoveToEvent,
    core::EngineEventListener,
    engine::{system::{System, SystemUpdateContext}, World},
    navigation::NavAgent,
};

use crate::ghost::{Ghost, GhostChase};
use crate::player::PlayerState;

pub struct GhostChaseSystem;

impl System for GhostChaseSystem {
    fn initialize(&mut self, _world: &mut World) {}

    fn update(&mut self, ctx: &mut SystemUpdateContext) {
        let target = match ctx.world.resources.get::<PlayerState>() {
            Some(p) => p.current_cell,
            None => return,
        };

        let entities = ctx.world.entity_manager.query_two_components::<Ghost, GhostChase>();
        for entity_id in entities {
            let is_idle = ctx.world.entity_manager
                .get_component::<NavAgent>(entity_id)
                .map(|a| a.is_idle())
                .unwrap_or(false);

            if is_idle {
                ctx.events.push(MoveToEvent { entity_id, target });
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
