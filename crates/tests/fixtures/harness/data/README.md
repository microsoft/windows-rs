# Test fixture format

This directory holds the data-driven fixtures consumed by the
`test_fixtures` harness (`crates/tests/fixtures/harness`). It is the
phase‑1 implementation of the consolidation plan in
[`docs/test-todo.md`](../../../../../docs/test-todo.md).

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

## `fixture.toml`

A deliberately tiny key/value subset of TOML, parsed without external
dependencies. Supported keys:

| key              | type       | applies to | meaning                                   |
|------------------|------------|------------|-------------------------------------------|
| `filter`         | string     | all        | namespace filter (default: `"Test"`)      |
| `references`     | string[]   | rdl/clang/bindgen/merge | extra reader/writer inputs   |
| `no_allow`       | bool       | bindgen    | pass `--no-allow` to bindgen              |
| `no_comment`     | bool       | bindgen    | pass `--no-comment` to bindgen            |
| `specific_deps`  | bool       | bindgen    | pass `--specific-deps` to bindgen         |
| `kind`           | string     | error      | `"reader"` (default) or `"writer"` — which stage must fail |

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

## Phase‑1 scope

This first cut intentionally lands the harness *alongside* the
existing `roundtrip/{rdl,clang,bindgen}`, `libs/rdl/tests/panic.rs`
and `libs/metadata/tests/merge.rs` tests rather than replacing them.
That gives us a chance to validate the harness on real fixtures
without losing coverage. Subsequent phases (see `docs/test-todo.md`)
migrate the bulk fixtures over, then delete the old crates.
