# Documentation Library Index

A collection of 62 long-form essays totaling over 148,000 words, written from first principles. Each essay is designed to be read as flowing prose — minimal headers, no jargon without explanation, a joy to read aloud.

## Bevy Engine (12 essays)

1. [What is Bevy?](bevy/01-what-is-bevy.md) — What a game engine is, how Bevy differs from Unity/Unreal/Godot, and why you might choose it
2. [The Entity Component System](bevy/02-entity-component-system.md) — Entities, components, systems, archetypes, and why ECS replaced inheritance
3. [Systems and Scheduling](bevy/03-systems-and-scheduling.md) — Automatic parallelism, schedules, system sets, and run conditions
4. [Resources and State Management](bevy/04-resources-and-state.md) — Singleton data, game states, transitions, events vs messages
5. [Components, Queries, and Data Modeling](bevy/05-components-and-queries.md) — Designing components, required components, query patterns, and filters
6. [The Rendering Pipeline](bevy/06-rendering-pipeline.md) — Dual-world architecture, PBR, GPU-driven rendering, ray tracing
7. [Building User Interfaces](bevy/07-user-interface.md) — ECS-native UI, Taffy layout, widgets, immediate-mode vs retained-mode
8. [The Asset System](bevy/08-asset-system.md) — Async loading, handles, hot-reloading, and the glTF pipeline
9. [Relationships and Hierarchy](bevy/09-relationships-and-hierarchy.md) — ChildOf/Children, entity disabling, cloning, custom relationships
10. [Observers, Events, and Messages](bevy/10-observers-and-events.md) ��� Observable events, buffered messages, lifecycle hooks
11. [The Plugin Architecture and Ecosystem](bevy/11-plugins-and-ecosystem.md) — Plugins, Avian physics, bevy_egui, organizing game code
12. [Performance and Optimization](bevy/12-performance-and-optimization.md) — Cache-friendly ECS, GPU-driven rendering, profiling

## Game Design (6 essays)

1. [What is Game Design?](game-design/01-what-is-game-design.md) — Design vs development, the MDA framework, the role of the designer
2. [Game Feel and Juice](game-design/02-game-feel-and-juice.md) — Responsiveness, screen shake, easing curves, the tactile quality of interaction
3. [Player Psychology](game-design/03-player-psychology.md) — Flow state, motivation, the curiosity loop, challenge curves
4. [Level Design](game-design/04-level-design.md) — Spatial design, teaching through environment, gating, pacing
5. [Balance and Systems](game-design/05-balance-and-systems.md) — Emergent gameplay, feedback loops, economy design
6. [Procedural Generation](game-design/06-procedural-generation.md) — Randomness vs authored content, Perlin noise, wave function collapse

## Philosophy (3 essays)

1. [Deterministic and Probabilistic Code](philosophy/01-deterministic-and-probabilistic.md) — Compilers and LLMs, certainty and probability, the verification sandwich
2. [Measurement and Self-Improvement](philosophy/02-measurement-and-self-improvement.md) — Empirical approaches to performance, process efficiency, Goodhart's Law
3. [Guardrails and Verification](philosophy/03-guardrails-and-verification.md) — Layered verification, type systems to CI pipelines, poka-yoke

## Vibe Coding (5 essays)

1. [What is Vibe Coding?](vibe-coding/01-what-is-vibe-coding.md) — The Karpathy spectrum, when AI generation works and when it doesn't
2. [Claude Code CLI](vibe-coding/02-claude-code-cli.md) — Tools, permissions, hooks, MCP servers, slash commands, CLAUDE.md
3. [Effective Prompting](vibe-coding/03-effective-prompting.md) — Context, examples, constraints, iterating on output
4. [Human-AI Collaboration](vibe-coding/04-human-ai-collaboration.md) — Delegation, review, trust, the feedback loop
5. [Workflows and Patterns](vibe-coding/05-workflows-and-patterns.md) — Plan-then-execute, test-driven AI dev, exploratory prototyping, loops

## GitHub Workflows (5 essays)

1. [GitHub as Development Platform](github-workflows/01-github-as-development-platform.md) — Issues, PRs, Actions, Discussions as integrated platform
2. [Issues and Project Management](github-workflows/02-issues-and-project-management.md) — Labels, milestones, templates, tracking work
3. [Pull Requests and Code Review](github-workflows/03-pull-requests-and-code-review.md) — Small PRs, review process, merge strategies
4. [GitHub Actions and CI/CD](github-workflows/04-github-actions-and-cicd.md) — Workflows, matrix builds, caching, Rust CI pipeline design
5. [AI Code Assistants](github-workflows/05-ai-code-assistants.md) — Claude GitHub App, Gemini Code Assist, Copilot, complementary roles

