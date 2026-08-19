param(
    [ValidateSet("debug", "release")]
    [string]$Profile = "release"
)

$ErrorActionPreference = "Stop"
$repo = (Resolve-Path (Join-Path $PSScriptRoot "..\..\..")).Path
$cargoArguments = @(
    "build", "-p", "reactor_matched", "--quiet"
)
if ($Profile -eq "release") {
    $cargoArguments += "--release"
}

Add-Type -AssemblyName UIAutomationClient
Add-Type -AssemblyName UIAutomationTypes

function Wait-ForElement(
    [System.Windows.Automation.AutomationElement]$Root,
    [System.Windows.Automation.Condition]$Condition,
    [string]$Description
) {
    $deadline = [DateTime]::UtcNow.AddSeconds(10)
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

function Wait-ForNoElement(
    [System.Windows.Automation.AutomationElement]$Root,
    [System.Windows.Automation.Condition]$Condition,
    [string]$Description
) {
    $deadline = [DateTime]::UtcNow.AddSeconds(10)
    do {
        $element = $Root.FindFirst(
            [System.Windows.Automation.TreeScope]::Descendants,
            $Condition
        )
        if ($null -eq $element) {
            return
        }
        Start-Sleep -Milliseconds 50
    }
    while ([DateTime]::UtcNow -lt $deadline)
    throw "Timed out waiting for $Description"
}

function Name-Condition([string]$Name) {
    return New-Object System.Windows.Automation.PropertyCondition(
        [System.Windows.Automation.AutomationElement]::NameProperty,
        $Name
    )
}

function Control-Type-Condition(
    [System.Windows.Automation.ControlType]$ControlType
) {
    return New-Object System.Windows.Automation.PropertyCondition(
        [System.Windows.Automation.AutomationElement]::ControlTypeProperty,
        $ControlType
    )
}

function Realized-Row-Count([System.Windows.Automation.AutomationElement]$List) {
    $elements = $List.FindAll(
        [System.Windows.Automation.TreeScope]::Descendants,
        [System.Windows.Automation.Condition]::TrueCondition
    )
    $count = 0
    foreach ($element in $elements) {
        if ($element.Current.Name -like "Declarative row *") {
            $count++
        }
    }
    return $count
}

function Wait-ForRealizedRows(
    [System.Windows.Automation.AutomationElement]$List,
    [int]$LogicalCount,
    [string]$Description
) {
    $deadline = [DateTime]::UtcNow.AddSeconds(10)
    do {
        $count = Realized-Row-Count $List
        if ($count -gt 0 -and $count -lt $LogicalCount) {
            return $count
        }
        Start-Sleep -Milliseconds 50
    }
    while ([DateTime]::UtcNow -lt $deadline)
    throw "$Description realized $count of $LogicalCount rows"
}

function Invoke-MatchedScenario([string]$Name, [string]$Executable) {
    $process = Start-Process -FilePath $Executable -PassThru
    try {
        $window = Wait-ForElement `
            ([System.Windows.Automation.AutomationElement]::RootElement) `
            (Name-Condition "windows-reactor matched workload") `
            "$Name window"
        $list = Wait-ForElement `
            $window `
            (Control-Type-Condition ([System.Windows.Automation.ControlType]::List)) `
            "$Name virtual list"
        $button = Wait-ForElement `
            $window `
            (Name-Condition "Toggle row count") `
            "$Name toggle button"

        $initial = Wait-ForElement `
            $window `
            (Name-Condition "Declarative rows: 5000") `
            "$Name initial row count"
        if ($null -eq $initial) {
            throw "$Name did not report 5,000 rows"
        }
        $null = Wait-ForRealizedRows $list 5000 $Name

        $button.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        $expanded = Wait-ForElement `
            $window `
            (Name-Condition "Declarative rows: 10000") `
            "$Name expanded row count"
        if ($null -eq $expanded) {
            throw "$Name did not report 10,000 rows"
        }
        $null = Wait-ForRealizedRows $list 10000 $Name

        $button.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        $null = Wait-ForElement `
            $window `
            (Name-Condition "Declarative rows: 5000") `
            "$Name restored row count"

        $increment = Wait-ForElement `
            $window `
            (Name-Condition "Increment Beta") `
            "$Name Beta increment button"
        $increment.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        $null = Wait-ForElement `
            $window `
            (Name-Condition "Beta: 1") `
            "$Name updated Beta row"

        $rotate = Wait-ForElement `
            $window `
            (Name-Condition "Rotate rows") `
            "$Name rotate button"
        $rotate.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        $null = Wait-ForElement `
            $window `
            (Name-Condition "Order: Beta, Gamma, Alpha") `
            "$Name rotated row order"
        $null = Wait-ForElement `
            $window `
            (Name-Condition "Beta: 1") `
            "$Name preserved Beta state"

        $styled = Wait-ForElement `
            $window `
            (Name-Condition "Matched styled panel") `
            "$Name styled panel"
        if ($styled.Current.HelpText -ne "Styled panel help") {
            throw "$Name styled panel exposed help text '$($styled.Current.HelpText)'"
        }
        $bounds = $styled.Current.BoundingRectangle
        if ($bounds.Width -lt 200 -or $bounds.Height -lt 50) {
            throw "$Name styled panel bounds were $($bounds.Width) x $($bounds.Height)"
        }

        $input = Wait-ForElement `
            $window `
            (Name-Condition "Matched text input") `
            "$Name text input"
        $input.GetCurrentPattern(
            [System.Windows.Automation.ValuePattern]::Pattern
        ).SetValue("matched input")
        $null = Wait-ForElement `
            $window `
            (Name-Condition "Input: matched input") `
            "$Name controlled text status"

        $toggle = Wait-ForElement `
            $window `
            (Name-Condition "Matched toggle") `
            "$Name toggle"
        $toggle.GetCurrentPattern(
            [System.Windows.Automation.TogglePattern]::Pattern
        ).Toggle()
        $null = Wait-ForElement `
            $window `
            (Name-Condition "Toggle: true") `
            "$Name controlled toggle status"

        $choice = Wait-ForElement `
            $window `
            (Name-Condition "Beta choice") `
            "$Name Beta choice"
        $choice.GetCurrentPattern(
            [System.Windows.Automation.SelectionItemPattern]::Pattern
        ).Select()
        $null = Wait-ForElement `
            $window `
            (Name-Condition "Selection: Beta choice") `
            "$Name controlled selection status"

        $openSecondary = Wait-ForElement `
            $window `
            (Name-Condition "Open matched secondary") `
            "$Name secondary-window button"
        $openSecondary.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        $null = Wait-ForElement `
            $window `
            (Name-Condition "Secondary windows opened: 1") `
            "$Name secondary-window state"
        $secondary = Wait-ForElement `
            ([System.Windows.Automation.AutomationElement]::RootElement) `
            (Name-Condition "windows-reactor matched secondary") `
            "$Name secondary window"
        $secondaryIncrement = Wait-ForElement `
            ([System.Windows.Automation.AutomationElement]::RootElement) `
            (Name-Condition "Increment secondary") `
            "$Name secondary increment button"
        $secondaryIncrement.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        $null = Wait-ForElement `
            ([System.Windows.Automation.AutomationElement]::RootElement) `
            (Name-Condition "Secondary count: 1") `
            "$Name secondary state update"
        $secondary.GetCurrentPattern(
            [System.Windows.Automation.WindowPattern]::Pattern
        ).Close()
        Wait-ForNoElement `
            ([System.Windows.Automation.AutomationElement]::RootElement) `
            (Name-Condition "windows-reactor matched secondary") `
            "$Name secondary window close"
        $null = Wait-ForElement `
            $window `
            (Name-Condition "Secondary windows opened: 1") `
            "$Name surviving main window"

        $command = Wait-ForElement `
            $window `
            (Name-Condition "Matched command") `
            "$Name command"
        $command.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        $null = Wait-ForElement `
            $window `
            (Name-Condition "Command: Matched command") `
            "$Name command result"

        $openDialog = Wait-ForElement `
            $window `
            (Name-Condition "Open matched dialog") `
            "$Name dialog button"
        $openDialog.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        $dialog = Wait-ForElement `
            ([System.Windows.Automation.AutomationElement]::RootElement) `
            (Name-Condition "Matched dialog") `
            "$Name dialog"
        $accept = Wait-ForElement `
            $dialog `
            (Name-Condition "Accept matched") `
            "$Name dialog primary button"
        $accept.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        $null = Wait-ForElement `
            $window `
            (Name-Condition "Dialog: primary") `
            "$Name dialog result"

        $image = Wait-ForElement `
            $window `
            (Name-Condition "Matched image") `
            "$Name image"
        $imageBounds = $image.Current.BoundingRectangle
        if ($imageBounds.Width -lt 40 -or $imageBounds.Height -lt 40) {
            throw "$Name image bounds were $($imageBounds.Width) x $($imageBounds.Height)"
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
        throw "Could not build the matched Reactor applications"
    }
    $directory = Join-Path $repo "target\$Profile"
    Invoke-MatchedScenario "Reactor" (Join-Path $directory "reactor_matched.exe")
    & (Join-Path $PSScriptRoot "matched-hooks.ps1") -Profile $Profile
    & (Join-Path $PSScriptRoot "matched-canvas.ps1") -Profile $Profile
    Write-Output "matched public acceptance scenarios passed"
}
finally {
    Pop-Location
}
