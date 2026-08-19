param(
    [Parameter(Mandatory = $true)]
    [string]$BaseRef,
    [ValidateRange(1, 10)]
    [int]$Repetitions = 5,
    [ValidateRange(2, 60)]
    [int]$DurationSeconds = 10
)

$ErrorActionPreference = "Stop"
$PSNativeCommandUseErrorActionPreference = $true

$repo = (Resolve-Path (Join-Path $PSScriptRoot "..\..\..\..")).Path
$benchmark = Join-Path $PSScriptRoot "..\benchmark.ps1"
$durationEnvironment = "WINDOWS_REACTOR_STRESS_PERFORMANCE_SECONDS"

function Get-Median([double[]]$Values) {
    $sorted = @($Values | Sort-Object)
    $middle = [int]($sorted.Count / 2)
    if ($sorted.Count % 2) {
        return $sorted[$middle]
    }
    return ($sorted[$middle - 1] + $sorted[$middle]) / 2
}

function Get-MedianMetrics([object[]]$Runs) {
    $properties = @(
        "startupMs",
        "rendersPerSec",
        "avgReconcileMs",
        "avgDiffMs",
        "avgMemoryMB",
        "peakMemoryMB",
        "avgPrivateMemoryMB",
        "peakPrivateMemoryMB",
        "avgHandles",
        "peakHandles",
        "allocBytesPerRender",
        "avgFps",
        "updates",
        "elementsCreated"
    )
    $result = [ordered]@{}
    foreach ($property in $properties) {
        $values = @($Runs | ForEach-Object { [double]$_.$property })
        if ($values.Count -gt 0 -and $null -ne $Runs[0].$property) {
            $result[$property] = Get-Median $values
        }
    }
    return [pscustomobject]$result
}

function Get-JsonMetrics([object[]]$Output, [string]$Description) {
    $json = $null
    foreach ($line in $Output) {
        if ("$line" -match "REACTOR_PERF_JSON (\{.*\})") {
            $json = $Matches[1]
        }
    }
    if (!$json) {
        throw "$Description did not emit REACTOR_PERF_JSON"
    }
    return $json | ConvertFrom-Json
}

function Invoke-ReactorStress([string]$Scenario) {
    $filter = "winui::tests::stress_performance::matched_stock_$Scenario"
    Set-Item "Env:$durationEnvironment" $DurationSeconds
    try {
        $output = & cargo test -p windows-reactor --release --quiet $filter -- `
            --ignored --nocapture --test-threads=1 2>&1
        if ($LASTEXITCODE -ne 0) {
            throw "Reactor $Scenario StressPerf failed`n$($output -join "`n")"
        }
    }
    finally {
        Remove-Item "Env:$durationEnvironment" -ErrorAction SilentlyContinue
    }
    return Get-JsonMetrics $output "Reactor $Scenario StressPerf"
}

function Assert-Max([double]$Actual, [double]$Maximum, [string]$Description) {
    if ($Actual -gt $Maximum) {
        throw "$Description is $Actual; maximum is $Maximum"
    }
}

function Assert-Min([double]$Actual, [double]$Minimum, [string]$Description) {
    if ($Actual -lt $Minimum) {
        throw "$Description is $Actual; minimum is $Minimum"
    }
}

Push-Location $repo
try {
    & $benchmark -BaseRef $BaseRef -Samples $Repetitions

    $runs = @{ update = @(); churn = @() }
    for ($iteration = 1; $iteration -le $Repetitions; $iteration++) {
        Write-Output "native performance repetition $iteration of $Repetitions"
        $runs.update += Invoke-ReactorStress "updates"
        $runs.churn += Invoke-ReactorStress "churn"
    }

    $update = Get-MedianMetrics $runs.update
    $churn = Get-MedianMetrics $runs.churn

    @(
        [pscustomobject]@{ Scenario = "update"; Metrics = $update }
        [pscustomobject]@{ Scenario = "churn"; Metrics = $churn }
    ) | ForEach-Object {
        [pscustomobject]@{
            App = "Reactor"
            Scenario = $_.Scenario
            StartupMs = "{0:N1}" -f $_.Metrics.startupMs
            ReconcileMs = "{0:N2}" -f $_.Metrics.avgReconcileMs
            RendersPerSec = "{0:N2}" -f $_.Metrics.rendersPerSec
            FPS = "{0:N2}" -f $_.Metrics.avgFps
            AllocKB = "{0:N1}" -f ($_.Metrics.allocBytesPerRender / 1KB)
            PeakMemoryMB = "{0:N1}" -f $_.Metrics.peakMemoryMB
            PeakPrivateMB = "{0:N1}" -f $_.Metrics.peakPrivateMemoryMB
            PeakHandles = "{0:N0}" -f $_.Metrics.peakHandles
        }
    } | Format-Table -AutoSize

    Assert-Max $update.startupMs 750 "Reactor update startup ms"
    Assert-Max $churn.startupMs 750 "Reactor churn startup ms"
    Assert-Min $update.updates 10 "Reactor update ticks"
    Assert-Min $churn.updates 10 "Reactor churn ticks"
    Assert-Min $churn.elementsCreated 400 "Reactor churn elements created"

    Write-Output "windows-reactor performance measurement passed"
}
finally {
    Pop-Location
}
