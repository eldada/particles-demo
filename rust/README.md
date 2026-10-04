# Particles Demo

Interactive particle fountain demo built with **Rust + wgpu**. Ships as a standalone macOS binary (Metal on Mac) with keyboard, mouse, and preset-driven scenes.

## Build & run

```bash
cargo run --release
```

Optional flags:

```bash
cargo run --release -- --particles 4000 --preset fire --seed 7 --vsync true
```

| Flag | Default | Description |
|------|---------|-------------|
| `--particles N` | `1600` | Particles per system |
| `--preset` | `fountain` | `fountain`, `classic`, `fire`, `snow`, `galaxy` |
| `--seed` | `42` | RNG seed (reproducible demos) |
| `--vsync` | `true` | Present mode |
| `--width` / `--height` | `1280` / `800` | Window size |

Release binary:

```bash
cargo build --release
./target/release/particles-demo
```

Textures are **embedded** in the binary (`assets/star.png`, `assets/hud.png`).

## Controls

| Input | Action |
|-------|--------|
| Arrow keys | Add gravity (cumulative) |
| Space | Reset / re-randomize particles |
| R | Clear gravity |
| P | Pause |
| `,` / `.` | Slower / faster time |
| `[` / `]` | Halve / double particle count |
| 1–5 | Presets: Fountain / Classic / Fire / Snow / Galaxy |
| LMB | Attract particles |
| Shift + LMB | Repel particles |
| RMB drag | Orbit camera |
| Scroll | Zoom |
| H | Toggle HUD |
| F | Toggle fullscreen |
| Esc | Quit |

## Presets

1. **Fountain** — single upward water-like jet
2. **Classic** — four colored fountains
3. **Fire** — warm rising sparks
4. **Snow** — slow falling flakes
5. **Galaxy** — tilted spiral: particles emerge from the center and orbit outward
