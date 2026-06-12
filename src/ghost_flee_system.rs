// Copyright (c) 2026 Nishant Sthalekar
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::any::Any;

use mithya_engine::{
    MoveToEvent,
    core::EngineEventListener,
    engine::{system::{System, SystemUpdateContext}, World},
    navigation::{grid_cell::GridCell, NavAgent, NavGrid},
};

use crate::ghost::{Ghost, GhostFrightened};
use crate::player::PlayerState;

pub struct GhostFleeSystem;

impl System for GhostFleeSystem {
    fn initialize(&mut self, _world: &mut World) {}

    fn update(&mut self, ctx: &mut SystemUpdateContext) {
        let player_cell = match ctx.world.resources.get::<PlayerState>() {
            Some(p) => p.current_cell,
            None => return,
        };

        if ctx.world.resources.get::<NavGrid>().is_none() {
            return;
        }

        let entities = ctx.world.entity_manager.query_two_components::<Ghost, GhostFrightened>();
        for entity_id in entities {
            let (is_idle, current_cell) = {
                let nav = ctx.world.entity_manager.get_component::<NavAgent>(entity_id);
                match nav {
                    Some(n) => (n.is_idle(), n.current_cell),
                    None => continue,
                }
            };

            if !is_idle {
                continue;
            }

            let target = {
                let nav = ctx.world.resources.get::<NavGrid>().unwrap();
                flee_target(current_cell, player_cell, nav)
            };

            ctx.events.push(MoveToEvent { entity_id, target });
        }
    }

    fn as_event_listener_mut(&mut self) -> Option<&mut dyn EngineEventListener> {
        None
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

fn flee_target(from: GridCell, threat: GridCell, nav: &NavGrid) -> GridCell {
    let directions = [
        GridCell::new(from.col - 1, from.row),
        GridCell::new(from.col + 1, from.row),
        GridCell::new(from.col, from.row - 1),
        GridCell::new(from.col, from.row + 1),
    ];

    directions
        .iter()
        .filter(|&&cell| nav.is_walkable(cell))
        .max_by_key(|&&cell| manhattan(cell, threat))
        .copied()
        .unwrap_or(from)
}

fn manhattan(a: GridCell, b: GridCell) -> i32 {
    (a.col - b.col).abs() + (a.row - b.row).abs()
}
