# Reactor2 additive migration checklist

## Goal

Rebuild the Reactor replacement as a reviewable, additive migration from master. Keep the existing
Reactor implementation operational until Reactor2 has independently passed its quality, behavior,
usability, and performance gates. Mechanical migrations to approved shared APIs are allowed when
they remove temporary compatibility APIs without changing old behavior.

## Repositories

- Baseline and migration: `D:\git\master-rs`
- Reactor2 reference only: `D:\git\dead_code_rs`
- Checklist: `D:\git\master-rs\checklist.md`
- Migration branch: `reactor2-migration`
- Baseline commit: `acaaa37be5`
- Reference commit: `243fcd5f7`

## Migration rules

- [x] Start from a clean clone of `origin/master`.
- [x] Create a dedicated migration branch.
- [x] Keep the old `windows-reactor`, generator, tests, samples, and CI operational.
- [x] Add new packages under Reactor2-specific paths.
- [x] Do not copy documentation, workflows, samples, or unrelated cleanup from the reference branch.
- [ ] Make one independently testable change per commit.
- [ ] Do not commit automatically.
- [x] Run old and new gates after every shared-crate change.
- [x] Justify every change outside Reactor2-specific paths in the table below.
- [ ] Revert experiments that do not show a measured benefit.
- [ ] Keep timing comparisons informational until repeated measurements show stable variance.
- [ ] Use allocation counts, native command counts, API snapshots, and compile failures as hard gates.

## Default additive paths

- `crates/libs/reactor2/**`
- `crates/tools/reactor2*/**`
- `crates/samples/reactor2/**`
- `crates/tests/libs/reactor2_*/**`
- `.github/workflows/reactor2.yml`
- `Cargo.lock`

Changes outside these paths require a separate justification and validation entry.

## Phase 1 - Isolated Reactor2 library

- [x] Add `crates/libs/reactor2` as unpublished package `windows-reactor2`.
- [x] Copy only the rewritten library and its source tests.
- [x] Preserve `extern crate self as windows_reactor` for internal path stability.
- [x] Alias `windows_reactor2` as `windows_reactor` in integration tests.
- [x] Add `crates/tools/reactor2` as package `tool_reactor2`.
- [x] Give `tool_reactor2` its own RDL and filter inputs.
- [x] Make `tool_reactor2` write only `crates/libs/reactor2/src/bindings.rs`.
- [x] Confirm running `tool_reactor2` leaves all existing generated files unchanged.
- [x] Confirm default-feature Reactor2 builds against untouched master dependencies.
- [x] Confirm existing Reactor still builds and tests.

Phase 1 staged the default feature first. Canvas and WebView were then enabled through the additive
shared APIs recorded below. Native and public API tests use the test-deployment seam; no test was
deleted to make the library checks pass.

The reference quality scripts and API snapshots are deferred to Phase 2. Their copied forms target
the old package and self-test paths, so including them before adaptation would provide misleading
results.

The all-feature probe initially reached only the expected Canvas and WebView seams. Both are now
resolved without changing an existing method signature or removing the old WebView integration.

### Phase 1 gates

- [x] `cargo fmt -p windows-reactor2 -p tool_reactor2`
- [x] `cargo check -p windows-reactor --quiet`
- [x] `cargo test -p windows-reactor --quiet`
- [x] `cargo check -p windows-reactor2 --quiet`
- [x] `cargo test -p windows-reactor2 --quiet`
- [x] `cargo clippy -p windows-reactor2 --all-targets`
- [x] `cargo test -p windows-reactor2 --all-features --quiet`
- [x] `cargo clippy -p windows-reactor2 --all-features --all-targets`
- [x] `cargo check -p windows-reactor2 --target i686-pc-windows-msvc --quiet`
- [x] `cargo check -p windows-reactor2 --all-features --quiet`
- [x] `cargo check -p windows-reactor2 --all-features --target i686-pc-windows-msvc --quiet`
- [x] Reactor2 generated-output neutrality

## Phase 2 - Reactor2-owned quality gates

