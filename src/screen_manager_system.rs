// Copyright (c) 2026 Nishant Sthalekar
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::any::{Any, TypeId};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

use mithya_engine::{
    WorldConfig,
    core::{EngineActionQueue, EngineEventListener, EngineEventQueue},
    engine::{system::{System, SystemUpdateContext}, World},
    input::{InputAction, InputActionEvent},
    navigation::grid_cell::GridCell,
};

use crate::config;
use crate::level_progression_system::spawn_level;
use crate::maze::{Maze, build_nav_grid};
use crate::player::PlayerState;
use crate::resources::GameScreen;

pub struct ScreenManagerSystem {
    action_signal: Arc<AtomicBool>,
    confirm_pressed: bool,
}

impl ScreenManagerSystem {
    pub fn new(action_signal: Arc<AtomicBool>) -> Self {
        Self {
            action_signal,
            confirm_pressed: false,
        }
    }
}

impl System for ScreenManagerSystem {
    fn initialize(&mut self, _world: &mut World) {}

    fn update(&mut self, ctx: &mut SystemUpdateContext) {
        let screen = ctx.world.resources.get::<GameScreen>().copied();

        // Keep WorldConfig.paused in sync with the current screen
        let should_pause = !matches!(screen, Some(GameScreen::Playing));
        if let Some(config) = ctx.world.resources.get_mut::<WorldConfig>() {
            config.paused = should_pause;
        }

        let Some(GameScreen::Title) = screen else {
            self.confirm_pressed = false;
            return;
        };

        let triggered = self.confirm_pressed || self.action_signal.swap(false, Ordering::Relaxed);
        self.confirm_pressed = false;

        if triggered {
            let maze = Maze::new();
            let nav_grid = build_nav_grid(&maze);
            let pacman_id = spawn_level(ctx.world, maze, nav_grid);

            let spawn_cell = GridCell::new(
                config::spawn::PACMAN_COL as i32,
                config::spawn::PACMAN_ROW as i32,
            );
            ctx.world.resources.insert(PlayerState::new(pacman_id, spawn_cell));

            if let Some(screen) = ctx.world.resources.get_mut::<GameScreen>() {
                *screen = GameScreen::Playing;
            }
        }
    }

    fn is_pausable(&self) -> bool { false }

    fn as_event_listener_mut(&mut self) -> Option<&mut dyn EngineEventListener> {
        Some(self)
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl EngineEventListener for ScreenManagerSystem {
    fn interested_events(&self) -> Vec<TypeId> {
        vec![TypeId::of::<InputActionEvent>()]
    }

    fn on_events(&mut self, events: &EngineEventQueue, _actions: &mut EngineActionQueue, _world: &World) {
        for event in events.iter_type::<InputActionEvent>() {
            if event.action == InputAction::Confirm {
                self.confirm_pressed = true;
            }
        }
    }
}
