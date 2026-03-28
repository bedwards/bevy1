# bevy1

A Bevy game development workspace accompanied by an extensive library of long-form essays on game development, engine architecture, tooling, and the philosophy of AI-assisted software engineering.

## Project Structure

```
bevy1/
├── games/              Game binaries
│   ├── hello_2d/       2D sprite-based demo with player movement
│   └── hello_3d/       3D scene with orbiting camera and shapes
├── libs/               Shared libraries
│   ├── shared/         Bevy plugins shared across all games
│   └── utils/          Pure Rust utilities (no Bevy dependency)
├── docs/               Long-form essay library
│   ├── bevy/           Bevy engine deep dives
│   ├── game-design/    Game design principles and theory
│   ├── vibe-coding/    AI-assisted development workflows
│   ├── github-workflows/ GitHub Issues, PRs, Actions, AI assistants
│   ├── tooling/        Open source developer tools
│   ├── philosophy/     Software engineering philosophy
│   ├── rust/           Rust language and ecosystem
│   └── bonus-topics/   Adjacent topics discovered during research
└── .github/workflows/  CI/CD pipelines
```

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (stable toolchain)
- System dependencies for Bevy (see [Bevy setup guide](https://bevyengine.org/learn/quick-start/getting-started/setup/))

### Running the Games

```bash
# 2D demo — arrow keys or WASD to move, Escape to quit
cargo run -p hello_2d

# 3D demo — watch shapes rotate with an orbiting camera, Escape to quit
cargo run -p hello_3d
```

### Development

```bash
cargo check --workspace          # Type-check everything
cargo clippy --workspace         # Lint
cargo fmt --all                  # Format
cargo test --workspace           # Run all tests
```

## Documentation Library

The `docs/` directory contains an extensive collection of long-form essays written from first principles. These are not reference documentation — they are flowing prose designed to build deep understanding. Topics span from the technical foundations of Bevy's Entity Component System to the philosophy of deterministic versus probabilistic code in AI-assisted development.

See [docs/INDEX.md](docs/INDEX.md) for a complete guide to the essay library.

## Technology

- **Engine**: [Bevy 0.18](https://bevyengine.org/) (January 2026)
- **Language**: Rust (2024 edition)
- **CI**: GitHub Actions (check, clippy, fmt, test, doc)

## License

Dual-licensed under MIT or Apache 2.0, at your option.
