// Copyright (c) 2026 Nishant Sthalekar
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::any::{Any, TypeId};

use glam::{Vec2, Vec3};
use mithya_engine::{
    Movement, Transform,
    core::{EngineActionQueue, EngineEventListener, EngineEventQueue},
    engine::{
        resources::Time,
        system::{System, SystemUpdateContext},
        World,
    },
    navigation::{grid_cell::GridCell, NavGrid},
};

use crate::maze::{Maze, GRID_WIDTH};
use crate::player::{Direction, PlayerState};

pub struct PacmanMovementSystem;

struct FrameResult {
    new_current_cell: GridCell,
    new_target_cell: Option<GridCell>,
    new_current_dir: Direction,
    new_queued_dir: Direction,
    snap_to: Option<Vec3>,
    move_intent: Vec2,
}

impl System for PacmanMovementSystem {
    fn initialize(&mut self, _world: &mut World) -> () {
    }

    fn update(&mut self, ctx: &mut SystemUpdateContext) {
        let player_id = match ctx.world.resources.get::<PlayerState>() {
            Some(p) => p.entity_id,
            None => return,
        };

        let delta = ctx.world.resources.get::<Time>().map(|t| t.delta).unwrap_or(0.0);

        // Consume and zero raw keyboard intent; read speed.
        // Zeroing prevents MovementSystem (if re-enabled) from double-applying the vector.
        let (desired_dir, speed) = match ctx.world.entity_manager.get_component_mut::<Movement>(player_id) {
            Some(m) => {
                let dir = vec2_to_direction(m.intent);
                m.intent = Vec2::ZERO;
                (dir, m.impulse)
            }
            None => return,
        };

        let current_position = match ctx.world.entity_manager.get_component::<Transform>(player_id) {
            Some(t) => t.position,
            None => return,
        };

        let result = {
            let nav = match ctx.world.resources.get::<NavGrid>() {
                Some(n) => n,
                None => return,
            };
            let maze = match ctx.world.resources.get::<Maze>() {
                Some(m) => m,
                None => return,
            };
            let tunnel_rows = maze.tunnel_rows();
            let player = match ctx.world.resources.get::<PlayerState>() {
                Some(p) => p,
                None => return,
            };
            compute_frame(player, current_position, desired_dir, nav, &tunnel_rows)
        };

        // Snap to tile center on arrival, then step in the new direction for this frame.
        let base = result.snap_to.unwrap_or(current_position);
        let new_pos = base + result.move_intent.extend(0.0) * (speed * delta);
        if let Some(t) = ctx.world.entity_manager.get_component_mut::<Transform>(player_id) {
            t.position = new_pos;
        }

        if let Some(player) = ctx.world.resources.get_mut::<PlayerState>() {
            player.current_cell = result.new_current_cell;
            player.target_cell = result.new_target_cell;
            player.current_direction = result.new_current_dir;
            player.queued_direction = result.new_queued_dir;
        }
    }

    fn as_event_listener_mut(&mut self) -> Option<&mut dyn EngineEventListener> {
        None
    }

    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}

impl EngineEventListener for PacmanMovementSystem {
    fn interested_events(&self) -> Vec<TypeId> {
        vec![]
    }
    fn on_events(&mut self, _e: &EngineEventQueue, _a: &mut EngineActionQueue, _w: &World) {}
}

fn wrap_col(cell: GridCell, tunnel_rows: &[usize]) -> GridCell {
    if !tunnel_rows.contains(&(cell.row as usize)) {
        return cell;
    }
    if cell.col < 0 {
        GridCell::new(GRID_WIDTH as i32 - 1, cell.row)
    } else if cell.col >= GRID_WIDTH as i32 {
        GridCell::new(0, cell.row)
    } else {
        cell
    }
}

fn compute_frame(
    player: &PlayerState,
    position: Vec3,
    desired_dir: Option<Direction>,
    nav: &NavGrid,
    tunnel_rows: &[usize],
) -> FrameResult {
    let queued = desired_dir.unwrap_or(player.queued_direction);

    let (arrived, snap_to) = match player.target_cell {
        Some(target) => {
            let target_world = nav.cell_to_world(target);
            if has_passed_center(position, target_world, player.current_direction) {
                (true, Some(target_world))
            } else {
                (false, None)
            }
        }
        None => (false, None),
    };

    // Still mid-tile: keep moving, buffer the queued direction for the next tile centre.
    if player.target_cell.is_some() && !arrived {
        return FrameResult {
            new_current_cell: player.current_cell,
            new_target_cell: player.target_cell,
            new_current_dir: player.current_direction,
            new_queued_dir: queued,
            snap_to: None,
            move_intent: direction_to_intent(player.current_direction),
        };
    }

    // At a tile centre: decide the next move.
    // Blocked directions are discarded here; they will be re-buffered next frame if the key is still held.
    let from_cell = if arrived { player.target_cell.unwrap() } else { player.current_cell };

    let mut new_target = None;
    let mut new_dir = player.current_direction;
    let mut new_queued = Direction::None;

    if queued != Direction::None {
        let candidate = wrap_col(step(from_cell, queued), tunnel_rows);
        if nav.is_walkable(candidate) {
            new_target = Some(candidate);
            new_dir = queued;
            new_queued = queued;
        }
        // Blocked: new_queued stays None — invalid input discarded at tile centre.
    }

    if new_target.is_none() && player.current_direction != Direction::None {
        let candidate = wrap_col(step(from_cell, player.current_direction), tunnel_rows);
        if nav.is_walkable(candidate) {
            new_target = Some(candidate);
        }
    }

    FrameResult {
        new_current_cell: from_cell,
        new_target_cell: new_target,
        new_current_dir: new_dir,
        new_queued_dir: new_queued,
        snap_to,
        move_intent: if new_target.is_some() { direction_to_intent(new_dir) } else { Vec2::ZERO },
    }
}

// Y-up world space: Up means row--, world Y increases; Down means row++, world Y decreases.
fn has_passed_center(pos: Vec3, target: Vec3, dir: Direction) -> bool {
    match dir {
        Direction::Right => pos.x >= target.x,
        Direction::Left  => pos.x <= target.x,
        Direction::Up    => pos.y >= target.y,
        Direction::Down  => pos.y <= target.y,
        Direction::None  => false,
    }
}

fn direction_to_intent(dir: Direction) -> Vec2 {
    match dir {
        Direction::Right => Vec2::new( 1.0,  0.0),
        Direction::Left  => Vec2::new(-1.0,  0.0),
        Direction::Up    => Vec2::new( 0.0,  1.0),
        Direction::Down  => Vec2::new( 0.0, -1.0),
        Direction::None  => Vec2::ZERO,
    }
}

fn step(cell: GridCell, dir: Direction) -> GridCell {
    let (dc, dr) = dir.delta();
    GridCell::new(cell.col + dc, cell.row + dr)
}

fn vec2_to_direction(intent: Vec2) -> Option<Direction> {
    if intent == Vec2::ZERO {
        return None;
    }
    if intent.x.abs() >= intent.y.abs() {
        if intent.x > 0.0 { Some(Direction::Right) } else { Some(Direction::Left) }
    } else {
        if intent.y > 0.0 { Some(Direction::Up) } else { Some(Direction::Down) }
    }
}
