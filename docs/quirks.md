# Reactor Quirks

Known behavioral gaps and potential pitfalls in `windows-reactor`.

## `.padding()` is silently ignored on `vstack` / `hstack`

In WinUI, `Padding` is a property of `Control`, not `Panel`. Since `StackPanel`
(the backing type for `vstack`/`hstack`) inherits from `Panel` rather than
`Control`, calling `.padding(...)` on a stack compiles and runs but has no
visual effect.

**Workaround:** Use `.margin(...)` on the stack (or on individual children)
instead, since `Margin` is defined on `FrameworkElement` and works everywhere.

**Resolution:** The `diagnostics` feature now emits a debug-mode warning via
`diag::unhandled_modifier` when `.padding()` is applied to an element that
doesn't inherit from `Control`. This helps developers notice the mistake early
without breaking existing code.
