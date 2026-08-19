# Reactor2 additive migration checklist

## Goal

Rebuild the Reactor replacement as a reviewable, additive migration from master. Keep the existing
Reactor implementation and every existing consumer unchanged until Reactor2 has independently
passed its quality, behavior, usability, and performance gates.

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
- [x] Keep the old `windows-reactor`, generator, tests, samples, and CI unchanged.
- [x] Add new packages under Reactor2-specific paths.
- [x] Do not copy documentation, workflows, samples, or unrelated cleanup from the reference branch.
- [ ] Make one independently testable change per commit.
- [ ] Do not commit automatically.
- [ ] Run old and new gates after every shared-crate change.
- [ ] Justify every change outside Reactor2-specific paths in the table below.
- [ ] Revert experiments that do not show a measured benefit.
- [ ] Keep timing comparisons informational until repeated measurements show stable variance.
- [ ] Use allocation counts, native command counts, API snapshots, and compile failures as hard gates.

## Default additive paths

- `crates/libs/reactor2/**`
- `crates/tools/reactor2/**`
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

Phase 1 stages the default feature first. The Canvas and WebView features remain declared so their
source stays reviewable, but they are not gates until their additive shared APIs are approved.
Native and public API tests require the test-deployment seam recorded below; no test is deleted to
make the initial library check pass.

The reference quality scripts and API snapshots are deferred to Phase 2. Their copied forms target
the old package and self-test paths, so including them before adaptation would provide misleading
results.

The all-feature probe reaches only the expected integration seams. Canvas needs the new session
finish method, fallible DPI/composition-scale setters, and `resize_with_dpi`. WebView needs the
Reactor-neutral `XamlWebViewHost`. No other master incompatibility was reported.

### Phase 1 gates

- [x] `cargo fmt -p windows-reactor2 -p tool_reactor2`
- [x] `cargo check -p windows-reactor --quiet`
- [x] `cargo test -p windows-reactor --quiet`
- [x] `cargo check -p windows-reactor2 --quiet`
- [x] `cargo test -p windows-reactor2 --quiet`
- [x] `cargo clippy -p windows-reactor2 --all-targets`
- [x] `cargo check -p windows-reactor2 --target i686-pc-windows-msvc --quiet`
- [x] Reactor2 generated-output neutrality

## Phase 2 - Reactor2-owned quality gates

- [ ] Add normalized public API snapshots.
- [ ] Add model coverage reporting and measured floors.
- [ ] Add native private fixtures.
- [ ] Add `test_reactor2_selftest`.
- [ ] Add Reactor2-only CI without editing the existing Reactor workflow.
- [ ] Establish clean and incremental compile-time baselines.
- [ ] Establish release `.rlib` and representative binary-size baselines.
- [ ] Establish startup, reconciliation, churn, allocation, and command baselines.

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

Shared changes must remain backward compatible and must not migrate an old consumer.

- [ ] Canvas: determine the smallest additive checked drawing/DPI APIs required by Reactor2.
- [ ] WebView: retain the old `reactor` feature and add an independent `xaml` host feature.
- [x] Reactor setup: add only the test deployment support required by Reactor2 tests.
- [ ] Test support: add neutral shared support only when both implementations can use it.

## Shared-change justification ledger

| Area | Proposed change | Why Reactor2 needs it | Why it belongs in the shared crate | Old gates |
| --- | --- | --- | --- | --- |
| Canvas | Not yet approved | Reactor2 needs observable draw and DPI failures | Pending investigation | Pending |
| WebView | Not yet approved | Reactor2 owns the XAML control and needs a Reactor-neutral host | Pending investigation | Pending |
| Reactor setup | Add `as_test`, returning the copied bootstrap directory from the existing helper | Cargo test executables load the bootstrap DLL from `target/<profile>/deps` | Runtime deployment belongs with the existing app/example deployment APIs; the addition does not change either existing path | `cargo test -p windows-reactor-setup`; old Reactor check/test |

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
