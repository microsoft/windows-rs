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

## Snapshot (June 2026, 124 tests)

| Scope | Region | Line |
|-------|-------:|-----:|
| **TOTAL** | **73.4%** | **75.6%** |

Progression: 69.3% / 71.4% → 71.7% / 74.0% (first fixture round) →
73.4% / 75.6% (reference + filter + minimal-closure fixtures below).

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
| `bindgen/types/cpp_handle.rs` | 46% | Explicit invalid-handle values, `Free` impls, and `AlsoUsableFor` — all driven by metadata attributes (`RAIIFree`, `InvalidHandleValue`, `AlsoUsableForAttribute`) that RDL cannot express. |
| `bindgen/types/cpp_method.rs` | 60% | Remaining gaps are array params (`NativeArrayInfoAttribute`, `MemorySizeAttribute`) and the COM-query pattern (`ComOutPtrAttribute`). RDL param attributes are limited to `#[in]`/`#[out]`/`#[opt]`/`#[retval]`, so these shapes are unreachable. |

RDL's param attribute vocabulary (`reader/param.rs`) is the hard ceiling for
`cpp_method.rs` and `cpp_handle.rs`: any codegen branch gated on a win32-metadata
attribute that has no RDL surface cannot be reached from these crates.

### (B) test_bindgen gaps — fixable with RDL fixtures

The reference, filter, and minimal-closure gaps from the previous snapshot are
now covered (see "Recommended fixtures" below):

| File | Was | Now | Note |
|------|----:|----:|------|
| `references.rs` | 48% | **100%** | `reference_numerics*.rdl` drive sibling-crate resolution. |
| `filter.rs` | 55% | **78%** | `filter_name_glob`, `method_filter_{enum,class,sugar,accessor}`. |
| `minimal_type_map.rs` | 40% | **61%** | `minimal_{delegate,class}_dep` + `reference_numerics_minimal`. |
| `cpp_method.rs` | 59% | **60%** | `ReturnValue` (void + `#[retval]`) folded into `method_return.rdl`; rest is attribute-gated (group A). |

The residual `minimal_type_map.rs` gap is the C++ closure arms (`CppStruct`,
`CppInterface`, `CppFn`, `CppConst`) — reachable only via a `#[win32]` member or
specific-type filter, since a whole-namespace `--filter Test` sets
`has_broad_filter` and bypasses `MinimalTypeMap::build` entirely.

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

### Round 1

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

### Round 2 — references, filters, minimal closure

The linchpin was the **reference path**: it is only reached when the input winmd
contains a WinRT probe namespace (e.g. `Windows.Foundation.Numerics`), which then
maps to a sibling crate (`windows_numerics`) and emits referenced types as
`windows_numerics::Vector3` instead of generating them. One fixture pair unlocked
both `references.rs` and the minimal reference-skip arms.

1. **`bindgen/input/reference_numerics.rdl`** (+ `_minimal` variant) — a
   `Test.Transform` struct whose field is `Windows::Foundation::Numerics::Vector3`.
   `references.rs` **48% → 100%**. The `_minimal` variant uses a *specific-type*
   filter (`Test.Transform`) so it routes through `MinimalTypeMap::build`.
   (Requires `windows-numerics` as a `test_bindgen` dependency so the golden
   compiles, alongside the existing `windows-core` / `-link` / `-result`.)
2. **`bindgen/input/minimal_delegate_dep.rdl`** + **`minimal_class_dep.rdl`** —
   member-filtered (`Sink::Subscribe`, `Factory::Create`) minimal fixtures that
   drive the Delegate and Class arms of `combine_minimal`.
   `minimal_type_map.rs` **40% → 61%**.
3. **`bindgen/input/filter_name_glob.rdl`** — a `Ns::Prefix*` glob filter
   (`Test::Wid*`), the only fixture exercising the `NameGlob` resolution arm.
4. **`bindgen/input/method_filter_enum.rdl`** — enum-variant member filter
   (`Color::{Red,Green}`).
5. **`bindgen/input/method_filter_class.rdl`** — class member-routing
   (`Widget::Act` locates the carrying interface).
6. **`bindgen/input/method_filter_sugar.rdl`** + **`method_filter_accessor.rdl`** —
   bare-name property sugar (`::Value` → get+put) and accessor-only sugar
   (`::get:Value` / `::set:Value`) in `expand_method_part`.

   Together (3)–(6) took `filter.rs` **55% → 78%**.
7. **`method_return.rdl`** gained a `ReturnValue` method (void return + `#[retval]`
   out-param), nudging `cpp_method.rs` to 60%.

## Test-crate consistency notes

The three harnesses were aligned during this review:

- **Scratch + codegen paths**: `test_rdl` used a hardcoded
  `../../../../target/test_rdl` directory; it now uses `OUT_DIR` like
  `test_bindgen` and `test_clang`.
- **`rust_ident` (build.rs)**: the three copies now share one keyword list and the
  `-`→`_` replacement.
- **`include!` placement**: all three `tests/*.rs` put the generated `#[test]`
  module include at the top.
- **Negative tests**: both `bindgen/tests/errors.rs` and `rdl/tests/errors.rs` use
  `#[should_panic]` (no bespoke catch-unwind harness).

Fixture audit: no two `input/*.rdl` files are byte-identical, and the
`*_minimal` / `*_sys` variants each differ from their base golden (minimal omits
the `RuntimeType::NAME` `ConstBuffer` and tweaks derives; sys flattens to raw
ABI), so none are redundant. The `const_value.rdl` literals were changed away
from approximations of π/e so the compiled goldens are clippy-clean.

## Dead code

No file is dead across the whole workspace. The 0% / low-coverage entries in
group (A) are all legacy-mode, umbrella-lib, or attribute-gated paths outside the
RDL test surface (`libraries.rs` is live tooling for `tools/msvc` and
`tools/gnu`), so they should be left as-is rather than deleted.
