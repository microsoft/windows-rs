# windows-reactor-next benchmarks

This crate measures the Rust-side `windows-reactor-next` planner and component frontend with
`RecordingRuntime`. It does not include WinUI control creation, layout, rendering, or COM calls.

Run the benchmark in release mode:

```powershell
cargo run -p test_reactor_next_bench --release --quiet -- --iters 500 --reps 12
```

The comparable incumbent benchmark is:

```powershell
cargo run -p test_reactor_bench --release --quiet -- --iters 500 --reps 12
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
| No-change update, 512 leaves | 7.5 us | 197 us | 26.3x |
| Change all, 512 leaves | 164 us | 410 us | 2.50x |
| Reverse 512 keyed leaves | 105 us | 278 us | 2.65x |
| Rotate 512 keyed leaves | 70 us | 271 us | 3.86x |
| Retained bytes per component scope | 3,628 | 2,552 | 0.70x |

Next remains flat for an isolated component message from 512 through 16,384 scopes. Broad
component reconciliation takes 0.45-0.53 ms at 512 rows and 3.9-4.7 ms at 4,096 rows, depending on
the keyed operation. The no-change row is the largest relative gap: the incumbent skips the shared
root in O(1), while next clones and traverses a candidate tree. Its 197 us absolute cost remains
below the provisional 1 ms bound, but candidate-tree cloning is still the main performance watch.

Virtual collection results:

| Workload | Size | Next |
| --- | ---: | ---: |
| Unchanged source | 10,000 | 1.28 ms |
| Same-key payload, 32 realized | 10,000 | 1.29 ms |
| Source reset, 32 re-realized | 10,000 | 1.42 ms |
| Realize and recycle | 32 | 63 us |

Mounting 512 `TextBox` controls took 258 us and allocated 778 KB in the checkpoint run. Adding one
distinct `ElementRef<TextBox>` per control took 359 us and 817 KB: +39% time and +5% transient
bytes for the reference-heavy initial validation path. A review run measured +23% time with the
same +5% bytes. Treat the time ratio as a noisy checkpoint measurement; the deterministic
allocation ratio is stable.

`mount_shutdown` includes cloning the input `View` and constructing `RecordingRuntime` inside each
timed iteration. It measures the repeatable setup needed to mount and retire a fresh tree, not a
pure planner mount. All timing commands must use `--release`; the harness does not reject a debug
build.

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

The 2.5-3.9x relative cost of changed or reordered broad reconciliation, the roughly 26x no-change
ratio, and their transient allocation volume remain the main performance watch. Do not add a second
mutable tree or rollback system to improve these numbers. Optimize repeated key/view collection or
copy-on-write mutation only when a measured application turn exceeds the absolute bounds.

## Virtual editor application gate

The virtual editor sample now owns a release-mode `RecordingRuntime` driver so the benchmark uses
the same controlled inputs, parent-owned durable task model, contexts, effects, focus references,
background completion path, and virtual rows as the application. Run it with:

```powershell
cargo run -p sample_reactor_next_virtual --bin reactor-next-virtual-perf `
    --features perf --release -- --samples 500
```

The August 21, 2026 run used the checkpoint toolchain and 500 samples after 16 warmups:

| Workload | Median | p95 | p99 | Bytes/op | Allocations/op |
| --- | ---: | ---: | ---: | ---: | ---: |
| Local controlled edit | 213 us | 258 us | 325 us | 531,780 | 1,355 |
| Broad selection change | 194 us | 243 us | 286 us | 520,919 | 1,212 |
| Redundant parent message | 185 us | 215 us | 248 us | 519,778 | 1,203 |
| Unchanged root component memo hit | 0.2 us | 0.3 us | 0.3 us | 80 | 2 |
| Value-equal root recomposition | 110 us | 192 us | 239 us | 519,661 | 1,202 |
| 32-row recycle/realize | 697 us | 885 us | 992 us | 1,055,413 | 6,774 |
| Background completion | 86 us | 101 us | 123 us | 519,781 | 1,203 |
| Mixed virtual cycle | 866 us | 1,018 us | 1,214 us | 2,110,829 | 9,207 |

