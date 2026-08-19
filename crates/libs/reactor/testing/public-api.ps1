param(
    [switch]$Update
)

$ErrorActionPreference = "Stop"
$PSNativeCommandUseErrorActionPreference = $true

function Test-Snapshot(
    [string]$Name,
    [string]$Snapshot,
    [string[]]$Arguments
) {
    $output = "target\reactor-public-api"
    New-Item -ItemType Directory -Force $output | Out-Null
    $actual = Join-Path $output "$Name.txt"
    & cargo public-api -p windows-reactor -ss --color never @Arguments |
        Set-Content -Path $actual -Encoding utf8
    if ($LASTEXITCODE -ne 0) {
        throw "failed to generate the $Name public API"
    }

    if ($Update) {
        Copy-Item -Path $actual -Destination $Snapshot -Force
        return
    }

    $difference = Compare-Object (Get-Content $Snapshot) (Get-Content $actual) -SyncWindow 0
    if ($null -ne $difference) {
        $difference | Format-Table -AutoSize | Out-String | Write-Error
        throw "$Name public API differs from $Snapshot"
    }
}

Test-Snapshot "default" "crates\libs\reactor\testing\public-api.txt" @()
Test-Snapshot "canvas" "crates\libs\reactor\testing\public-api-canvas.txt" @(
    "--features",
    "canvas"
)
Test-Snapshot "webview" "crates\libs\reactor\testing\public-api-webview.txt" @(
    "--features",
    "webview"
)
Test-Snapshot "all" "crates\libs\reactor\testing\public-api-all.txt" @("--all-features")