## Tooling (4 essays)

1. [The Command Line Renaissance](tooling/01-command-line-renaissance.md) — ripgrep, fd, bat, delta — modern replacements for classic Unix tools
2. [Rust Development Tools](tooling/02-rust-development-tools.md) — bacon, cargo-nextest, cargo-flamegraph, just, tokei, hyperfine
3. [MCP Servers](tooling/03-mcp-servers.md) — Model Context Protocol, extending AI assistants, the MCP ecosystem
4. [Editor and IDE Integration](tooling/04-editor-and-ide-integration.md) — rust-analyzer, VS Code, Zed, debugging Bevy games

## Rust (3 essays)

1. [Why Rust for Games](rust/01-why-rust-for-games.md) — Memory safety without GC, fearless concurrency, zero-cost abstractions
2. [Ownership and Borrowing](rust/02-ownership-and-borrowing.md) — Rust's central innovation, moves, borrows, the borrow checker
3. [Cargo and the Ecosystem](rust/03-cargo-and-ecosystem.md) — Build system, crates.io, workspaces, testing, documentation culture

## Bonus Topics (24 essays)

1. [The Game Loop](bonus-topics/01-the-game-loop.md) — The heartbeat of interactive software, delta time, fixed timestep
2. [Linear Algebra for Games](bonus-topics/02-linear-algebra-for-games.md) — Vectors, matrices, quaternions, transforms, interpolation
3. [State Machines in Games](bonus-topics/03-state-machines-in-games.md) — Finite state machines, hierarchical states, behavior trees
4. [Spatial Data Structures](bonus-topics/04-spatial-data-structures.md) — Grids, quadtrees, BVH, spatial hashing, proximity queries
5. [Audio in Games](bonus-topics/05-audio-in-games.md) — Sound triggering, mixing, spatial audio, sound design
6. [Color Theory for Games](bonus-topics/06-color-theory-for-games.md) — Color spaces, gamma correction, accessibility, visual communication
7. [Animation Systems](bonus-topics/07-animation-systems.md) — Keyframe, skeletal, procedural animation, blending, state machines
8. [Networking for Games](bonus-topics/08-networking-for-games.md) — Client-server, latency compensation, prediction, rollback
9. [Input Handling](bonus-topics/09-input-handling.md) — Hardware to game actions, dead zones, buffering, coyote time
10. [Debugging Games](bonus-topics/10-debugging-games.md) — Debug overlays, gizmos, Tracy profiler, time manipulation
11. [Scene Management](bonus-topics/11-scene-management.md) — Loading worlds, saving state, scene-state relationships
12. [Shaders and Materials](bonus-topics/12-shaders-and-materials.md) — GPU programming, vertex/fragment shaders, WGSL, custom materials
13. [Testing Games](bonus-topics/13-testing-games.md) — Unit tests, system tests, property-based testing, visual testing
14. [Entity Patterns](bonus-topics/14-entity-patterns.md) — Marker components, spawn patterns, Commands, archetype design
15. [Camera Systems](bonus-topics/15-camera-systems.md) — Projection, follow patterns, screen shake, Bevy's camera controllers
16. [Tilemaps and 2D Worlds](bonus-topics/16-tilemaps-and-2d-worlds.md) — Tile-based design, autotiling, collision, parallax scrolling
17. [Pathfinding](bonus-topics/17-pathfinding.md) — A*, navigation meshes, flow fields, hierarchical pathfinding
18. [WebAssembly and Cross-Platform](bonus-topics/18-webassembly-and-cross-platform.md) — WASM, browser targets, no_std, multi-platform CI
19. [Open Source Game Development](bonus-topics/19-open-source-game-development.md) — Building in public, Bevy's governance, contributing, licensing
20. [Save Systems](bonus-topics/20-save-systems.md) — Serialization, versioning, autosave, what to save vs reconstruct
21. [Accessibility in Games](bonus-topics/21-accessibility-in-games.md) — Input, visual, auditory, cognitive accessibility, Xbox guidelines
22. [Data-Oriented Design](bonus-topics/22-data-oriented-design.md) — Cache coherency, SoA vs AoS, thinking in transformations
23. [The Art of Playtesting](bonus-topics/23-the-art-of-playtesting.md) — Observing players, structured sessions, timing, iteration
24. [Particle Systems](bonus-topics/24-particle-systems.md) — Emitters, simulation, rendering, visual juice, emergence
