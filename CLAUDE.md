# CLAUDE.md

No automated tests. Standard `cargo build / run / check / clippy`.

## Architecture

Built on **MithyaEngine** (`github.com/naash/MithyaEngine`). Game code plugs in via the `GameLogic` trait and custom `System` implementations.

### Coordinate system

Y-up world space. `NavGrid::cell_to_world` uses origin `(-224, 248, 0)` for the 28×31 grid. Row index increases downward; Y increases upward — `cell_to_world` accounts for this inversion.

### Navigation

`NavGrid` does not implement `Clone`. This is why the movement system uses a two-phase pattern: compute everything under immutable borrows → drop them → apply via mutable `PlayerState` borrow. The intermediate value is `FrameResult`.

### Movement system

Tile-to-tile committed movement. `current_direction` keeps running until blocked; `queued_direction` (buffered input) is applied at the next tile centre if the turn is valid.

Tunnel (col −1 ↔ col 27): `wrap_col` in `try_start_move` handles column wrapping; `visual_position` suppresses interpolation during tunnel transitions to prevent the sprite sliding backwards.

### Adding a new system

1. Create `src/<name>.rs`, implement `System` and `EngineEventListener`.
2. `mod <name>;` in `main.rs`.
3. `systems_manager.add_system(MySystem, world)` in `PacmanGame::initialize`.
