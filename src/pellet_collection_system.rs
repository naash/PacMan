// Copyright (c) 2026 Nishant Sthalekar
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::any::Any;

use mithya_engine::{
    core::EngineEventListener,
    engine::{system::{System, SystemUpdateContext}, World},
};

use crate::events::PelletEatenEvent;
use crate::pellet::Pellet;
use crate::player::PlayerState;

pub struct PelletCollectionSystem;

impl System for PelletCollectionSystem {
    fn initialize(&mut self, _world: &mut World) {}

    fn update(&mut self, ctx: &mut SystemUpdateContext) {
        let player_cell = match ctx.world.resources.get::<PlayerState>() {
            Some(p) => p.current_cell,
            None => return,
        };

        let colliding: Vec<u32> = ctx.world.entity_manager
            .query_component::<Pellet>()
            .into_iter()
            .filter(|&id| {
                ctx.world.entity_manager.get_component::<Pellet>(id)
                    .map(|p| p.cell == player_cell)
                    .unwrap_or(false)
            })
            .collect();

        for pellet_id in colliding {
            ctx.world.entity_manager.destroy_entity(pellet_id);
            ctx.events.push(PelletEatenEvent);
        }
    }

    fn as_event_listener_mut(&mut self) -> Option<&mut dyn EngineEventListener> {
        None
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
