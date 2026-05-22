// Copyright (c) 2026 Nishant Sthalekar
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::any::TypeId;

use glam::Vec3;
use mithya_engine::{
    core::{EngineActionQueue, EngineEventListener, EngineEventQueue, Transform},
    engine::{
        resources::Time,
        system::{System, SystemUpdateContext},
        World,
    },
    input::{InputAction, InputActionEvent},
    navigation::{grid_cell::GridCell, NavGrid},
};

use crate::{
    maze::GRID_WIDTH,
    player::{Direction, PlayerState},
};

pub struct PacmanMovementSystem;

impl System for PacmanMovementSystem {
    fn initialize(&mut self, _world: &mut World) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn update(&mut self, ctx: &mut SystemUpdateContext) {
        let delta = ctx.world.resources.get::<Time>().map(|t| t.delta).unwrap_or(0.0);

        let mut new_queued: Option<Direction> = None;
        for event in ctx.events.iter_type::<InputActionEvent>() {
            let dir = match event.action {
                InputAction::MoveLeft  => Some(Direction::Left),
                InputAction::MoveRight => Some(Direction::Right),
                InputAction::MoveUp    => Some(Direction::Up),
                InputAction::MoveDown  => Some(Direction::Down),
                _ => None,
            };
            if dir.is_some() {
                new_queued = dir;
            }
        }

        // Compute new state while holding only immutable borrows.
        // NavGrid does not implement Clone, so we resolve everything here
        // before taking the mutable PlayerState borrow below.
        let result = {
            let nav = match ctx.world.resources.get::<NavGrid>() {
                Some(ng) => ng,
                None => return,
            };
            let player = match ctx.world.resources.get::<PlayerState>() {
                Some(p) => p,
                None => return,
            };
            compute_frame(player, nav, new_queued, delta)
        };

        if let Some(player) = ctx.world.resources.get_mut::<PlayerState>() {
            result.apply_to(player);
        }

        if let Some(t) = ctx.world.entity_manager.get_component_mut::<Transform>(result.entity_id) {
            t.position = result.visual_pos;
        }
    }

    fn as_event_listener_mut(&mut self) -> Option<&mut dyn EngineEventListener> {
        None
    }
}

impl EngineEventListener for PacmanMovementSystem {
    fn interested_events(&self) -> Vec<TypeId> {
        vec![]
    }
    fn on_events(&mut self, _e: &EngineEventQueue, _a: &mut EngineActionQueue, _w: &World) {}
}

#[derive(Clone, Copy)]
struct FrameResult {
    entity_id: u32,
    tile: GridCell,
    target: GridCell,
    current_direction: Direction,
    queued_direction: Direction,
    move_progress: f32,
    visual_pos: Vec3,
}

impl FrameResult {
    fn apply_to(self, player: &mut PlayerState) {
        player.tile = self.tile;
        player.target = self.target;
        player.current_direction = self.current_direction;
        player.queued_direction = self.queued_direction;
        player.move_progress = self.move_progress;
    }
}

fn compute_frame(
    player: &PlayerState,
    nav: &NavGrid,
    new_queued: Option<Direction>,
    delta: f32,
) -> FrameResult {
    let mut tile = player.tile;
    let mut target = player.target;
    let mut current = player.current_direction;
    let mut queued = new_queued.unwrap_or(player.queued_direction);
    let mut progress = player.move_progress;
    let speed: f32 = player.speed;

    step_movement(&mut tile, &mut target, &mut current, &mut queued, speed, &mut progress, nav, delta);

    let visual_pos = visual_position(tile, target, progress, nav);

    FrameResult {
        entity_id: player.entity_id,
        tile,
        target,
        current_direction: current,
        queued_direction: queued,
        move_progress: progress,
        visual_pos,
    }
}

fn step_movement(
    tile: &mut GridCell,
    target: &mut GridCell,
    current: &mut Direction,
    queued: &mut Direction,
    speed: f32,
    progress: &mut f32,
    nav: &NavGrid,
    delta: f32,
) {
    if *progress > 0.0 {
        *progress += speed * delta; // speed in tiles/sec
        if *progress >= 1.0 {
            *tile = *target;
            *progress = 0.0;
        } else {
            return;
        }
    }

    if *queued != Direction::None && try_start_move(tile, target, progress, *queued, nav) {
        *current = *queued;
        return;
    }

    if *current != Direction::None {
        try_start_move(tile, target, progress, *current, nav);
    }
}

fn try_start_move(
    tile: &GridCell,
    target: &mut GridCell,
    progress: &mut f32,
    dir: Direction,
    nav: &NavGrid,
) -> bool {
    let (dc, dr) = dir.delta();
    let next_col = wrap_col(tile.col + dc);
    let next_row = tile.row + dr;

    if nav.is_walkable(GridCell::new(next_col, next_row)) {
        *target = GridCell::new(next_col, next_row);
        *progress = f32::EPSILON;
        true
    } else {
        false
    }
}

fn wrap_col(col: i32) -> i32 {
    ((col % GRID_WIDTH as i32) + GRID_WIDTH as i32) % GRID_WIDTH as i32
}

fn visual_position(tile: GridCell, target: GridCell, progress: f32, nav: &NavGrid) -> Vec3 {
    let src = nav.cell_to_world(tile);

    if progress <= 0.0 {
        return src;
    }

    // Detect tunnel wrap: column jump larger than half the grid width.
    if (tile.col - target.col).abs() > GRID_WIDTH as i32 / 2 {
        return src;
    }

    let dst = nav.cell_to_world(target);
    src.lerp(dst, progress.min(1.0))
}
