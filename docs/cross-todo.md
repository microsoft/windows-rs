# Cross-platform staging plan

This document tracks the work needed to make the windows-rs libraries cross-platform
in stages, and to verify the result on a GitHub Linux runner.

## Current state

**Already cross-platform (no `#![cfg(windows)]`, no Win32 calls at runtime):**
- `windows-bindgen`, `windows-metadata`, `windows-rdl`, `riddle` — pure code-gen / metadata libs.
- `windows-numerics` — pure math.
- `windows-interface`, `windows-implement` — proc-macros (build on Linux already).
- `windows-link`, `windows-targets` — mostly just macros/linkage; the inner `cfg(all(windows, target_arch = "x86"))` gates the only Windows-specific bit.

**Hybrid (no top-level `cfg(windows)`, but inner `#[cfg(windows)]` gates the Win32-using parts; CI-enforced to build on Linux):**
- `windows-core` — most trait machinery (`Type`, `Interface`, `IUnknown` vtables, `Param`, `Array`, `GUID`, `IInspectable`) is portable; only ref-count/marshal helpers in `imp/` and a few functions in `inspectable.rs` are gated. Builds cleanly on Linux with default features and `--no-default-features`. (Stage 2.)
- `windows-result` — `HRESULT`, `Error`, `Result` types are portable; `RoOriginate*` paths in `hresult.rs`/`error.rs` are gated, with `#[cfg(not(windows))]` fallbacks that return inert `E_FAIL`-shaped values. Builds cleanly on Linux with default features and `--no-default-features`. (Stage 2.)
- `windows-strings` — `HSTRING`, `HStringBuilder`, the `h!` literal macro, and the helpers behind them (`hstring_header`, `ref_count`) are cross-platform; the only Win32 dependency — the heap allocator — is swapped at compile time inside `hstring_header.rs` (Windows: `kernel32` process heap, so `HSTRING`s allocated here remain interop-compatible with native callers; other targets: the Rust global allocator). `PCSTR`, `PCWSTR`, `PSTR`, `PWSTR`, `s!`/`w!`, and the UTF-8/UTF-16 `decode` helpers were already portable. `BSTR` and the generated `bindings` module that fronts `oleaut32`/`kernel32` remain `#[cfg(windows)]`-gated because `BSTR` is part of the OLE Automation ABI and must use `SysAllocStringLen`/`SysFreeString` so callers across the FFI boundary can free strings allocated here. The `OsStr`/`OsString`/`Path` interop on `HSTRING` is `#[cfg(all(feature = "std", windows))]` because it depends on `std::os::windows::ffi`. (Stage 3.)

**Hard-gated `#![cfg(windows)]` at the top of `lib.rs` (compile to nothing off Windows):**
- `windows-cppwinrt` — invokes `cppwinrt.exe`, inherently Windows.
- `windows-threading` — `WaitEvent` + thread-pool wrappers are Win32; trait shape (`SubmitThreadpoolCallback` analog) could be exposed.
- `windows-version` — `GetVersionEx`/`RtlGetVersion` only.
- `windows-registry` — Win32 registry APIs only.
- `windows-services` — Win32 SCM only.
- `windows-future` — `IAsyncAction`/etc. trait *shapes* are portable, but the executor (`async_spawn`, `Waker`) calls into `windows-threading`.
- `windows-collections` — same: trait *shapes* (`IVector`, `IMap`, `IIterable`) and the stock `BTreeMap`/`Vec` adapters are portable; only event-handler dispatch reaches into `windows-core` machinery that currently requires `windows`.

**CI today (`linux.yml`):** runs `cargo test` on `test_linux`, `test_clang`, `test_roundtrip_clang`, `test_rdl`, `test_roundtrip_rdl` against `x86_64-unknown-linux-gnu`, then enforces no diff. As of Stage 0 it also runs `cargo build` and `cargo doc` on every library currently believed to be cross-platform, plus `cargo test` for `test_numerics`, `test_metadata`, `test_link`, and `test_targets`. As of Stage 2 the Linux build/doc matrix also covers `windows-core` and `windows-result`, and the `--no-default-features` matrix covers them as well. As of **Stage 3** the build/doc matrix and the `--no-default-features` matrix also cover `windows-strings`.

## Stage 0 — Better Linux test harness (do this first, no library changes)

Goal: turn the Linux runner into a tight feedback loop so each subsequent stage can be validated cheaply. Changes are confined to `.github/workflows/linux.yml` and a small amount of test gating.

