# Copilot Instructions for windows-rs

Read this file at the start of every session. It contains the essential commands,
conventions, and architecture knowledge needed to work on this repository.

## Workflow

Do not create git commits automatically. The maintainer reviews all changes first
and handles commits manually. Make changes locally, run fmt/clippy/tests to verify,
and report back.

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
```

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

**Never hand-edit generated files.** The pipeline is:

1. **`tool_reactor`** — reads `crates/tools/reactor/src/winui.toml` + WinUI `.winmd`
   metadata → generates `generated.rs`, `generated_set_prop.rs`,
   `generated_attach_event.rs`, and `generated.txt` filter entries.

2. **`tool_bindings`** — reads filter `.txt` files from `crates/tools/bindings/src/`
   → runs `windows-bindgen` → generates `bindings.rs` in each crate:
   - `crates/libs/canvas/src/bindings.rs` (from `canvas.txt`)
   - `crates/libs/time/src/bindings.rs`, `numerics`, `reference`, etc.

3. **`tool_package`** — generates the published `windows` and `windows-sys` package
   crates using `--package` mode (per-namespace files + Cargo.toml features).

4. After regenerating, always verify: `cargo check -p <affected-crate> --quiet`

## Key Architecture Facts

### Crate relationships

- `windows-core` is the foundation — almost everything depends on it.
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
- **`.unwrap()` over `.expect("...")`** — the panic hook provides full context.
- **No `thread_local!` in app code** — use reactor hooks (`use_state`, `use_ref`)
  instead. `thread_local!` is reserved for framework plumbing.
- **Test naming**: Unit tests in `test_reactor`, integration tests in
  `test_reactor_selftest`. Canvas tests use WARP software rendering.

## Documentation

The `docs/` folder is organized as one page per crate under two trees:

- **`docs/guide/<crate>.md`** — user-facing guide for each crate (start at `docs/guide/readme.md`).
- **`docs/internals/<crate>.md`** — contributor notes on how each crate is built and maintained (start at `docs/internals/readme.md`).
- **`docs/readme.md`** — the documentation hub and crate index.

When making changes to a crate, update both its guide and internals pages as needed. For example, `windows-reactor` changes touch `docs/guide/windows-reactor.md` (getting started, hooks, controls) and `docs/internals/windows-reactor.md` (codegen, TOML, threading, COM pitfalls).
