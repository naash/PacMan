# CLAUDE.md

No automated tests. Standard `cargo build / run / check / clippy`.

## Architecture

Built on **MithyaEngine** (`github.com/naash/MithyaEngine`). Game plugs in via `GameLogic` trait; game logic lives in custom `System` implementations with `EngineEventListener` for cross-system communication.

## Configuration

All static values (spawn positions, scoring, timings, speeds, colors, sprite scales) are centralized in `src/config.rs` for easy tweaking. Organized by module: `maze`, `spawn`, `scoring`, `timing`, `movement`, `sprites`, `colors`, `camera`, `pellets`, `game_state`.

## Event-Driven Architecture

Systems emit and listen to events for loose coupling:
- **Collision** → `PlayerGhostCollisionEvent` → triggers death/scoring/ghost destruction
- **Collection** → `PelletEatenEvent`, `PowerPelletEatenEvent` → triggers scoring & mode changes
- Use `actions.push_anonymous()` to modify world state safely in event handlers (avoids borrow conflicts)

## System Execution Order

Order matters when events are processed same frame they're emitted:
1. `GhostPlayerCollisionSystem` — detect collisions, emit events, destroy frightened ghosts
2. `GameStateSystem` — handle player death/respawn
3. `ScoringSystem` — update score from events
4. `GhostModeSystem` — transition ghost modes based on events

## Navigation & Coordinate System

Y-up world space. `NavGrid::cell_to_world` uses origin `(-224, 248, 0)` for the 28×31 grid. Row index increases downward; Y increases upward — conversion handles inversion.

`NavGrid` doesn't implement `Clone` — use immutable borrow to read, drop, then apply changes via mutable `PlayerState` borrow.

Tunnel wraps at col −1 ↔ col 27; `wrap_col` in `try_start_move` handles it.

## Adding a New System

1. Create `src/<name>.rs`, implement `System` and optionally `EngineEventListener`
2. Add `mod <name>;` in `main.rs`
3. Register with `systems_manager.add_system(MySystem, world)` in `PacmanGame::initialize`
4. If using events, add `TypeId` to `interested_events()` and implement `on_events()`
