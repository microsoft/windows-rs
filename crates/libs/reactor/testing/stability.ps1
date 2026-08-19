param(
    [ValidateRange(1, 20)]
    [int]$Repetitions = 5
)

$ErrorActionPreference = "Stop"
$PSNativeCommandUseErrorActionPreference = $true

$repo = (Resolve-Path (Join-Path $PSScriptRoot "..\..\..\..")).Path
$native = Join-Path $repo "crates\tests\libs\reactor_selftest\native.ps1"

function Invoke-CargoTest {
    param([string[]]$Arguments)

    & cargo test @Arguments
    if ($LASTEXITCODE -ne 0) {
        throw "cargo test failed: $($Arguments -join ' ')"
    }
}

Push-Location $repo
try {
    Invoke-CargoTest @(
        "-p", "windows-reactor", "--all-features", "winui::tests::", "--",
        "--ignored", "--test-threads=1"
    )
    Invoke-CargoTest @(
        "-p", "windows-reactor", "--release", "--all-features", "winui::tests::", "--",
        "--ignored", "--test-threads=1"
    )

    for ($iteration = 1; $iteration -le $Repetitions; $iteration++) {
        Write-Output "windows-reactor stability repetition $iteration of $Repetitions"

        Invoke-CargoTest @(
            "-p", "windows-reactor", "--release", "--all-features",
            "winui::tests::hooks::", "--", "--ignored", "--test-threads=1"
        )
        Invoke-CargoTest @(
            "-p", "windows-reactor", "--release", "--all-features",
            "winui::tests::performance::dispatcher_and_composition_callbacks_run", "--",
            "--ignored", "--test-threads=1"
        )
        & $native -Profile release -Case input
        Invoke-CargoTest @(
            "-p", "windows-reactor", "--release", "--all-features",
            "winui::tests::window_lifecycle::", "--", "--ignored", "--test-threads=1"
        )
        Invoke-CargoTest @(
            "-p", "windows-reactor", "--release", "--all-features",
            "winui::tests::stress_performance::", "--", "--ignored", "--test-threads=1"
        )
    }
    Write-Output "windows-reactor stability matrix passed"
}
finally {
    Pop-Location
}
