# windows-reactor module simplification

## End goal

- `use windows_reactor::*` — all public API (DSL, hooks, widgets, types)
- `use windows_reactor::imp::*` — test infrastructure only (Reconciler,
  RecordingBackend, Op, RenderHost, test dispatchers)
- Everything else is `pub(crate)` or private — no `#[doc(hidden)]` hacks
- Module structure is flat and obvious: public items at root, internals
  behind one `imp` module

## C# Reactor reference model

The C# `Microsoft.UI.Reactor` uses this namespace split:
- `Microsoft.UI.Reactor` — `ReactorApp.Run`, enums, extensions (public)
- `Microsoft.UI.Reactor.Core` — `Element`, `Component`, hooks, `Ref<T>` (public)
- `Microsoft.UI.Reactor.Core.Internal` — reconciler, diff, LRU cache (hidden)
- `Microsoft.UI.Reactor.Controls` — DataGrid, VirtualList, etc. (public)
- `Microsoft.UI.Reactor.Hosting` — `ReactorHost`, windowing (public)

The Rust crate should follow the same spirit: public API at the root,
internals behind `#[doc(hidden)] pub mod imp`.

## Public API (used by apps/samples)

Everything a normal app imports via `use windows_reactor::*`.

### App lifecycle
- `App` — `App::run()`, `App::run_custom()`
- `bootstrap` module — `bootstrap::initialize()` (used as module path)

### Component model
- `Component` trait, `component()`, `memo()`
- `Element` enum, `group()`, `can_skip_update()`
- `error_boundary()`, `Fallback`

### Hooks (via `RenderCx`)
- `RenderCx`, `use_state`, `SetState<T>`, `Updater<T>`
- `use_reducer`, `Dispatch<A>`
- `use_effect`, `use_memo`, `use_callback`, `Callback<T>`
- `use_ref`, `HookRef<T>`, `use_async_state`, `AsyncSetState<T>`
- `use_color_scheme`, `Context<T>`, `Deps`

### Async resources
- `Resource<T>`, `ResourceView`, `use_resource`
- `MutationState<T>`, `MutationTrigger<T>`, `use_mutation`

### DSL / ElementExt
- `ElementExt` trait — `.margin()`, `.padding()`, `.width()`, `.height()`,
  `.background()`, `.foreground()`, `.opacity()`, `.with_key()`,
  `.on_click()`, `.tooltip()`, etc.

### Widgets (~50 factory functions + builder structs)
- Layout: `vstack`, `hstack`, `grid`, `border`, `scroll_view`, etc.
- Text: `text_block`, `text_box`, `rich_edit_box`, `password_box`, etc.
- Input: `button`, `check_box`, `toggle_switch`, `slider`, `combo_box`, etc.
- Collections: `list_view`, `grid_view`, `flip_view`, `virtual_list`
- Navigation: `navigation_view`, `tab_view`, `breadcrumb_bar`, etc.
- Dialogs: `content_dialog`, `teaching_tip`, `flyout`, `info_bar`, etc.
- Media: `image`, `progress_bar`, `progress_ring`, `shape`
- Interop: `swap_chain_panel`, `surface_image_source`, `animated_canvas`

### Layout & style types
- `Thickness`, `Color`, `Brush`, `GridLength`, `GridPlacement`
- `Orientation`, `HorizontalAlignment`, `VerticalAlignment`
- `Symbol`, `Modifiers`, `AttachedProps`
- `Size`, `InnerConstraints`
- `SelectionMode`, `TemplatedKind`

### Keyboard, pointer, accessibility, tooltip
- `KeyboardKey`, `KeyModifiers`, `KeyboardAccelerator`
- `PointerHandlers`, `PointerEventInfo`
- `AccessibilityModifiers`, `LiveSetting`, `HeadingLevel`
- `Tooltip`, `TooltipContent`, `TooltipPlacement`

### Animation
- `ImplicitTransitions`, `ScalarTransition`, `Vector3Transition`
- `AnimationConfig`, `LayoutAnimationConfig`, `Easing`

### Theme
- `ThemeRef`, `BrushBinding`, `tokens` module
- `ColorScheme`, `current_color_scheme`, `set_current_color_scheme`

### Rich text
- `RichText`, `RichTextBlock`, `RichTextParagraph`, `RichTextRun`, etc.

### Custom elements
- `CustomElement` trait, `CustomElementHandle`

### WinUI host & platform
- `Backdrop`, `RequestedTheme`, `PresenterKind`
- `set_backdrop`, `set_requested_theme`, `ReactorHost`
- `DispatcherTimer`, `Rendering`, `on_rendering`

