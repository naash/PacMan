# Pac-Man

A Pac-Man clone written in Rust, built on top of [MithyaEngine](https://github.com/naash/MithyaEngine).

## Features

- 28×31 tile maze with tunnel wrapping
- Four ghosts — Blinky, Pinky, Inky, Clyde — with chase, scatter, and frightened modes
- Pellets (10 pts), power pellets (50 pts), and ghost combo scoring (200 → 400 → 800 → 1600)
- 3 lives with death/respawn and brief invulnerability window
- Level progression with increasing difficulty
- egui HUD showing score, lives, and current level
- Title screen, level-complete overlay, and game-over screen

## Controls

| Action | Keys |
|--------|------|
| Move | Arrow keys or WASD |
| Start / Play Again | Enter or Space |

## Building

Requires a local checkout of MithyaEngine at the path in `Cargo.toml`.

```
cargo run
```

Standard tooling: `cargo build`, `cargo check`, `cargo clippy`.

## License

MIT
