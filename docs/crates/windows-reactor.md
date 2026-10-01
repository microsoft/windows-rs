# windows-reactor

`windows-reactor` is a typed declarative UI library backed by WinUI 3. Applications describe a
view from state, and Reactor reconciles that declaration into a retained object graph and a live
native window.

See the crate [readme](../../crates/libs/reactor/readme.md) for the user-facing introduction and
sample overview.

## Architecture

The pipeline has four layers:

```text
typed declarations -> retained generational arena -> generic mutations -> native adapter
```

Generated builders constrain property, event, and relation shapes. A `Border` owns one visual
content relation, panels own positional or keyed visual children, `TreeView` owns structural
`TreeNode` values, and list controls own keyed `DataItem` values. Invalid relation categories
cannot be assembled through the public builders.

Declarations reconcile directly into one retained arena. Retained relations store stable
generation-checked `ObjectId` values, while declarations use recursive Rust ownership for natural
builder composition. Reconciliation emits generic property, event, relation, focus, and lifecycle
mutations instead of control-specific planner commands.

The WinUI adapter maps those mutations to native objects and owns backend policy such as
controlled input feedback, TreeView structural content, ListView data templates, attachments,
virtualization, and native retirement. Recording tests use the same mutation protocol through
`RecordingAdapter`.

## Application and windows

`App::run_component` owns the WinUI application, dispatcher, component host, and initial native
window. `App::run_component_with_policy` also applies creation-time window policy.

Components can open, activate, and close windows through `ComponentContext`. Each live window owns
one independently scheduled component host. Rejectable window commands use a bounded queue.
Committed publication and latest size or color observations use durable entries coalesced by
window and observation generation, while close completion remains non-rejectable. The application
exits after its last component window closes.

`App::run_with` supports application lifetime independent of component windows. `AppContext`
opens independently scheduled windows and displays a keyed `Menu` at physical screen coordinates
through a lazily created hidden host. Closing the last component window does not end this mode;
the application exits when `AppContext::exit` is requested.

Components publish retained window state through `ViewContext::window_title`,
`window_visuals`, `on_window_size`, and `on_color_scheme`. One component scope owns each contract,
and removing that scope clears its publication or observation. `WindowVisuals` covers theme,
backdrop, icon, client size, and minimum or maximum client-size constraints.

## Components and lifecycle

Components use `ComponentContext<Self>`, `ViewContext<Self>`, and `View`. A component retains its
state, input, effective typed contexts, effects, references, tasks, timers, and one retained root
identity. It never retains a second rendered declaration tree.

`View::component` derives unkeyed identity from the component type and complete owning relation
path. `component(key, input)` supplies explicit application identity for dynamic movement. Root
replacement updates the retained subtree in place, while keyed reorder preserves component state
and native identity.

Callbacks, forwards, and messages queue UI-local component work. Background work retains its
`Send` boundary and uses generation checks so completion cannot reach a replaced or retired
scope. Removal cancels tasks, cleans effects, invalidates references, and rejects stale senders.
Typed-reference commands use bounded admission when callers can observe rejection. Observation
registration and revocation are durable lifecycle entries coalesced by object and observation, so
queue saturation cannot leave the Rust handle and native subscription out of sync.

`provide` applies the nearest typed context value to a visual subtree. Descendant components keep
the same shadowing across input changes, messages, window lifecycle work, and virtualized rows.

## Controls and values

`crates/tools/reactor/src/schema.toml` is the source of truth for generated controls, properties,
events, relations, controlled-state contracts, and capability checks. Shared values include
brushes, styles, resource overrides, keyboard accelerators, validated URIs, geometry, transitions,
window policy, pointer and keyboard payloads, menus, and rich text.

TreeView supports keyed structural nodes with nested nodes and optional visual content. ListView
uses keyed `DataItem` values; each item may use text-only content or own a rich visual subtree
while retaining data identity and native selection behavior. ItemsRepeater accepts a lazy
`VirtualSource`, realizes only requested rows, and retires row components and references when
native containers recycle.

