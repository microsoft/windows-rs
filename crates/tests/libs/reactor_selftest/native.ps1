param(
    [ValidateSet("debug", "release")]
    [string]$Profile = "debug",
    [ValidateSet("all", "smoke", "collections", "grid", "list-box", "combo-box", "radio-buttons", "selector-bar", "breadcrumb-bar", "auto-suggest-box", "teaching-tip", "flyout", "content-dialog", "command-bar", "media", "canvas", "canvas-image", "animated-canvas", "multi-window", "input", "values", "failure")]
    [string]$Case = "all"
)

$ErrorActionPreference = "Stop"

if ($Case -eq "all") {
    foreach ($name in @("smoke", "collections", "grid", "list-box", "combo-box", "radio-buttons", "selector-bar", "breadcrumb-bar", "auto-suggest-box", "teaching-tip", "flyout", "content-dialog", "command-bar", "media", "canvas", "canvas-image", "animated-canvas", "multi-window", "input", "values", "failure")) {
        & $PSCommandPath -Profile $Profile -Case $name
    }
    Write-Output "windows-reactor native self-test passed"
    return
}

Add-Type -AssemblyName UIAutomationClient
Add-Type -AssemblyName UIAutomationTypes
Add-Type -AssemblyName System.Windows.Forms
$references = @(
    "System.Windows.Forms"
    "System.Drawing.Common"
    "System.Drawing.Primitives"
    "System.Threading.Thread"
    "System.Threading"
    "System.ComponentModel.Primitives"
)
Add-Type `
    -ReferencedAssemblies $references `
    -TypeDefinition @"
using System.Runtime.InteropServices;
using System.Threading;
using System.Windows.Forms;

public static class MouseInput {
    [DllImport("user32.dll")]
    public static extern bool SetCursorPos(int x, int y);

    [DllImport("user32.dll")]
    public static extern void mouse_event(
        uint flags,
        uint dx,
        uint dy,
        uint data,
        System.UIntPtr extraInfo
    );
}

public static class DragSource {
    static Form form;
    static Thread thread;
    static readonly ManualResetEvent ready = new ManualResetEvent(false);

    public static int X { get; private set; }
    public static int Y { get; private set; }
    public static int Attempts { get; private set; }
    public static string LastEffect { get; private set; } = "not started";

    public static void Start() {
        ready.Reset();
        X = 0;
        Y = 0;
        Attempts = 0;
        LastEffect = "not started";
        thread = new Thread(() => {
            form = new Form {
                Text = "reactor drag source",
                StartPosition = FormStartPosition.Manual,
                Left = 40,
                Top = 40,
                Width = 240,
                Height = 120,
                TopMost = true
            };
            var label = new Label {
                Dock = DockStyle.Fill,
                Text = "Drag reactor text",
                TextAlign = System.Drawing.ContentAlignment.MiddleCenter
            };
            label.MouseDown += (sender, args) => {
                Attempts++;
                LastEffect = label
                    .DoDragDrop("reactor drop text", DragDropEffects.Copy)
                    .ToString();
            };
            form.Controls.Add(label);
            form.Shown += (sender, args) => {
                var point = label.PointToScreen(
                    new System.Drawing.Point(label.Width / 2, label.Height / 2)
                );
                X = point.X;
                Y = point.Y;
                ready.Set();
            };
            Application.Run(form);
        });
        thread.SetApartmentState(ApartmentState.STA);
        thread.Start();
        if (!ready.WaitOne(10000)) {
            throw new System.InvalidOperationException("drag source did not open");
        }
    }

    public static void Stop() {
        if (form != null && !form.IsDisposed) {
            form.BeginInvoke((MethodInvoker)(() => form.Close()));
        }
        if (thread != null) {
            thread.Join(5000);
        }
    }
}
"@

