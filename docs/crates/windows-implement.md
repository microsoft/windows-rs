# windows-implement

> The procedural macro that exposes a Rust type through COM or WinRT interfaces.

- 📦 [crates.io](https://crates.io/crates/windows-implement)
- 📖 [docs.rs](https://docs.rs/windows-implement)
- 🚀 [Getting started](../../crates/libs/implement/readme.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/implement)

`windows-implement` provides `#[implement]`. The macro builds the vtables, identity object,
reference count, `QueryInterface` routing, and conversions needed to expose a Rust struct through
one or more generated interfaces.

## Intended audience

Use this macro when Rust is the implementation side of COM or WinRT: a component, callback,
activation factory, shell extension, test object, or another object passed to an API as an
interface. Calling an existing interface does not require `#[implement]`.

Application and component code normally imports `windows_core::implement`. The separate
`windows-implement` dependency is an implementation detail of `windows-core`'s default
`proc-macros` feature. Its generated code also depends on `windows-core`, so direct consumption
offers no standalone runtime.

Interfaces usually come from a focused crate or custom
[`windows-bindgen`](windows-bindgen.md) output. A binary application may instead use the broad
[`windows`](windows.md) projection. Pair this macro with
[`#[interface]`](windows-interface.md) only for a manually owned or missing interface definition.
See [`windows-core`](windows-core.md) for a compact end-to-end example.

## Implementation model

For `#[implement(IValue)] struct Value`, the interface declaration or generated bindings provide an
`IValue_Impl` trait. The macro creates a `Value_Impl` wrapper. Implement the generated trait for
that wrapper, then convert the original `Value` into an interface type.

For an inherited interface, implement the `*_Impl` traits required by the interface and its parent
chain. List each independently exposed interface in `#[implement(...)]`; do not list a base merely
to compensate for a missing parent relationship in the interface definition.

The resulting interface value owns a COM reference. Cloning an interface increments the reference
count, dropping it decrements the count, and `cast` uses `QueryInterface`. The original Rust value
is dropped when the final COM reference is released.

Implementation methods use the projected signatures generated for the interface. Return
`windows_core::Result<T>` when offered by that trait, and return a precise `HRESULT` when the trait
uses raw status values. The generated thunk translates `Result` errors at the ABI boundary.

## Threading and agility

Implementations are agile by default. The macro exposes `IAgileObject` and an agile marshaler
unless the attribute includes `Agile = false`. This is a COM promise, not a convenience flag.

Before accepting the default, verify that:

- method calls may arrive from the apartments and threads allowed by the interface contract;
- shared mutable state is synchronized correctly;
- borrowed thread-affine resources never escape their owning apartment;
- destruction is valid on the thread that releases the final reference.

Use `Agile = false` for an apartment-affine object. Add an interface such as `IAgileObject`
explicitly only when the object implements that contract itself. For WinRT implementations,
`TrustLevel = Partial` or `TrustLevel = Full` controls the value reported by
`IInspectable::GetTrustLevel`; omit it unless the component's contract requires a nondefault trust
level.

## Errors, panics, and pointers

Treat every generated implementation method as an FFI boundary:

- Validate raw pointers before dereferencing them when the ABI permits validation.
- Initialize required out parameters on every success path and on failure paths required by the
  interface contract.
- Convert expected failures into `Error`, `Result`, or the documented HRESULT.
- Do not let a panic cross the generated external function. Remove panic paths from boundary code
  or catch them at a layer that can map them to a defined failure.
- Keep interface references alive for as long as stored raw pointers derived from them are used.

`Error::new` or `Error::from_hresult` preserves an HRESULT for projected `Result` methods. Use the
error code specified by the interface contract; an arbitrary failure code makes native callers
hard to diagnose.

Reference-counted interface fields can form cycles. COM reference counting does not collect them.
Use weak references where the interface model supports them or define an explicit teardown path.

## Practical component workflow

1. Generate or declare the interfaces and inspect their `*_Impl` traits.
2. Design a Rust state type with explicit synchronization and apartment assumptions.
3. Add `#[implement]` with only the interfaces the object must expose.
4. Implement every required `*_Impl` trait on the generated wrapper.
5. Convert the value to the narrowest interface returned to the caller.
6. Test `cast` for each supported and unsupported interface, not just direct method calls.
7. Test failure HRESULTs, out-parameter initialization, final release, and any cross-thread use
   promised by agility.

For static factories and exported components, the repository's robot component sample shows
`StaticComObject`, activation factory output, `OutRef`, and generated implementation traits. The
implementation test crates contain smaller examples for identity, inheritance, generic WinRT
interfaces, agility, and error propagation.

### Pitfalls

- Implementing the Rust trait does not make an incorrect interface declaration ABI-safe.
- Interior mutability without synchronization can violate the default agile contract.
- Returning an interface to native code transfers a reference-counted ownership interest; avoid
  temporary raw pointers that outlive their owner.
- `cast` success is governed by the interface list and inheritance graph generated into
  `QueryInterface`.
- Generic constraints from the annotated struct are carried into the generated wrapper. Keep them
  compatible with every generated `*_Impl` trait.

When procedural macros are disabled, `windows-core::implement_decl!` covers the common always-agile
case with a more explicit and narrower declarative syntax. Use its module documentation as the
grammar reference.

---

## Internal documentation

`windows-implement` is a `syn`/`quote` procedural macro crate. `src/lib.rs` parses the annotated
struct, interface list, generics, `Agile`, and `TrustLevel` options. `src/gen.rs` emits the wrapper,
vtable chains, identity and reference-counting implementation, interface conversions, and
`QueryInterface` routing.

The crate depends on the `*_Impl` traits and vtable constructors emitted by
`windows-interface` or `windows-bindgen`. Generated paths target `::windows_core`; the proc macro
crate is separated only to satisfy Rust's procedural macro packaging model.

Run `cargo test -p windows-implement`; the `test_implement` and `test_implement_core` crates cover
the runtime contract more broadly.