- [x] Add normalized public API snapshots with `tool_reactor2_public_api`.
- [x] Add model coverage reporting and measured floors.
- [x] Add native private fixtures.
- [x] Add a smoke-only `test_reactor2_selftest` with a Rust process/UI Automation harness.
- [x] Add Reactor2-only CI without editing the existing Reactor workflow.
- [ ] Establish clean and incremental compile-time baselines.
- [ ] Establish release `.rlib` and representative binary-size baselines.
- [ ] Establish startup, reconciliation, churn, allocation, and command baselines.

### Phase 2 gates

- [x] `cargo run -p tool_reactor2_public_api --quiet`
- [x] `cargo run -p tool_reactor2_coverage --quiet -- target\reactor2-coverage.json`
- [x] `cargo test -p windows-reactor-setup -p test_reactor2_support --quiet`
- [x] `cargo test -p test_reactor2_selftest --test native --quiet -- --test-threads=1`
- [x] `cargo test -p windows-reactor2 --all-features --quiet`
- [x] `cargo test -p windows-reactor --all-features --quiet`
- [x] Clippy with `-D warnings` for both Reactors and all affected test and tool packages
- [x] No PowerShell files or unvalidated fixture scenarios under Reactor2-owned paths
- [x] `.github\workflows\reactor2.yml` parses and uses the commands validated above

## Phase 3 - Incremental sample migration

- [ ] Add a Reactor2 sample package that aliases `windows-reactor2` as `windows-reactor`.
- [ ] Port basic rendering and counter samples.
- [ ] Port hooks and component samples.
- [ ] Port layout and basic controls.
- [ ] Port controlled text and value inputs.
- [ ] Port selection controls.
- [ ] Port collections and virtualization.
- [ ] Port flyouts, dialogs, windows, and overlays.
- [ ] Port Composition integration.
- [ ] Port Canvas integration.
- [ ] Port WebView integration.
- [ ] Port galleries and larger applications.

For each sample or related group:

- [ ] Start from the master sample.
- [ ] Apply only changes required by the Reactor2 API.
- [ ] Compare with the reference branch without copying unrelated edits.
- [ ] Keep behavior and visible output equivalent unless the difference is justified.
- [ ] Run old and new versions independently.
- [ ] Record source LOC, compile time, binary size, startup, allocation, and native commands.

## Phase 4 - Shared integration seams

Shared changes must keep old behavior working. Old consumers may be migrated mechanically when
that avoids carrying a temporary API or duplicate implementation.

- [x] Canvas: add the smallest checked drawing/DPI APIs required by Reactor2.
- [x] WebView: add one unconditional XAML host and retain only the temporary old `reactor` adapter
  feature.
- [x] Reactor setup: add only the test deployment support required by Reactor2 tests.
- [ ] Test support: add neutral shared support only when both implementations can use it.

## Shared-change justification ledger

| Area | Proposed change | Why Reactor2 needs it | Why it belongs in the shared crate | Old gates |
| --- | --- | --- | --- | --- |
| Canvas | Make the existing DPI/composition setters fallible; add `DrawingSession::finish` and `SwapChain::resize_with_dpi`; migrate old Reactor in the same step | Both Reactors need observable draw, resize, DPI, and composition-scale failures | Canvas owns the Direct2D/DXGI operations and error values; migrating the in-repo consumer avoids permanent duplicate setter APIs | Canvas check/test/Clippy; old Reactor check/test; Reactor2 Canvas check/test/Clippy |
| WebView | Add an unconditional hidden `XamlWebViewHost`; make the temporary old `reactor` adapter delegate to it | Reactor2 owns the XAML control and needs a Reactor-neutral host | The host is only 98 lines with 603 lines of bindings, and WebView owns the XAML-to-COM bridge; one implementation serves both Reactors until the marked adapter is deleted | WebView all-feature check/test/Clippy; old Reactor all-feature check/test; Reactor2 all-feature check/test/Clippy |
| Reactor setup | Add `as_test` and resolve the profile directory by finding the `build` ancestor | Cargo tests need the bootstrap DLL under `deps`; self-contained binaries need runtime DLLs beside the executable | Runtime deployment belongs with the existing app/example APIs; the old fixed-depth lookup put self-contained files under `target/<profile>/build/deps` | Setup unit/doctests; old Reactor check/test; Reactor2 native selftest |

