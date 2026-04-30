# Cross-platform staging plan

This document tracks the work needed to make the windows-rs libraries cross-platform
in stages, and to verify the result on a GitHub Linux runner.

## Current state

**Already cross-platform (no `#![cfg(windows)]`, no Win32 calls at runtime):**
- `windows-bindgen`, `windows-metadata`, `windows-rdl`, `riddle` — pure code-gen / metadata libs.
- `windows-numerics` — pure math.
- `windows-interface`, `windows-implement` — proc-macros (build on Linux already).
- `windows-link`, `windows-targets` — mostly just macros/linkage; the inner `cfg(all(windows, target_arch = "x86"))` gates the only Windows-specific bit.

**Hybrid (no top-level `cfg(windows)`, but inner `#[cfg(windows)]` gates the Win32-using parts):**
- `windows-core` — most trait machinery (`Type`, `Interface`, `IUnknown` vtables, `Param`, `Array`, `GUID`, `IInspectable`) is portable; only ref-count/marshal helpers in `imp/` and a few functions in `inspectable.rs` are gated.
- `windows-result` — `HRESULT`, `Error`, `Result` types are mostly portable; only the `From<windows_link>` and a few `RoOriginate*` paths in `hresult.rs`/`error.rs` are gated.

**Hard-gated `#![cfg(windows)]` at the top of `lib.rs` (compile to nothing off Windows):**
- `windows-cppwinrt` — invokes `cppwinrt.exe`, inherently Windows.
- `windows-strings` — only HSTRING/BSTR truly need Win32; PCSTR/PCWSTR/PSTR/PWSTR/literals are portable.
- `windows-threading` — `WaitEvent` + thread-pool wrappers are Win32; trait shape (`SubmitThreadpoolCallback` analog) could be exposed.
- `windows-version` — `GetVersionEx`/`RtlGetVersion` only.
- `windows-registry` — Win32 registry APIs only.
- `windows-services` — Win32 SCM only.
- `windows-future` — `IAsyncAction`/etc. trait *shapes* are portable, but the executor (`async_spawn`, `Waker`) calls into `windows-threading`.
- `windows-collections` — same: trait *shapes* (`IVector`, `IMap`, `IIterable`) and the stock `BTreeMap`/`Vec` adapters are portable; only event-handler dispatch reaches into `windows-core` machinery that currently requires `windows`.

**CI today (`linux.yml`):** runs `cargo test` on `test_linux`, `test_clang`, `test_roundtrip_clang`, `test_rdl`, `test_roundtrip_rdl` against `x86_64-unknown-linux-gnu`, then enforces no diff. There is no equivalent `cargo build` matrix that just confirms "library X compiles on Linux", and no Linux test for the libs that are nominally cross-platform (`windows-numerics`, `windows-metadata`, `riddle`, `windows-link`, `windows-interface`, `windows-implement`, `windows-targets`).

## Stage 0 — Better Linux test harness (do this first, no library changes)

Goal: turn the Linux runner into a tight feedback loop so each subsequent stage can be validated cheaply. Changes are confined to `.github/workflows/linux.yml` and a small new test crate.

1. Add an explicit "Linux build matrix" step that runs `cargo build --target x86_64-unknown-linux-gnu -p <crate>` for every library currently believed to be cross-platform: `windows-bindgen`, `windows-metadata`, `windows-rdl`, `riddle`, `windows-numerics`, `windows-link`, `windows-interface`, `windows-implement`, `windows-targets`. This locks in the current cross-platform surface and turns regressions into red CI immediately.
2. Add a `crates/tests/libs/cross_platform/` test crate (or fold into existing `test_linux`) that depends on each of those libs and exercises a small public-API smoke test per crate (e.g., parse a tiny winmd with `windows-metadata`, format a `Vector3` with `windows-numerics`, run `windows-bindgen` end-to-end against a checked-in tiny winmd, etc.). Wire it into `linux.yml`.
3. Add a single `cargo build --target x86_64-unknown-linux-gnu --workspace --exclude <not-yet-portable>` step. The exclude list is the explicit "not yet ported" registry, and shrinks each stage. This is the source of truth for "what is cross-platform today".
4. Add a `cargo doc` step on Linux for the cross-platform set so doc-tests and intra-doc links are also exercised off-Windows.
5. Keep using the existing `ubuntu-22.04` runner; only add `KyleMayes/install-llvm-action` for crates that actually need libclang (already done).

Outcome: from this point forward, "is crate X cross-platform?" is answered by a green check, not by reading source.

