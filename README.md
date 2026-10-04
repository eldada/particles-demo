# Particles Demo

Two interactive particle examples. Each lives in its own directory and has its own setup.

## [rust](rust/)

Desktop particle fountain built with Rust and wgpu. Ships as a standalone binary (Metal on macOS) with keyboard, mouse, and preset-driven scenes.

```bash
cd rust
cargo run --release
```

Details, flags, controls, and presets: [rust/README.md](rust/README.md).

## [npm](npm/)

Browser page (Ember Field): a small fire of particles inside a slowly rotating wireframe cube. Built with TypeScript and Three.js.

```bash
cd npm
npm install
npm run dev
```

Open http://127.0.0.1:5173/. Details, the static build, and controls: [npm/README.md](npm/README.md).
