param(
    [string]$Path = "target\reactor-coverage.json"
)

$ErrorActionPreference = "Stop"
$report = Get-Content $Path -Raw | ConvertFrom-Json
$files = $report.data[0].files
$requirements = @(
    @{ Suffix = "src\test_support\adapter.rs"; Branches = 60; Lines = 87 },
    @{ Suffix = "src\test_support\component.rs"; Branches = 0; Lines = 85 },
    @{ Suffix = "src\test_support\reconcile.rs"; Branches = 60; Lines = 65 },
    @{ Suffix = "src\test_support\reference.rs"; Branches = 0; Lines = 100 },
    @{ Suffix = "src\component.rs"; Branches = 62; Lines = 85 },
    @{ Suffix = "src\declaration.rs"; Branches = 65; Lines = 75 },
    @{ Suffix = "src\ir.rs"; Branches = 70; Lines = 88 },
    @{ Suffix = "src\reconcile.rs"; Branches = 68; Lines = 85 },
    @{ Suffix = "src\reference.rs"; Branches = 64; Lines = 72 },
    @{ Suffix = "src\window.rs"; Branches = 70; Lines = 95 }
)

$failed = $false
$rows = foreach ($requirement in $requirements) {
    $file = $files | Where-Object { $_.filename.EndsWith($requirement.Suffix) }
    if ($null -eq $file) {
        throw "Coverage report does not contain $($requirement.Suffix)"
    }

    $branches = [double]$file.summary.branches.percent
    $lines = [double]$file.summary.lines.percent
    if ($branches -lt $requirement.Branches -or $lines -lt $requirement.Lines) {
        $failed = $true
    }

    [pscustomobject]@{
        File = $requirement.Suffix
        Branches = "{0:N2}%" -f $branches
        RequiredBranches = "$($requirement.Branches)%"
        Lines = "{0:N2}%" -f $lines
        RequiredLines = "$($requirement.Lines)%"
    }
}

$rows | Format-Table -AutoSize
if ($failed) {
    throw "Reactor coverage fell below its required floor."
}
