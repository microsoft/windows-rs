# windows-interface

> The procedural macro that turns a Rust trait declaration into a COM interface.

- 📦 [crates.io](https://crates.io/crates/windows-interface)
- 📖 [docs.rs](https://docs.rs/windows-interface)
- 🚀 [Getting started](../../crates/libs/interface/readme.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/interface)

`windows-interface` provides the `#[interface]` attribute used to declare a COM interface that can
be called or implemented from Rust.

## Intended audience

Prefer interfaces from a focused crate or generated with
[`windows-bindgen`](windows-bindgen.md). Binary applications may also use interfaces from the broad
[`windows`](windows.md) projection. Declare an interface manually only when the ABI is owned by the
project, the interface is missing from metadata, or a focused interoperability test needs an
independent declaration.

The macro is normally consumed as `windows_core::interface`, not by depending on this crate
directly. `windows-core` re-exports it through its default `proc-macros` feature and supplies every
runtime type referenced by the generated code. The separate crate exists because procedural macros
must be compiled as a `proc-macro` crate.

See [`windows-core`](windows-core.md) for the paired declaration and implementation example. Use
the lower-level `interface_decl!` macro from `windows-core` only when the procedural macro feature
is disabled and its narrower grammar is sufficient.

## Designing a declaration

An interface declaration records an ABI, not an ordinary Rust trait design. Verify these items
before writing it:

1. Use the interface's exact IID. Reusing or mistyping an IID can make `QueryInterface` return an
   object with an incompatible vtable.
2. Name the direct base interface after `:`. Use `IUnknown` for a base COM interface.
3. Preserve method order, including every inherited method represented by the parent.
4. Match parameter types, pointer constness, return types, and Windows integer widths exactly.
5. Mark the trait `unsafe`; callers and implementors must uphold the ABI contract.

The accepted trait form is intentionally restricted. Methods take `&self`; async methods,
generics, explicit ABI declarations, variadics, default bodies, and non-documentation attributes
are rejected. One parent path describes the vtable inheritance chain.

For COM methods that return only an HRESULT status, `Result<()>` provides a projected caller and
implementor shape while the vtable entry returns `HRESULT`. An explicit `HRESULT` return preserves
the raw status. Do not replace an ABI out parameter with `Result<T>` by analogy: declare the
parameter shape expected by the macro and ABI. `Ref<T>` and `OutRef<T>` are supported for interface
input and output parameters and project to the corresponding `Param` and `OutParam` caller forms.

## What the macro generates

For an interface named `IValue`, `#[interface]` generates:

- the transparent `IValue` interface wrapper;
- an `Interface` implementation containing the IID and vtable type;
- `Deref` to the declared parent interface;
- caller methods that dispatch through the vtable;
- the `IValue_Vtbl` layout;
- the `IValue_Impl` trait used by `#[implement]`;
- conversions and `QueryInterface` support needed by `windows-core`.

Caller methods remain `unsafe` because the declaration cannot prove pointer validity or
API-specific invariants. The wrapper handles COM identity and reference counting through
`windows-core`; it does not validate a hand-written ABI.

## Practical workflow and pitfalls

Keep a manual interface declaration next to a citation or checked-in definition of its ABI. Add a
test that exercises both a known implementation and `QueryInterface` through the expected IID when
possible.

Common failures are severe:

- A wrong method order or signature dispatches through the wrong machine-level contract.
- A wrong parent produces an invalid inherited vtable layout.
- A correct IID paired with an incorrect layout can corrupt memory.
- Rust references in an ABI are valid only when their representation and lifetime are part of the
  agreed Rust-to-Rust contract. Public COM ABIs normally use Windows ABI types and pointers.
- `Result<()>` translates an error into HRESULT at the boundary; panics are not COM errors and must
  not unwind across an external call.

Prefer generating the declaration from `.winmd` with `windows-bindgen` when metadata can describe
the interface. That keeps caller and implementation projections aligned and avoids maintaining a
vtable by hand.

---

## Internal documentation

`windows-interface` is a `syn`/`quote` procedural macro crate. `src/lib.rs` parses the restricted
trait and GUID. `src/generation.rs` emits the interface wrapper, vtable, implementation trait,
inheritance traversal, and `Ref<T>`/`OutRef<T>` parameter adapters. `src/guid.rs` parses the IID.

The generated paths are rooted at `::windows_core`, which is why direct use still requires a
compatible `windows-core` dependency. The implementation macro consumes the generated `*_Impl`
traits and vtable constructors; changes to either macro must preserve that contract.

Run `cargo test -p windows-interface`; the `test_interface` and implementation test crates cover
cross-crate behavior.
