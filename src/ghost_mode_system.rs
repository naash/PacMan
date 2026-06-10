// Copyright (c) 2026 Nishant Sthalekar
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::any::Any;

use mithya_engine::{
    RandomMovement,
    core::EngineEventListener,
    core::entity_manager::EntityManager,
    engine::{resources::Time, system::{System, SystemUpdateContext}, World},
};

use crate::ghost::{
    FRIGHTENED_DURATION, Ghost, GhostChase, GhostFrightened, GhostMode, GhostModeResource,
    GhostScatter,
};

pub struct GhostModeSystem {
    last_printed_mode: GhostMode,
}

impl GhostModeSystem {
    pub fn new() -> Self {
        Self { last_printed_mode: GhostMode::Start }
    }
}

struct GhostFrame {
    entity_id: u32,
    current_mode: GhostMode,
}

fn detect_mode(em: &EntityManager, id: u32) -> GhostMode {
    if em.get_component::<GhostChase>(id).is_some() {
        GhostMode::Chase
    } else if em.get_component::<GhostScatter>(id).is_some() {
        GhostMode::Scatter
    } else if em.get_component::<GhostFrightened>(id).is_some() {
        GhostMode::Frightened
    } else {
        GhostMode::Start
    }
}

fn transition(em: &mut EntityManager, id: u32, from: GhostMode, to: GhostMode) {
    match from {
        GhostMode::Chase => {
            em.remove_component::<GhostChase>(id);
            em.add_component(id, RandomMovement::new());
        }
        GhostMode::Scatter    => { em.remove_component::<GhostScatter>(id); }
        GhostMode::Frightened => { em.remove_component::<GhostFrightened>(id); }
        GhostMode::Start      => {}
    }
    match to {
        GhostMode::Chase => {
            em.remove_component::<RandomMovement>(id);
            em.add_component(id, GhostChase);
        }
        GhostMode::Scatter    => em.add_component(id, GhostScatter),
        GhostMode::Frightened => em.add_component(id, GhostFrightened { timer: FRIGHTENED_DURATION }),
        GhostMode::Start      => {}
    }
}


impl System for GhostModeSystem {
    fn initialize(&mut self, _world: &mut World) {}

    fn update(&mut self, ctx: &mut SystemUpdateContext) {
        let delta = ctx.world.resources.get::<Time>().map(|t| t.delta).unwrap_or(0.0);

        if let Some(res) = ctx.world.resources.get_mut::<GhostModeResource>() {
            res.advance(delta);
        }

        let global_mode = ctx.world.resources.get::<GhostModeResource>()
            .map(|r| r.current_mode())
            .unwrap_or(GhostMode::Chase);

        if global_mode != self.last_printed_mode {
            println!("[GhostMode] {:?} → {:?}", self.last_printed_mode, global_mode);
            self.last_printed_mode = global_mode;
        }

        let frames: Vec<GhostFrame> = {
            let em = &ctx.world.entity_manager;
            em.query_component::<Ghost>()
                .into_iter()
                .map(|id| GhostFrame { entity_id: id, current_mode: detect_mode(em, id) })
                .collect()
        };

        for frame in frames {
            if frame.current_mode != global_mode {
                transition(&mut ctx.world.entity_manager, frame.entity_id, frame.current_mode, global_mode);
            }
        }
    }

    fn as_event_listener_mut(&mut self) -> Option<&mut dyn EngineEventListener> {
        None
        // TODO: listen for PowerPelletEatenEvent to trigger Frightened mode (Iteration 4)
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