1. ✅ Add an explicit "Linux build matrix" step that runs `cargo build --target x86_64-unknown-linux-gnu -p <crate>` for every library currently believed to be cross-platform: `windows-bindgen`, `windows-metadata`, `windows-rdl`, `riddle`, `windows-numerics`, `windows-link`, `windows-interface`, `windows-implement`, `windows-targets`. This locks in the current cross-platform surface and turns regressions into red CI immediately.
2. ✅ Reuse the existing per-lib test crates (`test_numerics`, `test_metadata`, `test_link`, `test_targets`) instead of inventing a new `cross_platform` umbrella crate — they already have unit-test coverage of the public surface and we only had to gate the genuinely Windows-only tests with `#[cfg(windows)]`:
   - `crates/tests/libs/metadata/tests/load_library.rs` (links `kernel32` to load a `.winmd` resource — Windows-only by definition).
   - `crates/tests/libs/link/tests/link.rs` and `crates/tests/libs/targets/tests/link.rs` (call `kernel32!SetLastError`/`GetLastError`; the `link!`/`windows_targets::link!` macros themselves do compile on Linux, but exercising them needs a Windows loader).
   - `crates/tests/libs/targets/tests/symbol.rs` (uses generated `windows::Win32::*` bindings — inherently Windows).

   The remaining tests (numerics math, `windows-metadata` reader/writer/merge, `test_targets/files.rs`) all run on Linux unchanged. This gives Stage 0 real test coverage for free.
3. ⏭️ A single `cargo build --workspace --exclude <not-yet-portable>` step is **not** added in Stage 0. The blocker is that the workspace currently contains generated mega-crates (`windows`, `windows-sys`) and many `test_*` crates that depend on them, so the exclude list would be enormous and noisy. It will become tractable once Stages 2–5 land and the "not portable" set shrinks to the genuinely-Windows crates listed in Stage 6. Keep this idea in the backlog.
4. ✅ Add a `cargo doc --no-deps -p <crate>` step on Linux for the cross-platform set so intra-doc links and the doc build path are exercised off-Windows.
5. ✅ Add a `cargo build --no-default-features -p <crate>` step for crates that opt into `no_std` (`windows-numerics`, `windows-link`, `windows-targets`) — this preserves the embedded-friendly path that the cross-cutting items section calls out.
6. ✅ Keep using the existing `ubuntu-22.04` runner; only add `KyleMayes/install-llvm-action` for crates that actually need libclang (already done).

**Stage 0 outcome:** "is crate X cross-platform?" is now answered by a green check; regressing any of the listed crates fails CI.

