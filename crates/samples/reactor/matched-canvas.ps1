param(
    [ValidateSet("debug", "release")]
    [string]$Profile = "release"
)

$ErrorActionPreference = "Stop"
$repo = (Resolve-Path (Join-Path $PSScriptRoot "..\..\..")).Path
$cargoArguments = @(
    "build", "-p", "reactor_matched_canvas", "--quiet"
)
if ($Profile -eq "release") {
    $cargoArguments += "--release"
}

Add-Type -AssemblyName UIAutomationClient
Add-Type -AssemblyName UIAutomationTypes

function Property-Condition($Property, $Value) {
    return New-Object System.Windows.Automation.PropertyCondition($Property, $Value)
}

function Wait-ForElement(
    [System.Windows.Automation.AutomationElement]$Root,
    [System.Windows.Automation.Condition]$Condition,
    [string]$Description
) {
    $deadline = [DateTime]::UtcNow.AddSeconds(15)
    do {
        $element = $Root.FindFirst(
            [System.Windows.Automation.TreeScope]::Descendants,
            $Condition
        )
        if ($null -ne $element) {
            return $element
        }
        Start-Sleep -Milliseconds 50
    }
    while ([DateTime]::UtcNow -lt $deadline)
    throw "Timed out waiting for $Description"
}

function Find-Frame-Count([System.Windows.Automation.AutomationElement]$Window) {
    $status = Wait-ForElement `
        $Window `
        (Property-Condition `
            ([System.Windows.Automation.AutomationElement]::AutomationIdProperty) `
            "canvas-frame-status") `
        "canvas frame status"
    if ($status.Current.Name -notmatch "^Canvas frames: ([0-9]+)$") {
        throw "Unexpected canvas frame status '$($status.Current.Name)'"
    }
    return [int]$Matches[1]
}

function Invoke-Button(
    [System.Windows.Automation.AutomationElement]$Window,
    [string]$Name
) {
    $button = Wait-ForElement `
        $Window `
        (Property-Condition `
            ([System.Windows.Automation.AutomationElement]::NameProperty) `
            $Name) `
        "$Name button"
    $button.GetCurrentPattern(
        [System.Windows.Automation.InvokePattern]::Pattern
    ).Invoke()
}

function Invoke-MatchedCanvas([string]$Name, [string]$Executable) {
    $process = Start-Process -FilePath $Executable -PassThru
    try {
        $window = Wait-ForElement `
            ([System.Windows.Automation.AutomationElement]::RootElement) `
            (Property-Condition `
                ([System.Windows.Automation.AutomationElement]::NameProperty) `
                "windows-reactor matched canvas") `
            "$Name canvas window"
        $surface = Wait-ForElement `
            $window `
            (Property-Condition `
                ([System.Windows.Automation.AutomationElement]::NameProperty) `
                "Matched drawing surface") `
            "$Name drawing surface"
        if ($surface.Current.BoundingRectangle.Width -lt 250) {
            throw "$Name drawing surface was not realized"
        }

        Start-Sleep -Milliseconds 300
        Invoke-Button $window "Refresh canvas status"
        $first = Find-Frame-Count $window
        if ($first -le 0) {
            throw "$Name did not paint its initial canvas frame"
        }

        Invoke-Button $window "Invalidate canvas"
        $null = Wait-ForElement `
            $window `
            (Property-Condition `
                ([System.Windows.Automation.AutomationElement]::NameProperty) `
                "Circle count: 6") `
            "$Name updated drawing state"
        Start-Sleep -Milliseconds 300
        Invoke-Button $window "Refresh canvas status"
        $second = Find-Frame-Count $window
        if ($second -le $first) {
            throw "$Name did not repaint after invalidation ($first -> $second)"
        }
    }
    finally {
        if (!$process.HasExited) {
            $null = $process.CloseMainWindow()
            if (!$process.WaitForExit(5000)) {
                Stop-Process -Id $process.Id
                $process.WaitForExit()
            }
        }
    }
}

Push-Location $repo
try {
    & cargo @cargoArguments
    if ($LASTEXITCODE -ne 0) {
        throw "Could not build the matched Canvas applications"
    }
    $directory = Join-Path $repo "target\$Profile"
    Invoke-MatchedCanvas "Reactor" (Join-Path $directory "reactor_matched_canvas.exe")
    Write-Output "matched Canvas acceptance passed"
}
finally {
    Pop-Location
}