### Backend trait (for custom/test backends)
- `Backend` trait, `ControlId`, `ControlKind`

### Dispatcher trait
- `Dispatcher` trait, `DispatcherQueuePriority`

### Re-exports from dependencies
- `windows_core::Result`
- `windows_time::{DateTime, TimeSpan}`

## Test infrastructure (NOT public API)

Items used only by test crates (`test_reactor`, `test_reactor_selftest`),
never by samples or gallery. Should move behind `imp`.

### Headless test harness
- `Reconciler` (216 test uses, 0 sample uses)
- `RecordingBackend` (212 test uses)
- `Op` enum (621 test uses)
- `RenderHost`, `RenderStats`, `RenderCompleteInfo`

### Test dispatchers
- `ChannelDispatcher`, `RunOnDemandDispatcher`
- `UiRerenderGuard`, `UiMarshaller`

### Platform types (for selftest)
- `WinUIBackend`, `WinUIDispatcher`

### Internal element variants
- `GroupElement`, `ComponentElement`, `ErrorBoundaryElement`

### Internal plumbing (never used externally)
- `ContextStack`, `ContextId`, `ContextProvision`, `ProviderElement`
- `TemplatedListImpl`, `TemplatedListElement`, `TemplatedListBuilder`
- `SendDispatcher`, `IntoCallback`, `IntoElements`
- `set_ui_rerender`, `request_ui_rerender_on_ui_thread`
- `with_active_host`, `Binding`, `PropBindings`
- `Widget` trait, `generated_bindings`, `rc_fn`
- `Event`, `EventHandler`, `Prop`, `PropValue`
- `bindings`, `app_shim`, `diagnostics`

### Dead code (delete)
- `winui/template_cache.rs` — entirely unused
- `ContextStack::depth()` — only used in same-file tests

### Future cleanup: `Option<Box<Vec/HashMap>>` anti-pattern ✅ DONE
`keyboard_accelerators` was `Option<Box<Vec<KeyboardAccelerator>>>` → now `Vec<KeyboardAccelerator>`.
`resources` was `Option<Box<HashMap<String, String>>>` → now `HashMap<String, String>`.
Empty collections already convey "not set" with zero heap allocation. The remaining
`Option<Box<Struct>>` fields (theme_bindings, animations, accessibility, tooltip,
pointer_handlers) are intentionally boxed — they contain large structs where the Box
keeps `Modifiers` small in the common case (8 bytes vs 48-100+ bytes inline).

## Nightly rustc dead_code false-positive (constraint)

With Rust 2024 on nightly, `pub(crate) mod` sub-modules inside a private
parent module trigger false dead_code warnings at scale (98 warnings when
core has 5+ `pub(crate) mod` children). Related to
https://github.com/rust-lang/rust/issues/135007.

**Solution**: Follow the windows-rs convention — never use `pub(crate)`.
All items are `pub`, visibility is controlled by module privacy in lib.rs.
Private modules (`mod core; mod winui;`) contain `pub mod` sub-modules
with `pub use X::*;` re-exports. This avoids the bug entirely.

## Validated changes from exploratory session

### 1. Migrate consumer imports
75 files use deep paths → change all to `windows_reactor::*`.

### 2. Eliminate `dsl/` directory
- `dsl/modifiers.rs` → `core/element_ext.rs`
- `dsl/factories.rs` — one-line re-export, delete

### 3. Flatten element.rs re-export chain
Remove backward-compat `pub use super::*` re-exports from element.rs.

### 4. Merge small modules
- `window.rs` → `geometry.rs` (rename `Size` → `WindowSize`)
- `pointer.rs` + `accessibility.rs` + `tooltip.rs` → `modifiers.rs`

### 5. Tighten sub-module visibility
All sub-modules in `core/` and `winui/` → `pub(crate) mod`.
Replace `public_submodules!`/`internal_submodules!` macros with plain decls.

### 6. Make top-level modules private (glob workaround)
```rust
// lib.rs
mod core;
mod winui;
mod app;
pub use core::*;
pub use winui::*;
pub use app::*;
```

### 7. Delete dead code
- `winui/template_cache.rs`
- `ContextStack::depth()`

### 8. Tighten internal types to `pub(crate)`
- `ContextStack`, `TemplatedListImpl`
- `set_ui_rerender`, `request_ui_rerender_on_ui_thread`
- `ComponentElement` struct, `ErrorBoundaryElement` struct

