# Reactor stress benchmark

`test_reactor_perf` compares windows-reactor with Microsoft.UI.Reactor using the same headless
70x70 stock grid, seed-42 update stream, update percentages, and ten-second duration.

The matching C# application is `StressPerf.ReactorOptimized` in
<https://github.com/microsoft/microsoft-ui-reactor>. The optimized variant memoizes unchanged
cells, matching the Rust harness's dirty-cell update path.

The text report, JSON line, and CSV files include average and peak working set plus average and peak
private bytes. Working set measures resident pages; private bytes is the better retained-memory
signal when evaluating logical-node storage or repeated mount/unmount behavior.

## Running

```powershell
cargo run -p test_reactor_perf --release -- --headless --percent 10 --duration 10 --json
```

Run the C# counterpart from the Microsoft.UI.Reactor repository:

```powershell
dotnet build tests\stress_perf\StressPerf.ReactorOptimized `
    -c Release -p:Platform=x64

tests\stress_perf\StressPerf.ReactorOptimized\bin\x64\Release\`
net10.0-windows10.0.22621.0\StressPerf.ReactorOptimized.exe `
    --headless --percent 10 --duration 10 --json
```

## Results

Measured on August 7, 2026 on a 12th Gen Intel Core i9-12900K with Rust
`1.99.0-nightly (87e5904f5)` and .NET SDK 10.0.302. Rust used the Reactor stabilization worktree
based on `4529e3ad`; the C# checkout was `c9191b97` (Microsoft.UI.Reactor 0.1.0-preview.13). Rust
used Windows App SDK 2.3.1; the C# repository used its validated Windows App SDK Runtime 2.1.3 and
WinUI 2.1.0 pins.

| Update | Reconcile ms Rust / C# | Diff ms Rust / C# | Renders/s Rust / C# | FPS Rust / C# | Alloc/render Rust / C# | GC gen0/1/2 Rust / C# |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 0% | 2.53 / 12.08 | 1.04 / 11.34 | 23.30 / 13.82 | 57.50 / 54.12 | 3.75M / 0.84M | 0/0/0 / 13/13/11 |
| 10% | 3.54 / 19.40 | 2.48 / 17.30 | 21.81 / 9.98 | 42.23 / 20.76 | 4.13M / 2.14M | 0/0/0 / 24/22/15 |
| 50% | 7.87 / 37.72 | 6.95 / 30.75 | 8.55 / 4.11 | 12.35 / 9.67 | 5.32M / 5.23M | 0/0/0 / 19/19/8 |
| 100% | 10.82 / 49.42 | 9.97 / 39.90 | 6.11 / 3.02 | 8.34 / 6.97 | 6.28M / 7.91M | 0/0/0 / 21/20/7 |

C# reconcile takes 4.6-5.5x as long across the sweep. Rust renders 1.7-2.2x as often and uses
32-35% less working set:

| Update | Working set MB Rust / C# | Rust reduction |
| ---: | ---: | ---: |
| 0% | 178.4 / 261.6 | 31.8% |
| 10% | 182.7 / 273.8 | 33.3% |
| 50% | 188.9 / 276.2 | 31.6% |
| 100% | 187.2 / 287.1 | 34.8% |

Rust produces no garbage collections. Its counted heap allocation per render is 4.5x C# at 0%
updates and 1.9x at 10%, approximately equal at 50%, and 21% lower at 100%. These columns are
runtime-specific and exclude native WinUI and COM allocations.