The mixed cycle performs a background completion, a selection-changing parent update, and a complete
32-row recycle/realize batch. `process_realizations` has a 32-request work budget, so the driver
drains both the recycle and realization turns before stopping the timer.

These are Rust planning, publication, effect, and `RecordingRuntime` command-application times.
They exclude WinUI control work, layout, rendering, and presentation. The integrated Rust path is
below the 4 ms profiling trigger, so no architectural optimization is justified by latency yet.
A valid controlled keystroke writes through to the parent task model and rebuilds the 1,000-item
source. Sharing immutable task payloads with `Rc<Task>` removed per-row title clones. Passing
selection directly in `RowProps` removed one context provider declaration per task; each provider
had only one consumer and added no useful fan-out. Together these changes cut local-edit bytes by
36%, allocations by 75%, and median time by 48%. The 0.2 us row is only the O(1) unchanged
component-props memo path. The forced recomposition changes a private render revision while
preserving the same application model and rebuilds the value-equal TaskEditor declaration.

Value-equal eager-source scaling after the same changes is:

| Tasks | Median | p95 | Bytes/op | Allocations/op |
| ---: | ---: | ---: | ---: | ---: |
| 1,000 | 88 us | 102 us | 519,661 | 1,202 |
| 10,000 | 1.72 ms | 1.78 ms | 4,355,696 | 10,202 |
| 100,000 | 21.8 ms | 24.6 ms | 38,787,955 | 100,202 |

The eager declaration remains linear. The 1,000-item application is comfortably below the gate,
and 10,000 items remain below the 4 ms profiling trigger at p95. A 100,000-item source needs a lazy
indexed source if an application requires that scale; this synthetic result alone does not justify
adding another public source model.

The live driver measured the same editor for 600 frames after 30 warmups. The active workload
requests a new virtual index every frame, alternates selection, edits a controlled row
every six frames, and delivers a background completion every 30 frames. The baseline opens the
same 1,000-item editor without those actions:

| Metric | Baseline | Active |
| --- | ---: | ---: |
| Frame median | 16.68 ms | 16.65 ms |
| Frame p95 | 17.16 ms | 18.10 ms |
| Frame p99 | 17.58 ms | 20.71 ms |
| Frame max | 18.35 ms | 58.85 ms |
| Frames over 25 ms | 0 / 600 | 2 / 600 |
| Frames over 33.4 ms | 0 / 600 | 2 / 600 |

The active host dispatched 602 turns at 1.20 ms median, 1.48 ms p95, and 2.06 ms p99. Its 1,202
native apply batches were 184 us median, 1.05 ms p95, and 1.39 ms p99. Baseline p95 already
exceeded 16.7 ms because presentation intervals straddle the display period; the active p95 delta
was 0.94 ms. The two long frames coincide with the roughly 17 ms host and native-apply maxima during
forced realization. An earlier run under system contention reached 33.33 ms p95 while host and
native-apply p95 rose to 5.08 and 3.31 ms; two immediate repeats produced the stable distribution
above. Use missed frames and correlated phase times rather than treating one frame percentile as a
frontend result.

The live runs also found recycle-order defects. WinUI clears and retires an element-factory
shell before its queued Pump recycle runs. `DetachRealized` then tried to clear the retired token
again and treated the expected absence as a fatal native error. The WinUI runtime now clears a
still-live shell but accepts an already-retired shell; attaching content remains strict. A later
120-frame soak found 79 leaked retired tokens when a recycle followed a realization that never
published. Rejected recycle work now acknowledges only retired tokens, and the repeated-reset soak
settles at zero.

Profile before changing architecture if Rust planning approaches 4 ms or sustained p95 frame time
exceeds 16.7 ms on the checkpoint machine. Start with repeated key/view collection, avoidable
subtree reconciliation, copy-on-write mutation granularity, unchanged child/property cloning, and
component boundaries. This run crossed the raw frame threshold, so the host/native phase
instrumentation and baseline comparison were completed before deciding not to change the
architecture. Preserve one-tree ownership and transactional publication.
