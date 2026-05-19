## Miri validation

### Goals

Miri validation in `windows-rs` is focused on:

1. memory safety
2. reference-count/lifetime correctness
3. unsafe ABI boundary behavior in `windows-*` crates

This coverage is intended to go beyond macro smoke tests.

### Runtime configuration

All Miri CI runs use:

- `-Zmiri-strict-provenance`
- `-Zmiri-symbolic-alignment-check`
- `-Zmiri-deterministic-concurrency`

### Accepted suppressions

`cfg(miri)` guards and `#[cfg_attr(miri, ignore = "...")]` are allowed only for tests that depend on environment interactions Miri does not model reliably (for example selected WinRT/COM/Win32 thread-error-info behavior). Suppressions must include a reason.

### CI tiers

| Tier | Trigger | Purpose |
| --- | --- | --- |
| PR tier | `pull_request`, `push` to `master` | Fast deterministic gate for regressions in high-value safety paths |
| Scheduled tier | weekly schedule, `workflow_dispatch` | Broader and slower sweep for additional ownership/refcount/error-path coverage |

### Miri-covered tests

| Crate | Test target | Tier |
| --- | --- | --- |
| `test_interface_core` | `--test macro_rules_decl` | PR + scheduled |
| `test_core` | `--test unknown` | PR |
| `test_result` | `--test ntstatus` | PR + scheduled |
| `test_strings` | `--test literals` | PR + scheduled |
| `test_implement_core` | `--lib com_object::basic` | PR |
| `test_interface_core` | `--test ref` | scheduled |
| `test_implement_core` | `--lib com_chain::interface_chain_query` | scheduled |
| `test_implement_core` | `--lib com_object::take` | scheduled |
| `test_result` | `--test slim_errors` | scheduled |
| `test_strings` | `--test hstring_builder` | scheduled |

### Triage policy

- Any PR-tier Miri regression is a merge blocker.
- Scheduled-tier failures are triaged immediately:
  - **blocker**: memory safety, refcount/lifetime, or ABI-surface defects
  - **follow-up**: flakiness, infrastructure, or unsupported-environment behavior

### Updating coverage

When adding new unsafe-heavy functionality or new tests that exercise unsafe ownership/lifetime/ABI behavior in these crates, update both:

1. `.github/workflows/miri.yml`
2. this coverage table
