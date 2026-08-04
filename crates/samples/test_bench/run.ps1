#!/usr/bin/env pwsh
# Builds a thin WinRT slice (Release) and runs four consumers against the same Rust WinRT
# component, printing a side-by-side table. The component (test_bench_component ->
# bench_component.dll) is a real WinRT component: RDL -> winmd -> windows-bindgen ->
# #[implement], activated registration-free via DllGetActivationFactory. The consumers:
#   - windows-rs    : the generated windows-rs projection
#   - windows-csharp: the generated raw IDisposable C# projection (delegate* unmanaged vtable
#                     calls, one small managed owner, callback-confined borrowed hot paths) --
#                     dogfooded from the same winmd via the windows-csharp generator at build time
#   - cppwinrt      : the header-only cppwinrt projection
#   - cswinrt 2     : the conventional C#/WinRT projection, latest released stable (2.x)
# All four activate the identical component, so per-call deltas are pure projection cost.
#
# The cswinrt 3 preview column is disabled: the package omits the `WinRT.Interop` assembly that its
# delegate marshaller loads, so the first event subscription throws. The project is a focused,
# ignored regression probe until a complete preview ships. See crates/samples/test_bench/readme.md.
#
# Every build runs through Invoke-Build, which stops the script on a non-zero exit instead of
# swallowing the error and running a stale binary. Stale executables are deleted before building,
# and the run loop fails if a consumer is missing or does not emit every metric.
#
# Metrics are listed once in $Metrics (just before the run loop); add a metric by emitting a
# matching `Name: <ms> ms` line from every consumer and adding its name to that array.
[CmdletBinding()]
param(
    [long]$Iterations = 10000000,
    [ValidateRange(1, 99)]
    [int]$Runs = 3
)

$ErrorActionPreference = 'Stop'
$here = $PSScriptRoot
$root = (Resolve-Path "$here/../../..").Path
$releaseDir = Join-Path $root 'target/release'
$csharp = Join-Path $here 'csharp'
$cswinrt2 = Join-Path $here 'cswinrt2'

# Runs a build step and stops the whole script if it fails, printing the captured output. A bare
# `... | Out-Null` swallows build errors: when a dotnet or cargo build fails, a stale executable
# from an earlier run is left in place and then produces silently wrong numbers (a dropped metric
# becomes a blank table cell instead of an error). Fail loudly instead.
function Invoke-Build([string]$label, [scriptblock]$step) {
    Write-Host $label -ForegroundColor Cyan
    $output = & $step 2>&1
    if ($LASTEXITCODE -ne 0) {
        $output | ForEach-Object { Write-Host $_ }
        throw "$label FAILED (exit code $LASTEXITCODE)."
    }
}

# Each consumer's binary directory and executable. Defined before the builds so stale executables
# can be deleted up front: if a later build fails, the missing-executable check in the run loop
# turns it into a hard error rather than a silent run of the previous build's binary.
$csBin = Join-Path $csharp 'bin/x64/Release/net10.0'
$cw2Bin = Join-Path $cswinrt2 'bin/x64/Release/net10.0-windows10.0.19041.0'

$consumers = @(
    @{ Name = 'cppwinrt'; Exe = (Join-Path $releaseDir 'test_bench_cpp.exe') },
    @{ Name = 'windows-rs'; Exe = (Join-Path $releaseDir 'test_bench_rust.exe') },
    @{ Name = 'windows-csharp'; Exe = (Join-Path $csBin 'test_bench_cs.exe') },
    @{ Name = 'cswinrt 2'; Exe = (Join-Path $cw2Bin 'test_bench_cswinrt2.exe') }
)

# Delete stale executables so a failed rebuild cannot leave an earlier binary behind to run.
foreach ($c in $consumers) { Remove-Item $c.Exe -Force -ErrorAction SilentlyContinue }

# Build the component first, alone, so its build script (the winmd writer) never races the
# consumer build scripts that read the same winmd.
Invoke-Build 'Building Rust component (release)...' {
    cargo build --release --manifest-path "$root/Cargo.toml" -p test_bench_component
}
Invoke-Build 'Building Rust and C++ consumers (release)...' {
    cargo build --release --manifest-path "$root/Cargo.toml" -p test_bench_rust -p test_bench_cpp
}
# Build the windows-csharp consumer crate so its build script regenerates Bench.cs before the
# csproj compiles it.
Invoke-Build 'Building windows-csharp consumer crate (regenerates Bench.cs)...' {
    cargo build --release --manifest-path "$root/Cargo.toml" -p test_bench_cs
}
Invoke-Build 'Building windows-csharp consumer (dotnet)...' { dotnet build $csharp -c Release }
Invoke-Build 'Building cswinrt 2 consumer (dotnet)...' { dotnet build $cswinrt2 -c Release }

# WinRT registration-free activation probes for a module named after the type's namespace
# (Bench.dll), loaded from the executable's directory. Stage the component cdylib under that
# name beside each consumer's binary.
Copy-Item (Join-Path $releaseDir 'bench_component.dll') (Join-Path $releaseDir 'Bench.dll') -Force
Copy-Item (Join-Path $releaseDir 'bench_component.dll') (Join-Path $csBin 'Bench.dll') -Force
Copy-Item (Join-Path $releaseDir 'bench_component.dll') (Join-Path $cw2Bin 'Bench.dll') -Force