**Stage 0 findings worth carrying into later stages:**
- `windows-link` and `windows-targets` are linked into the workspace twice (path dep + crates.io dep pulled in transitively by `windows-bindgen`'s codegen of itself), so any tooling that walks the workspace needs to disambiguate via fully-qualified package specs (`-p 'path+file://...#windows-link@0.2.1'`). `cargo build -p windows-link` works fine; `cargo build -p windows-link -p windows-bindgen` together hits the ambiguity. The current workflow side-steps this by building each crate in its own `cargo` invocation.
- Several `test_*` crates contain a mix of portable and Windows-only tests in separate files. The `#[cfg(windows)]` per-file pattern works well and is the lowest-friction way to keep the Linux job green; new Windows-only tests should follow the same convention rather than living in a `windows`-only test crate.

## Stage 1 — Promote crates that are *already* portable but not advertised

**Status: ✅ done by Stage 0.**

The Stage-0 build matrix in `linux.yml` already covers every crate this stage was meant to add (`windows-numerics`, `windows-link`, `windows-targets`, `windows-interface`, `windows-implement`, `riddle`), so there is no library-logic work left here — Stage 0 already fulfills the contract.

The one remaining ambition from the original Stage 1 — "add a Linux trybuild/expand test that does not depend on `windows`/`windows-core` runtime symbols" for the proc-macro crates — is deferred. The existing `test_interface` / `test_implement` integration test crates depend on the `windows` mega-crate (Windows-only), so a Linux trybuild would require a new lean test crate. That is its own bounded piece of work and not on the critical path for higher stages.

Risk: none — this stage is now purely documentation.

## Stage 2 — Make `windows-core` and `windows-result` build on Linux

**Status: ✅ done in code; locked in by CI as of this stage.**

When this plan was written, the assumption was that `windows-core` and `windows-result` would need substantial gating work to compile on Linux. Investigation showed that work had already happened incrementally:

- `windows-result` has no top-level `#![cfg(windows)]`; `lib.rs` already gates `mod com`, `mod strings`, and `mod bstr` behind `#[cfg(windows)]`, and `error.rs`/`hresult.rs` already provide `#[cfg(not(windows))]` fallbacks for `from_thread()` and friends that return inert `E_FAIL`-shaped values. `cargo build -p windows-result --target x86_64-unknown-linux-gnu` succeeds with `RUSTFLAGS=-D warnings`, both with default features and `--no-default-features`.
- `windows-core` likewise has no top-level `#![cfg(windows)]`. `imp/`, `inspectable.rs`, and the COM ref-count helpers are individually gated, so the public types (`GUID`, `HRESULT`, `Type`, `Interface`, `Param`, `Array`, `IUnknown`, `IInspectable` definitions) all compile on Linux. `cargo build -p windows-core --target x86_64-unknown-linux-gnu` succeeds with `RUSTFLAGS=-D warnings`, both with default features and `--no-default-features`.

What this stage actually changes:

1. ✅ Add `windows-core` and `windows-result` to the Stage-0 Linux build matrix in `.github/workflows/linux.yml` so a regression that re-introduces a Win32 dependency on the portable surface fails CI immediately.
2. ✅ Add both crates to the `--no-default-features` matrix to preserve the `no_std` path on Linux.
3. ✅ Add both crates to the `cargo doc --no-deps` matrix on Linux so intra-doc links and the doc build path are exercised off-Windows.
4. ⏭️ **Deferred:** Linux-only unit tests for the portable pieces (GUID parsing, HRESULT formatting, `Error::new`/`Display`, Param trait derivations). The existing `test_core` / `test_result` crates pull in the `windows` mega-crate (which is Windows-only), so adding Linux unit tests requires a separate, lean test crate that depends only on `windows-core` / `windows-result`. Tracked for a follow-up PR — the build-only matrix already catches the most common regressions (`error[E0432]: unresolved import`, accidentally Windows-only public items, etc.).
5. ⏭️ **Deferred:** dropping `default-target = "x86_64-pc-windows-msvc"` from `[package.metadata.docs.rs]` in `crates/libs/core/Cargo.toml` and `crates/libs/result/Cargo.toml`. Both crates still expose meaningful Windows-only items behind `#[cfg(windows)]`, so flipping the docs.rs default to a Linux target would render strictly less content. Revisit once `#[doc(cfg(...))]` annotations are in place so docs.rs can show both surfaces from a single Linux build.

Risk: low. The code work was already done by previous incremental changes; this stage only enforces it.

## Stage 3 — Make the HSTRING family cross-platform via a swappable allocator

**Status: ✅ done; locked in by CI.**

The first iteration of Stage 3 split the crate so that everything HSTRING- or BSTR-shaped stayed `#[cfg(windows)]`-gated. Reviewer feedback narrowed the scope: only the `HSTRING` family needs to be made cross-platform, and the only thing inherently Windows-specific about it is the *allocator*. `BSTR` stays Windows-only because it's part of the OLE Automation ABI — callers across the FFI boundary expect to free `BSTR`s with `SysFreeString`, so substituting another allocator would break interop. The generated `crates/libs/strings/src/bindings.rs` is left untouched.

1. ✅ Localized the allocator swap to `crates/libs/strings/src/hstring_header.rs` (the only file that does HSTRING allocation):
   - `heap_alloc(bytes)` / `heap_free(ptr, bytes)` are `#[cfg]`-gated helpers private to that file.
   - On Windows they call into the existing generated `bindings::HeapAlloc` / `HeapFree` / `GetProcessHeap` (no changes to `bindings.rs`).
   - On other targets they call `alloc::alloc::alloc` / `dealloc` with a `Layout` of `(bytes, align_of::<HStringHeader>())`. `HStringHeader::free` rebuilds the byte size from `HStringHeader::len` (which is a no-op on Windows where `HeapFree` ignores it).
2. ✅ `HSTRING`, `HStringBuilder`, `HStringHeader`, `RefCount`, the `to_hstring()` methods on `PCWSTR`/`PWSTR`, and the `h!` literal macro are no longer `#[cfg(windows)]`-gated. The `h!` macro is interop-clean across platforms because `HSTRING_REFERENCE_FLAG` strings (the static, ref-counted-skip variant the macro emits) never touch the allocator.
3. ✅ `BSTR` (`crates/libs/strings/src/bstr.rs`) and the generated `bindings` module remain gated `#[cfg(windows)]` in `lib.rs`. They are unchanged from upstream.
4. ✅ The only `HSTRING` surface still gated is the `std::os::windows::ffi::OsStr` / `OsString` / `Path` interop — `from_wide`, `encode_wide`, `to_os_string`, the `From<&Path>` impl, and the `OsString`/`OsStr` `PartialEq` impls. Those rely on `std::os::windows`, which the standard library only ships on Windows. They remain gated `#[cfg(all(feature = "std", windows))]`.
5. ✅ Smoke-tested locally on Linux: HSTRING construction / clone / drop, HStringBuilder writeback, and the `h!` macro literal (including cloning, which exercises the `HSTRING_REFERENCE_FLAG` duplicate path) all round-trip back to the original UTF-8 string.
6. ✅ The crate-level `#[debugger_visualizer(natvis_file = ...)]` is `#[cfg_attr(windows, ...)]` since the `.natvis` file is irrelevant on other targets.
7. ✅ `windows-strings` is in the Linux build / `--no-default-features` / doc matrix in `.github/workflows/linux.yml`.

Net effect:

```rust
// works on every target — no cfg dance for HSTRING callers
use windows_strings::{HSTRING, HStringBuilder, PCSTR, PCWSTR, s, w, h};

let s: HSTRING = "hello".into();   // allocates via kernel32 on Windows,
                                   // via the Rust allocator elsewhere
let l = h!("literal");             // static, no allocator involved

// BSTR remains Windows-only — opt in with #[cfg(windows)] if you need it
#[cfg(windows)]
let b: windows_strings::BSTR = "world".into();
```

**Findings worth carrying into later stages:**
- The "swap the allocator under a `#[cfg]`-gated private helper" pattern in `hstring_header.rs` is a clean template for any other Win32-flavoured primitive whose only OS dependency is the heap. Stage 4's `windows-threading` shape may be able to use the same trick (e.g. fall back to `std::thread::spawn` for `submit`).
- `windows_link::link!` already no-ops the `#[link(...)]` attribute off-Windows, but generated `bindings.rs` files are still `#[cfg(windows)] mod bindings;` here so callers don't accidentally rely on stub-linking on other targets.
- ⏭️ **Deferred:** Linux-only unit tests for `windows-strings`. The existing `test_strings` crate depends on the `windows` mega-crate and is therefore Windows-only; a lean Linux-friendly test crate is its own bounded follow-up. The smoke test exercised here lives outside the repo. The build / doc matrix already catches the most common regressions.
- ⏭️ **Deferred:** dropping `default-target = "x86_64-pc-windows-msvc"` from `crates/libs/strings/Cargo.toml`'s `[package.metadata.docs.rs]`. This becomes natural once `#[doc(cfg(...))]` annotations are in place so docs.rs can render both surfaces (Windows and non-Windows) from a single Linux build.

Risk realised: low. `bindings.rs` is unchanged. No callers in `windows-core` were affected because `windows-core` re-exports `windows_strings::*` from inside its own `#[cfg(windows)]` `windows.rs`; that gate is preserved. Anyone who today writes `use windows_strings::HSTRING` on Linux now gets a working `HSTRING` instead of an `unresolved import` error.

## Stage 4 — `windows-threading` and `windows-future` shapes

**Status: ✅ done.**

These were the pieces blocking `windows-collections` Linux tests (tests fail because `windows_threading::submit` is not available without `#[cfg(windows)]`). The benchmark below decided which of two viable shapes to take.

### Open design question — keep `windows-threading`, or drop it from `windows-future`?

`windows-future` only uses `windows-threading` for one thing: a fire-and-forget `submit(closure)` per `IAsyncAction::spawn` / `IAsyncOperation::spawn` (see `crates/libs/future/src/async_spawn.rs`). The waker integration in `future::AsyncFuture::poll` does *not* go through `windows-threading` — it's driven by the WinRT `Completed` handler. So the Stage-4 question reduced to: is `TrySubmitThreadpoolCallback` materially faster than `std::thread::spawn` for that one submit?

If `std::thread::spawn` were "good enough" the cleanest outcome would be for `windows-future` to drop the `windows-threading` dependency entirely. If the Win32 pool is materially faster, we keep the dependency and instead make `submit` cross-platform: leave the Win32 path as the Windows fast path, and add a `std::thread::spawn` fallback on non-Windows, gated behind a default-on `std` feature so the crate's `no_std` story is preserved (`unimplemented!()` body when `no_std` + non-Windows).

### Benchmark

A self-contained, no-extra-deps Windows-only benchmark lives at [`crates/libs/threading/examples/threading_bench.rs`](../crates/libs/threading/examples/threading_bench.rs). It compares `windows_threading::submit` vs `std::thread::spawn` on three workloads representative of `windows-future`'s usage:

- `single`  — submit one closure, wait, repeat × 1000 (approximates `IAsync*::spawn(...).await` round trip).
- `burst`   — submit 10 000 closures back-to-back, wait for them all.
- `steady`  — submit 100 000 closures back-to-back, wait for them all (warm-up amortizes).

Each closure does a single atomic increment, so the dispatch path dominates the measurement. The benchmark file itself is `#![cfg(windows)]` and uses no third-party deps. Run with:

```text
cargo run --release --example threading_bench -p windows-threading
```

### Result

Measured on Windows (release):

| Workload | win32-pool | std::spawn | Ratio (spawn / pool) |
|---|---|---|---|
| `single` | 15.67 µs/submit · 63 813/s | 67.20 µs/submit · 14 880/s | ~4.3× |
| `burst`  |  0.46 µs/submit · 2 180 027/s | 41.16 µs/submit · 24 296/s | ~90× |
| `steady` |  0.46 µs/submit · 2 162 686/s | 42.15 µs/submit · 23 724/s | ~91× |

Two observations: per-submit cost on the Win32 pool drops from 15.67 µs (cold round trip) to 0.46 µs (warm queued), confirming worker-thread reuse; `std::thread::spawn` stays flat at ~42 µs because every submit pays a full `CreateThread` round trip. Throughput delta on burst/steady is two orders of magnitude.

### Decision

The Win32 pool is materially faster — well past the "≥ 5–10× in `steady`" threshold from the original decision rule. Keep the `windows-threading` dependency in `windows-future`, and instead make `windows_threading::submit` itself cross-platform.

### Implementation

1. `windows-threading` gained a default-on `std` feature. `submit` now has three cfg-gated bodies:
    - **Windows** (any feature config): unchanged Win32 fast path through `TrySubmitThreadpoolCallback`.
    - **Non-Windows + `std`** (the default): `std::thread::spawn(f)`, fire-and-forget.
    - **Non-Windows + `no_std`**: `unimplemented!()` so the crate still links but signals at runtime that no portable thread primitive is configured.
2. The other surface (`Pool` / `for_each` / `thread_id` / `sleep`) stays Windows-only — it's an explicit Win32 product, not a portable threading API, and `windows-future` does not use it.
3. `windows-future` is now buildable on Linux via this fallback. The runtime async tests in `crates/tests/libs/future/` remain Windows-only because they go through `RoInitialize`/COM, which is out of scope for Stage 4; only the build/doc paths are exercised on Linux.
4. Both `windows-threading` and `windows-future` are now in the Linux build and doc matrix in `.github/workflows/linux.yml`. `windows-threading` is also in the `--no-default-features` matrix so the `no_std`-without-`std` path stays green.

This unblocks Stage 5 (`windows-collections` on Linux).

## Stage 5 — `windows-collections` cross-platform

After Stages 2 and 4, `windows-collections` should build on Linux because all its dependencies do. The only blocker today is `windows-future`'s use of `windows-threading`.

1. Drop any inner `#[cfg(windows)]` that becomes unnecessary.
2. Light up `crates/tests/libs/collections` on Linux. The pure stock-collection paths (`StockVector`, `StockMap`, `StockObservableMap`) do not require any Win32 runtime; the event-firing tests work because dispatch goes through `windows-core` (made portable in Stage 2).
3. Add to the Linux test-run list in `linux.yml`.

Risk: low once Stages 2 and 4 are done.

## Stage 6 — Honest Windows-only crates stay Windows-only

`windows-cppwinrt`, `windows-registry`, `windows-services`, `windows-version`, and the generated mega-crates (`windows-sys`, `windows`) are inherently Windows. They keep `#![cfg(windows)]`. The Stage-0 Linux build step uses `--workspace --exclude` for exactly these crates — that exclude list is the documented surface of "not portable" and should shrink to only this set by the end of Stage 5.

## Cross-cutting items

- **Documentation:** after each stage, update `docs/` (and the per-crate `readme.md`) to state which targets the crate supports. Add a top-level "Cross-platform support matrix" doc.
- **`docs.rs`:** for crates that become portable, drop `default-target = "x86_64-pc-windows-msvc"` so docs.rs builds them on Linux too (faster, and exercises the Linux path).
- **MSRV / no_std:** every crate already opts into `no_std` via `cfg_attr`; the Linux work must preserve that. Add a `--no-default-features` build of the cross-platform crates to the Linux job.
- **Ordering rationale:** Stage 0 unlocks cheap verification; Stages 1–2 are prerequisites; Stages 3–5 each depend only on prior stages, so they can land as independent PRs.
