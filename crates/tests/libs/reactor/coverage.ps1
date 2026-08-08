param(
    [string]$Path = "target\reactor-coverage.json"
)

$ErrorActionPreference = "Stop"
$report = Get-Content $Path -Raw | ConvertFrom-Json
$files = $report.data[0].files
$requirements = @(
    @{ Suffix = "reconciler.rs"; Branches = 62; Lines = 82 },
    @{ Suffix = "reconciler\child.rs"; Branches = 75; Lines = 90 },
    @{ Suffix = "reconciler\templated.rs"; Branches = 65; Lines = 90 },
    @{ Suffix = "reconciler\widget_dispatch.rs"; Branches = 55; Lines = 65 },
    @{ Suffix = "reconciler\wrappers.rs"; Branches = 55; Lines = 79 }
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
    throw "Reconciler coverage fell below its required floor."
}
