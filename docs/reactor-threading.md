# Threading: Remaining Opportunities

Completed work (delegate `Send` removal, `UiThreadSendWrapper` elimination) is
archived in git history. This document covers only **open optimization
opportunities**.

---

## Current `thread_local!` inventory

| Location | Name | Holds | COM? |
|----------|------|-------|------|
| `app.rs` | `HOST_SLOT` | `ReactorHost` | Contains STA COM refs |
| `app.rs` | `APP_SLOT` | `Application` | STA-affine |
| `host.rs` | `ROOT_FRAMEWORK_ELEMENT` | `IFrameworkElement` | STA-affine |
| `host.rs` | `ROOT_WINDOW` | `Window` | STA-affine |
| `host.rs` | `PENDING_THEME` | `Cell<Option<ElementTheme>>` | No — pure enum |
| `host.rs` | `PENDING_TALL` | `Cell<Option<bool>>` | No — pure flag |
| `dispatcher.rs` | `UI_RERENDER` | `Rc<dyn Fn()>` | No — pure Rust |
| `template_cache.rs` | `SHARED_TEMPLATE` | `DataTemplate` | STA-affine |
| `theme.rs` | `CURRENT_COLOR_SCHEME` | `Cell<ColorScheme>` | No — pure enum |
| `diagnostics.rs` | `EXPECT_PANIC` | `Cell<u32>` | No — counter |
| `modifiers.rs` | `OPS_REGISTRY` | `FxHashMap<TypeId, (CloneFn, EqFn)>` | No — fn ptrs |

---

## 1. Pure-Rust thread_locals that don't need to be thread_local

Six of the eleven thread_locals hold **no COM objects** and exist solely as
ambient-access globals. The `thread_local!` mechanism adds per-access overhead
(TLS lookup + borrow) and makes the code harder to reason about.

### `CURRENT_COLOR_SCHEME` — field on the host

This is a `Cell<ColorScheme>` read by `use_color_scheme`. It is written by
the host on `ActualThemeChanged` and read during render. Both occur on the
same call stack. It could be a field on `RenderHost` / passed through
`RenderCx` — the infrastructure already exists (`RenderCx` carries
`window_dimensions: Rc<Cell<WindowDimensions>>` in the same pattern).

**Benefit:** Eliminates TLS lookup on every `use_color_scheme` call. Compile-
time proof that only the host writes it.

### `UI_RERENDER` — field on the host

This is an `Rc<dyn Fn()>` set once by `RenderHost::set_marshaller` and read
only during `request_ui_rerender_on_ui_thread`. It exists as a thread_local
because multiple modules need to call it without a `&self` handle.

**Alternative:** Pass the rerender closure through the reconciler (already
done — `Reconciler.request_rerender` is `Rc<dyn Fn()>`). The thread_local
appears to be a holdover from before the reconciler was refactored to carry
this field. Audit call sites of `request_ui_rerender_on_ui_thread` — if they
all have access to the reconciler or host, the thread_local can be removed.

### `PENDING_THEME` / `PENDING_TALL` — transient state, not a cache

These are `Cell<Option<…>>` slots that buffer a theme or titlebar-height
request until `ROOT_FRAMEWORK_ELEMENT` / `ROOT_WINDOW` is available. They
are set by `set_requested_theme` / titlebar setup and drained in
`post_render`. These are one-shot latches, not persistent caches.

**Alternative:** Make these methods on `RenderHost` and store the pending
values as fields. Removes a TLS access on every theme/titlebar change.

### `EXPECT_PANIC` — legitimately thread-local

The panic hook runs on the panicking thread and checks this counter. This is
correct use of `thread_local!` and should remain.

### `OPS_REGISTRY` — compile-time opportunity

See §3 below.

---

## 2. STA-affine COM caches — correct but worth questioning

`HOST_SLOT`, `APP_SLOT`, `ROOT_FRAMEWORK_ELEMENT`, `ROOT_WINDOW`, and
`SHARED_TEMPLATE` hold apartment-affine COM handles. These **cannot** be
stored in a global `static` because WinRT objects in an STA must only be
accessed from their creating thread.

However, `thread_local!` is not the only option:

### Alternative: fields on owned structs

The reactor already has a single-owner hierarchy: `App` → `RenderHost` →
`Reconciler`. If these COM handles lived as fields on `RenderHost` (which
already owns the window and root element internally), consumers wouldn't need
ambient TLS lookups.

The reason they're thread-locals today is that **free functions** like
`set_requested_theme`, `set_backdrop`, and `with_active_host` need access
without a `&self` parameter. These free-function APIs exist for ergonomics in
`use_effect` closures where the user doesn't have a host reference.

