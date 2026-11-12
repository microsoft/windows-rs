# Max CPU

A Reactor version of Kenny Kerr's
[Max CPU](https://weblogs.asp.net/kennykerr/Max-CPU_3A00_-Another-one-for-your-developer-toolbox_2E002E002E00_/)
for exercising CPU graphs and memory usage reporting.

```powershell
cargo run -p reactor-maxcpu --release
```

The CPU slider selects how many available logical processors to stress. The app starts at zero load.
Workers run at lower thread priority, park when inactive, and stop and join when the window closes.
No processor affinity is set, so Windows can move the work between processors. This generates
sustained CPU usage, not a calibrated percentage or a hardware stability test.

The memory slider allocates 0-16 GB in 256 MB steps, starting at zero. A background worker commits
and writes the memory so it contributes to the process working set. The label shows allocated and
requested MB, excluding the app's baseline usage. Lowering the slider releases allocations; closing
the window releases them all. Allocation failures appear below the slider. Windows can page out
or compress memory under pressure, so resident usage can differ from the allocated amount.

The window starts always on top, resizes horizontally only, and disables minimization. Move both
sliders to zero to release the load. High settings consume power, generate heat, use memory, and
affect other applications.
Do not run it during unrelated performance measurements.

Run beside the process monitor's **Performance** page or Task Manager to watch CPU and memory usage
rise and fall. Their CPU percentages can differ because the sample measures processor time rather
than Task Manager's frequency-adjusted utilization.
