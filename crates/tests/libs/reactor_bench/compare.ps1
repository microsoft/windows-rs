param(
    [Parameter(Mandatory = $true)]
    [string]$BaseRef,
    [double]$MaxByteRegressionPercent = 10
)

$ErrorActionPreference = "Stop"
$PSNativeCommandUseErrorActionPreference = $true
$benchmarkFormatVersion = 1
$root = (Resolve-Path (Join-Path $PSScriptRoot "..\..\..\..")).Path
$baseCommit = & git -C $root merge-base HEAD $BaseRef
if ($LASTEXITCODE -ne 0 -or !$baseCommit) {
    throw "Could not resolve merge base for $BaseRef"
}
$baseCommit = $baseCommit.Trim()

$tempRoot = if ($env:RUNNER_TEMP) { $env:RUNNER_TEMP } else { [System.IO.Path]::GetTempPath() }
$baseDirectory = Join-Path $tempRoot "windows-rs-reactor-base-$PID"

function Invoke-Benchmark([string]$workingDirectory, [string]$rustFlags) {
    Push-Location $workingDirectory
    try {
        $previousRustFlags = $env:RUSTFLAGS
        $env:RUSTFLAGS = $rustFlags
        $output = & cargo run -p test_reactor_bench --release --quiet -- `
            --iters 500 --reps 12
        if ($LASTEXITCODE -ne 0) {
            throw "Benchmark failed in $workingDirectory"
        }
    } finally {
        $env:RUSTFLAGS = $previousRustFlags
        Pop-Location
    }

    $rows = @{}
    $formatVersion = $null
    $inMemory = $false
    foreach ($line in $output) {
        if ($line -match "^reactor-benchmark-format:\s+(\d+)\s*$") {
            if ($null -ne $formatVersion) {
                throw "Benchmark output contains more than one format marker in $workingDirectory"
            }
            $formatVersion = [int]$Matches[1]
        } elseif ($line -eq "idle component memory") {
            $inMemory = $true
            continue
        }
        if (!$inMemory -and
            $line -match "^\s*(\S+)\s+(\d+)\s+([\d.]+)\s+([\d.]+)\s+([\d.]+)\s*$") {
            $key = "$($Matches[1])/$($Matches[2])"
            $rows[$key] = [pscustomobject]@{
                Time = [double]$Matches[3]
                Bytes = [double]$Matches[4]
                Allocs = [double]$Matches[5]
                RetainedBytes = $null
            }
        } elseif ($inMemory -and
            $line -match "^\s*(\d+)\s+(\d+)\s+([\d.]+)\s*$") {
            $key = "idle_memory/$($Matches[1])"
            $rows[$key] = [pscustomobject]@{
                Time = $null
                Bytes = $null
                Allocs = $null
                RetainedBytes = [double]$Matches[2]
            }
        }
    }
    return [pscustomobject]@{
        FormatVersion = $formatVersion
        Rows = $rows
    }
}

try {
    & git -C $root worktree add --detach $baseDirectory $baseCommit
    if ($LASTEXITCODE -ne 0) {
        throw "Could not create benchmark worktree for $baseCommit"
    }
    $baseResult = Invoke-Benchmark $baseDirectory ""
    $currentResult = Invoke-Benchmark $root "-D warnings"
    if ($currentResult.FormatVersion -ne $benchmarkFormatVersion) {
        throw "Current benchmark output does not use format $benchmarkFormatVersion"
    }
    if ($null -eq $baseResult.FormatVersion) {
        Write-Warning (
            "The merge base predates reactor benchmark format $benchmarkFormatVersion; " +
            "comparison starts after the final benchmark cutover."
        )
        exit 0
    }
    if ($baseResult.FormatVersion -ne $benchmarkFormatVersion) {
        throw (
            "Merge-base benchmark format $($baseResult.FormatVersion) is incompatible with " +
            "current format $benchmarkFormatVersion"
        )
    }

    $base = $baseResult.Rows
    $current = $currentResult.Rows
    $required = @(
        "mount_shutdown/512",
        "textbox_mount/512",
        "reference_mount/512",
        "update_no_change/512",
        "update_1_changed/512",
        "update_all_changed/512",
        "keyed_reverse/512",
        "keyed_rotate1/512",
        "keyed_reverse/4096",
        "root_replace/1",
        "idle_memory/513",
        "idle_memory/4097",
        "idle_memory/16385"
    )

    $failed = $false
    $rows = foreach ($key in $required) {
        if (!$base.ContainsKey($key) -or !$current.ContainsKey($key)) {
            throw "Benchmark output does not contain $key in both revisions"
        }
        $before = $base[$key]
        $after = $current[$key]

        if ($null -ne $before.RetainedBytes) {
            $change = 100 * ($after.RetainedBytes - $before.RetainedBytes) /
                [Math]::Max($before.RetainedBytes, 1)
            if ($change -gt $MaxByteRegressionPercent) {
                $failed = $true
            }
            [pscustomobject]@{
                Benchmark = $key
                BaseNs = "-"
                CurrentNs = "-"
                TimeChange = "-"
                BaseBytes = "{0:N0}" -f $before.RetainedBytes
                CurrentBytes = "{0:N0}" -f $after.RetainedBytes
                ByteChange = "{0:+0.0;-0.0;0.0}%" -f $change
                BaseAllocs = "-"
                CurrentAllocs = "-"
            }
            continue
        }

        $timeChange = 100 * ($after.Time - $before.Time) / [Math]::Max($before.Time, 1)
        $byteChange = 100 * ($after.Bytes - $before.Bytes) / [Math]::Max($before.Bytes, 1)
        if ($byteChange -gt $MaxByteRegressionPercent -or $after.Allocs -gt $before.Allocs) {
            $failed = $true
        }
        [pscustomobject]@{
            Benchmark = $key
            BaseNs = "{0:N1}" -f $before.Time
            CurrentNs = "{0:N1}" -f $after.Time
            TimeChange = "{0:+0.0;-0.0;0.0}%" -f $timeChange
            BaseBytes = "{0:N1}" -f $before.Bytes
            CurrentBytes = "{0:N1}" -f $after.Bytes
            ByteChange = "{0:+0.0;-0.0;0.0}%" -f $byteChange
            BaseAllocs = "{0:N2}" -f $before.Allocs
            CurrentAllocs = "{0:N2}" -f $after.Allocs
        }
    }

    $rows | Format-Table -AutoSize
    if ($failed) {
        throw "Reactor benchmark regressed beyond the allowed memory floor."
    }
} finally {
    if (Test-Path $baseDirectory) {
        & git -C $root worktree remove --force $baseDirectory
    }
}
