# Copilot Instructions for windows-rs

Read this file at the start of every session. It contains the essential commands,
conventions, and architecture knowledge needed to work on this repository.

## Workflow

Do not create git commits automatically. The maintainer reviews all changes first
and handles commits manually. Make changes locally, run fmt/clippy/tests to verify,
and report back.

## Repository Layout

Cargo workspace (`resolver = "3"`). Members are globbed from:

- `crates/libs/*` - the published/library crates (`windows`, `windows-sys`,
  `windows-core`, plus `windows-bindgen`, `metadata`, `rdl`, `riddle`, and the
  newer `reactor`/`canvas`/`webview`/`window` crates). See `docs/readme.md` for
  the full categorized crate index, and `docs/crates/<crate>.md` per crate.
- `crates/tools/*` - code generators and CI helpers, run via `cargo run -p tool_*`.
- `crates/tests/*/*` - test crates; `crates/tests/libs/<crate>` mirrors each
  library crate (e.g. `test_reactor`, `test_webview`). Crate names are `test_<dir>`.
- `crates/samples/*/*` - runnable examples.

The crates fall into rough groups (see `docs/readme.md` for the authoritative list):
core & errors (`windows-core`, `windows-result`, `windows-strings`); values &
collections (`numerics`, `collections`, `reference`, `time`); async & threading
(`future`, `threading`); system services (`registry`, `services`, `version`); COM
macros & linking (`implement`, `interface`, `link`); UI & graphics
(`reactor`, `canvas`, `webview`, `window`, `animation`, `reactor-setup`); codegen &
metadata tooling (`bindgen`, `metadata`, `rdl`, `riddle`, `cppwinrt`); and the full
API projection (`windows`, `windows-sys`).

## Before Finalizing Changes

```sh
cargo fmt --all
```

CI enforces rustfmt. Always format before finalizing changes.

## Build & Test Commands

### Core crates

```sh
cargo check -p windows-core --quiet
cargo check -p windows --quiet
cargo clippy -p <crate> --all-targets
cargo test -p <crate>

# Run a single test by name filter
cargo test -p <crate> <test_name_substring>
```

CI sets `RUSTFLAGS: -D warnings`, so any warning fails the build. Workspace-wide
lints are configured in the root `Cargo.toml` (`[workspace.lints]`) - clippy lints
like `uninlined_format_args`, `redundant_clone`, and `semicolon_if_nothing_returned`
are promoted to warnings and therefore enforced.

### Reactor

```sh
# Regenerate codegen (after editing winui.toml or tool_reactor source)
cargo run -p tool_reactor --quiet

# Regenerate bindings (after editing filter .txt files)
cargo run -p tool_bindings --quiet

# Verify reactor compiles
cargo check -p windows-reactor --quiet

# Unit tests (headless, fast)
cargo test -p test_reactor --quiet

# Integration tests (launches WinUI window)
cargo run -p test_reactor_selftest
cargo run -p test_reactor_selftest -- --headless    # CI mode
cargo run -p test_reactor_selftest -- --filter Name  # single fixture

# Clippy
cargo clippy -p windows-reactor --all-targets
```

### Canvas

```sh
cargo check -p windows-canvas --quiet
cargo test -p windows-canvas --quiet
cargo clippy -p windows-canvas --all-targets
```

### Full workspace

```sh
cargo run -p tool_clippy_all    # runs clippy across all crates
```

## Code Generation Pipeline

**Never hand-edit generated files.** Generated outputs are committed, and CI fails
if regenerating produces a diff (the `gen` workflow runs each `cargo run -p tool_*`
and rejects any change; the `test` workflow likewise fails if tests modify tracked
files). After editing generators or filters, re-run the tool and commit the result.

The core `windows` / `windows-sys` crates are generated from Windows metadata
(`.winmd`) via `windows-bindgen` (driven by `tool_package`). `windows-metadata`,
`windows-rdl`, and `riddle` support reading/authoring that metadata. The reactor /
canvas / webview pipelines layer on top:

1. **`tool_reactor`** - reads `crates/tools/reactor/src/winui.toml` + WinUI `.winmd`
   metadata -> generates `generated.rs`, `generated_set_prop.rs`,
   `generated_attach_event.rs`, and `generated.txt` filter entries.

2. **`tool_bindings`** - reads filter `.txt` files from `crates/tools/bindings/src/` -> runs `windows-bindgen` -> generates `bindings.rs` in each crate:
   - `crates/libs/canvas/src/bindings.rs` (from `canvas.txt`)
   - `crates/libs/time/src/bindings.rs`, `numerics`, `reference`, etc.

3. **`tool_package`** - generates the published `windows` and `windows-sys` package
   crates using `--package` mode (per-namespace files + Cargo.toml features).

4. After regenerating, always verify: `cargo check -p <affected-crate> --quiet`

## Key Architecture Facts

### Crate relationships

- `windows-core` is the foundation - almost everything depends on it.
- `windows` is the umbrella crate that re-exports from `windows-core`,
  `windows-numerics`, `windows-time`, `windows-collections`, `windows-reference`, etc.
- `windows-reactor` depends on `windows-core` (not `windows`) and uses minimal
  bindings generated with `--minimal --flat` mode.
- `windows-canvas` similarly uses minimal bindings for D2D/DXGI/DWrite/WIC.
- `windows-animation` wraps Win32 UIAnimation Manager COM APIs.

### Reactor architecture

- WinUI backend is in `crates/libs/reactor/src/backend/winui/`.
- The TOML config (`winui.toml`) declares ~52 WinUI controls. Keys are WinUI
  metadata names; the tool infers types, setter patterns, and event handlers
  from `.winmd` files.
- COM casts: classes Deref to their default interface (zero-cost). Only cast
  to non-default parent interfaces. The `Param` trait handles parent-class
  conversions automatically.

### Canvas architecture

- Canvas wraps D2D/DXGI behind safe Rust types (`GpuDevice`, `SwapChain`,
  `DrawingSession`, `PathBuilder`).
- `animated_canvas()` (reactor feature) renders on UI thread via
  `CompositionTarget::Rendering`.
- Device-lost recovery is automatic.

## Conventions

- **Panics**: Use `panic!` only for invariant violations. Use `diag::` helpers
  for missing features (warn in debug, no-op in release).
- **`.unwrap()` over `.expect("...")`** - the panic hook provides full context.
- **No `thread_local!` in app code** - use reactor hooks (`use_state`, `use_ref`)
  instead. `thread_local!` is reserved for framework plumbing.
- **Test naming**: Unit tests in `test_reactor`, integration tests in
  `test_reactor_selftest`. Canvas tests use WARP software rendering.

## Documentation

The `docs/` folder has one page per crate:

- **`docs/crates/<crate>.md`** - a single page per crate covering both usage and internals (how the crate is built and maintained: codegen pipeline, generated files, conventions). It links to the crate's own `readme.md` for the user-facing intro and quick example.
- **`crates/libs/<crate>/readme.md`** - the user-facing introduction with a quick example (also the crates.io / docs.rs landing).
- **`docs/readme.md`** - the documentation hub and crate index.

`docs/` also holds `contributing.md`, `code_of_conduct.md`, and `security.md`.

When making changes to a crate, update its `docs/crates/<crate>.md` page and its `readme.md` as needed. For example, `windows-reactor` changes touch `docs/crates/windows-reactor.md` (codegen, TOML, threading, COM pitfalls, plus the conceptual overview) and `crates/libs/reactor/readme.md` (getting started and the quick example).
