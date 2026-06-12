# windows-reactor module simplification plan

## Summary of changes

### What was done

1. **All 75 consumer files migrated** from deep paths (`windows_reactor::core::*`,
   `dsl::*`, `winui::*`) to root imports (`windows_reactor::*`).

2. **lib.rs re-exports expanded** — all public items are available through
   `use windows_reactor::*` with explicit named re-exports per module.

3. **element.rs re-export chain flattened** — backward-compat `pub use super::*`
   re-exports removed. lib.rs now re-exports directly from source modules.

4. **All sub-modules tightened** — every sub-module in core/, winui/ changed
   from `pub mod` to `pub(crate) mod`. Deep paths like
   `windows_reactor::core::reconciler::*` no longer compile for consumers.

5. **`dsl/` directory eliminated** — `dsl/modifiers.rs` moved to
   `core/element_ext.rs`, `dsl/factories.rs` was a single re-export line
   (now handled by lib.rs).

6. **Small modules merged**:
   - `window.rs` (Size, InnerConstraints) → `geometry.rs`
   - `pointer.rs` (PointerHandlers, PointerEventInfo) → `modifiers.rs`
   - `accessibility.rs` (AccessibilityModifiers, LiveSetting, HeadingLevel) → `modifiers.rs`
   - `tooltip.rs` (Tooltip, TooltipContent, TooltipPlacement) → `modifiers.rs`
   - `Size` renamed to `WindowSize` to avoid collision with `bindings::Size`

7. **Redundant imports cleaned up** — duplicate `use windows_reactor::X` +
   glob patterns consolidated.

### What's blocked

**Making top-level modules private** (`pub mod core` → `mod core`) is blocked
by a nightly rustc MIR generation bug: the compiler skips MIR for items in
private top-level modules even when `pub use`-re-exported. `pub(crate) use`
re-exports in core/mod.rs also trigger MIR failures — `pub use` is required.

The modules remain `#[doc(hidden)] pub mod` with a comment explaining why.
This is a one-line change once the rustc bug is fixed.

Last checked: nightly 1.98.0 (2026-06-11).

### Module count reduction

Before: 4 directories (core/, dsl/, winui/, widgets/) with 35+ modules
After: 3 directories (core/, winui/, widgets/) with 27 modules in core/

Files deleted: 7 (dsl/mod.rs, dsl/factories.rs, dsl/modifiers.rs,
accessibility.rs, pointer.rs, tooltip.rs, window.rs)

Net line reduction: ~670 lines

## Verification

```sh
cargo fmt -p windows-reactor -p test_reactor -p gallery
cargo check -p windows-reactor --quiet
cargo clippy -p windows-reactor --all-targets
cargo check -p gallery --quiet
cargo check -p test_reactor --quiet
cargo test -p test_reactor --quiet
```
