# Test fixture format

This directory holds the data-driven fixtures consumed by the
`test_fixtures` harness (`crates/tests/fixtures/harness`). It is the
single living document for how the parser/generator test suite is
organised; older planning notes have been removed now that the
consolidation is complete.

## Layout

```
crates/tests/fixtures/harness/data/
    <group>/
        <name>/
            input.rdl | input.h | input-*.rdl   # required
            fixture.toml                        # optional
            expected.rdl | expected.rs | expected.err
```

`build.rs` in the harness crate enumerates every `<group>/<name>/`
directory at compile time and emits one `#[test]` per fixture. Each
generated `#[test]` calls back into the harness's `run_fixture(group,
name)` dispatcher; nothing in `data/` is itself Rust code.

The harness writes scratch output under
`$OUT_DIR/scratch/<group>/<name>/`, so concurrent tests never share
filesystem state and Cargo's normal parallel `#[test]` execution is
preserved (this is what keeps the suite as fast as the existing
`roundtrip/{rdl,clang,bindgen}` crates).

## Groups

| group     | inputs                       | check                                                |
|-----------|------------------------------|------------------------------------------------------|
| `rdl`     | `input.rdl`                  | RDL → winmd → RDL, diff vs. `expected.rdl`           |
| `clang`   | `input.h`                    | Clang → RDL → winmd → RDL, diff vs. `expected.rdl`   |
| `bindgen` | `input.rdl` + `fixture.toml` | RDL → winmd → bindgen, diff vs. `expected.rs`        |
| `error`   | `input.rdl` + `expected.err` (+ optional `defs-*.rdl` for `kind = "writer"`) | reader **or** writer fails with the expected message |
| `merge`   | `input-*.rdl` (≥ 2)          | each → winmd → merge → RDL, diff vs. `expected.rdl`  |
| `winmd_to_rdl` | `fixture.toml` only (`winmd_input` + `filter`) | writer reads a prebuilt winmd, diff vs. `expected.rdl` |

## `fixture.toml`

A deliberately tiny key/value subset of TOML, parsed without external
dependencies. Supported keys:

| key              | type       | applies to | meaning                                   |
|------------------|------------|------------|-------------------------------------------|
| `filter`         | string     | all        | namespace filter (default: `"Test"`)      |
| `references`     | string[]   | rdl/clang/bindgen/merge/winmd_to_rdl | extra reader/writer inputs (paths relative to the fixture dir for `winmd_to_rdl`) |
| `winmd_input`    | string     | winmd_to_rdl | path (relative to the fixture dir) of the prebuilt winmd to filter |
| `no_allow`       | bool       | bindgen    | pass `--no-allow` to bindgen              |
| `no_comment`     | bool       | bindgen    | pass `--no-comment` to bindgen            |
| `specific_deps`  | bool       | bindgen    | pass `--specific-deps` to bindgen         |
| `kind`           | string     | error      | `"reader"` (default), `"reader_no_input"`, or `"writer"` — which stage must fail |
| `arch_inputs`    | string[]   | merge      | per-input arch tagging, e.g. `["input-x86.rdl=X86", "input-x64.rdl=X64"]`. Arches are `X86`/`X64`/`Arm64` or `\|`-joined. When set, the harness uses `Merger::arch_input` so types present in only some arches get a `SupportedArchitecture` attribute. |
| `outputs`        | string[]   | rdl        | run the writer multiple times. Each entry is `"<expected>=<filter[;filter...]>"`; `;` separates multiple `writer.filter(...)` calls in a single invocation. When omitted, the runner falls back to one writer with `filter` (default `"Test"`) and `expected.rdl`. |

The format is a strict subset of real TOML so a fixture written today
will keep parsing if the harness later swaps in a full TOML crate.

## Adding a fixture

1. Make a new directory under `data/<group>/<your-name>/`.
2. Drop the `input.*` files in. For `bindgen`, also drop a
   `fixture.toml` that records the CLI options.
3. Run `UPDATE_GOLDEN=1 cargo test -p test_fixtures` to regenerate
   the `expected.*` files.
4. Inspect the generated goldens, commit everything together.

After the initial run, plain `cargo test -p test_fixtures` asserts
that the actual output matches the committed golden.

## Filtering

Each fixture is a normal `#[test]`, so you can use the standard Cargo
filter syntax:

```sh
# everything
cargo test -p test_fixtures

# just the bindgen group
cargo test -p test_fixtures bindgen_

# a single fixture
cargo test -p test_fixtures rdl_enum
```

## What stays bespoke (and why)

A handful of test files under `crates/tests/` deliberately do **not** use
this fixture format. They exercise things the harness doesn't model
(runtime semantics of generated bindings, direct builder/reader-API
calls, OS-level I/O failures, structural assertions on attribute
values), and forcing them through a byte-stable RDL roundtrip would lose
coverage rather than add it:

- `tests/libs/rdl/tests/{assembly_name, const-underlying, const-underlying-rdl, directory, error, exclusive-to, fn_abi, guid-derive, split, struct_fields, struct_values, writer_errors}.rs`
- `tests/libs/metadata/tests/{empty, struct, attribute, class, interface, reader, load_library, assembly_name}.rs`
- `tests/libs/bindgen/tests/{bool, deps, delegate_*, panic, ref_params}.rs`
- `tests/{misc,winrt}/**`

If a future change makes one of these expressible as a roundtrip,
golden-Rust, or error fixture, migrate it; otherwise leave it alone.
