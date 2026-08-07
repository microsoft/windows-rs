## RDL parser library and ECMA-335 generator

The [windows-rdl](https://crates.io/crates/windows-rdl) crate compiles **RDL** (Rust Definition
Language) - a Rust-like text format for describing Windows APIs - into ECMA-335 `.winmd` metadata,
and back again.

* [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)
* [Releases](https://github.com/microsoft/windows-rs/releases)

Start by adding the following to your Cargo.toml file:

```toml
[dependencies.windows-rdl]
version = "0.100"
```

Use the `reader` to compile `.rdl` source into a `.winmd`, and the `writer` to regenerate canonical
`.rdl` from a `.winmd`:

```rust,no_run
windows_rdl::reader()
    .input("example.rdl")
    .output("example.winmd")
    .write()
    .unwrap();

windows_rdl::writer()
    .input("example.winmd")
    .output("example.rdl")
    .write()
    .unwrap();
```

Use `.check()` to run the same parse, validation, resolution, and encoding pipeline without
writing a `.winmd`. Use `.check_all()` to collect independent diagnostics from every input.

Use `.reference("dependency.winmd")` when the RDL refers to types defined by another metadata file.
Use `.input_text(source)` or `.input_texts(sources)` for RDL already in memory. Use
`.reference_default()` for the standard Windows metadata.

Use `.input_text_named("schema.rdl", source)` or `.input_texts_named(sources)` when in-memory source
names should appear in diagnostics. `Diagnostic` stores a severity, optional code, source labels,
notes, and help. `DiagnosticReport` stores the collected diagnostics and their original source
text. `Error` is a small owned wrapper that dereferences to its `Diagnostic`.

The reader reports `RDL0001` for duplicate symbols, `RDL0002` for accepted syntax that cannot be
represented in metadata, `RDL0003`/`RDL0004` for import failures, and `RDL0005` for generic-arity
errors. The writer likewise rejects metadata forms that have no lossless RDL spelling rather than
emitting incomplete source.

The `riddle` binary provides the same operations from a terminal:

```text
riddle check example.rdl
riddle build example.rdl --out example.winmd
riddle fmt example.rdl
riddle fmt --check example.rdl
```

`formatter::format` and `formatter::format_named` validate complete RDL source and return a
diagnostic on invalid input. Regular comments and documentation comments are preserved.

The winmd writer matches `Param` rows by ECMA-335 `Param.Sequence`, not table order. Sparse methods
still emit every signature parameter, using `pN` and the reader's type-based default direction when
a row is absent. Sequence 0 return attributes are emitted on the return type. Duplicate and
out-of-range sequences are errors.

The writer reads raw direction and optionality through `MethodParam::direction()` and
`is_optional()`. Reserved, retval, and count attributes remain separate pseudos/custom attributes;
the metadata layer does not merge them with projection policy.

Canonical output spells the input direction as `#[in]`; the reader also accepts Rust's raw
identifier spelling, `#[r#in]`.

Some metadata states do not have a lossless RDL spelling. Parameter direction cannot be neither
In nor Out because an omitted direction is inferred. Attributes on a void return row cannot be
written because there is no return type to carry them. `#[len_param(N)]` and `#[size_param(N)]`
store raw parameter positions, so reordering parameters also requires updating `N`. Pointer chains
must use one constness throughout, such as `*mut *mut T` or `*const *const T`; mixed chains are
rejected. Explicit-layout types can be written as RDL unions only when every instance field has
offset zero.