No shared change is approved merely because it exists on the reference branch.

## Comparison matrix

| Metric | Old Reactor | Reactor2 | Gate |
| --- | ---: | ---: | --- |
| Clean check | Pending | Pending | Report |
| Touched incremental check | Pending | Pending | Report |
| Release build | Pending | Pending | Report |
| Release `.rlib` | Pending | Pending | Report |
| Representative executable | Pending | Pending | Report |
| Startup | Pending | Pending | Report |
| Ordinary reconciliation | Pending | Pending | Report |
| Churn reconciliation | Pending | Pending | Report |
| Allocation per render | Pending | Pending | Hard floor |
| Native commands | Pending | Pending | Hard floor |
| Sample source LOC | Pending | Pending | Review |
| Public API | Pending | Pending | Snapshot |

## Final cutover - blocked until all prior phases pass

- [ ] Stop adding features during cutover.
- [ ] Delete the old Reactor implementation.
- [ ] Move `reactor2` to `reactor`.
- [ ] Rename `windows-reactor2` to `windows-reactor`.
- [ ] Retarget the Reactor2 generator output.
- [ ] Remove the old Reactor generator.
- [ ] Move Reactor2 samples and tests to their final paths.
- [ ] Remove temporary A/B-only packages.
- [ ] Run the full workspace, generation, API, coverage, native, package, and performance gates.
- [ ] Review the final diff against master for unexplained churn.

## Decision log

- 2026-08-19: Accepted the compile-time cost of the typed implementation as a current tradeoff.
- 2026-08-19: Chose an additive shadow migration instead of rebasing the replacement branch.
- 2026-08-19: Existing consumers remain untouched until Reactor2 is independently validated.
- 2026-08-19: Shared-crate changes require separate evidence; reference-branch presence is not
  sufficient justification.
- 2026-08-19: Stage default Reactor2 first. Canvas and WebView remain deferred until their shared
  seams have independent old-stack gates.
- 2026-08-19: Approved the additive Reactor setup `as_test` API because Cargo test executables use
  a distinct output directory; existing app and example deployment behavior remains unchanged.
- 2026-08-19: Replaced the temporary checked Canvas method names with fallible canonical setters
  and migrated old Reactor, avoiding permanent duplicate APIs.
- 2026-08-19: Made the Reactor-neutral WebView XAML host unconditional and made the marked
  temporary old `reactor` adapter delegate to it, avoiding both a tiny permanent feature and
  duplicate XAML initialization implementations.
- 2026-08-19: Transitional code intended for final-cutover deletion must carry the exact
  `// TODO: remove when done` comment.
- 2026-08-19: Reactor2 API snapshots normalize only `windows_reactor2` to the final
  `windows_reactor` crate name; all four snapshots match the reference branch exactly. The
  `tool_reactor2_public_api` package keeps this gate separate from binding generation.
- 2026-08-19: Fresh Reactor2 coverage passed the reviewed floors: app 76.77% branches/90.54%
  lines, engine 74.87%/92.60%, and WinUI 36.74%/55.21%. All 44 ignored native fixtures passed
  during collection.
- 2026-08-19: Do not propagate the copied PowerShell native harness. Reactor2 native acceptance
  tests use Rust integration tests, one fixture process per test, shared RAII process handling, and
  direct UI Automation. The initial self-test fixture contains only the validated smoke surface;
  fixture code is added with each later Rust scenario rather than copied in advance.
- 2026-08-19: Reactor2 CI is independent of the existing Reactor workflow. Stable quality and API
  checks, self-contained native smoke testing, and nightly coverage run as separate jobs so each
  failure identifies one gate.
