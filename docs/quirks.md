# Reactor Quirks

Known behavioral gaps and potential pitfalls in `windows-reactor`.

## `.padding()` is silently ignored on `vstack` / `hstack`

In WinUI, `Padding` is a property of `Control`, not `Panel`. Since `StackPanel`
(the backing type for `vstack`/`hstack`) inherits from `Panel` rather than
`Control`, calling `.padding(...)` on a stack compiles and runs but has no
visual effect.

**Workaround:** Use `.margin(...)` on the stack (or on individual children)
instead, since `Margin` is defined on `FrameworkElement` and works everywhere.

**TODO:** Consider whether `windows-reactor` should:
- Emit a compile-time or runtime warning when `.padding()` is applied to a
  widget that doesn't support it.
- Silently translate `.padding()` on stacks into child margins or wrap in a
  `Border` internally.
- Remove `.padding()` from the `ElementExt` blanket impl and only expose it on
  widgets where it actually works (buttons, borders, content controls, etc.).
