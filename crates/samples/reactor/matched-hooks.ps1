param(
    [ValidateSet("debug", "release")]
    [string]$Profile = "release"
)

$ErrorActionPreference = "Stop"
$repo = (Resolve-Path (Join-Path $PSScriptRoot "..\..\..")).Path
$cargoArguments = @(
    "build", "-p", "reactor_matched_hooks", "--quiet"
)
if ($Profile -eq "release") {
    $cargoArguments += "--release"
}

Add-Type -AssemblyName UIAutomationClient
Add-Type -AssemblyName UIAutomationTypes

function Name-Condition([string]$Name) {
    return New-Object System.Windows.Automation.PropertyCondition(
        [System.Windows.Automation.AutomationElement]::NameProperty,
        $Name
    )
}

function Wait-ForElement(
    [System.Windows.Automation.AutomationElement]$Root,
    [string]$Name,
    [string]$Description
) {
    $deadline = [DateTime]::UtcNow.AddSeconds(10)
    $condition = Name-Condition $Name
    do {
        $element = $Root.FindFirst(
            [System.Windows.Automation.TreeScope]::Descendants,
            $condition
        )
        if ($null -ne $element) {
            return $element
        }
        Start-Sleep -Milliseconds 50
    }
    while ([DateTime]::UtcNow -lt $deadline)
    throw "Timed out waiting for $Description"
}

function Wait-ForNoElement(
    [System.Windows.Automation.AutomationElement]$Root,
    [string]$Name,
    [string]$Description
) {
    $deadline = [DateTime]::UtcNow.AddSeconds(10)
    $condition = Name-Condition $Name
    do {
        if ($null -eq $Root.FindFirst(
            [System.Windows.Automation.TreeScope]::Descendants,
            $condition
        )) {
            return
        }
        Start-Sleep -Milliseconds 50
    }
    while ([DateTime]::UtcNow -lt $deadline)
    throw "Timed out waiting for $Description"
}

function Invoke-Button(
    [System.Windows.Automation.AutomationElement]$Window,
    [string]$Name,
    [string]$Description
) {
    $button = Wait-ForElement $Window $Name $Description
    $button.GetCurrentPattern(
        [System.Windows.Automation.InvokePattern]::Pattern
    ).Invoke()
}

function Invoke-MatchedHooks([string]$Name, [string]$Executable) {
    $process = Start-Process -FilePath $Executable -PassThru
    try {
        $window = Wait-ForElement `
            ([System.Windows.Automation.AutomationElement]::RootElement) `
            "windows-reactor matched hooks" `
            "$Name hooks window"

        $null = Wait-ForElement $window "Resource: ready value 0" "$Name initial resource"
        Invoke-Button $window "Refresh hook status" "$Name hook refresh"
        $null = Wait-ForElement $window "Effects: 1" "$Name initial effect"

        Invoke-Button $window "Change effect dependency" "$Name dependency change"
        $null = Wait-ForElement $window "Effect mounted: 1" "$Name changed effect"
        Invoke-Button $window "Refresh hook status" "$Name cleanup refresh"
        $null = Wait-ForElement $window "Effects: 2" "$Name second effect"
        $null = Wait-ForElement $window "Cleanups: 1" "$Name dependency cleanup"

        Invoke-Button $window "Remove effect" "$Name effect removal"
        Wait-ForNoElement $window "Effect mounted: 1" "$Name effect unmount"
        Invoke-Button $window "Refresh hook status" "$Name unmount refresh"
        $null = Wait-ForElement $window "Cleanups: 2" "$Name unmount cleanup"

        Invoke-Button $window "Advance resource" "$Name first resource advance"
        $null = Wait-ForElement $window "Resource key: 1" "$Name slow resource"
        Invoke-Button $window "Advance resource" "$Name second resource advance"
        $null = Wait-ForElement $window "Resource key: 2" "$Name failed resource key"
        $null = Wait-ForElement $window "Resource: error 2" "$Name resource failure"
        Start-Sleep -Milliseconds 900
        $null = Wait-ForElement `
            $window `
            "Resource: error 2" `
            "$Name stale resource suppression"

        Invoke-Button $window "Advance resource" "$Name recovery advance"
        $null = Wait-ForElement $window "Resource key: 3" "$Name recovery key"
        $null = Wait-ForElement $window "Resource: ready value 3" "$Name resource recovery"
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
        throw "Could not build the matched hooks applications"
    }
    $directory = Join-Path $repo "target\$Profile"
    Invoke-MatchedHooks "Reactor" (Join-Path $directory "reactor_matched_hooks.exe")
    Write-Output "matched hooks and resources acceptance passed"
}
finally {
    Pop-Location
}