## Proposed implementation plan

### Phase 1: Structural cleanup (no API changes) ✅
- [x] Migrate consumer imports to root (75+ files)
- [x] Eliminate `dsl/` directory (moved ElementExt → core/element_ext.rs)
- [x] Flatten element.rs re-export chain (lib.rs re-exports from source modules)
- [x] Delete dead code (template_cache.rs, ContextStack::depth())
- [x] Merge small modules (window→geometry, pointer+accessibility+tooltip→modifiers)

### Phase 2: Visibility tightening ✅
- [x] Replace macros with plain `pub(crate) mod` declarations
- [x] All sub-modules in core/ and winui/ are `pub(crate) mod`
- [x] core/mod.rs uses `pub use X::*` (for lib.rs glob), `pub(crate) use` for internal-only
- [x] Top-level modules: `#[doc(hidden)] pub mod core/winui` (MIR bug prevents `mod`)
- [x] lib.rs simplified to 3 glob re-exports: `pub use app/core/winui::*`

### Phase 3: Public/internal separation ✅
- [x] Created `imp/` module with re-exports of test infrastructure
- [x] Excluded test items from root namespace (named re-exports in core/mod.rs)
- [x] Updated test crates to `use windows_reactor::imp::*`
- [x] `#[doc(hidden)]` only on core/winui (MIR workaround) and imp (by design)

### Phase 4: Final visibility tightening ✅
- [x] Removed all `#[doc(hidden)]` from items already behind `imp`
- [x] Split core/mod.rs into `pub use` (public API) and `pub(crate) use` (internal)
- [x] `ComponentElement`, `ComponentObject`, `ErrorBoundaryElement` → `pub(crate)` re-exports
- [x] `panic_message`, `set_ui_rerender`, `request_ui_rerender_on_ui_thread` → `pub(crate)`
- [x] Added `imp` import to `test_reactor_perf` (missed in Phase 3)
- [x] Full `cargo test --all` passes — zero regressions

## Current state

All phases complete. The crate uses the standard windows-rs visibility convention:
- No `pub(crate)` anywhere (one exception: `widget_header` macro re-export)
- No `#[doc(hidden)]` on `core` or `winui` — they are plain `mod` (private)
- `#[doc(hidden)] pub mod imp` for test infrastructure (intentional)
- All items inside modules are `pub`; visibility controlled by module privacy

```rust
// lib.rs structure
mod core;           // private — all internals
mod winui;          // private — WinUI backend, hooks, host
mod app;            // private
mod diagnostics;    // private
mod app_shim;       // private
mod bindings;       // private (generated FFI)

pub mod bootstrap;  // public module (used as path: bootstrap::initialize())

#[doc(hidden)]
pub mod imp;        // test infrastructure

pub use app::*;
pub use core::*;
pub use winui::hooks::*;
pub use winui::host::*;
```

### API surface
- **Public API** (`windows_reactor::*`): ~50 widget factories, hooks, DSL, app lifecycle
- **Test infrastructure** (`windows_reactor::imp::*`): Reconciler, RecordingBackend, Op, RenderHost, test dispatchers
- **Internal** (private modules): backend plumbing, reconciler internals, WinUI backend

### Remaining `#[doc(hidden)]` (intentional)
- `lib.rs`: `#[doc(hidden)] pub mod imp;` — test-only, should not appear in docs

## Future improvements (not blocking)

1. ~~**`Option<Box<Vec/HashMap>>` anti-pattern**~~ — Done (Phase 4).

2. ~~**Remove `#[doc(hidden)]` when MIR bug is fixed**~~ — Done. Switched to
   windows-rs convention (no `pub(crate)`, module privacy controls visibility).
   `core` and `winui` are now plain `mod` (private) with zero false warnings.

3. ~~**Further `pub(crate)` tightening**~~ — Replaced by convention change.
   All items are `pub`, visibility controlled by module privacy. Only one
   exception: `pub(crate) use widget_header` macro in `core/widgets/mod.rs`
   (macros can't be re-exported from private modules).

## Verification commands

```sh
cargo fmt -p windows-reactor -p test_reactor -p test_reactor_selftest
cargo clippy -p windows-reactor --all-targets
cargo check -p windows-reactor --quiet
cargo check -p gallery --quiet
cargo check -p test_reactor --quiet
cargo check -p test_reactor_selftest --quiet
cargo check -p test_reactor_perf --quiet
cargo test -p test_reactor --quiet
cargo test --all   # full regression check
```
