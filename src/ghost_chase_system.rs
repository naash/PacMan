// Copyright (c) 2026 Nishant Sthalekar
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::any::Any;

use mithya_engine::{
    MoveToEvent,
    core::{EngineEventListener, entity_manager::EntityManager},
    engine::{system::{System, SystemUpdateContext}, World},
    navigation::{grid_cell::GridCell, NavAgent, NavGrid},
};

use crate::ghost::{Ghost, GhostChase, GhostKind};
use crate::maze::{GRID_HEIGHT, GRID_WIDTH};
use crate::player::{Direction, PlayerState};

pub struct GhostChaseSystem;

impl System for GhostChaseSystem {
    fn initialize(&mut self, _world: &mut World) {}

    fn update(&mut self, ctx: &mut SystemUpdateContext) {
        let (player_cell, player_dir) = match ctx.world.resources.get::<PlayerState>() {
            Some(p) => (p.current_cell, p.current_direction),
            None => return,
        };

        if ctx.world.resources.get::<NavGrid>().is_none() {
            return;
        }

        let blinky_cell = find_blinky_cell(&ctx.world.entity_manager);

        let entities = ctx.world.entity_manager.query_two_components::<Ghost, GhostChase>();
        for entity_id in entities {
            let (kind, scatter_corner, is_idle, my_cell) = {
                let ghost = ctx.world.entity_manager.get_component::<Ghost>(entity_id);
                let nav   = ctx.world.entity_manager.get_component::<NavAgent>(entity_id);
                match (ghost, nav) {
                    (Some(g), Some(n)) => (g.kind, g.scatter_corner, n.is_idle(), n.current_cell),
                    _ => continue,
                }
            };

            if !is_idle {
                continue;
            }

            let raw_target = match kind {
                GhostKind::Blinky => player_cell,
                GhostKind::Pinky  => pinky_target(player_cell, player_dir),
                GhostKind::Inky   => inky_target(player_cell, player_dir, blinky_cell.unwrap_or(player_cell)),
                GhostKind::Clyde  => clyde_target(player_cell, my_cell, scatter_corner),
            };

            // Fall back to player's cell if the computed target is a wall or out of bounds.
            let target = {
                let nav = ctx.world.resources.get::<NavGrid>().unwrap();
                if nav.is_walkable(raw_target) { raw_target } else { player_cell }
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

fn clamp_to_grid(cell: GridCell) -> GridCell {
    GridCell::new(
        cell.col.clamp(0, GRID_WIDTH as i32 - 1),
        cell.row.clamp(0, GRID_HEIGHT as i32 - 1),
    )
}

// Pinky targets 4 tiles ahead of Pac-Man in his current direction.
fn pinky_target(player_cell: GridCell, dir: Direction) -> GridCell {
    let (dc, dr) = dir.delta();
    clamp_to_grid(GridCell::new(player_cell.col + dc * 4, player_cell.row + dr * 4))
}

// Inky reflects Blinky through a pivot 2 tiles ahead of Pac-Man.
fn inky_target(player_cell: GridCell, dir: Direction, blinky_cell: GridCell) -> GridCell {
    let (dc, dr) = dir.delta();
    let pivot_col = player_cell.col + dc * 2;
    let pivot_row = player_cell.row + dr * 2;
    clamp_to_grid(GridCell::new(pivot_col * 2 - blinky_cell.col, pivot_row * 2 - blinky_cell.row))
}

// Clyde targets Pac-Man when far (>8 tiles), otherwise retreats to scatter corner.
fn clyde_target(player_cell: GridCell, my_cell: GridCell, scatter_corner: GridCell) -> GridCell {
    let dist = (player_cell.col - my_cell.col).abs() + (player_cell.row - my_cell.row).abs();
    if dist > 8 { player_cell } else { scatter_corner }
}

fn find_blinky_cell(em: &EntityManager) -> Option<GridCell> {
    em.query_component::<Ghost>()
        .into_iter()
        .find(|&id| {
            em.get_component::<Ghost>(id)
                .map(|g| g.kind == GhostKind::Blinky)
                .unwrap_or(false)
        })
        .and_then(|id| em.get_component::<NavAgent>(id))
        .map(|a| a.current_cell)
}
