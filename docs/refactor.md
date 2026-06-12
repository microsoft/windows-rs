# windows-reactor module simplification

## Summary of changes

1. **All consumer files migrated** from deep paths (`windows_reactor::core::*`,
   `dsl::*`, `winui::*`) to root imports (`windows_reactor::*`).

2. **`core/` renamed to `imp/`** — follows the `windows-core` pattern. lib.rs
   declares `#[doc(hidden)] pub mod imp` and re-exports everything via
   `pub use imp::*`. All sub-modules inside imp are `pub(crate) mod`.

3. **lib.rs dramatically simplified** — 3 glob re-exports:
   `pub use app::*`, `pub use imp::*`, `pub use winui::*`.

4. **All top-level modules private** — `app` and `winui` are `mod` (private).
   `imp` is `#[doc(hidden)] pub mod` (pub only to work around the MIR bug
   below and to let test crates reach internals when needed).

5. **element.rs re-export chain flattened** — backward-compat `pub use super::*`
   re-exports removed.

6. **All sub-modules tightened** — every sub-module in imp/ and winui/ is
   `pub(crate) mod`. Public items reach consumers only through the `pub use`
   chain in imp/mod.rs → lib.rs.

7. **`dsl/` directory eliminated** — `dsl/modifiers.rs` moved to
   `imp/element_ext.rs`, `dsl/factories.rs` was a single re-export line.

8. **Small modules merged**:
   - `window.rs` (Size, InnerConstraints) → `geometry.rs`
   - `pointer.rs` (PointerHandlers, PointerEventInfo) → `modifiers.rs`
   - `accessibility.rs` (AccessibilityModifiers, LiveSetting, HeadingLevel) → `modifiers.rs`
   - `tooltip.rs` (Tooltip, TooltipContent, TooltipPlacement) → `modifiers.rs`
   - `Size` renamed to `WindowSize` to avoid collision with `bindings::Size`

9. **Internal types tightened** — `ContextStack`, `TemplatedListImpl`,
   `RenderCx::set_context_stack`, `RenderCx::take_read_contexts`,
   `set_ui_rerender`, `request_ui_rerender_on_ui_thread` downgraded
   to `pub(crate)`.

10. **`#[doc(hidden)]` removed** from items that no longer need it —
    `Op`, `RecordingBackend`, `Reconciler`, `RenderHost`, `GroupElement`,
    `UiRerenderGuard`, `UiMarshaller`, `ChannelDispatcher`,
    `RunOnDemandDispatcher`, etc. These are now just `pub` inside `imp`
    sub-modules, reachable via glob re-exports.

### MIR bug note

Explicit named re-exports from private modules (`mod X; pub use X::Y;`)
trigger a nightly rustc bug where MIR is not generated for the re-exported
items. Glob re-exports (`pub use X::*`) do not trigger this bug — the
compiler correctly generates MIR even when `X` is private.

See: https://github.com/rust-lang/rust/issues/135007

Recheck when this issue is fixed — with the fix it should be possible to
make `imp` a plain `mod` instead of `#[doc(hidden)] pub mod`.

### Stats

Files deleted: 7 (dsl/mod.rs, dsl/factories.rs, dsl/modifiers.rs,
accessibility.rs, pointer.rs, tooltip.rs, window.rs)

Files renamed: core/ → imp/ (entire directory, ~90 files)

## Verification

```sh
cargo fmt -p windows-reactor -p test_reactor -p gallery
cargo check -p windows-reactor --quiet
cargo clippy -p windows-reactor --all-targets
cargo check -p gallery --quiet
cargo check -p test_reactor --quiet
cargo test -p test_reactor --quiet
```
