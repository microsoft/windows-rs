# Reactor stress benchmark

`test_reactor_perf` compares windows-reactor with Microsoft.UI.Reactor using the same headless
70x70 stock grid, seed-42 update stream, update percentages, and ten-second duration.

The matching C# application is `StressPerf.ReactorOptimized` in
<https://github.com/microsoft/microsoft-ui-reactor>. The optimized variant memoizes unchanged
cells, matching the Rust harness's dirty-cell update path.

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

Measured on August 5, 2026 on a 12th Gen Intel Core i9-12900K with .NET SDK 10.0.302. The C#
checkout was `c9191b97` (Microsoft.UI.Reactor 0.1.0-preview.13). Rust used Windows App SDK 2.3.1;
the C# repository used its current Windows App SDK Runtime 2.1.3 and WinUI 2.1.0 pins. An override
to the 2.3 components did not restore cleanly, so the table keeps the C# project's validated pins.

| Update | Reconcile ms Rust / C# | Diff ms Rust / C# | Renders/s Rust / C# | FPS Rust / C# | Alloc/render Rust / C# | GC gen0/1/2 Rust / C# |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 0% | 2.29 / 11.84 | 1.00 / 11.05 | 24.40 / 14.17 | 57.63 / 55.49 | 3.75M / 0.84M | 0/0/0 / 14/13/12 |
| 10% | 3.53 / 19.69 | 2.50 / 17.61 | 23.29 / 13.22 | 51.74 / 28.34 | 4.13M / 2.16M | 0/0/0 / 34/30/20 |
| 50% | 7.87 / 39.08 | 7.01 / 32.91 | 8.59 / 4.12 | 13.42 / 9.60 | 5.32M / 5.23M | 0/0/0 / 18/18/7 |
| 100% | 10.90 / 52.08 | 10.04 / 42.52 | 6.01 / 3.23 | 8.17 / 7.56 | 6.28M / 7.94M | 0/0/0 / 22/21/7 |

C# reconcile takes 4.8-5.6x as long across the sweep, while Rust produces no garbage collections.
The allocation columns are runtime-specific and exclude native WinUI and COM allocations.