**Tradeoff:** Eliminating these thread-locals would require threading a
context handle through effect closures (like React's `useContext`). The
`thread_local!` approach trades some per-access overhead for API simplicity.
This is a reasonable design choice, not a bug.

### `SHARED_TEMPLATE` — the cache is justified

`DataTemplate` parsing (`XamlReader::Load`) involves XML parsing + XAML
object instantiation. Caching the result in a `OnceCell` avoids repeated work
for every templated list item realization. The `thread_local!` is correct
because `DataTemplate` is STA-affine. **No change needed.**

---

## 3. `OPS_REGISTRY` — eliminate via compile-time dispatch

`OPS_REGISTRY` is a `thread_local! { FxHashMap<TypeId, (CloneFn, EqFn)> }`
that stores function pointers for type-erased clone/eq on `AttachedProps`
values. Every `AttachedProps::set::<T>()` call does a TLS lookup + HashMap
insert. Every `Clone` / `PartialEq` on `AttachedProps` does a TLS lookup +
HashMap get per entry.

This exists because `AttachedProps` stores `Box<dyn Any>` and needs to
recover clone/eq capability at runtime. The registry is a holdover from when
attached props were more dynamic. Today, the set of attached-prop types is
known at compile time.

### Option A: Store ops inline alongside the value

```rust
struct AttachedEntry {
    value: Box<dyn Any>,
    clone: fn(&dyn Any) -> Box<dyn Any>,
    eq: fn(&dyn Any, &dyn Any) -> bool,
}
struct AttachedProps(FxHashMap<TypeId, AttachedEntry>);
```

Each entry carries its own vtable. No global registry needed. The two extra
function pointers (16 bytes) per entry are negligible.

**Benefit:** Eliminates `OPS_REGISTRY` entirely — zero TLS overhead, zero
HashMap lookup for ops, no registration step. `Clone` and `PartialEq` become
simple field accesses.

### Option B: Custom trait object with Clone + PartialEq

```rust
trait AttachedValue: Any {
    fn clone_box(&self) -> Box<dyn AttachedValue>;
    fn eq_box(&self, other: &dyn Any) -> bool;
    fn as_any(&self) -> &dyn Any;
}
impl<T: Clone + PartialEq + 'static> AttachedValue for T { ... }

struct AttachedProps(FxHashMap<TypeId, Box<dyn AttachedValue>>);
```

The clone/eq vtable entries are embedded in the trait object's vtable by the
compiler. No explicit registry, no TLS.

**Benefit:** Same as Option A but more idiomatic Rust — leverages the
compiler's vtable machinery instead of manual function pointers.

### Recommendation: Option B

It removes the `OPS_REGISTRY` thread_local entirely, removes the
`register_ops::<T>()` call from every `set()`, and makes `AttachedProps`
self-contained. The overhead vs today is zero (trait object vtable is the same
size as the current `Box<dyn Any>` vtable, just with extra methods).

---

## 4. `AgileDispatcherQueue` — remove the wrapper

`DispatcherQueue` implements `IAgileObject` and is documented as free-threaded.
Today we wrap it in a newtype with `unsafe impl Send/Sync`:

```rust
struct AgileDispatcherQueue { queue: DispatcherQueue }
unsafe impl Send for AgileDispatcherQueue {}
unsafe impl Sync for AgileDispatcherQueue {}
```

### Path to removal

If `windows-bindgen` emits `Send + Sync` for types whose metadata declares
`IAgileObject` (via `[MarshalingBehavior(Agile)]`), the wrapper becomes
unnecessary. This is tracked upstream.

In the meantime, the wrapper is small (3 lines of unsafe) and correct. Low
priority.

---

## 5. Summary of actionable items

| Item | Complexity | Benefit |
|------|-----------|---------|
| Replace `OPS_REGISTRY` with inline ops (§3) | Small — ~50 lines | Eliminates TLS + HashMap per attached-prop access |
| Move `CURRENT_COLOR_SCHEME` to `RenderCx` | Small — plumbing | Eliminates TLS on every `use_color_scheme` |
| Audit `UI_RERENDER` call sites | Small — may already be dead | Potential thread_local removal |
| Move `PENDING_THEME` / `PENDING_TALL` to `RenderHost` fields | Trivial | Two fewer TLS slots |
| Remove `AgileDispatcherQueue` wrapper | Blocked on upstream | Removes 3 lines of unsafe |
| Eliminate STA COM thread_locals | Large — API redesign | Not recommended (API ergonomics tradeoff) |

### What NOT to change

- **`EXPECT_PANIC`** — legitimately thread-local (panic hook context).
- **`SHARED_TEMPLATE`** — STA-affine COM cache, justified performance win.
- **`HOST_SLOT` / `APP_SLOT` / `ROOT_*`** — STA-affine COM handles; the
  free-function API ergonomics justify the TLS approach unless a broader
  context-passing redesign is undertaken.
- **`RefCell` usage** — correct for single-threaded interior mutability;
  no atomics overhead.
- **`Arc<dyn SendDispatcher>`** — the one legitimate cross-thread boundary
  (background tasks posting to UI thread).
