// Copyright (c) 2026 Nishant Sthalekar
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::any::{Any, TypeId};

use mithya_engine::{
    core::{EngineEventListener, EngineActionQueue, EngineEventQueue},
    engine::{system::{System, SystemUpdateContext}, World},
};

use crate::config;
use crate::events::{PelletEatenEvent, PowerPelletEatenEvent, PlayerGhostCollisionEvent};
use crate::resources::ScoreResource;

pub struct ScoringSystem;

impl System for ScoringSystem {
    fn initialize(&mut self, _world: &mut World) {}

    fn update(&mut self, _ctx: &mut SystemUpdateContext) {}

    fn as_event_listener_mut(&mut self) -> Option<&mut dyn EngineEventListener> {
        Some(self)
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl EngineEventListener for ScoringSystem {
    fn interested_events(&self) -> Vec<TypeId> {
        vec![
            TypeId::of::<PelletEatenEvent>(),
            TypeId::of::<PowerPelletEatenEvent>(),
            TypeId::of::<PlayerGhostCollisionEvent>(),
        ]
    }

    fn on_events(&mut self, events: &EngineEventQueue, actions: &mut EngineActionQueue, _world: &World) {
        let pellet_count = events.iter_type::<PelletEatenEvent>().count();
        let power_pellet_count = events.iter_type::<PowerPelletEatenEvent>().count();
        let ghost_collisions: Vec<bool> = events.iter_type::<PlayerGhostCollisionEvent>()
            .map(|e| e.is_frightened)
            .collect();

        actions.push_anonymous(move |world| {
            if let Some(score) = world.resources.get_mut::<ScoreResource>() {
                if pellet_count > 0 {
                    let pellet_points = pellet_count as u32 * config::scoring::PELLET;
                    score.score += pellet_points;
                    println!("[Score] Pellets x{} = +{} points | Total score: {}", pellet_count, pellet_points, score.score);
                }

                if power_pellet_count > 0 {
                    let power_points = power_pellet_count as u32 * config::scoring::POWER_PELLET;
                    score.score += power_points;
                    println!("[Score] Power pellets x{} = +{} points | Total score: {}", power_pellet_count, power_points, score.score);
                    score.ghost_combo = 0;
                    println!("[Score] Ghost combo reset to 0");
                }

                for is_frightened in ghost_collisions {
                    if is_frightened {
                        let points = config::scoring::GHOST_COMBO[score.ghost_combo.min(3) as usize];
                        let combo_num = score.ghost_combo + 1;
                        score.score += points;
                        println!("[Score] Ghost #{} eaten = +{} points | Total score: {}", combo_num, points, score.score);
                        score.ghost_combo += 1;
                    }
                }
            }
        });
    }
}