$root = Resolve-Path (Join-Path $PSScriptRoot "..\..\..\..")
$cargoArgs = @("build", "-p", "test_reactor_selftest", "--quiet")
if ($Profile -eq "release") {
    $cargoArgs += "--release"
}
$cargoArgs += @("--config", "profile.$Profile.panic=`"abort`"")
& cargo @cargoArgs
if ($LASTEXITCODE -ne 0) {
    throw "failed to build test_reactor_selftest"
}

$executable = Join-Path $root "target\$Profile\test_reactor_selftest.exe"
$artifactDirectory = Split-Path $executable
Copy-Item (
    Join-Path $PSScriptRoot "..\canvas\test.png"
) (Join-Path $artifactDirectory "reactor-native-media.png") -Force
Copy-Item (
    Join-Path $PSScriptRoot "media.svg"
) (Join-Path $artifactDirectory "reactor-native-media.svg") -Force
$buttonCondition = New-Object System.Windows.Automation.PropertyCondition(
    [System.Windows.Automation.AutomationElement]::ControlTypeProperty,
    [System.Windows.Automation.ControlType]::Button
)
$listCondition = New-Object System.Windows.Automation.PropertyCondition(
    [System.Windows.Automation.AutomationElement]::ControlTypeProperty,
    [System.Windows.Automation.ControlType]::List
)
$itemCondition = New-Object System.Windows.Automation.PropertyCondition(
    [System.Windows.Automation.AutomationElement]::ControlTypeProperty,
    [System.Windows.Automation.ControlType]::ListItem
)
$dataItemCondition = New-Object System.Windows.Automation.PropertyCondition(
    [System.Windows.Automation.AutomationElement]::ControlTypeProperty,
    [System.Windows.Automation.ControlType]::DataItem
)
$gridCondition = New-Object System.Windows.Automation.AndCondition(
    $listCondition,
    (New-Object System.Windows.Automation.PropertyCondition(
        [System.Windows.Automation.AutomationElement]::NameProperty,
        "Virtual tiles"
    ))
)
$listBoxCondition = New-Object System.Windows.Automation.AndCondition(
    $listCondition,
    (New-Object System.Windows.Automation.PropertyCondition(
        [System.Windows.Automation.AutomationElement]::NameProperty,
        "Keyed choices"
    ))
)
$comboBoxCondition = New-Object System.Windows.Automation.AndCondition(
    (New-Object System.Windows.Automation.PropertyCondition(
        [System.Windows.Automation.AutomationElement]::ControlTypeProperty,
        [System.Windows.Automation.ControlType]::ComboBox
    )),
    (New-Object System.Windows.Automation.PropertyCondition(
        [System.Windows.Automation.AutomationElement]::NameProperty,
        "Keyed combo choices"
    ))
)
$radioButtonsCondition = New-Object System.Windows.Automation.PropertyCondition(
    [System.Windows.Automation.AutomationElement]::NameProperty,
    "Keyed radio choices"
)
$radioButtonCondition = New-Object System.Windows.Automation.PropertyCondition(
    [System.Windows.Automation.AutomationElement]::ControlTypeProperty,
    [System.Windows.Automation.ControlType]::RadioButton
)
$selectorBarCondition = New-Object System.Windows.Automation.PropertyCondition(
    [System.Windows.Automation.AutomationElement]::NameProperty,
    "Keyed selector bar"
)
$breadcrumbBarCondition = New-Object System.Windows.Automation.PropertyCondition(
    [System.Windows.Automation.AutomationElement]::NameProperty,
    "Keyed breadcrumb bar"
)
$autoSuggestBoxCondition = New-Object System.Windows.Automation.PropertyCondition(
    [System.Windows.Automation.AutomationElement]::NameProperty,
    "Keyed auto suggest box"
)
$editCondition = New-Object System.Windows.Automation.PropertyCondition(
    [System.Windows.Automation.AutomationElement]::ControlTypeProperty,
    [System.Windows.Automation.ControlType]::Edit
)
$textBoxCondition = New-Object System.Windows.Automation.AndCondition(
    $editCondition,
    (New-Object System.Windows.Automation.PropertyCondition(
        [System.Windows.Automation.AutomationElement]::NameProperty,
        "Text input"
    ))
)
$passwordCondition = New-Object System.Windows.Automation.AndCondition(
    $editCondition,
    (New-Object System.Windows.Automation.PropertyCondition(
        [System.Windows.Automation.AutomationElement]::NameProperty,
        "Password input"
    ))
)
$sliderCondition = New-Object System.Windows.Automation.AndCondition(
    (New-Object System.Windows.Automation.PropertyCondition(
        [System.Windows.Automation.AutomationElement]::ControlTypeProperty,
        [System.Windows.Automation.ControlType]::Slider
    )),
    (New-Object System.Windows.Automation.PropertyCondition(
        [System.Windows.Automation.AutomationElement]::NameProperty,
        "Native slider"
    ))
)
$numberBoxCondition = New-Object System.Windows.Automation.AndCondition(
    (New-Object System.Windows.Automation.PropertyCondition(
        [System.Windows.Automation.AutomationElement]::ControlTypeProperty,
        [System.Windows.Automation.ControlType]::Spinner
    )),
    (New-Object System.Windows.Automation.PropertyCondition(
        [System.Windows.Automation.AutomationElement]::NameProperty,
        "Native number box"
    ))
)
$ratingCondition = New-Object System.Windows.Automation.PropertyCondition(
    [System.Windows.Automation.AutomationElement]::NameProperty,
    "Native rating"
)
$colorPickerCondition = New-Object System.Windows.Automation.PropertyCondition(
    [System.Windows.Automation.AutomationElement]::NameProperty,
    "Native color picker"
)
$datePickerCondition = New-Object System.Windows.Automation.PropertyCondition(
    [System.Windows.Automation.AutomationElement]::NameProperty,
    "Native date picker"
)
$calendarDatePickerCondition = New-Object System.Windows.Automation.PropertyCondition(
    [System.Windows.Automation.AutomationElement]::NameProperty,
    "Native calendar date picker"
)
$checkBoxCondition = New-Object System.Windows.Automation.PropertyCondition(
    [System.Windows.Automation.AutomationElement]::ControlTypeProperty,
    [System.Windows.Automation.ControlType]::CheckBox
)

function Start-Host([string[]]$Arguments, [string]$ErrorPath = "") {
    $start = @{
        FilePath = $executable
        PassThru = $true
    }
    if ($Arguments.Count -ne 0) {
        $start.ArgumentList = $Arguments
    }
    if ($ErrorPath) {
        $start.RedirectStandardError = $ErrorPath
    }
    $process = Start-Process @start
    $null = $process.Handle
    $deadline = (Get-Date).AddSeconds(20)
    do {
        Start-Sleep -Milliseconds 20
        $process.Refresh()
        $identityCondition = New-Object System.Windows.Automation.AndCondition(
            (New-Object System.Windows.Automation.PropertyCondition(
                [System.Windows.Automation.AutomationElement]::ProcessIdProperty,
                $process.Id
            )),
            (New-Object System.Windows.Automation.PropertyCondition(
                [System.Windows.Automation.AutomationElement]::NameProperty,
                "windows-reactor native self-test"
            ))
        )
        $windowCondition = New-Object System.Windows.Automation.AndCondition(
            $identityCondition,
            (New-Object System.Windows.Automation.PropertyCondition(
                [System.Windows.Automation.AutomationElement]::ControlTypeProperty,
                [System.Windows.Automation.ControlType]::Window
            ))
        )
        $window = [System.Windows.Automation.AutomationElement]::RootElement.FindFirst(
            [System.Windows.Automation.TreeScope]::Descendants,
            $windowCondition
        )
    } until ($null -ne $window -or $process.HasExited -or (Get-Date) -gt $deadline)

    if ($null -eq $window) {
        throw "native self-test window did not open"
    }

    [pscustomobject]@{
        Process = $process
        Root = $window
    }
}

function Stop-Host($hostRun) {
    $process = $hostRun.Process
    if (!$process.HasExited) {
        try {
            if ($Case -eq "smoke") {
                $null = Wait-Text $hostRun.Process "Rows: 5000"
                $null = Wait-Text $hostRun.Process "Text value: initial"
                $null = Wait-Text $hostRun.Process "Checked value: false"
            }

            $window = $hostRun.Root.GetCurrentPattern(
                [System.Windows.Automation.WindowPattern]::Pattern
            )
            $window.Close()
        } catch {
            $null = $process.CloseMainWindow()
        }
        if (!$process.WaitForExit(5000)) {
            Stop-Process -Id $process.Id
        }
    }
    $process.WaitForExit()
    $process.Refresh()
}

function Find-Text($scope, [string]$name) {
    $condition = New-Object System.Windows.Automation.PropertyCondition(
        [System.Windows.Automation.AutomationElement]::NameProperty,
        $name
    )
    $scope.FindFirst([System.Windows.Automation.TreeScope]::Descendants, $condition)
}

function Wait-Text($process, [string]$name) {
    $nameCondition = New-Object System.Windows.Automation.PropertyCondition(
        [System.Windows.Automation.AutomationElement]::NameProperty,
        $name
    )
    $condition = New-Object System.Windows.Automation.AndCondition(
        (New-Object System.Windows.Automation.PropertyCondition(
            [System.Windows.Automation.AutomationElement]::ProcessIdProperty,
            $process.Id
        )),
        $nameCondition
    )
    $deadline = (Get-Date).AddSeconds(10)
    do {
        $element = [System.Windows.Automation.AutomationElement]::RootElement.FindFirst(
            [System.Windows.Automation.TreeScope]::Descendants,
            $condition
        )
        if ($null -ne $element) {
            return $element
        }
        Start-Sleep -Milliseconds 20
    } until ((Get-Date) -gt $deadline)
    throw "text '$name' did not appear"
}

function Wait-HelpText($control, [string]$value) {
    $deadline = (Get-Date).AddSeconds(3)
    do {
        if ($control.Current.HelpText -eq $value) {
            return
        }
        Start-Sleep -Milliseconds 10
    } until ((Get-Date) -gt $deadline)
    throw "help text '$value' did not appear; current value is '$($control.Current.HelpText)'"
}

function Wait-StableHelpText($control) {
    $deadline = (Get-Date).AddSeconds(5)
    $value = $control.Current.HelpText
    $stableSince = Get-Date
    do {
        Start-Sleep -Milliseconds 20
        $current = $control.Current.HelpText
        if ($current -ne $value) {
            $value = $current
            $stableSince = Get-Date
        } elseif (((Get-Date) - $stableSince).TotalMilliseconds -ge 500) {
            return $value
        }
    } until ((Get-Date) -gt $deadline)
    throw "help text did not become stable; current value is '$value'"
}

function Get-ListItems($root) {
    $items = [System.Collections.Generic.List[System.Windows.Automation.AutomationElement]]::new()
    $stack = [System.Collections.Stack]::new()
    $stack.Push($root)
    $walker = [System.Windows.Automation.TreeWalker]::ControlViewWalker
    $visited = 0
    while ($stack.Count -ne 0) {
        $parent = $stack.Pop()
        $child = $walker.GetFirstChild($parent)
        while ($null -ne $child) {
            $visited++
            if ($visited -gt 512) {
                throw "ListBox control view exceeded the bounded traversal limit"
            }
            if ($child.Current.ControlType -eq [System.Windows.Automation.ControlType]::ListItem) {
                $items.Add($child)
            } else {
                $stack.Push($child)
            }
            $child = $walker.GetNextSibling($child)
        }
    }
    $items.ToArray()
}

function Wait-Text-Optional($process, [string]$name, [int]$seconds) {
    $condition = New-Object System.Windows.Automation.AndCondition(
        (New-Object System.Windows.Automation.PropertyCondition(
            [System.Windows.Automation.AutomationElement]::ProcessIdProperty,
            $process.Id
        )),
        (New-Object System.Windows.Automation.PropertyCondition(
            [System.Windows.Automation.AutomationElement]::NameProperty,
            $name
        ))
    )
    $deadline = (Get-Date).AddSeconds($seconds)
    do {
        try {
            $element = [System.Windows.Automation.AutomationElement]::RootElement.FindFirst(
                [System.Windows.Automation.TreeScope]::Descendants,
                $condition
            )
        } catch {
            $element = $null
        }
        if ($null -ne $element) {
            return $element
        }
        Start-Sleep -Milliseconds 20
    } until ((Get-Date) -gt $deadline)
    return $null
}

function Wait-Text-Hidden($process, [string]$name, [string]$description) {
    $condition = New-Object System.Windows.Automation.AndCondition(
        (New-Object System.Windows.Automation.PropertyCondition(
            [System.Windows.Automation.AutomationElement]::ProcessIdProperty,
            $process.Id
        )),
        (New-Object System.Windows.Automation.PropertyCondition(
            [System.Windows.Automation.AutomationElement]::NameProperty,
            $name
        ))
    )
    $deadline = (Get-Date).AddSeconds(3)
    do {
        $element = [System.Windows.Automation.AutomationElement]::RootElement.FindFirst(
            [System.Windows.Automation.TreeScope]::Descendants,
            $condition
        )
        if ($null -eq $element -or $element.Current.IsOffscreen) {
            return
        }
        Start-Sleep -Milliseconds 20
    } until ((Get-Date) -gt $deadline)
    throw "$description remained visible"
}

function Wait-Control($process, $controlCondition, [string]$description) {
    $condition = New-Object System.Windows.Automation.AndCondition(
        (New-Object System.Windows.Automation.PropertyCondition(
            [System.Windows.Automation.AutomationElement]::ProcessIdProperty,
            $process.Id
        )),
        $controlCondition
    )
    $deadline = (Get-Date).AddSeconds(10)
    do {
        $element = [System.Windows.Automation.AutomationElement]::RootElement.FindFirst(
            [System.Windows.Automation.TreeScope]::Descendants,
            $condition
        )
        if ($null -ne $element) {
            return $element
        }
        Start-Sleep -Milliseconds 20
    } until ((Get-Date) -gt $deadline)
    if ($process.HasExited) {
        $message = if ($script:activeError -and (Test-Path $script:activeError)) {
            Get-Content $script:activeError -Raw
        } else {
            ""
        }
        throw "$description did not appear; process exited with $($process.ExitCode): $message"
    }
    $processCondition = New-Object System.Windows.Automation.PropertyCondition(
        [System.Windows.Automation.AutomationElement]::ProcessIdProperty,
        $process.Id
    )
    $controls = [System.Windows.Automation.AutomationElement]::RootElement.FindAll(
        [System.Windows.Automation.TreeScope]::Descendants,
        $processCondition
    )
    $summary = if ($controls.Count) {
        0..([Math]::Min(10, $controls.Count - 1)) | ForEach-Object {
            $current = $controls.Item($_).Current
            "$($current.ControlType.ProgrammaticName):$($current.Name)"
        }
    } else {
        @()
    }
    throw "$description did not appear; process tree: $($summary -join ', ')"
}

function Button-Condition([string]$name) {
    New-Object System.Windows.Automation.AndCondition(
        $buttonCondition,
        (New-Object System.Windows.Automation.PropertyCondition(
            [System.Windows.Automation.AutomationElement]::NameProperty,
            $name
        ))
    )
}

function Item-Button-Names($process) {
    $condition = New-Object System.Windows.Automation.AndCondition(
        (New-Object System.Windows.Automation.PropertyCondition(
            [System.Windows.Automation.AutomationElement]::ProcessIdProperty,
            $process.Id
        )),
        $buttonCondition
    )
    $buttons = [System.Windows.Automation.AutomationElement]::RootElement.FindAll(
        [System.Windows.Automation.TreeScope]::Descendants,
        $condition
    )
    @($buttons | ForEach-Object {
        $name = $_.Current.Name
        if ($name -like "Item *") {
            $name
        }
    })
}

$buttonCondition = New-Object System.Windows.Automation.PropertyCondition(
    [System.Windows.Automation.AutomationElement]::ControlTypeProperty,
    [System.Windows.Automation.ControlType]::Button
)

if ($Case -eq "failure") {
    $failureError =
        Join-Path $artifactDirectory "windows-reactor-selftest-failure-$([guid]::NewGuid()).txt"
    $script:activeError = $failureError
    $failureRun = Start-Host @("--failure") $failureError
    $failureProcess = $failureRun.Process
    try {
        $button = Wait-Control $failureRun.Process (
            Button-Condition "Trigger native failure"
        ) "failure Button"
        $invoke = $button.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        )
        $invoke.Invoke()
        $exited = $failureProcess.WaitForExit(10000)
        $failureMessage = Get-Content $failureError -Raw
        if (
            !$exited -and
            $failureMessage -notlike "*thread caused non-unwinding panic. aborting.*"
        ) {
            throw "post-launch native failure did not terminate the application: $failureMessage"
        }
        if ($exited -and $failureProcess.ExitCode -eq 0) {
            throw "post-launch native failure exited successfully"
        }
        if ($failureMessage -notlike "*native runtime failed*") {
            throw "post-launch native failure was not surfaced: $failureMessage"
        }
    } finally {
        Stop-Host $failureRun
        Remove-Item $failureError -ErrorAction SilentlyContinue
    }
    Write-Output "windows-reactor native failure self-test passed"
    return
}

$normalError =
    Join-Path $artifactDirectory "windows-reactor-selftest-$Case-$([guid]::NewGuid()).txt"
$script:activeError = $normalError
$hostArgs = @()
if ($Case -eq "grid") {
    $hostArgs += "--grid"
}
if ($Case -eq "list-box") {
    $hostArgs += "--list-box"
}
if ($Case -eq "combo-box") {
    $hostArgs += "--combo-box"
}
if ($Case -eq "radio-buttons") {
    $hostArgs += "--radio-buttons"
}
if ($Case -eq "selector-bar") {
    $hostArgs += "--selector-bar"
}
if ($Case -eq "breadcrumb-bar") {
    $hostArgs += "--breadcrumb-bar"
}
if ($Case -eq "auto-suggest-box") {
    $hostArgs += "--auto-suggest-box"
}
if ($Case -eq "teaching-tip") {
    $hostArgs += "--teaching-tip"
}
if ($Case -eq "flyout") {
    $hostArgs += "--flyout"
}
if ($Case -eq "content-dialog") {
    $hostArgs += "--content-dialog"
}
if ($Case -eq "command-bar") {
    $hostArgs += "--command-bar"
}
if ($Case -eq "media") {
    $hostArgs += "--media"
}
if ($Case -eq "canvas") {
    $hostArgs += "--canvas"
}
if ($Case -eq "canvas-image") {
    $hostArgs += "--canvas-image"
}
if ($Case -eq "animated-canvas") {
    $hostArgs += "--animated-canvas"
}
if ($Case -eq "multi-window") {
    $hostArgs += "--multi-window"
}
$hostRun = Start-Host $hostArgs $normalError
$dragSourceStarted = $false
try {
    if ($Case -eq "multi-window") {
        $windowTypeCondition = New-Object System.Windows.Automation.PropertyCondition(
            [System.Windows.Automation.AutomationElement]::ControlTypeProperty,
            [System.Windows.Automation.ControlType]::Window
        )
        $firstCondition = New-Object System.Windows.Automation.AndCondition(
            $windowTypeCondition,
            (New-Object System.Windows.Automation.PropertyCondition(
                [System.Windows.Automation.AutomationElement]::NameProperty,
                "windows-reactor native self-test"
            ))
        )
        $secondCondition = New-Object System.Windows.Automation.AndCondition(
            $windowTypeCondition,
            (New-Object System.Windows.Automation.PropertyCondition(
                [System.Windows.Automation.AutomationElement]::NameProperty,
                "windows-reactor secondary window"
            ))
        )
        $first = Wait-Control $hostRun.Process $firstCondition "first application window"
        $second = Wait-Control $hostRun.Process $secondCondition "second application window"
        $firstRuntimeId = $first.GetRuntimeId() -join ","

        $increment = Wait-Control $hostRun.Process (
            Button-Condition "Increment second window"
        ) "second-window Button"
        $increment.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        $null = Wait-Text $hostRun.Process "Second window count: 1"

        $openDialog = Wait-Control $hostRun.Process (
            Button-Condition "Open second window dialog"
        ) "second-window ContentDialog Button"
        $openDialog.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        $null = Wait-Text $hostRun.Process "Second window dialog content"
        $openFirstDialog = Wait-Control $hostRun.Process (
            Button-Condition "Open first window dialog"
        ) "first-window ContentDialog Button"
        $openFirstDialog.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        $null = Wait-Text $hostRun.Process "First window dialog content"
        $closeFirstDialog = Wait-Control $hostRun.Process (
            Button-Condition "Close first dialog"
        ) "first-window ContentDialog close Button"
        $closeFirstDialog.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        Wait-Text-Hidden $hostRun.Process (
            "First window dialog content"
        ) "first-window ContentDialog"
        $closeSecondDialog = Wait-Control $hostRun.Process (
            Button-Condition "Close second dialog"
        ) "second-window ContentDialog close Button"
        $closeSecondDialog.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        Wait-Text-Hidden $hostRun.Process (
            "Second window dialog content"
        ) "second-window ContentDialog"

        $first.GetCurrentPattern(
            [System.Windows.Automation.WindowPattern]::Pattern
        ).Close()
        $null = Wait-Text $hostRun.Process "First close requests: 1"
        $first = Wait-Control $hostRun.Process $firstCondition "rejected first application window"
        if (($first.GetRuntimeId() -join ",") -ne $firstRuntimeId) {
            throw "rejected close replaced the first native window"
        }

        $second.GetCurrentPattern(
            [System.Windows.Automation.WindowPattern]::Pattern
        ).Close()
        $deadline = (Get-Date).AddSeconds(5)
        do {
            Start-Sleep -Milliseconds 20
            $second = [System.Windows.Automation.AutomationElement]::RootElement.FindFirst(
                [System.Windows.Automation.TreeScope]::Descendants,
                (New-Object System.Windows.Automation.AndCondition(
                    (New-Object System.Windows.Automation.PropertyCondition(
                        [System.Windows.Automation.AutomationElement]::ProcessIdProperty,
                        $hostRun.Process.Id
                    )),
                    $secondCondition
                ))
            )
        } until ($null -eq $second -or (Get-Date) -gt $deadline)
        if ($null -ne $second) {
            throw "accepted close left the second native window open"
        }
        $null = Wait-Control $hostRun.Process $firstCondition "surviving first application window"

        $first.GetCurrentPattern(
            [System.Windows.Automation.WindowPattern]::Pattern
        ).Close()
        if (!$hostRun.Process.WaitForExit(5000)) {
            throw "closing the final application window did not return"
        }
    }

    if ($Case -eq "media") {
        $status = Wait-Control $hostRun.Process (
            New-Object System.Windows.Automation.PropertyCondition(
                [System.Windows.Automation.AutomationElement]::NameProperty,
                "Media status"
            )
        ) "media status"
        Wait-HelpText $status "bitmap: loaded; svg: loaded; failure: reported"
        foreach ($name in @("Symbol icon", "Font icon", "Bitmap icon", "Image icon")) {
            $null = Wait-Control $hostRun.Process (Button-Condition $name) "$name command"
        }
        $more = Wait-Control $hostRun.Process (
            Button-Condition "More options for Reactor media command bar"
        ) "media CommandBar More Button"
        $more.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        foreach ($name in @("Secondary symbol icon", "Secondary image icon")) {
            $null = Wait-Control $hostRun.Process (Button-Condition $name) "$name command"
        }
        [System.Windows.Forms.SendKeys]::SendWait("{ESC}")
        $pending = Wait-Control $hostRun.Process (
            Button-Condition "Start pending image"
        ) "pending Image Button"
        $pending.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
    }

    if ($Case -eq "animated-canvas") {
        $status = Wait-Control $hostRun.Process (
            New-Object System.Windows.Automation.PropertyCondition(
                [System.Windows.Automation.AutomationElement]::NameProperty,
                "Animated canvas status"
            )
        ) "animated canvas status"
        Wait-HelpText $status "ready"
        $publish = Wait-Control $hostRun.Process (
            Button-Condition "Publish animated canvas metrics"
        ) "publish animated canvas metrics button"
        $publish.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        $deadline = [DateTime]::UtcNow.AddSeconds(5)
        do {
            $initialSample = $status.Current.HelpText
            if ($initialSample -match "^draws: ([1-9][0-9]*)$") {
                break
            }
            Start-Sleep -Milliseconds 25
        } while ([DateTime]::UtcNow -lt $deadline)
        if ($initialSample -notmatch "^draws: ([1-9][0-9]*)$") {
            throw "animated canvas did not draw an initial frame"
        }
        $publish.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        $sampleSeconds = 5
        $hostRun.Process.Refresh()
        $cpuStart = $hostRun.Process.TotalProcessorTime.TotalMilliseconds
        Start-Sleep -Seconds $sampleSeconds
        $hostRun.Process.Refresh()
        $cpuMsPerSecond = (
            $hostRun.Process.TotalProcessorTime.TotalMilliseconds - $cpuStart
        ) / $sampleSeconds
        $publish.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        $sample = Wait-StableHelpText $status
        if ($sample -notmatch "^draws: ([1-9][0-9]*)$") {
            throw "animated canvas status is invalid"
        }
        $frameDelta = [int]$Matches[1]
        $framesPerSecond = $frameDelta / $sampleSeconds
        if ($frameDelta -lt (10 * $sampleSeconds)) {
            throw "animated canvas did not sustain frame delivery"
        }
        Write-Output "animated canvas: $([math]::Round($framesPerSecond, 1)) frames/s, CPU $([math]::Round($cpuMsPerSecond, 1)) ms/s"
        $remove = Wait-Control $hostRun.Process (
            Button-Condition "Remove animated canvas"
        ) "remove animated canvas button"
        $remove.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        Wait-Text $hostRun.Process "Animated canvas removed" | Out-Null
        Start-Sleep -Milliseconds 250
        $publish.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        if ((Wait-StableHelpText $status) -ne "draws: 0") {
            throw "removed animated canvas continued drawing"
        }
    }

    if ($Case -eq "canvas") {
        $status = Wait-Control $hostRun.Process (
            New-Object System.Windows.Automation.PropertyCondition(
                [System.Windows.Automation.AutomationElement]::NameProperty,
                "Canvas status"
            )
        ) "canvas status"
        $initialStatus = Wait-StableHelpText $status
        if (
            $initialStatus -notmatch
            "^draws: ([1-9][0-9]*); width: 240; devices: 1$"
        ) {
            throw "demand canvas did not draw an initial frame; current status is '$initialStatus'"
        }

        $initialDraws = [int]$Matches[1]
        $invalidate = Wait-Control $hostRun.Process (
            Button-Condition "Invalidate canvas"
        ) "canvas invalidation button"
        $invalidate.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        $expectedDraws = $initialDraws + 1
        Wait-HelpText $status "draws: $expectedDraws; width: 240; devices: 1"
        Start-Sleep -Milliseconds 250
        if (
            $status.Current.HelpText -ne
            "draws: $expectedDraws; width: 240; devices: 1"
        ) {
            throw "demand canvas drew while clean"
        }

        $resize = Wait-Control $hostRun.Process (
            Button-Condition "Resize canvas"
        ) "canvas resize button"
        $resize.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        $expectedDraws++
        Wait-HelpText $status "draws: $expectedDraws; width: 320; devices: 1"

        $zero = Wait-Control $hostRun.Process (
            Button-Condition "Zero canvas"
        ) "zero canvas button"
        $zero.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        Start-Sleep -Milliseconds 250
        if (
            $status.Current.HelpText -ne
            "draws: $expectedDraws; width: 320; devices: 1"
        ) {
            throw "zero-size canvas queued a frame"
        }

        $restoreSize = Wait-Control $hostRun.Process (
            Button-Condition "Restore canvas size"
        ) "restore canvas size button"
        $restoreSize.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        $expectedDraws++
        Wait-HelpText $status "draws: $expectedDraws; width: 240; devices: 1"

        $loseDevice = Wait-Control $hostRun.Process (
            Button-Condition "Lose canvas device"
        ) "lose canvas device button"
        $loseDevice.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        $expectedDraws++
        Wait-HelpText $status "draws: $expectedDraws; width: 240; devices: 2"

        $remove = Wait-Control $hostRun.Process (
            Button-Condition "Remove canvas"
        ) "remove canvas button"
        $remove.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        Start-Sleep -Milliseconds 100
        $invalidate.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        Start-Sleep -Milliseconds 250
        if (
            $status.Current.HelpText -ne
            "draws: $expectedDraws; width: 240; devices: 2"
        ) {
            throw "removed canvas accepted a stale invalidation"
        }

        $restoreNode = Wait-Control $hostRun.Process (
            Button-Condition "Restore canvas node"
        ) "restore canvas node button"
        $restoreNode.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        Start-Sleep -Milliseconds 500
        if (
            $status.Current.HelpText -notmatch
            "^draws: ([0-9]+); width: 240; devices: ([0-9]+)$"
        ) {
            throw "restored canvas did not draw; current status is '$($status.Current.HelpText)'"
        }
        if ([int]$Matches[1] -le $expectedDraws) {
            throw "restored canvas did not create a new surface frame"
        }
        if ([int]$Matches[2] -le 2) {
            throw "restored canvas did not create a new device"
        }
    }

    if ($Case -eq "canvas-image") {
        $status = Wait-Control $hostRun.Process (
            New-Object System.Windows.Automation.PropertyCondition(
                [System.Windows.Automation.AutomationElement]::NameProperty,
                "Canvas image status"
            )
        ) "canvas image status"
        $initialStatus = Wait-StableHelpText $status
        if (
            $initialStatus -notmatch
            "^draws: ([1-9][0-9]*); width: 240; devices: 1; surfaces: 1$"
        ) {
            throw "canvas image did not draw its initial surface; current status is '$initialStatus'"
        }
        $draws = [int]$Matches[1]

        $invalidate = Wait-Control $hostRun.Process (
            Button-Condition "Invalidate canvas image"
        ) "canvas image invalidation button"
        $invalidate.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        Wait-HelpText $status "draws: $($draws + 1); width: 240; devices: 1; surfaces: 1"
        $draws++

        $lose = Wait-Control $hostRun.Process (
            Button-Condition "Lose canvas image device"
        ) "canvas image device-loss button"
        $lose.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        Wait-HelpText $status "draws: $($draws + 1); width: 240; devices: 2; surfaces: 2"
        $draws++

        $resize = Wait-Control $hostRun.Process (
            Button-Condition "Resize canvas image"
        ) "canvas image resize button"
        $resize.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        Wait-HelpText $status "draws: $($draws + 1); width: 320; devices: 2; surfaces: 3"
        $draws++

        $zero = Wait-Control $hostRun.Process (
            Button-Condition "Zero canvas image"
        ) "zero canvas image button"
        $zero.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        Start-Sleep -Milliseconds 250
        if ($status.Current.HelpText -ne "draws: $draws; width: 320; devices: 2; surfaces: 3") {
            throw "zero-sized canvas image drew unexpectedly"
        }

        $restoreSize = Wait-Control $hostRun.Process (
            Button-Condition "Restore canvas image size"
        ) "restore canvas image size button"
        $restoreSize.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        Wait-HelpText $status "draws: $($draws + 1); width: 240; devices: 2; surfaces: 4"
        $draws++

        $remove = Wait-Control $hostRun.Process (
            Button-Condition "Remove canvas image"
        ) "remove canvas image button"
        $remove.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        Wait-Text $hostRun.Process "Canvas image removed" | Out-Null

        $restoreNode = Wait-Control $hostRun.Process (
            Button-Condition "Restore canvas image node"
        ) "restore canvas image node button"
        $restoreNode.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        Wait-HelpText $status "draws: $($draws + 1); width: 240; devices: 3; surfaces: 5"
    }

    if ($Case -eq "command-bar") {
        $status = Wait-Control $hostRun.Process (
            New-Object System.Windows.Automation.PropertyCondition(
                [System.Windows.Automation.AutomationElement]::NameProperty,
                "Command bar status"
            )
        ) "CommandBar status"
        Wait-HelpText $status (
            "Open: 0; pinned: False; secondary: 0; reversed: False; present: True"
        )

        $open = Wait-Control $hostRun.Process (
            Button-Condition "Open command"
        ) "AppBarButton"
        $open.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        Wait-HelpText $status (
            "Open: 1; pinned: False; secondary: 0; reversed: False; present: True"
        )

        $pin = Wait-Control $hostRun.Process (
            Button-Condition "Pin command"
        ) "AppBarToggleButton"
        $pin.GetCurrentPattern(
            [System.Windows.Automation.TogglePattern]::Pattern
        ).Toggle()
        Wait-HelpText $status (
            "Open: 1; pinned: True; secondary: 0; reversed: False; present: True"
        )

        $reorder = Wait-Control $hostRun.Process (
            Button-Condition "Reorder commands"
        ) "Reorder AppBarButton"
        $reorder.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        Wait-HelpText $status (
            "Open: 1; pinned: True; secondary: 0; reversed: True; present: True"
        )
        $open = Wait-Control $hostRun.Process (
            Button-Condition "Open command"
        ) "reordered AppBarButton"
        $open.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        Wait-HelpText $status (
            "Open: 2; pinned: True; secondary: 0; reversed: True; present: True"
        )

        $more = Wait-Control $hostRun.Process (
            Button-Condition "More options for Reactor command bar"
        ) "CommandBar More Button"
        $more.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        $secondary = Wait-Control $hostRun.Process (
            Button-Condition "Secondary command"
        ) "secondary AppBarButton"
        $secondary.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        Wait-HelpText $status (
            "Open: 2; pinned: True; secondary: 1; reversed: True; present: True"
        )

        $more = Wait-Control $hostRun.Process (
            Button-Condition "More options for Reactor command bar"
        ) "CommandBar More Button"
        $more.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        $remove = Wait-Control $hostRun.Process (
            Button-Condition "Remove command bar"
        ) "remove AppBarButton"
        $remove.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        Wait-HelpText $status (
            "Open: 2; pinned: True; secondary: 1; reversed: True; present: False"
        )

        $restore = Wait-Control $hostRun.Process (
            Button-Condition "Restore command bar"
        ) "Restore CommandBar Button"
        $restore.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        Wait-HelpText $status (
            "Open: 2; pinned: True; secondary: 1; reversed: True; present: True"
        )
        $more = Wait-Control $hostRun.Process (
            Button-Condition "More options for Reactor command bar"
        ) "restored CommandBar More Button"
        $more.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
    }

    if ($Case -eq "content-dialog") {
        $status = Wait-Control $hostRun.Process (
            New-Object System.Windows.Automation.PropertyCondition(
                [System.Windows.Automation.AutomationElement]::NameProperty,
                "Content dialog status"
            )
        ) "ContentDialog status"
        Wait-HelpText $status "Dialog open: False; result: None"

        $openDialog = Wait-Control $hostRun.Process (
            Button-Condition "Open content dialog"
        ) "Open ContentDialog Button"
        $openDialog.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        $null = Wait-Text $hostRun.Process "Reactor content dialog"
        $null = Wait-Text $hostRun.Process "Dialog content 0"
        Wait-HelpText $status "Dialog open: True; result: None"

        $updateContent = Wait-Control $hostRun.Process (
            Button-Condition "Update dialog content"
        ) "Update ContentDialog content Button"
        $updateContent.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        $null = Wait-Text $hostRun.Process "Dialog content 1"
        Wait-Text-Hidden $hostRun.Process "Dialog content 0" "replaced ContentDialog content"

        $primary = Wait-Control $hostRun.Process (
            Button-Condition "Primary"
        ) "ContentDialog primary Button"
        $primary.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        Wait-HelpText $status "Dialog open: False; result: Primary"
        Wait-Text-Hidden $hostRun.Process "Dialog content 1" "ContentDialog after primary result"

        $openDialog.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        $null = Wait-Text $hostRun.Process "Dialog content 1"
        $secondary = Wait-Control $hostRun.Process (
            Button-Condition "Secondary"
        ) "ContentDialog secondary Button"
        $secondary.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        Wait-HelpText $status "Dialog open: False; result: Secondary"

        $openDialog.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        $null = Wait-Text $hostRun.Process "Dialog content 1"
        $close = Wait-Control $hostRun.Process (
            Button-Condition "Close"
        ) "ContentDialog close Button"
        $close.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        Wait-HelpText $status "Dialog open: False; result: None"

        $openDialog.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        $null = Wait-Text $hostRun.Process "Dialog content 1"
        $removeDialog = Wait-Control $hostRun.Process (
            Button-Condition "Remove open dialog"
        ) "Remove open ContentDialog Button"
        $removeDialog.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        Wait-Text-Hidden $hostRun.Process "Dialog content 1" "removed ContentDialog"
        $null = Wait-Text $hostRun.Process "Content dialog removed"
        Wait-HelpText $status "Dialog open: True; result: None"

        $restoreDialog = Wait-Control $hostRun.Process (
            Button-Condition "Restore content dialog"
        ) "Restore ContentDialog Button"
        $restoreDialog.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        $null = Wait-Text $hostRun.Process "Dialog content 1"
    }

    if ($Case -eq "flyout") {
        $status = Wait-Control $hostRun.Process (
            New-Object System.Windows.Automation.PropertyCondition(
                [System.Windows.Automation.AutomationElement]::NameProperty,
                "Flyout status"
            )
        ) "Flyout status"
        Wait-HelpText $status "Flyout version: 0; opened: 0; closed: 0"

        $openFlyout = Wait-Control $hostRun.Process (
            Button-Condition "Open flyout"
        ) "DropDownButton"
        $expandFlyout = $openFlyout.GetCurrentPattern(
            [System.Windows.Automation.ExpandCollapsePattern]::Pattern
        )
        $expandFlyout.Expand()
        Wait-HelpText $status "Flyout version: 0; opened: 1; closed: 0"
        $null = Wait-Text $hostRun.Process "Flyout content 0"

        $updateContent = Wait-Control $hostRun.Process (
            Button-Condition "Update flyout content"
        ) "Flyout content update Button"
        $updateContent.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        $null = Wait-Text $hostRun.Process "Flyout content 1"
        Wait-Text-Hidden $hostRun.Process "Flyout content 0" "replaced Flyout content"
        Wait-HelpText $status "Flyout version: 1; opened: 1; closed: 0"

        [System.Windows.Forms.SendKeys]::SendWait("{ESC}")
        Wait-Text-Hidden $hostRun.Process "Flyout content 1" "Flyout after Escape"
        Wait-HelpText $status "Flyout version: 1; opened: 1; closed: 1"

        $expandFlyout.Expand()
        $null = Wait-Text $hostRun.Process "Flyout content 1"
        Wait-HelpText $status "Flyout version: 1; opened: 2; closed: 1"

        $removeOwner = Wait-Control $hostRun.Process (
            Button-Condition "Remove flyout owner"
        ) "Remove Flyout owner Button"
        $removeOwner.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        Wait-Text-Hidden $hostRun.Process "Flyout content 1" "Flyout after owner removal"
        $null = Wait-Text $hostRun.Process "Flyout owner removed"
        Wait-HelpText $status "Flyout version: 1; opened: 2; closed: 1"
    }

    if ($Case -eq "teaching-tip") {
        $status = Wait-Control $hostRun.Process (
            New-Object System.Windows.Automation.PropertyCondition(
                [System.Windows.Automation.AutomationElement]::NameProperty,
                "Teaching tip status"
            )
        ) "TeachingTip status"
        Wait-HelpText $status "TeachingTip open: True; closed: 0; actions: 0"

        $openTip = Wait-Control $hostRun.Process (
            Button-Condition "Open teaching tip"
        ) "Open TeachingTip Button"
        $null = Wait-Text $hostRun.Process "Reactor teaching tip"

        $action = Wait-Control $hostRun.Process (
            Button-Condition "Advance"
        ) "TeachingTip action Button"
        $owner = Wait-Control $hostRun.Process (
            New-Object System.Windows.Automation.PropertyCondition(
                [System.Windows.Automation.AutomationElement]::NameProperty,
                "Teaching tip owner"
            )
        ) "TeachingTip owner"
        $actionCenter = $action.Current.BoundingRectangle.X +
            ($action.Current.BoundingRectangle.Width / 2)
        $ownerCenter = $owner.Current.BoundingRectangle.X +
            ($owner.Current.BoundingRectangle.Width / 2)
        if ([Math]::Abs($actionCenter - $ownerCenter) -gt 800) {
            throw "TeachingTip is not positioned near its target owner"
        }
        $action.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        Wait-HelpText $status "TeachingTip open: True; closed: 0; actions: 1"

        $replaceOwner = Wait-Control $hostRun.Process (
            Button-Condition "Replace teaching tip owner"
        ) "Replace TeachingTip owner Button"
        $replaceOwner.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        $null = Wait-Control $hostRun.Process (
            New-Object System.Windows.Automation.AndCondition(
                (New-Object System.Windows.Automation.PropertyCondition(
                    [System.Windows.Automation.AutomationElement]::ControlTypeProperty,
                    [System.Windows.Automation.ControlType]::Text
                )),
                (New-Object System.Windows.Automation.PropertyCondition(
                    [System.Windows.Automation.AutomationElement]::NameProperty,
                    "Teaching tip owner"
                ))
            )
        ) "replacement TeachingTip text owner"
        $null = Wait-Text $hostRun.Process "Reactor teaching tip"
        Wait-HelpText $status "TeachingTip open: True; closed: 0; actions: 1"

        $programmaticClose = Wait-Control $hostRun.Process (
            Button-Condition "Close teaching tip programmatically"
        ) "Programmatic TeachingTip close Button"
        $programmaticClose.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        Wait-HelpText $status "TeachingTip open: False; closed: 0; actions: 1"
        Wait-Text-Hidden $hostRun.Process "Reactor teaching tip" "TeachingTip after programmatic close"

        $openTip.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        $null = Wait-Text $hostRun.Process "Reactor teaching tip"
        $closeCondition = New-Object System.Windows.Automation.AndCondition(
            (New-Object System.Windows.Automation.PropertyCondition(
                [System.Windows.Automation.AutomationElement]::ProcessIdProperty,
                $hostRun.Process.Id
            )),
            (Button-Condition "Close")
        )
        $deadline = (Get-Date).AddSeconds(3)
        do {
            $closeButtons = [System.Windows.Automation.AutomationElement]::RootElement.FindAll(
                [System.Windows.Automation.TreeScope]::Descendants,
                $closeCondition
            )
            $close = $closeButtons |
                Where-Object {
                    !$_.Current.IsOffscreen -and $_.Current.BoundingRectangle.Width -gt 100
                } |
                Select-Object -First 1
            if ($null -ne $close) {
                break
            }
            Start-Sleep -Milliseconds 20
        } until ((Get-Date) -gt $deadline)
        if ($null -eq $close) {
            throw "TeachingTip has no visible close Button"
        }
        $close.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        Wait-HelpText $status "TeachingTip open: False; closed: 1; actions: 1"

        $openTip.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        $null = Wait-Text $hostRun.Process "Reactor teaching tip"
        $removeOwner = Wait-Control $hostRun.Process (
            Button-Condition "Remove teaching tip owner"
        ) "Remove TeachingTip owner Button"
        $removeOwner.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        Wait-Text-Hidden $hostRun.Process "Reactor teaching tip" "TeachingTip after owner removal"
        Wait-HelpText $status "TeachingTip open: True; closed: 1; actions: 1"
    }

    if ($Case -eq "radio-buttons") {
        $radioButtons = Wait-Control $hostRun.Process $radioButtonsCondition "RadioButtons"
        if ($radioButtons.Current.HelpText -ne "Select one keyed radio choice") {
            throw "unexpected RadioButtons help text: $($radioButtons.Current.HelpText)"
        }
        $status = Wait-Control $hostRun.Process (
            New-Object System.Windows.Automation.PropertyCondition(
                [System.Windows.Automation.AutomationElement]::NameProperty,
                "Radio selection status"
            )
        ) "RadioButtons selection status"
        $items = @($radioButtons.FindAll(
            [System.Windows.Automation.TreeScope]::Descendants,
            $radioButtonCondition
        ))
        if ($items.Count -ne 3 -or
            $items[0].Current.Name -ne "Choice" -or
            $items[1].Current.Name -ne "Choice" -or
            $items[2].Current.Name -ne "Other") {
            throw "unexpected initial keyed RadioButtons items"
        }
        $items[1].GetCurrentPattern(
            [System.Windows.Automation.SelectionItemPattern]::Pattern
        ).Select()
        Wait-HelpText $status "Selected radio key: 20"

        $reorder = Wait-Control $hostRun.Process (
            Button-Condition "Reorder radio choices"
        ) "Reorder RadioButtons choices Button"
        $reorder.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        Start-Sleep -Milliseconds 100
        $radioButtons = Wait-Control $hostRun.Process $radioButtonsCondition "reordered RadioButtons"
        $items = @($radioButtons.FindAll(
            [System.Windows.Automation.TreeScope]::Descendants,
            $radioButtonCondition
        ))
        $selected = $items[2].GetCurrentPattern(
            [System.Windows.Automation.SelectionItemPattern]::Pattern
        )
        if ($items.Count -ne 3 -or
            $items[0].Current.Name -ne "Other" -or
            $items[1].Current.Name -ne "Choice" -or
            $items[2].Current.Name -ne "Choice" -or
            !$selected.Current.IsSelected) {
            $summary = $items | ForEach-Object {
                $selection = $_.GetCurrentPattern(
                    [System.Windows.Automation.SelectionItemPattern]::Pattern
                )
                "$($_.Current.Name):$($selection.Current.IsSelected)"
            }
            throw "RadioButtons selection did not follow key 20 through reorder: $($summary -join ', '); status $($status.Current.HelpText)"
        }
        Wait-HelpText $status "Selected radio key: 20"

        $toggleKey = Wait-Control $hostRun.Process (
            Button-Condition "Toggle radio key 20"
        ) "Toggle RadioButtons key Button"
        $toggleKey.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        Start-Sleep -Milliseconds 100
        $radioButtons = Wait-Control $hostRun.Process $radioButtonsCondition "filtered RadioButtons"
        $items = @($radioButtons.FindAll(
            [System.Windows.Automation.TreeScope]::Descendants,
            $radioButtonCondition
        ))
        if ($items.Count -ne 2) {
            throw "RadioButtons did not remove key 20"
        }
        foreach ($item in $items) {
            $selection = $item.GetCurrentPattern(
                [System.Windows.Automation.SelectionItemPattern]::Pattern
            )
            if ($selection.Current.IsSelected) {
                throw "RadioButtons selected a replacement while key 20 was absent"
            }
        }
        Wait-HelpText $status "Selected radio key: 20"

        $toggleKey.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        Start-Sleep -Milliseconds 100
        $radioButtons = Wait-Control $hostRun.Process $radioButtonsCondition "restored RadioButtons"
        $items = @($radioButtons.FindAll(
            [System.Windows.Automation.TreeScope]::Descendants,
            $radioButtonCondition
        ))
        $selected = $items[2].GetCurrentPattern(
            [System.Windows.Automation.SelectionItemPattern]::Pattern
        )
        if ($items.Count -ne 3 -or !$selected.Current.IsSelected) {
            throw "RadioButtons did not restore selection for returning key 20"
        }
    }

    if ($Case -eq "selector-bar") {
        $selector = Wait-Control $hostRun.Process $selectorBarCondition "SelectorBar"
        if ($selector.Current.HelpText -ne "Select one keyed selector item") {
            throw "unexpected SelectorBar help text: $($selector.Current.HelpText)"
        }
        $status = Wait-Control $hostRun.Process (
            New-Object System.Windows.Automation.PropertyCondition(
                [System.Windows.Automation.AutomationElement]::NameProperty,
                "SelectorBar selection status"
            )
        ) "SelectorBar selection status"
        $shared = $selector.FindFirst(
            [System.Windows.Automation.TreeScope]::Descendants,
            (New-Object System.Windows.Automation.PropertyCondition(
                [System.Windows.Automation.AutomationElement]::NameProperty,
                "Shared"
            ))
        )
        if ($null -eq $shared) {
            throw "SelectorBar Shared item was not found"
        }
        $shared.GetCurrentPattern(
            [System.Windows.Automation.SelectionItemPattern]::Pattern
        ).Select()
        Wait-HelpText $status "Selected selector key: 20; events: 1"

        $reverse = Wait-Control $hostRun.Process (
            Button-Condition "Reverse selector items"
        ) "Reverse SelectorBar items Button"
        $reverse.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        Start-Sleep -Milliseconds 100
        $selector = Wait-Control $hostRun.Process $selectorBarCondition "reordered SelectorBar"
        $shared = $selector.FindFirst(
            [System.Windows.Automation.TreeScope]::Descendants,
            (New-Object System.Windows.Automation.PropertyCondition(
                [System.Windows.Automation.AutomationElement]::NameProperty,
                "Shared"
            ))
        )
        $selection = $shared.GetCurrentPattern(
            [System.Windows.Automation.SelectionItemPattern]::Pattern
        )
        if (!$selection.Current.IsSelected) {
            throw "SelectorBar selection did not follow key 20 through reorder"
        }
        Wait-HelpText $status "Selected selector key: 20; events: 1"
    }

    if ($Case -eq "breadcrumb-bar") {
        $breadcrumb = Wait-Control $hostRun.Process $breadcrumbBarCondition "BreadcrumbBar"
        if ($breadcrumb.Current.HelpText -ne "Invoke one keyed breadcrumb item") {
            throw "unexpected BreadcrumbBar help text: $($breadcrumb.Current.HelpText)"
        }
        $status = Wait-Control $hostRun.Process (
            New-Object System.Windows.Automation.PropertyCondition(
                [System.Windows.Automation.AutomationElement]::NameProperty,
                "BreadcrumbBar click status"
            )
        ) "BreadcrumbBar click status"
        $documents = $breadcrumb.FindFirst(
            [System.Windows.Automation.TreeScope]::Descendants,
            (New-Object System.Windows.Automation.PropertyCondition(
                [System.Windows.Automation.AutomationElement]::NameProperty,
                "Documents"
            ))
        )
        if ($null -eq $documents) {
            throw "BreadcrumbBar Documents item was not found"
        }
        $documents.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        Wait-HelpText $status "Clicked breadcrumb key: 20; events: 1"

        $reverse = Wait-Control $hostRun.Process (
            Button-Condition "Reverse breadcrumb items"
        ) "Reverse BreadcrumbBar items Button"
        $reverse.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        Start-Sleep -Milliseconds 100
        $breadcrumb = Wait-Control $hostRun.Process $breadcrumbBarCondition "reordered BreadcrumbBar"
        $documents = $breadcrumb.FindFirst(
            [System.Windows.Automation.TreeScope]::Descendants,
            (New-Object System.Windows.Automation.PropertyCondition(
                [System.Windows.Automation.AutomationElement]::NameProperty,
                "Documents"
            ))
        )
        if ($null -eq $documents) {
            throw "reordered BreadcrumbBar Documents item was not found"
        }
        $documents.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        Wait-HelpText $status "Clicked breadcrumb key: 20; events: 2"
    }

    if ($Case -eq "auto-suggest-box") {
        $control = Wait-Control $hostRun.Process $autoSuggestBoxCondition "AutoSuggestBox"
        if ($control.Current.HelpText -ne "Search keyed fruit suggestions") {
            throw "unexpected AutoSuggestBox help text: $($control.Current.HelpText)"
        }
        $status = Wait-Control $hostRun.Process (
            New-Object System.Windows.Automation.PropertyCondition(
                [System.Windows.Automation.AutomationElement]::NameProperty,
                "AutoSuggestBox status"
            )
        ) "AutoSuggestBox status"
        $edit = $control.FindFirst(
            [System.Windows.Automation.TreeScope]::Descendants,
            $editCondition
        )
        if ($null -eq $edit) {
            throw "AutoSuggestBox edit field was not found"
        }
        $value = $edit.GetCurrentPattern(
            [System.Windows.Automation.ValuePattern]::Pattern
        )
        $value.SetValue("ap")
        Wait-HelpText $status "Text: ap; chosen: ; submitted: "

        $apple = Wait-Control $hostRun.Process (
            New-Object System.Windows.Automation.AndCondition(
                $itemCondition,
                (New-Object System.Windows.Automation.PropertyCondition(
                    [System.Windows.Automation.AutomationElement]::NameProperty,
                    "Apple"
                ))
            )
        ) "Apple AutoSuggestBox suggestion"
        $apple.GetCurrentPattern(
            [System.Windows.Automation.SelectionItemPattern]::Pattern
        ).Select()
        Wait-HelpText $status "Text: Apple; chosen: 10; submitted: "

        $control = Wait-Control $hostRun.Process $autoSuggestBoxCondition "updated AutoSuggestBox"
        $edit = $control.FindFirst(
            [System.Windows.Automation.TreeScope]::Descendants,
            $editCondition
        )
        $edit.SetFocus()
        [System.Windows.Forms.SendKeys]::SendWait("{ENTER}")
        Wait-HelpText $status "Text: Apple; chosen: 10; submitted: Apple"
    }

    if ($Case -eq "combo-box") {
        $comboBox = Wait-Control $hostRun.Process $comboBoxCondition "ComboBox"
        if ($comboBox.Current.HelpText -ne "Select one keyed combo choice") {
            throw "unexpected ComboBox help text: $($comboBox.Current.HelpText)"
        }
        $status = Wait-Control $hostRun.Process (
            New-Object System.Windows.Automation.PropertyCondition(
                [System.Windows.Automation.AutomationElement]::NameProperty,
                "Combo selection status"
            )
        ) "ComboBox selection status"
        $expand = $comboBox.GetCurrentPattern(
            [System.Windows.Automation.ExpandCollapsePattern]::Pattern
        )
        $expand.Expand()
        Start-Sleep -Milliseconds 100
        $items = @(Get-ListItems $comboBox)
        if ($items.Count -ne 3 -or
            $items[0].Current.Name -ne "Choice" -or
            $items[1].Current.Name -ne "Choice" -or
            $items[2].Current.Name -ne "Other") {
            throw "unexpected initial keyed ComboBox items"
        }
        $items[1].GetCurrentPattern(
            [System.Windows.Automation.SelectionItemPattern]::Pattern
        ).Select()
        $expand.Collapse()
        Wait-HelpText $status "Selected combo key: 20"

        $reorder = Wait-Control $hostRun.Process (
            Button-Condition "Reorder combo choices"
        ) "Reorder ComboBox choices Button"
        $reorder.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        Start-Sleep -Milliseconds 100
        $comboBox = Wait-Control $hostRun.Process $comboBoxCondition "reordered ComboBox"
        $expand = $comboBox.GetCurrentPattern(
            [System.Windows.Automation.ExpandCollapsePattern]::Pattern
        )
        $expand.Expand()
        Start-Sleep -Milliseconds 100
        $items = @(Get-ListItems $comboBox)
        $selectionPattern = $comboBox.GetCurrentPattern(
            [System.Windows.Automation.SelectionPattern]::Pattern
        )
        $selected = $items[2].GetCurrentPattern(
            [System.Windows.Automation.SelectionItemPattern]::Pattern
        )
        if ($items.Count -ne 3 -or
            $items[0].Current.Name -ne "Other" -or
            $items[1].Current.Name -ne "Choice" -or
            $items[2].Current.Name -ne "Choice" -or
            !$selected.Current.IsSelected -or
            $selectionPattern.Current.GetSelection().Count -ne 1) {
            $summary = $items | ForEach-Object {
                $selection = $_.GetCurrentPattern(
                    [System.Windows.Automation.SelectionItemPattern]::Pattern
                )
                "$($_.Current.Name):$($selection.Current.IsSelected)"
            }
            throw "ComboBox selection did not follow key 20 through reorder: $($summary -join ', ')"
        }
        $expand.Collapse()
        Wait-HelpText $status "Selected combo key: 20"
    }

    if ($Case -eq "list-box") {
        $listBox = Wait-Control $hostRun.Process $listBoxCondition "ListBox"
        if ($listBox.Current.ControlType -ne [System.Windows.Automation.ControlType]::List) {
            throw "ListBox does not expose the native List automation role"
        }
        if ($listBox.Current.HelpText -ne "Select keyed choices") {
            throw "unexpected ListBox help text: $($listBox.Current.HelpText)"
        }
        $selectionPattern = $listBox.GetCurrentPattern(
            [System.Windows.Automation.SelectionPattern]::Pattern
        )
        $status = Wait-Text $hostRun.Process "Choice selection status"
        $items = @(Get-ListItems $listBox)
        if ($items.Count -ne 3 -or
            $items[0].Current.Name -ne "Choice" -or
            $items[1].Current.Name -ne "Choice" -or
            $items[2].Current.Name -ne "Other") {
            throw "unexpected initial keyed ListBox items"
        }
        $secondChoice = $items[1].GetCurrentPattern(
            [System.Windows.Automation.SelectionItemPattern]::Pattern
        )
        $secondChoice.Select()
        Wait-HelpText $status "Selected choice keys: 20"
        if (!$secondChoice.Current.IsSelected) {
            throw "duplicate-label ListBox item did not expose selected state"
        }

        $reorder = Wait-Control $hostRun.Process (
            Button-Condition "Reorder choices"
        ) "Reorder choices Button"
        $reorder.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        Start-Sleep -Milliseconds 100
        $listBox = Wait-Control $hostRun.Process $listBoxCondition "reordered ListBox"
        $items = @(Get-ListItems $listBox)
        if ($items.Count -ne 3 -or
            $items[0].Current.Name -ne "Other" -or
            $items[1].Current.Name -ne "Choice" -or
            $items[2].Current.Name -ne "Choice") {
            throw "ListBox did not apply the keyed reorder"
        }
        $selectedIndex = -1
        for ($index = 0; $index -lt $items.Count; $index++) {
            $selection = $items[$index].GetCurrentPattern(
                [System.Windows.Automation.SelectionItemPattern]::Pattern
            )
            if ($selection.Current.IsSelected) {
                $selectedIndex = $index
            }
        }
        if ($selectedIndex -ne 2 -or
            $selectionPattern.Current.GetSelection().Count -ne 1) {
            throw "ListBox selection did not follow key 20 through reorder"
        }
        Wait-HelpText $status "Selected choice keys: 20"
        $items[1].GetCurrentPattern(
            [System.Windows.Automation.SelectionItemPattern]::Pattern
        ).AddToSelection()
        Wait-HelpText $status "Selected choice keys: 10,20"

        $toggleKey = Wait-Control $hostRun.Process (
            Button-Condition "Toggle choice key 20"
        ) "Toggle choice key 20 Button"
        $toggleKey.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        Start-Sleep -Milliseconds 100
        $listBox = Wait-Control $hostRun.Process $listBoxCondition "filtered ListBox"
        $items = @(Get-ListItems $listBox)
        if ($items.Count -ne 2 -or
            $selectionPattern.Current.GetSelection().Count -ne 1) {
            throw "ListBox did not preserve the present selected key while key 20 was absent"
        }
        Wait-HelpText $status "Selected choice keys: 10,20"
        $toggleKey.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        Start-Sleep -Milliseconds 100
        $listBox = Wait-Control $hostRun.Process $listBoxCondition "restored ListBox"
        $items = @(Get-ListItems $listBox)
        if ($items.Count -ne 3 -or
            $selectionPattern.Current.GetSelection().Count -ne 2) {
            throw "ListBox did not restore selection for the returning key 20"
        }
    }

    if ($Case -eq "grid") {
        $grid = Wait-Control $hostRun.Process $gridCondition "GridView"
        if ($grid.Current.ControlType -ne [System.Windows.Automation.ControlType]::List) {
            throw "GridView does not expose the native List automation role"
        }
        if ($grid.Current.HelpText -ne "Scrollable virtual tile results") {
            throw "unexpected GridView help text: $($grid.Current.HelpText)"
        }
        $selectionPattern = $grid.GetCurrentPattern(
            [System.Windows.Automation.SelectionPattern]::Pattern
        )
        $scrollPattern = $grid.GetCurrentPattern(
            [System.Windows.Automation.ScrollPattern]::Pattern
        )
        $selectedStatus = Wait-Text $hostRun.Process "Selected tile status"
        $invokedStatus = Wait-Text $hostRun.Process "Invoked tile status"
        $items = $grid.FindAll(
            [System.Windows.Automation.TreeScope]::Descendants,
            $itemCondition
        )
        if ($items.Count -lt 8 -or $items.Count -gt 512) {
            throw "unexpected realized GridView item count: $($items.Count)"
        }
        $visible = @()
        foreach ($item in $items) {
            if ($item.Current.Name -notmatch "^Tile [0-9]+$") {
                throw "unexpected GridView item name: $($item.Current.Name)"
            }
            $null = $item.GetCurrentPattern(
                [System.Windows.Automation.InvokePattern]::Pattern
            )
            $null = $item.GetCurrentPattern(
                [System.Windows.Automation.SelectionItemPattern]::Pattern
            )
            if (!$item.Current.IsOffscreen) {
                $visible += $item
            }
        }
        if ($visible.Count -lt 4) {
            throw "GridView exposes too few visible items: $($visible.Count)"
        }
        $columns = @($visible | ForEach-Object {
            [Math]::Round($_.Current.BoundingRectangle.Left)
        } | Select-Object -Unique)
        $rows = @($visible | ForEach-Object {
            [Math]::Round($_.Current.BoundingRectangle.Top)
        } | Select-Object -Unique)
        if ($columns.Count -lt 2 -or $rows.Count -lt 2) {
            throw "GridView did not arrange visible items in rows and columns"
        }
        foreach ($item in $visible) {
            $bounds = $item.Current.BoundingRectangle
            if ($bounds.Width -lt 200 -or $bounds.Width -gt 260) {
                throw "GridView item width does not follow its 160-pixel root: $bounds"
            }
            if ($bounds.Height -lt 120 -or $bounds.Height -gt 170) {
                throw "GridView item height does not follow its 100-pixel root: $bounds"
            }
        }

        $first = $visible[0]
        $firstName = $first.Current.Name
        $firstSelection = $first.GetCurrentPattern(
            [System.Windows.Automation.SelectionItemPattern]::Pattern
        )
        $firstSelection.Select()
        $firstKey = $firstName.Substring(5)
        Wait-HelpText $selectedStatus "Selected tile key: $firstKey"
        if (!$firstSelection.Current.IsSelected) {
            throw "GridView item did not expose its controlled selected state"
        }
        $selectedItems = $selectionPattern.Current.GetSelection()
        if ($selectedItems.Count -ne 1 -or $selectedItems[0].Current.Name -ne $firstName) {
            throw "GridView host selection does not match the controlled item"
        }
        $first.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        Wait-HelpText $invokedStatus "Invoked tile key: $firstKey"

        $first.SetFocus()
        Start-Sleep -Milliseconds 50
        [System.Windows.Forms.SendKeys]::SendWait("{RIGHT}")
        Start-Sleep -Milliseconds 100
        $right = [System.Windows.Automation.AutomationElement]::FocusedElement
        if ($right.Current.ProcessId -ne $hostRun.Process.Id -or
            $right.Current.Name -eq $firstName) {
            throw "Right arrow did not move GridView keyboard focus"
        }
        $firstBounds = $first.Current.BoundingRectangle
        $rightBounds = $right.Current.BoundingRectangle
        if ($rightBounds.Left -le $firstBounds.Left -or
            [Math]::Abs($rightBounds.Top - $firstBounds.Top) -gt 10) {
            throw "Right arrow did not move GridView focus within the row"
        }
        $rightName = $right.Current.Name
        [System.Windows.Forms.SendKeys]::SendWait("{DOWN}")
        Start-Sleep -Milliseconds 100
        $down = [System.Windows.Automation.AutomationElement]::FocusedElement
        if ($down.Current.ProcessId -ne $hostRun.Process.Id -or
            $down.Current.Name -eq $rightName) {
            throw "Down arrow did not move GridView keyboard focus"
        }
        if ($down.Current.BoundingRectangle.Top -le $rightBounds.Top) {
            throw "Down arrow did not move GridView focus to a lower row"
        }

        $initialNames = @($visible | ForEach-Object { $_.Current.Name })
        $scrollPattern.Scroll(
            [System.Windows.Automation.ScrollAmount]::NoAmount,
            [System.Windows.Automation.ScrollAmount]::LargeIncrement
        )
        Start-Sleep -Milliseconds 150
        $scrolledItems = $grid.FindAll(
            [System.Windows.Automation.TreeScope]::Descendants,
            $itemCondition
        )
        if ($scrolledItems.Count -lt 8 -or $scrolledItems.Count -gt 512) {
            throw "unexpected post-scroll GridView item count: $($scrolledItems.Count)"
        }
        $recycledNameFound = $false
        foreach ($item in $scrolledItems) {
            if ($item.Current.Name -notmatch "^Tile [0-9]+$") {
                throw "unexpected post-scroll GridView item name: $($item.Current.Name)"
            }
            if (!$item.Current.IsOffscreen -and $initialNames -notcontains $item.Current.Name) {
                $recycledNameFound = $true
            }
        }
        if (!$recycledNameFound) {
            throw "GridView scroll did not expose recycled keyed tile peers"
        }

        $toggle = Wait-Control $hostRun.Process (
            Button-Condition "Tiles toggle"
        ) "Tiles toggle Button"
        $toggle.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        $null = Wait-Text $hostRun.Process "No tiles available"
        $grid = Wait-Control $hostRun.Process $gridCondition "empty GridView"
        if ($grid.FindAll(
            [System.Windows.Automation.TreeScope]::Descendants,
            $itemCondition
        ).Count -ne 0) {
            throw "empty GridView still exposes item peers"
        }
        $toggle.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        $grid = Wait-Control $hostRun.Process $gridCondition "repopulated GridView"
        $deadline = (Get-Date).AddSeconds(3)
        do {
            $restoredItem = $grid.FindFirst(
                [System.Windows.Automation.TreeScope]::Descendants,
                $itemCondition
            )
            if ($null -ne $restoredItem) {
                break
            }
            Start-Sleep -Milliseconds 10
        } until ((Get-Date) -gt $deadline)
        if ($null -eq $restoredItem) {
            throw "repopulated GridView did not restore item peers"
        }
    }

    if ($Case -eq "collections") {
        $button = Wait-Control $hostRun.Process (Button-Condition "Rows toggle") "Rows toggle Button"
        $list = Wait-Control $hostRun.Process $listCondition "initial ListView"

        if ($button.Current.HelpText -ne "Changes the virtual row count") {
            throw "unexpected accessibility help text: $($button.Current.HelpText)"
        }
        if ($list.Current.Name -ne "Virtual rows") {
            throw "unexpected ListView accessible name: $($list.Current.Name)"
        }
        if ($list.Current.HelpText -ne "Scrollable virtual row results") {
            throw "unexpected ListView help text: $($list.Current.HelpText)"
        }
        $selectionPattern = $list.GetCurrentPattern(
            [System.Windows.Automation.SelectionPattern]::Pattern
        )
        if ($selectionPattern.Current.CanSelectMultiple) {
            throw "single-selection ListView reports multi-selection support"
        }
        if ($selectionPattern.Current.GetSelection().Count -ne 0) {
            throw "ListView has an unexpected initial native selection"
        }

        $null = Wait-Text $hostRun.Process "Rows: 5000"
        $realizedItems = $list.FindAll(
            [System.Windows.Automation.TreeScope]::Descendants,
            $itemCondition
        )
        $realized = $realizedItems.Count
        if ($realized -lt 10 -or $realized -gt 100) {
            throw "unexpected realized item count: $realized"
        }
        $names = [System.Collections.Generic.HashSet[string]]::new()
        $visible = 0
        foreach ($item in $realizedItems) {
            if ($item.Current.Name -notmatch "^Row [0-9]+$") {
                throw "unexpected realized ListViewItem name: $($item.Current.Name)"
            }
            if (!$names.Add($item.Current.Name)) {
                throw "duplicate realized ListViewItem name: $($item.Current.Name)"
            }
            $null = $item.GetCurrentPattern(
                [System.Windows.Automation.InvokePattern]::Pattern
            )
            $null = $item.GetCurrentPattern(
                [System.Windows.Automation.SelectionItemPattern]::Pattern
            )
            if (!$item.Current.IsOffscreen) {
                $visible++
            }
        }
        if ($visible -eq 0) {
            throw "ListView exposes no visible realized items"
        }
    }

    if ($Case -eq "input") {
        $button = Wait-Control $hostRun.Process (Button-Condition "Rows toggle") "Rows toggle Button"
        $button.SetFocus()
        Start-Sleep -Milliseconds 50
        if (!$button.Current.HasKeyboardFocus) {
            throw "Button did not receive keyboard focus"
        }

        $null = Wait-Text $hostRun.Process "Ctrl+S invocations: 0"
        [System.Windows.Forms.SendKeys]::SendWait("^s")
        $null = Wait-Text $hostRun.Process "Ctrl+S invocations: 1"
        [System.Windows.Forms.SendKeys]::SendWait("^s")
        $null = Wait-Text $hostRun.Process "Ctrl+S invocations: 2"

        $pointer = Wait-Text $hostRun.Process "Pointer target"
    $bounds = $pointer.Current.BoundingRectangle
    if ($bounds.Width -le 0 -or $bounds.Height -le 0 -or $pointer.Current.IsOffscreen) {
        throw "Pointer target has no visible bounds: $bounds"
    }
    $pointerX = [int]($bounds.Left + ($bounds.Width / 2))
    $pointerY = [int]($bounds.Top + ($bounds.Height / 2))
    if (![MouseInput]::SetCursorPos($pointerX, $pointerY)) {
        throw "failed to position the pointer target cursor"
    }
    Start-Sleep -Milliseconds 50
    [MouseInput]::mouse_event(0x0002, 0, 0, 0, [System.UIntPtr]::Zero)
    $null = Wait-Text $hostRun.Process "Pointer pressed: 1 capture: true"
    if (![MouseInput]::SetCursorPos($pointerX + 300, $pointerY + 200)) {
        throw "failed to move the captured pointer"
    }
    $null = Wait-Text $hostRun.Process "Pointer moved: true"
    [MouseInput]::mouse_event(0x0004, 0, 0, 0, [System.UIntPtr]::Zero)
    $null = Wait-Text $hostRun.Process "Pointer released: 1"
    $null = Wait-Text $hostRun.Process "Pointer capture lost: 1"

    $dropTarget = Wait-Text $hostRun.Process "Drop target"
    $dropBounds = $dropTarget.Current.BoundingRectangle
    if ($dropBounds.Width -le 0 -or $dropBounds.Height -le 0 -or $dropTarget.Current.IsOffscreen) {
        throw "Drop target has no visible bounds: $dropBounds"
    }
    [DragSource]::Start()
    $dragSourceStarted = $true
    $dropX = [int]($dropBounds.Left + ($dropBounds.Width / 2))
    $dropY = [int]($dropBounds.Top + ($dropBounds.Height / 2))
    $dropped = $null
    $injectionAttempts = 0
    while (
        $null -eq $dropped -and
        [DragSource]::Attempts -lt 3 -and
        $injectionAttempts -lt 6
    ) {
        $injectionAttempts++
        $previousAttempts = [DragSource]::Attempts
        if (![MouseInput]::SetCursorPos([DragSource]::X, [DragSource]::Y)) {
            throw "failed to position the drag source cursor"
        }
        Start-Sleep -Milliseconds 100
        [MouseInput]::mouse_event(0x0002, 0, 0, 0, [System.UIntPtr]::Zero)
        $dragDeadline = (Get-Date).AddSeconds(1)
        while (
            [DragSource]::Attempts -eq $previousAttempts -and
            (Get-Date) -lt $dragDeadline
        ) {
            Start-Sleep -Milliseconds 10
        }
        if ([DragSource]::Attempts -eq $previousAttempts) {
            [MouseInput]::mouse_event(0x0004, 0, 0, 0, [System.UIntPtr]::Zero)
            Start-Sleep -Milliseconds 100
            continue
        }
        Start-Sleep -Milliseconds 100
        1..10 | ForEach-Object {
            $x = [int]([DragSource]::X + (2 * $_))
            $y = [int]([DragSource]::Y + (2 * $_))
            if (![MouseInput]::SetCursorPos($x, $y)) {
                throw "failed to begin the native drag"
            }
            Start-Sleep -Milliseconds 20
        }
        $dragX = [DragSource]::X + 20
        $dragY = [DragSource]::Y + 20
        1..30 | ForEach-Object {
            $x = [int]($dragX + (($dropX - $dragX) * $_ / 30))
            $y = [int]($dragY + (($dropY - $dragY) * $_ / 30))
            if (![MouseInput]::SetCursorPos($x, $y)) {
                throw "failed to move the native drag over the drop target"
            }
            Start-Sleep -Milliseconds 20
        }
        Start-Sleep -Milliseconds 250
        [MouseInput]::mouse_event(0x0004, 0, 0, 0, [System.UIntPtr]::Zero)
        $dropped = Wait-Text-Optional $hostRun.Process "Dropped text: reactor drop text" 2
    }
    if ($null -eq $dropped) {
        $hostRun.Process.Refresh()
        $message = Get-Content $normalError -Raw -ErrorAction SilentlyContinue
        $processCondition = New-Object System.Windows.Automation.PropertyCondition(
            [System.Windows.Automation.AutomationElement]::ProcessIdProperty,
            $hostRun.Process.Id
        )
        $dropStatus = [System.Windows.Automation.AutomationElement]::RootElement.FindAll(
            [System.Windows.Automation.TreeScope]::Descendants,
            $processCondition
        ) | ForEach-Object {
            $_.Current.Name
        } | Where-Object {
            $_ -like "Dropped text:*"
        }
        throw "native drag/drop did not complete after $injectionAttempts injections (source attempts: $([DragSource]::Attempts), effect: $([DragSource]::LastEffect), status: $dropStatus, exited: $($hostRun.Process.HasExited), code: $($hostRun.Process.ExitCode)): $message"
    }
        [DragSource]::Stop()
        $dragSourceStarted = $false
    }

    if ($Case -eq "collections") {
        $list = Wait-Control $hostRun.Process $listCondition "ListView before item invocation"
    $firstItem = $list.FindFirst(
        [System.Windows.Automation.TreeScope]::Descendants,
        $itemCondition
    )
    $firstItem.GetCurrentPattern(
        [System.Windows.Automation.InvokePattern]::Pattern
    ).Invoke()
    $null = Wait-Text $hostRun.Process "Invoked row key: 0"

    $invoke = $button.GetCurrentPattern(
        [System.Windows.Automation.InvokePattern]::Pattern
    )
    $invoke.Invoke()
    $null = Wait-Text $hostRun.Process "Rows: 10000"
    $invoke.Invoke()
    $null = Wait-Text $hostRun.Process "Rows: 5000"
    1..8 | ForEach-Object {
        $invoke.Invoke()
        $expected = if ($_ % 2) { "Rows: 10000" } else { "Rows: 5000" }
        $null = Wait-Text $hostRun.Process $expected
    }

    $item2 = Wait-Control $hostRun.Process (Button-Condition "Item 2") "Item 2 Button"
    $item2.SetFocus()
    Start-Sleep -Milliseconds 50
    $item2 = Wait-Control $hostRun.Process (Button-Condition "Item 2") "focused Item 2 Button"
        if (!$item2.Current.HasKeyboardFocus) {
        throw "Item 2 did not receive focus before keyed reorder"
    }
    $item2.GetCurrentPattern(
        [System.Windows.Automation.InvokePattern]::Pattern
    ).Invoke()
    $deadline = (Get-Date).AddSeconds(10)
    do {
        $itemOrder = Item-Button-Names $hostRun.Process
        if (($itemOrder -join ",") -eq "Item 3,Item 1,Item 2") {
            break
        }
        Start-Sleep -Milliseconds 20
    } until ((Get-Date) -gt $deadline)
    if (($itemOrder -join ",") -ne "Item 3,Item 1,Item 2") {
        throw "keyed automation order was not updated: $($itemOrder -join ', ')"
    }
    $deadline = (Get-Date).AddSeconds(2)
    do {
        $item2 = Wait-Control $hostRun.Process (
            Button-Condition "Item 2"
        ) "moved Item 2 Button"
        if ($item2.Current.HasKeyboardFocus) {
            break
        }
        Start-Sleep -Milliseconds 20
    } until ((Get-Date) -gt $deadline)
    if (!$item2.Current.HasKeyboardFocus) {
        $focused = [System.Windows.Automation.AutomationElement]::FocusedElement
        if ($focused.Current.ProcessId -ne $hostRun.Process.Id) {
            throw "desktop input moved focus outside the test process during keyed reorder"
        }
            throw "keyed reorder moved focus to '$($focused.Current.Name)' inside the test process"
        }
    }

    if ($Case -eq "values") {
        $edit = Wait-Control $hostRun.Process $textBoxCondition "TextBox"
    $value = $edit.GetCurrentPattern(
        [System.Windows.Automation.ValuePattern]::Pattern
    )
    $value.SetValue("edited")
    $null = Wait-Text $hostRun.Process "Text value: edited"
    $null = Wait-Text $hostRun.Process "Text events: 1"
    $value.SetValue("edited again")
    $null = Wait-Text $hostRun.Process "Text value: edited again"
    $null = Wait-Text $hostRun.Process "Text events: 2"

    $password = Wait-Control $hostRun.Process $passwordCondition "PasswordBox"
    $passwordValue = $password.GetCurrentPattern(
        [System.Windows.Automation.ValuePattern]::Pattern
    )
    $passwordValue.SetValue("secret entry")
    $null = Wait-Text $hostRun.Process "Password length: 12"
    $null = Wait-Text $hostRun.Process "Password events: 1"

    $slider = Wait-Control $hostRun.Process $sliderCondition "Slider"
    $range = $slider.GetCurrentPattern(
        [System.Windows.Automation.RangeValuePattern]::Pattern
    )
    $range.SetValue(75)
    $null = Wait-Text $hostRun.Process "Slider value: 75"
    $null = Wait-Text $hostRun.Process "Slider events: 1"

    $numberBox = Wait-Control $hostRun.Process $numberBoxCondition "NumberBox"
    $numberRange = $numberBox.GetCurrentPattern(
        [System.Windows.Automation.RangeValuePattern]::Pattern
    )
    $numberRange.SetValue(42)
    $null = Wait-Text $hostRun.Process "NumberBox value: 42"
    $null = Wait-Text $hostRun.Process "NumberBox events: 1"

    $rating = Wait-Control $hostRun.Process $ratingCondition "RatingControl"
    $ratingBounds = $rating.Current.BoundingRectangle
    if ($ratingBounds.Width -le 0 -or $ratingBounds.Height -le 0 -or $rating.Current.IsOffscreen) {
        throw "RatingControl has no visible bounds: $ratingBounds"
    }
    $ratingX = [int]($ratingBounds.Left + ($ratingBounds.Width * 0.47))
    $ratingY = [int]($ratingBounds.Top + ($ratingBounds.Height / 2))
    if (![MouseInput]::SetCursorPos($ratingX, $ratingY)) {
        throw "failed to position the RatingControl cursor"
    }
    Start-Sleep -Milliseconds 50
    [MouseInput]::mouse_event(0x0002, 0, 0, 0, [System.UIntPtr]::Zero)
    [MouseInput]::mouse_event(0x0004, 0, 0, 0, [System.UIntPtr]::Zero)
    Start-Sleep -Milliseconds 100
    $ratingRange = $rating.GetCurrentPattern(
        [System.Windows.Automation.RangeValuePattern]::Pattern
    )
    if ($ratingRange.Current.Value -ne 4) {
        throw "RatingControl click selected $($ratingRange.Current.Value): $ratingBounds"
    }
    $null = Wait-Text $hostRun.Process "Rating value: 4"
    $null = Wait-Text $hostRun.Process "Rating events: 1"

    $colorPicker = Wait-Control $hostRun.Process $colorPickerCondition "ColorPicker"
    $hexInput = $colorPicker.FindFirst(
        [System.Windows.Automation.TreeScope]::Descendants,
        $editCondition
    )
    if ($null -eq $hexInput) {
        throw "ColorPicker hex input did not appear"
    }
    $hexValue = $hexInput.GetCurrentPattern(
        [System.Windows.Automation.ValuePattern]::Pattern
    )
    $hexValue.SetValue("#00FF00")
    $null = Wait-Text $hostRun.Process "Color value: #00FF00"
    $null = Wait-Text $hostRun.Process "Color events: 1"

    $datePicker = Wait-Control $hostRun.Process $datePickerCondition "DatePicker"
    $dateParts = $datePicker.FindAll(
        [System.Windows.Automation.TreeScope]::Descendants,
        [System.Windows.Automation.Condition]::TrueCondition
    )
    $datePart = $null
    foreach ($candidate in $dateParts) {
        if ($candidate.Current.IsKeyboardFocusable) {
            $datePart = $candidate
            break
        }
    }
    if ($null -eq $datePart) {
        $details = ($dateParts | ForEach-Object {
            "$($_.Current.ControlType.ProgrammaticName):$($_.Current.Name)"
        }) -join ", "
        throw "DatePicker has no focusable descendant: $details"
    }
    $datePart.GetCurrentPattern(
        [System.Windows.Automation.InvokePattern]::Pattern
    ).Invoke()
    Start-Sleep -Milliseconds 100
    $monthCondition = New-Object System.Windows.Automation.AndCondition(
        $listCondition,
        (New-Object System.Windows.Automation.PropertyCondition(
            [System.Windows.Automation.AutomationElement]::NameProperty,
            "month"
        ))
    )
    $month = Wait-Control $hostRun.Process $monthCondition "DatePicker month selector"
    $months = $month.FindAll(
        [System.Windows.Automation.TreeScope]::Children,
        $itemCondition
    )
    $months.Item(1).GetCurrentPattern(
        [System.Windows.Automation.SelectionItemPattern]::Pattern
    ).Select()
    $accept = Wait-Control $hostRun.Process (
        Button-Condition "Accept"
    ) "DatePicker Accept Button"
    $accept.GetCurrentPattern(
        [System.Windows.Automation.InvokePattern]::Pattern
    ).Invoke()
    $null = Wait-Text $hostRun.Process "DatePicker events: 1"

    $calendarDatePicker = Wait-Control $hostRun.Process `
        $calendarDatePickerCondition "CalendarDatePicker"
    $calendarDatePicker.GetCurrentPattern(
        [System.Windows.Automation.InvokePattern]::Pattern
    ).Invoke()
    $calendarCondition = New-Object System.Windows.Automation.PropertyCondition(
        [System.Windows.Automation.AutomationElement]::ControlTypeProperty,
        [System.Windows.Automation.ControlType]::Calendar
    )
    $calendar = Wait-Control $hostRun.Process $calendarCondition "CalendarDatePicker calendar"
    $calendarItems = $calendar.FindAll(
        [System.Windows.Automation.TreeScope]::Descendants,
        $dataItemCondition
    )
    $differentDate = $null
    foreach ($candidate in $calendarItems) {
        $selection = $candidate.GetCurrentPattern(
            [System.Windows.Automation.SelectionItemPattern]::Pattern
        )
        if (!$selection.Current.IsSelected) {
            $differentDate = $candidate
            break
        }
    }
    if ($null -eq $differentDate) {
        $details = ($calendarItems | ForEach-Object { $_.Current.Name }) -join ", "
        throw "CalendarDatePicker has no unselected date: $details"
    }
    $differentDate.GetCurrentPattern(
        [System.Windows.Automation.SelectionItemPattern]::Pattern
    ).Select()
    $null = Wait-Text $hostRun.Process "CalendarDatePicker events: 1"

    $checkBox = Wait-Control $hostRun.Process $checkBoxCondition "CheckBox"
    $toggle = $checkBox.GetCurrentPattern(
        [System.Windows.Automation.TogglePattern]::Pattern
    )
    $toggle.Toggle()
    $null = Wait-Text $hostRun.Process "Checked value: true"
    $null = Wait-Text $hostRun.Process "Toggle events: 1"
    if ($toggle.Current.ToggleState -ne [System.Windows.Automation.ToggleState]::On) {
        throw "CheckBox did not expose the checked state"
    }
    $toggle.Toggle()
    $null = Wait-Text $hostRun.Process "Checked value: false"
    $null = Wait-Text $hostRun.Process "Toggle events: 2"

    $radioOne = Wait-Control $hostRun.Process (
        New-Object System.Windows.Automation.PropertyCondition(
            [System.Windows.Automation.AutomationElement]::NameProperty,
            "Native radio one"
        )
    ) "first individual RadioButton"
    $radioTwo = Wait-Control $hostRun.Process (
        New-Object System.Windows.Automation.PropertyCondition(
            [System.Windows.Automation.AutomationElement]::NameProperty,
            "Native radio two"
        )
    ) "second individual RadioButton"
    $firstSelection = $radioOne.GetCurrentPattern(
        [System.Windows.Automation.SelectionItemPattern]::Pattern
    )
    $secondSelection = $radioTwo.GetCurrentPattern(
        [System.Windows.Automation.SelectionItemPattern]::Pattern
    )
    if (!$firstSelection.Current.IsSelected -or $secondSelection.Current.IsSelected) {
        throw "unexpected initial individual RadioButton selection"
    }
    $secondSelection.Select()
    $null = Wait-Text $hostRun.Process "Radio selection: 1"
    if ($firstSelection.Current.IsSelected -or !$secondSelection.Current.IsSelected) {
        throw "individual RadioButton group did not update native selection"
    }

    $programmatic = Wait-Control $hostRun.Process (
        Button-Condition "Programmatic controls"
    ) "programmatic controls Button"
    $programmatic.GetCurrentPattern(
        [System.Windows.Automation.InvokePattern]::Pattern
    ).Invoke()
    $null = Wait-Text $hostRun.Process "Text value: programmatic"
    $null = Wait-Text $hostRun.Process "Checked value: true"
    $null = Wait-Text $hostRun.Process "NumberBox value: empty"
    $null = Wait-Text $hostRun.Process "Rating value: empty"
    $null = Wait-Text $hostRun.Process "Color value: #0000FF"
    $null = Wait-Text $hostRun.Process "DatePicker value: empty"
    $null = Wait-Text $hostRun.Process "CalendarDatePicker value: empty"
    $null = Wait-Text $hostRun.Process "Choose a calendar date"
    Start-Sleep -Milliseconds 200
    $null = Wait-Text $hostRun.Process "Text events: 2"
    $null = Wait-Text $hostRun.Process "Toggle events: 2"
    $null = Wait-Text $hostRun.Process "NumberBox events: 1"
    $null = Wait-Text $hostRun.Process "Rating events: 1"
    $null = Wait-Text $hostRun.Process "Color events: 1"
    $null = Wait-Text $hostRun.Process "DatePicker events: 1"
    $null = Wait-Text $hostRun.Process "CalendarDatePicker events: 1"
    }

    if ($Case -eq "collections") {
        $scroll = $list.GetCurrentPattern(
        [System.Windows.Automation.ScrollPattern]::Pattern
    )
    $scroll.SetScrollPercent(
        [System.Windows.Automation.ScrollPattern]::NoScroll,
        100.0
    )
    Start-Sleep -Milliseconds 200
    $items = $list.FindAll(
        [System.Windows.Automation.TreeScope]::Descendants,
        $itemCondition
    )
    $selected = $items.Item($items.Count - 1)
    $selection = $selected.GetCurrentPattern(
        [System.Windows.Automation.SelectionItemPattern]::Pattern
    )
    $selection.Select()
    $selectedName = $selected.Current.Name
    Start-Sleep -Milliseconds 50
    $hostSelection = $list.GetCurrentPattern(
        [System.Windows.Automation.SelectionPattern]::Pattern
    ).Current.GetSelection()
    if ($hostSelection.Count -ne 1 -or $hostSelection.Item(0).Current.Name -ne $selectedName) {
        throw "ListView SelectionPattern does not report the selected realized item"
    }

    $scroll.SetScrollPercent(
        [System.Windows.Automation.ScrollPattern]::NoScroll,
        0.0
    )
    Start-Sleep -Milliseconds 200
    $list = Wait-Control $hostRun.Process $listCondition "ListView after scrolling to top"
    $scroll = $list.GetCurrentPattern(
        [System.Windows.Automation.ScrollPattern]::Pattern
    )
    $scroll.SetScrollPercent(
        [System.Windows.Automation.ScrollPattern]::NoScroll,
        100.0
    )
    Start-Sleep -Milliseconds 200
    $selected = Find-Text $list $selectedName
    if ($null -eq $selected) {
        throw "selected item '$selectedName' was not realized again"
    }
    $selection = $selected.GetCurrentPattern(
        [System.Windows.Automation.SelectionItemPattern]::Pattern
    )
    if (!$selection.Current.IsSelected) {
        throw "native selection was not preserved through recycling"
    }

    1..50 | ForEach-Object {
        $list = Wait-Control $hostRun.Process $listCondition "ListView during close stress"
        $scroll = $list.GetCurrentPattern(
            [System.Windows.Automation.ScrollPattern]::Pattern
        )
        $scroll.SetScrollPercent(
            [System.Windows.Automation.ScrollPattern]::NoScroll,
            $(if ($_ % 2) { 0.0 } else { 100.0 })
        )
    }
    Start-Sleep -Milliseconds 300
    $realized = $list.FindAll(
        [System.Windows.Automation.TreeScope]::Descendants,
        $itemCondition
    ).Count
        if ($realized -lt 10 -or $realized -gt 100) {
            throw "unexpected realized item count after scroll stress: $realized"
        }

        $clear = Wait-Control $hostRun.Process (Button-Condition "Rows clear") "Rows clear Button"
        $clear.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        $null = Wait-Text $hostRun.Process "Rows: 0"
        $list = Wait-Control $hostRun.Process $listCondition "empty ListView"
        $empty = Find-Text $list "No rows available"
        if ($null -eq $empty -or $empty.Current.IsOffscreen) {
            throw "ListView empty state is not exposed as visible automation content"
        }
        if ($list.FindAll(
            [System.Windows.Automation.TreeScope]::Descendants,
            $itemCondition
        ).Count -ne 0) {
            throw "empty ListView still exposes realized ListViewItem peers"
        }
        $selectionPattern = $list.GetCurrentPattern(
            [System.Windows.Automation.SelectionPattern]::Pattern
        )
        if ($selectionPattern.Current.GetSelection().Count -ne 0) {
            throw "empty ListView retains a native automation selection"
        }

        $button = Wait-Control $hostRun.Process (Button-Condition "Rows toggle") "Rows toggle Button"
        $button.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        $null = Wait-Text $hostRun.Process "Rows: 5000"
        $list = Wait-Control $hostRun.Process $listCondition "repopulated ListView"
        if ($null -ne (Find-Text $list "No rows available")) {
            throw "repopulated ListView still exposes the empty state"
        }
    }
} finally {
    if ($dragSourceStarted) {
        [DragSource]::Stop()
    }
    Stop-Host $hostRun
    Remove-Item $normalError -ErrorAction SilentlyContinue
}
if ($hostRun.Process.ExitCode -ne 0) {
    throw "$Case native self-test exited with $($hostRun.Process.ExitCode)"
}

Write-Output "windows-reactor native $Case self-test passed"
