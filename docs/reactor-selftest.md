# Testing

## Unit Tests (`test_reactor`)

The `test_reactor` crate (`crates/tests/libs/reactor`) contains fast,
headless unit tests for the core hooks, reconciler, DSL, and element logic.
These use `RenderCx::for_test()` and `RecordingBackend` — no WinUI window
required.

```sh
cargo test -p test_reactor
```

---

## Integration Tests (`test_reactor_selftest`)

The `test_reactor_selftest` crate (`crates/tests/libs/reactor_selftest`) is
the live integration test suite. It launches a real WinUI 3 window, mounts
reactor components through the full render pipeline, and asserts against the
live visual tree. Output is [TAP 14](https://testanything.org/) for
machine-parseable pass/fail reporting.

### Harness Architecture

| Component | File | Purpose |
|-----------|------|---------|
| `main.rs` | Entry point | CLI flags, boots `App`, spawns fixture runner |
| `harness.rs` | `Harness` struct | Window + `RenderHost`, assertion helpers, visual-tree queries, programmatic drivers |
| `registry.rs` | `FIXTURES` array | Static dispatch table (fixture name → async fn) |
| `exec.rs` | Dispatcher integration | `spawn_root` + `YieldLow` for cooperative scheduling |

### Running

```sh
# Full run (opens a window, interactive mode)
cargo run -p test_reactor_selftest

# CI / headless (auto-exits with exit code)
cargo run -p test_reactor_selftest -- --headless

# Filter to a specific fixture
cargo run -p test_reactor_selftest -- --filter Tooltip

# Slow mode (400ms pause between fixtures for visual inspection)
cargo run -p test_reactor_selftest -- --slow

# List all fixture names (no WinUI launch)
cargo run -p test_reactor_selftest -- --list-fixtures
```

### CI

Requires the Windows App Runtime and an interactive desktop session
(e.g. `windows-2025` GitHub Actions runners).

### Adding a New Fixture

1. Implement the fixture function in the appropriate file under
   `src/fixtures/` (or create a new module).
2. Add the entry to `registry.rs` — the `FIXTURES` array is the single
   source of truth for execution order and `--list-fixtures` output.
3. Use the `Harness` API:
   - `h.mount(root)` — mount a component
   - `h.render().await` — pump the dispatcher until idle
   - `h.render_until("label", pred).await` — pump until predicate holds
   - `h.check("name", condition)` — emit TAP ok/not-ok
   - `h.check_eq("name", expected, actual)` — equality assertion with diag
   - `h.find_text("...")`, `h.find_button("...")` — query visual tree
   - `h.click_button("...")` — invoke via automation peer
   - `h.dump_tree()` — textual snapshot of the visual tree
4. Run locally with `--filter YourFixtureName` to iterate.
