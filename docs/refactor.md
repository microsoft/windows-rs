# windows-reactor module simplification

## Summary of changes

1. **All consumer files migrated** from deep paths (`windows_reactor::core::*`,
   `dsl::*`, `winui::*`) to root imports (`windows_reactor::*`).

2. **lib.rs dramatically simplified** — from 60 lines of explicit re-exports to
   3 lines: `pub use app::*`, `pub use core::*`, `pub use winui::*`.

3. **All top-level modules made private** — `app`, `core`, `winui` are now
   `mod` (private), not `pub mod`. Deep paths like
   `windows_reactor::core::reconciler::*` no longer compile.

4. **element.rs re-export chain flattened** — backward-compat `pub use super::*`
   re-exports removed. core/mod.rs now handles all public re-exports.

5. **All sub-modules tightened** — every sub-module in core/ and winui/ is
   `pub(crate) mod`. Public items reach consumers only through the `pub use`
   chain in core/mod.rs → lib.rs.

6. **`dsl/` directory eliminated** — `dsl/modifiers.rs` moved to
   `core/element_ext.rs`, `dsl/factories.rs` was a single re-export line.

7. **Small modules merged**:
   - `window.rs` (Size, InnerConstraints) → `geometry.rs`
   - `pointer.rs` (PointerHandlers, PointerEventInfo) → `modifiers.rs`
   - `accessibility.rs` (AccessibilityModifiers, LiveSetting, HeadingLevel) → `modifiers.rs`
   - `tooltip.rs` (Tooltip, TooltipContent, TooltipPlacement) → `modifiers.rs`
   - `Size` renamed to `WindowSize` to avoid collision with `bindings::Size`

8. **Internal types tightened** — `ContextStack`, `TemplatedListImpl`,
   `RenderCx::set_context_stack`, `RenderCx::take_read_contexts` downgraded
   to `pub(crate)` since they're never used by consumers.

### MIR bug note

Explicit named re-exports from private modules (`mod X; pub use X::Y;`)
trigger a nightly rustc bug where MIR is not generated for the re-exported
items. Glob re-exports (`pub use X::*`) do not trigger this bug — the
compiler correctly generates MIR even when `X` is private.

### Stats

Files deleted: 7 (dsl/mod.rs, dsl/factories.rs, dsl/modifiers.rs,
accessibility.rs, pointer.rs, tooltip.rs, window.rs)

## Verification

```sh
cargo fmt -p windows-reactor -p test_reactor -p gallery
cargo check -p windows-reactor --quiet
cargo clippy -p windows-reactor --all-targets
cargo check -p gallery --quiet
cargo check -p test_reactor --quiet
cargo test -p test_reactor --quiet
```
