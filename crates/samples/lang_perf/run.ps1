#!/usr/bin/env pwsh
# Builds and runs the language-projection benchmarks (Release) and prints a comparison
# table. C#/WinRT runs on the JIT runtime by default; pass -IncludeAot to also publish
# and measure a Native AOT build. The C# benchmark is built with `dotnet` rather than
# cargo. The no-op WinRT component is built as a package first so its cdylib
# (langperf.dll) lands next to the consumer executables for activation.
[CmdletBinding()]
param(
    [long]$Iterations = 10000000,
    [switch]$IncludeAot
)

$ErrorActionPreference = 'Stop'
$root = (Resolve-Path "$PSScriptRoot/../../..").Path
$releaseDir = Join-Path $root 'target/release'
$csharp = Join-Path $PSScriptRoot 'csharp'

Write-Host 'Building component (release)...' -ForegroundColor Cyan
cargo build --release --manifest-path "$root/Cargo.toml" -p lang_perf_component | Out-Null

Write-Host 'Building Rust and C++ consumers (release)...' -ForegroundColor Cyan
cargo build --release --manifest-path "$root/Cargo.toml" -p lang_perf_rust -p lang_perf_cpp | Out-Null

$aotExe = $null
if ($IncludeAot) {
    Write-Host 'Publishing C# Native AOT...' -ForegroundColor Cyan
    # The Native AOT linker step shells out to vswhere.exe to locate the MSVC toolchain.
    $env:PATH = "C:\Program Files (x86)\Microsoft Visual Studio\Installer;$env:PATH"
    $aotDir = Join-Path $csharp 'aot-publish'
    dotnet publish $csharp -c Release -r win-x64 /p:PublishAot=true -o $aotDir | Out-Null
    $aotExe = Join-Path $aotDir 'lang_perf_cs.exe'
}

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
$results += Invoke-Bench 'C#/JIT' {
    # The C# consumer resolves langperf.dll via PATH at runtime.
    $env:PATH = "$releaseDir;$env:PATH"
    dotnet run -c Release --project $csharp -- --iterations $Iterations
}
if ($IncludeAot) {
    $results += Invoke-Bench 'C#/AOT' {
        $env:PATH = "$releaseDir;$env:PATH"
        & $aotExe --iterations $Iterations
    }
}
$results += Invoke-Bench 'C++' {
    & (Join-Path $releaseDir 'lang_perf_cpp.exe') --iterations $Iterations
}
$results += Invoke-Bench 'Rust' {
    & (Join-Path $releaseDir 'lang_perf_rust.exe') --iterations $Iterations
}

$metrics = 'Create', 'Int32', 'String', 'Object', 'Cast', 'Error'
$langs = $results | ForEach-Object { $_.Language }
Write-Host "`n## Results ($Iterations iterations, milliseconds)`n"
Write-Host (('| {0,-6} | ' -f 'Metric') + (($langs | ForEach-Object { '{0,8}' -f $_ }) -join ' | ') + ' |')
Write-Host ('|' + ('-' * 8) + '|' + (($langs | ForEach-Object { '-' * 10 }) -join '|') + '|')
foreach ($metric in $metrics) {
    $cells = $results | ForEach-Object { '{0,8}' -f $_.$metric }
    Write-Host (('| {0,-6} | ' -f $metric) + ($cells -join ' | ') + ' |')
}
