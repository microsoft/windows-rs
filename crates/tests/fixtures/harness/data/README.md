# Test fixture format

This directory holds the data-driven fixtures consumed by the
`test_fixtures` harness (`crates/tests/fixtures/harness`).

## Layout

```
crates/tests/fixtures/harness/data/
    <group>/
        <name>/
            input.rdl | input.h | input-*.rdl   # required
            fixture.toml                        # optional
            expected.rdl | expected.rs | expected.err
```

`build.rs` enumerates every `<group>/<name>/` directory and emits one
`#[test]` per fixture; each test calls back into `run_fixture(group,
name)`. Scratch output goes under `$OUT_DIR/scratch/<group>/<name>/` so
parallel `#[test]` execution is preserved.

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

A tiny key/value subset of TOML, parsed without external dependencies.
Supported keys:

| key              | type       | applies to | meaning                                   |
|------------------|------------|------------|-------------------------------------------|
| `filter`         | string     | all        | namespace filter (default: `"Test"`)      |
| `references`     | string[]   | rdl/clang/bindgen/merge/winmd_to_rdl | extra reader/writer inputs (paths relative to the fixture dir for `winmd_to_rdl`) |
| `winmd_input`    | string     | winmd_to_rdl | path (relative to the fixture dir) of the prebuilt winmd to filter |
| `no_allow`       | bool       | bindgen    | pass `--no-allow` to bindgen              |
| `no_comment`     | bool       | bindgen    | pass `--no-comment` to bindgen            |
| `specific_deps`  | bool       | bindgen    | pass `--specific-deps` to bindgen         |
| `kind`           | string     | error      | `"reader"` (default), `"reader_no_input"`, or `"writer"` — which stage must fail |
| `arch_inputs`    | string[]   | merge      | per-input arch tagging, e.g. `["input-x86.rdl=X86", "input-x64.rdl=X64"]`. Arches are `X86`/`X64`/`Arm64` or `\|`-joined. |
| `outputs`        | string[]   | rdl        | run the writer multiple times. Each entry is `"<expected>=<filter[;filter...]>"`; `;` separates multiple `writer.filter(...)` calls. |

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

Each fixture is a normal `#[test]`, so standard Cargo filtering works:

```sh
cargo test -p test_fixtures              # everything
cargo test -p test_fixtures bindgen_     # just the bindgen group
cargo test -p test_fixtures rdl_enum     # a single fixture
```

## What stays bespoke

A handful of test files under `crates/tests/` deliberately do not use
this fixture format because they exercise things the harness doesn't
model (runtime semantics of generated bindings, direct builder/reader
API calls, OS-level I/O failures, structural attribute assertions):

- `tests/libs/rdl/tests/{assembly_name, const-underlying, const-underlying-rdl, directory, error, exclusive-to, fn_abi, guid-derive, split, struct_fields, struct_values, writer_errors}.rs`
- `tests/libs/metadata/tests/{empty, struct, attribute, class, interface, reader, load_library, assembly_name}.rs`
- `tests/libs/bindgen/tests/{bool, deps, delegate_*, panic, ref_params}.rs`
- `tests/{misc,winrt}/**`

Migrate them as future changes make them expressible as a roundtrip,
golden-Rust, or error fixture; otherwise leave them alone.
