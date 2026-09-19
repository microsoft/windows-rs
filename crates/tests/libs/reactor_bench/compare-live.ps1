param(
    [ValidateSet("grid", "list")]
    [string]$Surface = "grid",
    [ValidateSet("text", "rotate", "reverse", "churn")]
    [string]$Workload = "text",
    [int]$Count = 512,
    [int]$Updates = 120,
    [int]$ChurnCount = 64
)

$ErrorActionPreference = "Stop"

cargo build -p test-reactor-bench --bin reactor-live-compare --release --quiet
if ($LASTEXITCODE -ne 0) {
    throw "reactor-live-compare build failed"
}

$arguments = @(
    "--surface", $Surface,
    "--workload", $Workload,
    "--count", $Count,
    "--updates", $Updates,
    "--churn-count", $ChurnCount
)

& ".\target\release\reactor-live-compare.exe" --frontend reactor @arguments
if ($LASTEXITCODE -ne 0) {
    throw "Reactor live benchmark failed"
}

& ".\target\release\reactor-live-compare.exe" --frontend reactor2 @arguments
if ($LASTEXITCODE -ne 0) {
    throw "Reactor2 live benchmark failed"
}