## Stage 1 — Promote crates that are *already* portable but not advertised

No code changes to library logic, just lift `cfg(windows)` gates that exist only out of caution and add the crates to the Stage 0 build matrix.

1. **`windows-numerics`** — already cfg-clean, just add to Linux CI matrix and add a Linux smoke test.
2. **`windows-link`** — verify the macro expands on Linux (the inner `#[cfg(...)]` already handles the x86 case); add Linux build + a tiny `link!` use site test.
3. **`windows-interface`, `windows-implement`** — proc-macros; add a Linux trybuild/expand test that does *not* depend on `windows`/`windows-core` runtime symbols, only macro expansion.
4. **`windows-targets`** — same treatment.
5. **`riddle`** — already cross-platform; add to matrix and run its existing tests on Linux if not already.

Risk: very low. This is mostly CI plumbing that documents and enforces the current state.

## Stage 2 — Make `windows-core` and `windows-result` build on Linux

These two are the keystone: once they build on Linux, every higher-level crate that wants to be portable can opt in.

1. **`windows-result`**: relax the gates in `hresult.rs`/`error.rs` so that the *types and Display/Debug/From impls* compile unconditionally; keep only the function bodies that call `RoOriginate*`/`GetErrorInfo` under `#[cfg(windows)]`, providing inert fallbacks (e.g., `Error::from_thread()` returns a plain `E_FAIL` on non-Windows). The `#![cfg_attr(not(windows), expect(unused_imports))]` attribute already shows the intent — finish the job so `cargo build -p windows-result --target x86_64-unknown-linux-gnu` succeeds.
2. **`windows-core`**: similar treatment for `imp/mod.rs`, `imp/weak_ref_count.rs`, `imp/delegate_box.rs`, `inspectable.rs`. The pattern is to keep struct/trait definitions unconditional and gate only the bodies that call `CoTaskMemAlloc`/`RoOriginateError`/`WindowsCreateString` etc. Where a function must exist on both platforms, provide a `#[cfg(not(windows))]` stub that panics or returns a sensible default — these stubs are never wired up because the COM machinery is unreachable without Win32, but they let the crate type-check.
3. Add `windows-core` and `windows-result` to the Stage-0 Linux matrix and add Linux-only unit tests for the portable pieces (GUID parsing, HRESULT formatting, `Error::new`/`Display`, Param trait derivations).

Risk: medium. The trick is making sure the gating does not leak into the public API (no functions disappear on Linux, only their bodies become stubs). `#[doc(cfg(...))]` on Windows-only items keeps the docs honest.

## Stage 3 — Split `windows-strings` and expose portable string types

1. Move `pcstr`, `pcwstr`, `pstr`, `pwstr`, `literals`, and `decode` (utf16↔utf8) out from under `#![cfg(windows)]`. These are pure pointer/encoding utilities and have no Win32 dependency apart from the `strlen` extern, which can be replaced with `core::ffi::CStr::from_ptr(...).to_bytes().len()` or kept (every libc has `strlen`).
2. Keep `HSTRING`, `BSTR`, `hstring_builder`, `hstring_header`, `ref_count` under `#[cfg(windows)]` (they call `WindowsCreateString*`, `SysAllocString*`).
3. Net effect: `use windows_strings::PCWSTR` works on Linux; `use windows_strings::HSTRING` still requires Windows (or compiles to a stub type behind the same gating pattern as Stage 2 if needed for downstream crates).

Risk: low–medium. Need to audit re-exports (e.g., `windows-core` re-exporting HSTRING) and make sure Linux builds don't break.

## Stage 4 — `windows-threading` and `windows-future` shapes

These are the pieces blocking `windows-collections` Linux tests today (tests fail because `windows_threading::submit` is not available without `#[cfg(windows)]`).

1. **`windows-threading`**: keep `WaitEvent`/`Pool` Windows-only, but expose the `submit(callback)` entry point as a trait/function that compiles on Linux with a stub body (`unimplemented!()` or a `std::thread::spawn` fallback gated behind a `std` feature). The goal is for *callers* to compile on Linux even if they cannot actually run.
2. **`windows-future`**: with that stub in place, `IAsyncAction`/`IAsyncOperation`/`Waker` plumbing builds on Linux. Mark the runtime tests `#[cfg(windows)]` and add Linux-only build-only checks.
3. Add both crates to the Linux build matrix (build-only at first).

Risk: medium. `windows-future`'s waker integration with the OS thread pool is the trickiest part — keep its body Windows-only and make non-Windows `submit` panic.

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
