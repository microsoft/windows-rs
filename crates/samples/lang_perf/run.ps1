#!/usr/bin/env pwsh
# Runs the language-projection benchmark matrix.
[CmdletBinding()]
param(
    [long]$Iterations = 10000000,
    [switch]$IncludeAot
)

$ErrorActionPreference = 'Stop'
$root = (Resolve-Path "$PSScriptRoot/../../..").Path
$releaseDir = Join-Path $root 'target/release'
$csharp = Join-Path $PSScriptRoot 'csharp'

Write-Host 'Building Rust and C++ consumers (release)...' -ForegroundColor Cyan
# Generate lang.winmd before parallel readers can lock it.
cargo build --release --manifest-path "$root/Cargo.toml" -p lang_perf_component | Out-Null
cargo build --release --manifest-path "$root/Cargo.toml" -p lang_perf_component_cpp -p lang_perf_rust -p lang_perf_cpp | Out-Null

# WinRT activation and the C# consumer both resolve the component as LangPerf.dll.
$components = 'rust', 'cpp'
$componentLabel = @{ rust = 'Rust'; cpp = 'C++' }
$componentDir = @{}
foreach ($comp in $components) {
    $dir = Join-Path $releaseDir "component_$comp"
    New-Item -ItemType Directory -Force -Path $dir | Out-Null
    Copy-Item (Join-Path $releaseDir "langperf_$comp.dll") (Join-Path $dir 'LangPerf.dll') -Force
    $componentDir[$comp] = $dir
}

$aotExe = $null
if ($IncludeAot) {
    Write-Host 'Publishing C# Native AOT...' -ForegroundColor Cyan
    $env:PATH = "C:\Program Files (x86)\Microsoft Visual Studio\Installer;$env:PATH"
    $aotDir = Join-Path $csharp 'aot-publish'
    dotnet publish $csharp -c Release -r win-x64 /p:PublishAot=true -o $aotDir | Out-Null
    $aotExe = Join-Path $aotDir 'lang_perf_cs.exe'
}

$basePath = $env:PATH

$consumers = [System.Collections.Generic.List[object]]::new()
$consumers.Add(@{ Name = 'C#/JIT'; Kind = 'cs' })
if ($IncludeAot) { $consumers.Add(@{ Name = 'C#/AOT'; Kind = 'csaot' }) }
$consumers.Add(@{ Name = 'C++'; Kind = 'native'; Exe = 'lang_perf_cpp.exe' })
$consumers.Add(@{ Name = 'Rust'; Kind = 'native'; Exe = 'lang_perf_rust.exe' })

$results = @()
foreach ($consumer in $consumers) {
    foreach ($comp in $components) {
        $combo = "$($consumer.Name)->$($componentLabel[$comp])"
        Write-Host "Running $combo..." -ForegroundColor Cyan

        switch ($consumer.Kind) {
            'native' {
                $lines = & (Join-Path $releaseDir $consumer.Exe) --iterations $Iterations --component $comp
            }
            'cs' {
                $env:PATH = "$($componentDir[$comp]);$basePath"
                $lines = dotnet run -c Release --project $csharp -- --iterations $Iterations
            }
            'csaot' {
                $env:PATH = "$($componentDir[$comp]);$basePath"
                $lines = & $aotExe --iterations $Iterations
            }
        }
        $lines | Write-Host

        $loaded = ($lines | Select-String -Pattern '->\s+(\S+)\s+component' | Select-Object -First 1).Matches.Groups[1].Value
        if ($loaded -and $loaded -ne $componentLabel[$comp]) {
            Write-Warning "$combo loaded the $loaded component, not $($componentLabel[$comp])"
        }

        $row = [ordered]@{ Combo = $combo }
        foreach ($line in $lines) {
            if ($line -match '^(\w+):\s+(\d+)\s*ms$') {
                $row[$matches[1]] = [int]$matches[2]
            }
        }
        $results += [pscustomobject]$row
    }
}

$metrics = 'Create', 'Int32', 'String', 'Object', 'Cast', 'Event', 'AddRemove', 'IterateVector', 'GetMany', 'Map', 'Async', 'Reference', 'Error'
$combos = $results | ForEach-Object { $_.Combo }
$width = [Math]::Max(8, ($combos | Measure-Object -Property Length -Maximum).Maximum)

Write-Host "`n## Matrix results ($Iterations iterations, milliseconds, consumer->component)`n"
$header = '| {0,-8} | ' -f 'Metric'
$header += (($combos | ForEach-Object { "{0,$width}" -f $_ }) -join ' | ') + ' |'
Write-Host $header
Write-Host ('|' + ('-' * 10) + '|' + (($combos | ForEach-Object { '-' * ($width + 2) }) -join '|') + '|')
foreach ($metric in $metrics) {
    $cells = $results | ForEach-Object { "{0,$width}" -f $_.$metric }
    Write-Host (('| {0,-8} | ' -f $metric) + ($cells -join ' | ') + ' |')
}