Attachments such as ToolTip, Flyout, Menu, CommandBarFlyout, and ContentDialog are owned by their
declaration target. Stable updates preserve the native attachment, while replacement and
destruction clear native ownership in the required order.

Controlled properties use an expected-feedback contract generated from the schema. Native echoes
from application writes are suppressed, native changes enter the ordered event stream, and
callback revision checks reject stale delivery. RichEditBox text uses the same contract with its
native document adapter and LF normalization.

## Code generation

`tool-reactor` reads the schema and checked-in WinUI metadata, then writes:

| Output | Purpose |
| --- | --- |
| `crates/libs/reactor/src/generated.rs` | IDs, contracts, values, and shared generated API |
| `crates/libs/reactor/src/generated_declarations.rs` | Typed declaration builders |
| `crates/libs/reactor/src/native/generated.rs` | Native realization dispatch |
| `crates/libs/reactor/src/native/bindings.rs` | Minimal WinUI bindings |
| `crates/libs/canvas/src/reactor_bindings.rs` | Canvas bridge bindings |
| `crates/tests/libs/reactor_selftest/src/generated_coverage.rs` | Live contract fixture |
| `crates/tests/libs/reactor_selftest/coverage.md` | Generated live coverage inventory |

Do not edit these files by hand. After changing the schema, parity mappings, generator, or WinUI
metadata, run:

```text
cargo run -p tool-reactor --quiet
cargo check -p windows-reactor --quiet
```

The frozen semantic baseline in `parity-baseline.toml` verifies all 79 controls, 233 properties,
68 events, 42 slots, three selection contracts, 158 capabilities, and two lifecycle contracts.
`parity.toml` records reviewed representation mappings. The Rust checker applies mapping kinds
generically rather than branching on individual controls.

## Validation

Run the smallest relevant commands while developing:

```text
cargo run -p tool-reactor --quiet -- --check-parity
cargo run -p tool-reactor --quiet -- --live-coverage-report
cargo test -p tool-reactor --quiet
cargo test -p windows-reactor --all-features --quiet
cargo test -p reactor-gallery --quiet
cargo clippy -p windows-reactor --all-features --all-targets
cargo +nightly llvm-cov -p windows-reactor --all-features --branch \
    --json --output-path target\reactor-coverage.json
crates\tests\libs\reactor_selftest\coverage.ps1
```

The real WinUI fixture and application-host fixtures require the Windows App Runtime:

```text
cargo run -p test-reactor-selftest --quiet -- --headless
cargo run -p test-reactor-selftest --bin application_lifecycle --quiet -- replacement
cargo run -p test-reactor-selftest --bin application_lifecycle --quiet -- multiple
cargo run -p test-reactor-selftest --bin application_lifecycle --quiet -- startup-error
cargo run -p test-reactor-selftest --bin application_menu --quiet
cargo run -p test-reactor-selftest --bin canvas_integration --quiet
cargo run -p test-reactor-selftest --bin webview_integration --quiet
cargo run -p test-reactor-selftest --bin window_state --quiet
powershell -File crates\tests\libs\reactor_selftest\run_samples.ps1
```

The lifecycle cases cover replacement of the last window, shutdown after multiple windows close,
and propagation of startup errors. The sample smoke script builds and launches every default
Reactor, Composition, and WebView sample, captures runtime output, and fails if a sample exits
early or writes to stderr.

`test-reactor-bench` provides replacement-only retained-memory, update, lifecycle, component, and
virtualization measurements:

```text
cargo run -p test-reactor-bench --release --quiet
cargo run -p test-reactor-bench --release --quiet -- \
    --architecture-gate --gate-samples 6
```

Before submitting generator changes, run generation twice and confirm that the generated file
hashes are unchanged.

## Limits

- Component roots are visual declarations. Structural and data objects are owned through visual
  control relations.
- Declaration depth is limited to 128 and a retained graph is limited to 65,536 objects.
- Adapter validation or application failure poisons the runtime and clears retained state.
- Native accessibility behavior requires live testing; recording tests validate contracts and
  mutation shape but cannot replace WinUI acceptance coverage.
- Dense 10,000-item reorder remains a backend cost that should be measured against the frame
  budget when changing container synchronization.
