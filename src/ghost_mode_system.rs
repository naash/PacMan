// Copyright (c) 2026 Nishant Sthalekar
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::any::{Any, TypeId};

use glam;

use mithya_engine::{
    RandomMovement, Render,
    core::{
        EngineEventListener,
        engine_events::{EngineActionQueue, EngineEventQueue},
    },
    core::entity_manager::EntityManager,
    engine::{resources::Time, system::{System, SystemUpdateContext}, World},
    navigation::NavAgent,
};

use crate::ghost::{
    FRIGHTENED_DURATION, Ghost, GhostChase, GhostFrightened, GhostMode, GhostModeResource,
    GhostScatter, PowerPelletEatenEvent,
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
    frightened_timer: Option<f32>,
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
        }
        GhostMode::Scatter => {
            em.remove_component::<GhostScatter>(id);
        }
        GhostMode::Frightened => {
            em.remove_component::<GhostFrightened>(id);
        }
        GhostMode::Start => {}
    }
    let tint = match to {
        GhostMode::Chase      => Some([1.0_f32, 0.4, 0.4, 1.0]),
        GhostMode::Frightened => Some([0.4_f32, 0.4, 1.0, 1.0]),
        _                     => None,
    };
    if let Some(render) = em.get_component_mut::<Render>(id) {
        render.tint = tint;
    }

    match to {
        GhostMode::Chase => {
            em.remove_component::<RandomMovement>(id);
            em.add_component(id, GhostChase);
        }
        GhostMode::Scatter => {
            if em.get_component::<RandomMovement>(id).is_none() {
                em.add_component(id, RandomMovement::new());
            }
            em.add_component(id, GhostScatter);
        }
        GhostMode::Frightened => {
            em.add_component(id, GhostFrightened { timer: FRIGHTENED_DURATION });
            em.remove_component::<RandomMovement>(id);
            if let Some(nav) = em.get_component_mut::<NavAgent>(id) {
                nav.target_cell = None;
                nav.path.clear();
                nav.move_input = glam::Vec2::ZERO;
            }
        }
        GhostMode::Start => {
            if em.get_component::<RandomMovement>(id).is_none() {
                em.add_component(id, RandomMovement::new());
            }
        }
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

        if global_mode != self.last_printed_mode && global_mode != GhostMode::Frightened {
            println!("[GhostMode] {:?} → {:?}", self.last_printed_mode, global_mode);
            self.last_printed_mode = global_mode;
        }

        let frames: Vec<GhostFrame> = {
            let em = &ctx.world.entity_manager;
            em.query_component::<Ghost>()
                .into_iter()
                .map(|id| {
                    let current_mode = detect_mode(em, id);
                    let frightened_timer = em.get_component::<GhostFrightened>(id).map(|f| f.timer);
                    GhostFrame { entity_id: id, current_mode, frightened_timer }
                })
                .collect()
        };

        for frame in frames {
            if frame.current_mode == GhostMode::Frightened {
                let new_timer = frame.frightened_timer.unwrap_or(0.0) - delta;
                if new_timer <= 0.0 {
                    println!("[GhostMode] Frightened → {:?} (timer expired)", global_mode);
                    transition(&mut ctx.world.entity_manager, frame.entity_id, GhostMode::Frightened, global_mode);
                } else if let Some(f) = ctx.world.entity_manager.get_component_mut::<GhostFrightened>(frame.entity_id) {
                    f.timer = new_timer;
                }
            } else if frame.current_mode != global_mode {
                transition(&mut ctx.world.entity_manager, frame.entity_id, frame.current_mode, global_mode);
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

impl EngineEventListener for GhostModeSystem {
    fn interested_events(&self) -> Vec<TypeId> {
        vec![TypeId::of::<PowerPelletEatenEvent>()]
    }

    fn on_events(&mut self, _events: &EngineEventQueue, actions: &mut EngineActionQueue, _world: &World) {
        actions.push_anonymous(|world| {
            let ghost_ids: Vec<u32> = world.entity_manager.query_component::<Ghost>();
            for id in ghost_ids {
                let current = detect_mode(&world.entity_manager, id);
                if current == GhostMode::Frightened {
                    if let Some(f) = world.entity_manager.get_component_mut::<GhostFrightened>(id) {
                        f.timer = FRIGHTENED_DURATION;
                    }
                    if let Some(nav) = world.entity_manager.get_component_mut::<NavAgent>(id) {
                        nav.target_cell = None;
                        nav.path.clear();
                        nav.move_input = glam::Vec2::ZERO;
                    }
                } else {
                    println!("[GhostMode] {:?} → Frightened (power pellet)", current);
                    transition(&mut world.entity_manager, id, current, GhostMode::Frightened);
                }
            }
        });
    }
}
