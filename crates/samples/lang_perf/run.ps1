#!/usr/bin/env pwsh
# Builds and runs the three language-projection benchmarks (Release) and prints a
# comparison table. The C# consumer is not a cargo member, so it is built and run
# with `dotnet` directly. The no-op WinRT component is built as a package first so
# its cdylib (langperf.dll) lands next to the consumer executables for activation.
[CmdletBinding()]
param(
    [long]$Iterations = 10000000
)

$ErrorActionPreference = 'Stop'
$root = (Resolve-Path "$PSScriptRoot/../../..").Path
$releaseDir = Join-Path $root 'target/release'

Write-Host 'Building component (release)...' -ForegroundColor Cyan
cargo build --release --manifest-path "$root/Cargo.toml" -p lang_perf_component | Out-Null

Write-Host 'Building Rust and C++ consumers (release)...' -ForegroundColor Cyan
cargo build --release --manifest-path "$root/Cargo.toml" -p lang_perf_rust -p lang_perf_cpp | Out-Null

function Invoke-Bench {
    param([string]$Language, [scriptblock]$Run)

    Write-Host "Running $Language..." -ForegroundColor Cyan
    $lines = & $Run
    $lines | Write-Host

    $row = [ordered]@{ Language = $Language }
    foreach ($line in $lines) {
        if ($line -match '^(\w+):\s+(\d+)\s*ms$') {
            $row[$matches[1]] = [int]$matches[2]
        }
    }
    [pscustomobject]$row
}

$results = @()
$results += Invoke-Bench 'C++/WinRT' {
    & (Join-Path $releaseDir 'lang_perf_cpp.exe') --iterations $Iterations
}
$results += Invoke-Bench 'C#/WinRT' {
    # The C# consumer resolves langperf.dll via PATH at runtime.
    $env:PATH = "$releaseDir;$env:PATH"
    dotnet run -c Release --project (Join-Path $PSScriptRoot 'csharp') -- --iterations $Iterations
}
$results += Invoke-Bench 'Rust' {
    & (Join-Path $releaseDir 'lang_perf_rust.exe') --iterations $Iterations
}

$metrics = 'Create', 'Int32', 'String', 'Object', 'Cast'
$langs = $results | ForEach-Object { $_.Language }
Write-Host "`n## Results ($Iterations iterations, milliseconds)`n"
Write-Host (('| {0,-6} | ' -f 'Metric') + (($langs | ForEach-Object { '{0,10}' -f $_ }) -join ' | ') + ' |')
Write-Host ('|' + ('-' * 8) + '|' + (($langs | ForEach-Object { '-' * 12 }) -join '|') + '|')
foreach ($metric in $metrics) {
    $cells = $results | ForEach-Object { '{0,10}' -f $_.$metric }
    Write-Host (('| {0,-6} | ' -f $metric) + ($cells -join ' | ') + ' |')
}
