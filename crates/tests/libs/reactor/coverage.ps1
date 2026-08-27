param(
    [string]$Path = "target\reactor-coverage.json"
)

$ErrorActionPreference = "Stop"
$report = Get-Content $Path -Raw | ConvertFrom-Json
$files = $report.data[0].files
$requirements = @(
    @{ Suffix = "core\component.rs"; Branches = 65; Lines = 89 },
    @{ Suffix = "core\engine.rs"; Branches = 65; Lines = 93 },
    @{ Suffix = "core\pump\lifecycle.rs"; Branches = 55; Lines = 91 },
    @{ Suffix = "core\pump\mod.rs"; Branches = 78; Lines = 92 },
    @{ Suffix = "core\pump\native_work.rs"; Branches = 65; Lines = 80 },
    @{ Suffix = "core\pump\plan.rs"; Branches = 64; Lines = 98 },
    @{ Suffix = "core\pump\planner\element.rs"; Branches = 70; Lines = 88 },
    @{ Suffix = "core\pump\planner\topology.rs"; Branches = 64; Lines = 80 },
    @{ Suffix = "core\pump\planner\view.rs"; Branches = 68; Lines = 83 },
    @{ Suffix = "core\pump\publish.rs"; Branches = 82; Lines = 91 },
    @{ Suffix = "core\pump\turn.rs"; Branches = 54; Lines = 80 },
    @{ Suffix = "core\scheduler.rs"; Branches = 62; Lines = 87 },
    @{ Suffix = "core\virtual_model.rs"; Branches = 70; Lines = 97 },
    @{ Suffix = "native\recording.rs"; Branches = 63; Lines = 91 },
    @{ Suffix = "src\reference.rs"; Branches = 65; Lines = 87 }
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
