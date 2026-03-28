# CLAUDE.md

## Project Overview
Bevy game development workspace with multiple games and shared libraries.
Accompanied by an extensive documentation library of long-form essays.

## Workspace Structure
- `games/` — Individual game binaries (hello_2d, hello_3d, and future games)
- `libs/` — Shared libraries (shared for Bevy plugins, utils for pure Rust helpers)
- `docs/` — Long-form essay library organized by topic
- `.github/workflows/` — CI/CD pipelines

## Build Commands
- `cargo check --workspace` — Type-check everything
- `cargo clippy --workspace --all-targets` — Lint
- `cargo fmt --all` — Format
- `cargo test --workspace` — Run all tests
- `cargo run -p hello_2d` — Run the 2D demo
- `cargo run -p hello_3d` — Run the 3D demo

## Conventions
- Bevy 0.18 patterns: Required Components (not Bundles), ChildOf/Children relationships
- Systems return Result where fallible
- Workspace dependencies in root Cargo.toml
- Dev profile optimizes dependencies (opt-level 2) for Bevy performance
- Essays are first-principles, flowing prose — minimal headers, no jargon without explanation
