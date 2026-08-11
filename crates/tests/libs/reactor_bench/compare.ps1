param(
    [Parameter(Mandatory = $true)]
    [string]$BaseRef,
    [double]$MaxTimeRegressionPercent = 25,
    [double]$MaxByteRegressionPercent = 10
)

$ErrorActionPreference = "Stop"
$root = (Resolve-Path (Join-Path $PSScriptRoot "..\..\..\..")).Path
$baseCommit = & git -C $root merge-base HEAD $BaseRef
if ($LASTEXITCODE -ne 0 -or !$baseCommit) {
    throw "Could not resolve merge base for $BaseRef"
}
$baseCommit = $baseCommit.Trim()

$tempRoot = if ($env:RUNNER_TEMP) { $env:RUNNER_TEMP } else { [System.IO.Path]::GetTempPath() }
$baseDirectory = Join-Path $tempRoot "windows-rs-reactor-base-$PID"

function Invoke-Benchmark([string]$workingDirectory) {
    Push-Location $workingDirectory
    try {
        $output = & cargo run -p test_reactor_bench --release --quiet -- --iters 1000 --reps 12
        if ($LASTEXITCODE -ne 0) {
            throw "Benchmark failed in $workingDirectory"
        }
    } finally {
        Pop-Location
    }

    $rows = @{}
    foreach ($line in $output) {
        if ($line -match "^\s*(\S+)\s+(\d+)\s+([\d.]+)\s+([\d.]+)\s+([\d.]+)\s+\d+\s+\d+\s+\d+\s*$") {
            $key = "$($Matches[1])/$($Matches[2])"
            $rows[$key] = [pscustomobject]@{
                Time = [double]$Matches[3]
                Bytes = [double]$Matches[4]
                Allocs = [double]$Matches[5]
            }
        }
    }
    return $rows
}

try {
    & git -C $root worktree add --detach $baseDirectory $baseCommit
    if ($LASTEXITCODE -ne 0) {
        throw "Could not create benchmark worktree for $baseCommit"
    }

    $base = Invoke-Benchmark $baseDirectory
    $current = Invoke-Benchmark $root
    $required = @(
        "component_mount/1",
        "templated_mount/64",
        "templated_mount/4096",
        "mount_unmount/64",
        "mount_unmount/512",
        "update_1_changed/512",
        "update_no_change/512",
        "keyed_reverse/64",
        "keyed_reverse/512",
        "keyed_rotate1/512"
    )

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
        $allocationRegression =
            $byteChange -gt $MaxByteRegressionPercent -or $after.Allocs -gt $before.Allocs
        $timeRegression = $timeChange -gt $MaxTimeRegressionPercent
        if ($allocationRegression) {
            $failed = $true
        }
        if ($timeRegression) {
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
        }
    }

    $rows | Format-Table -AutoSize
    foreach ($warning in $timingWarnings) {
        Write-Warning $warning
    }
    if ($failed) {
        throw "Reconciler benchmark regressed beyond the allowed memory floor."
    }
} finally {
    if (Test-Path $baseDirectory) {
        & git -C $root worktree remove --force $baseDirectory
    }
}
