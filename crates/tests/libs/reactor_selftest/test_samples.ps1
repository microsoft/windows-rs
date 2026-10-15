$ErrorActionPreference = "Stop"
$runner = Join-Path $PSScriptRoot "run_samples.ps1"
$sampleTestState = @{
    Paths = @()
    ExitCode = 0
    Calls = 0
}

function git {
    if ($args -notcontains "--no-renames" -or $args -notcontains "--name-only") {
        throw "Unexpected git command: $args"
    }
    $sampleTestState.Calls++
    $global:LASTEXITCODE = $sampleTestState.ExitCode
    $sampleTestState.Paths
}

function Start-Process {
    throw "Listing samples must not build or launch a process"
}

function Assert-Selection([hashtable]$Options, [string[]]$Expected) {
    $actual = @(& $runner @Options -List | ForEach-Object { $_.Package } | Sort-Object)
    $expectedNames = @($Expected | Sort-Object)
    if (($actual -join ",") -ne ($expectedNames -join ",")) {
        throw "Expected [$expectedNames], got [$actual]"
    }
}

function Assert-Failure([hashtable]$Options, [string]$Expected) {
    $message = ""
    try {
        & $runner @Options -List | Out-Null
    } catch {
        $message = $_.Exception.Message
    }
    if (!$message.Contains($Expected)) {
        throw "Expected error '$Expected', got '$message'"
    }
}

$previousExitCode = $global:LASTEXITCODE
try {
    $all = @(& $runner -List)
    foreach ($family in "reactor", "composition", "webview") {
        if (@($all | Where-Object { $_.Family -eq $family }).Count -eq 0) {
            throw "Full sweep omitted $family"
        }
    }
    Assert-Selection @{ Package = @("reactor-counter") } @("reactor-counter")
    if ($sampleTestState.Calls -ne 0) {
        throw "Unfiltered and explicit package runs must not need git history"
    }

    $options = @{ Package = @("reactor-counter"); ChangedSince = "base" }
    Assert-Selection $options @("reactor-counter")

    $sampleTestState.Paths = @(
        "crates/samples/reactor/translation-transition/src/main.rs",
        "crates/samples/reactor/translation-transition/Cargo.toml",
        "crates/samples/reactor/counter/src/main.rs"
    )
    Assert-Selection $options @("reactor-counter", "reactor-translation-transition")

    # --no-renames reports both paths, so a move selects both surviving packages.
    $sampleTestState.Paths = @(
        "crates/samples/reactor/counter/old.rs",
        "crates/samples/reactor/workbench/new.rs",
        "crates/samples/reactor/deleted-package/src/main.rs",
        "crates/samples/reactor/counter-other/src/main.rs"
    )
    Assert-Selection $options @("reactor-counter", "reactor-workbench")

    $sampleTestState.Paths = @("crates/samples/webview/reactor/src/main.rs")
    Assert-Selection $options @("reactor-counter", "webview-reactor")
    Assert-Selection @{
        Package = @("reactor-counter")
        ChangedSince = "base"
        Family = @("reactor")
    } @("reactor-counter")
    Assert-Selection @{ ChangedSince = "base" } @("webview-reactor")

    Assert-Failure @{ Package = @("not-a-sample"); ChangedSince = "base" } `
        "unknown or excluded sample package(s)"
    Assert-Failure @{ Package = @("reactor-counter"); Family = @("webview") } `
        "no matching sample packages"

    $sampleTestState.Paths = @()
    Assert-Failure @{ ChangedSince = "base" } "no matching sample packages"
    $sampleTestState.ExitCode = 128
    Assert-Failure $options "git diff failed with exit code 128"
} finally {
    $global:LASTEXITCODE = $previousExitCode
}

Write-Host "Sample selection tests passed."
