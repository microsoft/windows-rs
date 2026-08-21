# windows-reactor-next benchmarks

This crate measures the Rust-side `windows-reactor-next` planner and component frontend with
`RecordingRuntime`. It does not include WinUI control creation, layout, rendering, or COM calls.

Run the benchmark in release mode:

```powershell
cargo run -p test_reactor_next_bench --release --quiet -- --iters 500 --reps 12
```

The comparable incumbent benchmark is:

```powershell
cargo run -p test_reactor_bench --release --quiet -- --iters 1000 --reps 12
```

The two harnesses do not have identical frontend semantics. Compare the local component row to
`dirty_component`, broad native trees by operation and size, and retained component memory. The
next benchmark also covers virtual collections and reference-heavy publication, which have no
incumbent row.

## August 21, 2026 checkpoint

Toolchain: `rustc 1.99.0-nightly (969b803cb 2026-08-09)`. Times are the best of 12 release-mode
passes on the same machine. Heap values count Rust allocator traffic.

| Workload | Incumbent | Next | Ratio |
| --- | ---: | ---: | ---: |
| Local component message | 593 ns | 700 ns | 1.18x |
| Local message bytes | 457 | 430 | 0.94x |
| Mount/shutdown, 512 leaves | 211 us | 561 us | 2.66x |
| Change all, 512 leaves | 164 us | 410 us | 2.50x |
| Reverse 512 keyed leaves | 105 us | 278 us | 2.65x |
| Rotate 512 keyed leaves | 70 us | 271 us | 3.86x |
| Retained bytes per component scope | 3,628 | 2,552 | 0.70x |

Next remains flat for an isolated component message from 512 through 16,384 scopes. Broad
component reconciliation takes 0.45-0.53 ms at 512 rows and 3.9-4.7 ms at 4,096 rows, depending on
the keyed operation.

Virtual collection results:

| Workload | Size | Next |
| --- | ---: | ---: |
| Unchanged source | 10,000 | 1.28 ms |
| Same-key payload, 32 realized | 10,000 | 1.29 ms |
| Source reset, 32 re-realized | 10,000 | 1.42 ms |
| Realize and recycle | 32 | 63 us |

Mounting 512 `TextBox` controls takes 258 us and allocates 778 KB. Adding one distinct
`ElementRef<TextBox>` per control takes 359 us and 817 KB: +39% time and +5% transient bytes for
the reference-heavy initial validation path.

Isolated `cargo check` target directories measured 5.134 seconds for a clean
`windows-reactor` check and 2.712 seconds for `windows-reactor-next`. Five source-only package
rebuilds had medians of 3.601 and 1.502 seconds. The release counter executables were 2,975,744 and
991,232 bytes. The applications are representative thin counters rather than identical generated
control sets, so binary size is a package-level signal, not a per-feature comparison.

## Bounds

Use these bounds until integrated samples provide a better workload:

- isolated component message time <= 1.5x the incumbent and independent of unrelated scope count;
- clean and source-only compile time <= the incumbent;
- retained component memory <= the incumbent;
- 512-row broad reconciliation < 1 ms and 4,096-row broad reconciliation < 8 ms;
- 10,000-item virtual source update < 2 ms and 32-row realize/recycle < 100 us;
- reference-heavy mount adds < 50% time and < 10% transient bytes over the same unreferenced
  controls;
- thin release binary <= the incumbent, while each generated control tranche continues recording
  its incremental binary growth.

The 2.5-3.9x relative cost of broad reconciliation and its transient allocation volume remain the
main performance watch. Do not add a second mutable tree or rollback system to improve this number.
Profile the integrated virtual editor first, then optimize repeated key/view collection or
copy-on-write mutation only when a measured application turn exceeds these bounds.
