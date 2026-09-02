# windows-link

> A macro for declaring Windows DLL imports without an import library.

- 📦 [crates.io](https://crates.io/crates/windows-link)
- 📖 [docs.rs](https://docs.rs/windows-link)
- 🚀 [Getting started](../../crates/libs/link/readme.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/link)

`windows-link` provides the `link!` macro. It emits a raw external function declaration using
Rust's `raw-dylib` linking mode and a function-pointer type alias with the same name.

## Intended audience

This crate is for binding generators and authors of narrow low-level bindings. The generated
`windows` and `windows-sys` crates already declare and link the Windows APIs they expose, so
application code should not redeclare those functions.

Use `windows-link` directly when:

- an exported Windows function is absent from the available metadata;
- a project owns a small reviewed raw binding;
- generated `--sys` bindings use `link!` as their runtime link mechanism;
- the function-pointer type emitted beside the declaration is useful to an abstraction.

Use [`windows-bindgen`](windows-bindgen.md) when several metadata-backed declarations are needed.
A binary application may use [`windows`](windows.md) or [`windows-sys`](windows-sys.md) when the
published umbrella crates already contain the API. Reusable libraries should prefer focused crates
and narrow generated bindings.

Unlike `#[interface]` and `#[implement]`, `link!` is not a procedural macro normally re-exported for
application use. Rich generated bindings invoke it through `windows-core`; raw generated bindings
depend on `windows-link` directly. Most users therefore encounter this crate as binding
infrastructure rather than as an application dependency.

## Declaration contract

The README shows the macro grammar and its function-pointer alias. Every token in a declaration is
part of the ABI contract:

- The library literal must name the DLL that exports the symbol on supported Windows versions.
- The ABI literal must match the export, commonly `"system"` for Windows APIs.
- The optional link-name literal maps the Rust identifier to a differently named export.
- Parameter and return types must match the export exactly, including pointer constness and integer
  width.
- The declaration is unsafe even when a particular function happens to accept only integers.

On x86 Windows, the macro requests undecorated import names. On other Windows architectures it
uses a verbatim raw-dylib import. Do not add architecture-specific spelling to compensate without
checking the actual exported symbol and Rust's ABI behavior.

The type alias has the same identifier as the function because Rust has separate value and type
namespaces. This permits code such as `let callback: GetTickCount = GetTickCount;`. The alias is an
`unsafe extern` function pointer and carries the same caller obligations as the direct declaration.

## Practical workflow

Before adding a declaration:

1. Search the `windows` and `windows-sys` API reference for the symbol.
2. Confirm the DLL, export name, calling convention, and exact signature from an authoritative
   Windows definition.
3. Decide which Windows versions and architectures the surrounding crate supports.
4. Put the declaration in a private FFI module.
5. Add a safe wrapper only if its preconditions, ownership, result convention, and cleanup can be
   expressed correctly.
6. Test on each supported architecture when decoration, variadics, or unusual ABI types are
   involved.

`link!` only arranges static import resolution. It is not delayed loading and does not make an
optional export safe to call on older systems. For optional APIs, resolve the symbol dynamically
and keep the module loaded for at least as long as the function pointer can be called.

### Pitfalls

- A successful link does not prove the Rust signature is ABI-compatible.
- DLL forwarding and API-set names are Windows implementation details; use the library contract
  documented for the API.
- The macro does not generate feature gates. Add the surrounding `cfg` and Cargo feature policy in
  the binding crate.
- A declaration produced on a non-Windows target does not make the Windows export available there.
- Variadic and uncommon calling conventions have Rust and architecture restrictions. Prefer
  metadata-backed `windows-sys` output when it supports the declaration.
- Do not duplicate a symbol declaration with a different signature in the same program.

---

## Internal documentation

The crate is hand-written and `no_std`. `src/lib.rs` contains the three `link!` expansions: x86
Windows, other Windows architectures, and non-Windows parsing support. The Windows expansions use
`kind = "raw-dylib"` and `+verbatim`; x86 also sets `import_name_type = "undecorated"`.

`windows-bindgen` emits this macro for sys-style bindings unless `--extern` is selected.
`windows-core` re-exports the macro for rich and minimal generated bindings, while `windows-sys`
calls it through its direct dependency.

Run `cargo test -p windows-link`; generated binding tests exercise its wider use.