# Metric names in report order, defined here so the run loop can verify every consumer emitted
# every one. Add a metric by emitting a matching `Name: <ms> ms` line from every consumer and
# adding its name here.
$Metrics = 'Create', 'Int32', 'String', 'Add', 'Cast', 'CastOwned', 'Interface', 'Object', 'Event', 'AddRemove', 'Vector', 'IterateVector', 'GetMany', 'Map', 'Lookup', 'VectorView', 'MapView', 'Reference', 'Async', 'Error'

$timeSamples = @{}
$memSamples = @{}
$times = @{}
$mem = @{}
$leak = @{}
foreach ($c in $consumers) {
    if (-not (Test-Path $c.Exe)) {
        throw "$($c.Name): executable not found at '$($c.Exe)'. Its build must have failed."
    }
    $timeSamples[$c.Name] = @{}
    foreach ($metric in $Metrics) {
        $timeSamples[$c.Name][$metric] = @()
    }
    $memSamples[$c.Name] = @()

    for ($run = 1; $run -le $Runs; $run++) {
        Write-Host "Running $($c.Name) ($Iterations iterations, run $run/$Runs)..." -ForegroundColor Cyan
        $lines = & $c.Exe --iterations $Iterations
        if ($LASTEXITCODE -ne 0) {
            $lines | ForEach-Object { Write-Host $_ }
            throw "$($c.Name): consumer exited with code $LASTEXITCODE."
        }
        $lines | Write-Host
        $t = @{}
        $runMem = $null
        $runLeak = $null
        foreach ($line in $lines) {
            if ($line -match '^(\w+):\s+(\d+)\s*ms$') {
                $t[$matches[1]] = [int]$matches[2]
            }
            elseif ($line -match '\(([\d.]+) bytes/object\)') {
                $runMem = [double]$matches[1]
            }
            elseif ($line -match '^Leak:\s+(-?\d+)$') {
                $runLeak = [int]$matches[1]
            }
        }

        $missing = @($Metrics | Where-Object { -not $t.ContainsKey($_) })
        if ($missing.Count -gt 0) {
            throw "$($c.Name): missing metric line(s): $($missing -join ', '). Run output above is incomplete."
        }
        if ($null -eq $runMem) { throw "$($c.Name): no 'bytes/object' memory line in output." }
        if ($null -eq $runLeak) { throw "$($c.Name): no 'Leak:' line in output." }
        if ($runLeak -ne 0) { throw "$($c.Name): native leak count was $runLeak on run $run." }

        foreach ($metric in $Metrics) {
            $timeSamples[$c.Name][$metric] += $t[$metric]
        }
        $memSamples[$c.Name] += $runMem
        $leak[$c.Name] = $runLeak
    }
}

$names = $consumers | ForEach-Object { $_.Name }
$w = 21
function Row($label, $cells) {
    $out = '| {0,-13} |' -f $label
    foreach ($cell in $cells) { $out += (' {0,' + $w + '} |') -f $cell }
    Write-Host $out
}

function Integer($value) {
    [string]::Format([System.Globalization.CultureInfo]::InvariantCulture, '{0:N0}', $value)
}

function Median($values) {
    $ordered = @($values | Sort-Object)
    $middle = [Math]::Floor($ordered.Count / 2)
    if ($ordered.Count % 2) {
        return $ordered[$middle]
    }
    return ($ordered[$middle - 1] + $ordered[$middle]) / 2
}

function PairwiseWinners($values) {
    $cells = @($values | ForEach-Object { Integer $_ })
    foreach ($group in @(@(0, 1), @(2, 3))) {
        $winner = ($group | ForEach-Object { $values[$_] } | Measure-Object -Minimum).Minimum
        foreach ($index in $group) {
            if ($values[$index] -eq $winner) {
                $cells[$index] = "**$($cells[$index])**"
            }
        }
    }
    return $cells
}

foreach ($c in $consumers) {
    $times[$c.Name] = @{}
    foreach ($metric in $Metrics) {
        $times[$c.Name][$metric] = Median $timeSamples[$c.Name][$metric]
    }
    $mem[$c.Name] = Median $memSamples[$c.Name]
}

$runLabel = if ($Runs -eq 1) { '1 run' } else { "median of $Runs runs" }
Write-Host "`n## Metrics ($Iterations iterations; $runLabel; milliseconds except Memory, lower is better)`n"
Row 'Metric' $names
Write-Host ('|' + ('-' * 15) + '|' + (($names | ForEach-Object { '-' * ($w + 2) }) -join '|') + '|')
foreach ($metric in $Metrics) {
    $values = @($names | ForEach-Object { $times[$_][$metric] })
    Row $metric (PairwiseWinners $values)
}
$memoryValues = @($names | ForEach-Object { [Math]::Round($mem[$_]) })
Row 'Memory' (PairwiseWinners $memoryValues)
Write-Host "`nError runs a reduced $([Math]::Min($Iterations, 1000000))-iteration loop: it calls a method that always returns a"
Write-Host 'failing HRESULT. Rust observes it as a `Result` (a branch, no unwind); the other three throw and'
Write-Host 'catch an exception, which is orders of magnitude more expensive, so a full-count loop would'
Write-Host 'dominate the run.'

Write-Host "`n## Leak (live native objects above baseline after the run, 0 is correct)`n"
Row 'Metric' $names
Write-Host ('|' + ('-' * 15) + '|' + (($names | ForEach-Object { '-' * ($w + 2) }) -join '|') + '|')
Row 'Leak' ($names | ForEach-Object { Integer $leak[$_] })
