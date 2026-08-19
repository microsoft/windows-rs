param(
    [Parameter(Mandatory = $true)]
    [string]$BaseRef,
    [int]$Samples = 5,
    [double]$MaxTimeRegressionPercent = 15,
    [double]$MaxByteRegressionPercent = 10
)

$ErrorActionPreference = "Stop"
$root = (Resolve-Path (Join-Path $PSScriptRoot "..\..\..")).Path
$baseCommit = & git -C $root merge-base HEAD $BaseRef
if ($LASTEXITCODE -ne 0 -or !$baseCommit) {
    throw "Could not resolve merge base for $BaseRef"
}
$baseCommit = $baseCommit.Trim()

$baseDirectory = Join-Path $root "target\reactor-benchmark-base-$PID"

function Get-Median([double[]]$Values) {
    $sorted = @($Values | Sort-Object)
    $middle = [int]($sorted.Count / 2)
    if ($sorted.Count % 2) {
        return $sorted[$middle]
    }
    return ($sorted[$middle - 1] + $sorted[$middle]) / 2
}

function Invoke-Benchmark([string]$WorkingDirectory) {
    Push-Location $WorkingDirectory
    try {
        $env:WINDOWS_REACTOR_BENCH_ITERS = "1000"
        $env:WINDOWS_REACTOR_BENCH_REPS = "9"
        try {
            $output = & cargo test -p windows-reactor --release --quiet `
                tests::benchmark::benchmark -- --ignored --nocapture --test-threads=1
        } finally {
            Remove-Item Env:WINDOWS_REACTOR_BENCH_ITERS -ErrorAction SilentlyContinue
            Remove-Item Env:WINDOWS_REACTOR_BENCH_REPS -ErrorAction SilentlyContinue
        }
        if ($LASTEXITCODE -ne 0) {
            throw "Benchmark failed in $WorkingDirectory"
        }
    } finally {
        Pop-Location
    }

    $rows = @{}
    foreach ($line in $output) {
        if ($line -match "^\s*(\S+)\s+(\d+)\s+([\d.]+)\s+([\d.]+)\s+([\d.]+)\s+([\d.]+)\s*$") {
            $key = "$($Matches[1])/$($Matches[2])"
            $rows[$key] = [pscustomobject]@{
                Time = [double]$Matches[3]
                Bytes = [double]$Matches[4]
                Allocs = [double]$Matches[5]
                Commands = [double]$Matches[6]
            }
        }
    }
    return $rows
}

function Invoke-Samples([string]$WorkingDirectory) {
    $collected = @{}
    1..$Samples | ForEach-Object {
        $rows = Invoke-Benchmark $WorkingDirectory
        foreach ($key in $rows.Keys) {
            if (!$collected.ContainsKey($key)) {
                $collected[$key] = @{
                    Time = @()
                    Bytes = @()
                    Allocs = @()
                    Commands = @()
                }
            }
            $collected[$key]["Time"] += [double]$rows[$key].Time
            $collected[$key]["Bytes"] += [double]$rows[$key].Bytes
            $collected[$key]["Allocs"] += [double]$rows[$key].Allocs
            $collected[$key]["Commands"] += [double]$rows[$key].Commands
        }
    }

    $rows = @{}
    foreach ($key in $collected.Keys) {
        $rows[$key] = [pscustomobject]@{
            Time = Get-Median ([double[]]$collected[$key]["Time"])
            Bytes = Get-Median ([double[]]$collected[$key]["Bytes"])
            Allocs = Get-Median ([double[]]$collected[$key]["Allocs"])
            Commands = Get-Median ([double[]]$collected[$key]["Commands"])
        }
    }
    return $rows
}

try {
    & git -C $root worktree add --detach $baseDirectory $baseCommit
    if ($LASTEXITCODE -ne 0) {
        throw "Could not create benchmark worktree for $baseCommit"
    }

    $current = Invoke-Samples $root
    $benchmarkPaths = @("crates\libs\reactor\testing\unit\benchmark.rs")
    $baseHasBenchmark = $benchmarkPaths |
        Where-Object { Test-Path (Join-Path $baseDirectory $_) } |
        Select-Object -First 1
    if (!$baseHasBenchmark) {
        $introductionCommits =
            & git -C $root log --all --diff-filter=A --format=%H -- $benchmarkPaths
        $baselineShouldHaveBenchmark = $false
        foreach ($introductionCommit in $introductionCommits) {
            & git -C $root merge-base --is-ancestor $introductionCommit $baseCommit
            if ($LASTEXITCODE -eq 0) {
                $baselineShouldHaveBenchmark = $true
                break
            }
        }
        if ($baselineShouldHaveBenchmark) {
            throw "The merge base should contain the Reactor benchmark, but it is missing."
        }
        Write-Warning "The merge base predates the Reactor benchmark; no comparison is available."
        $current.GetEnumerator() |
            Sort-Object Key |
            ForEach-Object {
                [pscustomobject]@{
                    Benchmark = $_.Key
                    CurrentNs = "{0:N1}" -f $_.Value.Time
                    Bytes = "{0:N1}" -f $_.Value.Bytes
                    Allocs = "{0:N2}" -f $_.Value.Allocs
                    Commands = "{0:N2}" -f $_.Value.Commands
                }
            } |
            Format-Table -AutoSize
        return
    }

    $base = Invoke-Samples $baseDirectory
    $required = @(
        "mount_drop/64",
        "no_change/1",
        "one_change/1",
        "dirty_component/3",
        "keyed_rotate1/64",
        "keyed_rotate1/512",
        "virtual_event/1",
        "virtual_size/1",
        "virtual_keyed_same/1000",
        "virtual_keyed_move/1000",
        "virtual_selection/1000"
    )
    $currentFloors = @{
        "keyed_same/512" = @(96528, 520, 0)
        "keyed_append/520" = @(144445, 2498, 20)
        "keyed_sparse/512" = @(240868, 5322, 44)
        "keyed_reverse/512" = @(179280, 1658, 511)
        "application_validate/512" = @(136, 5, 0)
        "application_validate/4096" = @(136, 5, 0)
    }

    $failed = $false
    $timingWarnings = @()
    $rows = foreach ($key in $required) {
        if (!$base.ContainsKey($key) -or !$current.ContainsKey($key)) {
            throw "Benchmark output does not contain $key in both revisions"
        }

        $before = $base[$key]
        $after = $current[$key]
        $timeChange = 100 * ($after.Time - $before.Time) / $before.Time
        $byteChange = 100 * ($after.Bytes - $before.Bytes) / $before.Bytes
        if (
            $byteChange -gt $MaxByteRegressionPercent -or
            $after.Allocs -gt $before.Allocs + 0.01 -or
            $after.Commands -gt $before.Commands + 0.01
        ) {
            $failed = $true
        }
        if ($timeChange -gt $MaxTimeRegressionPercent) {
            $timingWarnings +=
                "$key time increased by $('{0:N1}' -f $timeChange)% on the hosted runner."
        }

        [pscustomobject]@{
            Benchmark = $key
            BaseNs = "{0:N1}" -f $before.Time
            CurrentNs = "{0:N1}" -f $after.Time
            TimeChange = "{0:+0.0;-0.0;0.0}%" -f $timeChange
            BaseBytes = "{0:N1}" -f $before.Bytes
            CurrentBytes = "{0:N1}" -f $after.Bytes
            BaseAllocs = "{0:N2}" -f $before.Allocs
            CurrentAllocs = "{0:N2}" -f $after.Allocs
            BaseCommands = "{0:N2}" -f $before.Commands
            CurrentCommands = "{0:N2}" -f $after.Commands
        }
    }

    $rows | Format-Table -AutoSize
    foreach ($warning in $timingWarnings) {
        Write-Warning $warning
    }
    $floorRows = foreach ($key in $currentFloors.Keys | Sort-Object) {
        if (!$current.ContainsKey($key)) {
            throw "Benchmark output does not contain $key"
        }
        $floor = $currentFloors[$key]
        $value = $current[$key]
        if (
            $value.Bytes -gt $floor[0] -or
            $value.Allocs -gt $floor[1] -or
            $value.Commands -gt $floor[2]
        ) {
            $failed = $true
        }
        [pscustomobject]@{
            Benchmark = $key
            Bytes = "{0:N1}" -f $value.Bytes
            MaxBytes = "{0:N1}" -f $floor[0]
            Allocs = "{0:N2}" -f $value.Allocs
            MaxAllocs = "{0:N2}" -f $floor[1]
            Commands = "{0:N2}" -f $value.Commands
            MaxCommands = "{0:N2}" -f $floor[2]
        }
    }
    $floorRows | Format-Table -AutoSize
    if ($failed) {
        throw "Reactor benchmark regressed beyond the allocation or command floor."
    }
} finally {
    if (Test-Path $baseDirectory) {
        & git -C $root worktree remove --force $baseDirectory
    }
}
