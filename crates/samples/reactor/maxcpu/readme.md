# Max CPU

A Reactor version of Kenny Kerr's
[Max CPU](https://weblogs.asp.net/kennykerr/Max-CPU_3A00_-Another-one-for-your-developer-toolbox_2E002E002E00_/)
for exercising CPU graphs.

```powershell
cargo run -p reactor-maxcpu --release
```

The slider selects how many available logical processors to stress. The app starts at zero load.
Workers run at lower thread priority, park when inactive, and stop and join when the window closes.
No processor affinity is set, so Windows can move the work between processors. This generates
sustained CPU usage, not a calibrated percentage or a hardware stability test.

The window starts always on top and disables minimization. Move the slider to zero to release the
CPU. High settings consume power, generate heat, and affect other applications.
Do not run it during unrelated performance measurements.

Run beside the process monitor's **Performance** page or Task Manager to watch the CPU graph rise
and fall. Their CPU percentages can differ because the sample measures processor time rather than
Task Manager's frequency-adjusted utilization.
