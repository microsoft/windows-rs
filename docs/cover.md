# Code coverage: windows-bindgen and windows-rdl

This is a data-driven review of test coverage for `windows-bindgen` and
`windows-rdl`, measured with `cargo llvm-cov` over the three dedicated test
crates. It complements [bindgen-test.md](bindgen-test.md) (which catalogs *what*
features are tested) by reporting *how much* of the library code those tests
actually exercise, and where the remaining gaps are.

## How to reproduce

```sh
cargo llvm-cov --no-cfg-coverage \
  -p test_bindgen -p test_clang -p test_rdl \
  -p windows-bindgen -p windows-rdl
```

The lib packages must be listed alongside the test crates, otherwise their
regions are not reported. Use `--summary-only` for just the totals.

## Snapshot (June 2026, 115 tests)

| Scope | Region | Line |
|-------|-------:|-----:|
| **TOTAL** | **71.7%** | **74.0%** |

Up from 69.3% / 71.4% after adding the four targeted fixtures below.

## What each test crate covers

The three crates partition the three subsystems with almost no overlap, so a gap
in one subsystem maps directly to one test crate:

| Test crate | Pipeline | Sole driver of | Also drives |
|------------|----------|----------------|-------------|
| `test_clang`  | C header → RDL golden   | `rdl/src/clang/*`  | — |
| `test_rdl`    | RDL → winmd → RDL        | `rdl/src/writer/*`  | `rdl/src/reader/*` |
| `test_bindgen`| RDL → winmd → Rust golden| `bindgen/src/*`     | `rdl/src/reader/*` |

`rdl/src/reader/*` is the one shared subsystem (both `test_rdl` and
`test_bindgen` parse RDL). `test_rdl` does not invoke `windows-bindgen` at all,
and `test_clang` stops at RDL text, so all of `bindgen/src/*` is covered only by
`test_bindgen`.

## Remaining gaps

### (A) Legacy / out-of-scope — intentionally untested

These are not reachable from RDL-driven tests and are deliberately left
uncovered. Do not try to cover them here.

| File | Line % | Why |
|------|-------:|-----|
| `bindgen/libraries.rs` | 0% | Import-lib generator; only called by `tools/msvc` + `tools/gnu` for the umbrella libs. Loads the bundled `default` winmd — unreachable from RDL. Live tooling, not dead code. |
| `bindgen/package_writer.rs` | 11% | `--package` mode for the legacy published crates. |
| `bindgen/minimal_type_map.rs` | 40% | `--minimal` dependency graph; only `minimal_deps.rdl` touches it. |

### (B) test_bindgen gaps — fixable with RDL fixtures

| File | Line % | Missing |
|------|-------:|---------|
| `references.rs` | 48% | Cross-namespace sibling reference resolution. |
| `filter.rs` | 55% | Filter branches beyond the current `method_filter_*` fixtures. |
| `types/cpp_handle.rs` | 46% | Explicit invalid-handle values, `Free` impls, and `AlsoUsableFor` — all driven by metadata attributes (`RAIIFree`, `InvalidHandleValue`, `AlsoUsableForAttribute`) that RDL cannot express. Like group (A), not reachable from RDL. |
| `types/cpp_method.rs` | 59% | COM method param/return shape combinations. |

### (C) test_clang gaps

| File | Line % | Missing |
|------|-------:|---------|
| `clang/mod.rs` | 57% | Driver branches (multiple includes, namespace/library directive combos). |

### (D) test_rdl gaps

| File | Line % | Missing |
|------|-------:|---------|
| `writer/mod.rs` | 52% | winmd → RDL driver branches. |
| `reader/attribute_ref.rs` | 69% | Reference-winmd attribute path (`find_in_reference`) — needs an external reference winmd, which these harnesses do not supply. |

## Recommended fixtures — implemented

All four landed, with measured before → after line coverage:

1. **`clang/input/sal_params.h`** — SAL annotations via portable `annotate`
   stubs. `clang/annotation.rs` **39% → 83%**.
2. **`bindgen/input/const_value.rdl`** + **`const_string_sys.rdl`** — f32/f64/
   string constants (default + sys). `value.rs` **48% → 56%**,
   `cpp_const.rs` **51% → 66%**.
3. **`bindgen/input/enum_scoped.rdl`** — `ScopedEnumAttribute` defined in RDL and
   applied to a win32 enum, exercising the scoped-enum codegen path.
   `cpp_enum.rs` **47% → 73%**.
4. **`rdl/input/attribute_args.rdl`** — an RDL-defined attribute applied with
   string, enum, flags (`A | B`), and named (`Name = value`) arguments.
   `reader/attribute_ref.rs` **49% → 69%**, `writer/attribute.rs` **64% → 96%**.

## Dead code

No file is dead across the whole workspace. The 0% / low-coverage entries in
group (A) are all legacy-mode or umbrella-lib paths outside the RDL test surface
(`libraries.rs` is live tooling for `tools/msvc` and `tools/gnu`), so they should
be left as-is rather than deleted.
