param(
    [ValidateSet("debug", "release")]
    [string]$Profile = "release",
    [string]$Sample
)

$ErrorActionPreference = "Stop"

Add-Type -AssemblyName UIAutomationClient
Add-Type -AssemblyName UIAutomationTypes
Add-Type -AssemblyName System.Windows.Forms
Add-Type @"
using System;
using System.Runtime.InteropServices;

public static class NativePointer {
    [DllImport("user32.dll")]
    public static extern bool SetCursorPos(int x, int y);

    [DllImport("user32.dll")]
    public static extern bool SetForegroundWindow(IntPtr window);

    [DllImport("user32.dll")]
    public static extern void mouse_event(uint flags, uint dx, uint dy, uint data, UIntPtr extra);
}
"@

$root = Resolve-Path (Join-Path $PSScriptRoot "..\..\..\..")
$cargoArgs = @(
    "build",
    "-p", "reactor_samples",
    "--profile", $Profile,
    "--example", "command_bar",
    "--example", "command_bar_flyout",
    "--example", "content_dialog",
    "--example", "counter",
    "--example", "card",
    "--example", "calculator",
    "--example", "button_icon",
    "--example", "button_icon_dynamic",
    "--example", "button_icon_glyph_change",
    "--example", "calendar_view",
    "--example", "icon_elements",
    "--example", "expander",
    "--example", "exit_transition",
    "--example", "flyout",
    "--example", "flip_view",
    "--example", "keyed_list_reorder",
    "--example", "info_badge",
    "--example", "info_bar",
    "--example", "image_icon_size",
    "--example", "list_view",
    "--example", "lightweight_resources",
    "--example", "memo_widget_descendant",
    "--example", "menu_bar",
    "--example", "menu_flyout",
    "--example", "navigation",
    "--example", "navigation_view",
    "--example", "navigation_view_icons",
    "--example", "navigation_view_pane",
    "--example", "opacity_transition",
    "--example", "person_picture",
    "--example", "pivot",
    "--example", "pointer_position",
    "--example", "radio_button",
    "--example", "responsive_navigation",
    "--example", "rich_edit_box",
    "--example", "rich_text",
    "--example", "scale_transition",
    "--example", "shape",
    "--example", "split_view",
    "--example", "split_button",
    "--example", "tab_view",
    "--example", "tab_view_add_button",
    "--example", "tab_view_item_key",
    "--example", "theme_brush",
    "--example", "tictactoe",
    "--example", "text_box_border",
    "--example", "tooltip_placement",
    "--example", "time_picker",
    "--example", "title_bar",
    "--example", "tree_view",
    "--example", "use_color_scheme",
    "--example", "use_resource",
    "--example", "use_resource_retry",
    "--quiet"
)
& cargo @cargoArgs
if ($LASTEXITCODE -ne 0) {
    throw "failed to build Reactor sample smoke tests"
}

function Wait-Until([scriptblock]$Action, [string]$Description) {
    $deadline = [DateTime]::UtcNow.AddSeconds(15)
    do {
        $result = & $Action
        if ($null -ne $result) {
            return $result
        }
        Start-Sleep -Milliseconds 50
    } while ([DateTime]::UtcNow -lt $deadline)
    throw "timed out waiting for $Description"
}

function Wait-Window([System.Diagnostics.Process]$Process) {
    $processCondition = New-Object System.Windows.Automation.PropertyCondition(
        [System.Windows.Automation.AutomationElement]::ProcessIdProperty,
        $Process.Id
    )
    $windowCondition = New-Object System.Windows.Automation.PropertyCondition(
        [System.Windows.Automation.AutomationElement]::ControlTypeProperty,
        [System.Windows.Automation.ControlType]::Window
    )
    $condition = New-Object System.Windows.Automation.AndCondition(
        $processCondition,
        $windowCondition
    )
    Wait-Until {
        [System.Windows.Automation.AutomationElement]::RootElement.FindFirst(
            [System.Windows.Automation.TreeScope]::Children,
            $condition
        )
    } "sample window"
}

function Wait-Control(
    [System.Windows.Automation.AutomationElement]$Window,
    [System.Windows.Automation.Condition]$Condition,
    [string]$Description
) {
    Wait-Until {
        $Window.FindFirst(
            [System.Windows.Automation.TreeScope]::Descendants,
            $Condition
        )
    } $Description
}

function Wait-Process-Control(
    [System.Diagnostics.Process]$Process,
    [System.Windows.Automation.Condition]$Condition,
    [string]$Description
) {
    $processCondition = New-Object System.Windows.Automation.PropertyCondition(
        [System.Windows.Automation.AutomationElement]::ProcessIdProperty,
        $Process.Id
    )
    $combined = New-Object System.Windows.Automation.AndCondition(
        $processCondition,
        $Condition
    )
    Wait-Until {
        [System.Windows.Automation.AutomationElement]::RootElement.FindFirst(
            [System.Windows.Automation.TreeScope]::Descendants,
            $combined
        )
    } $Description
}

function Name-Condition([string]$Name) {
    New-Object System.Windows.Automation.PropertyCondition(
        [System.Windows.Automation.AutomationElement]::NameProperty,
        $Name
    )
}

function Id-Condition([string]$Id) {
    New-Object System.Windows.Automation.PropertyCondition(
        [System.Windows.Automation.AutomationElement]::AutomationIdProperty,
        $Id
    )
}

function Stop-Sample(
    [System.Diagnostics.Process]$Process,
    [System.Windows.Automation.AutomationElement]$Window
) {
    if (!$Process.HasExited) {
        $Window.GetCurrentPattern(
            [System.Windows.Automation.WindowPattern]::Pattern
        ).Close()
        if (!$Process.WaitForExit(5000)) {
            Stop-Process -Id $Process.Id
            throw "sample did not exit after its window closed"
        }
    }
}

function Click-Control(
    [System.Diagnostics.Process]$Process,
    [System.Windows.Automation.AutomationElement]$Control
) {
    $rect = $Control.Current.BoundingRectangle
    $x = [int]($rect.X + ($rect.Width / 2))
    $y = [int]($rect.Y + ($rect.Height / 2))
    [NativePointer]::SetForegroundWindow($Process.MainWindowHandle) | Out-Null
    [NativePointer]::SetCursorPos($x, $y) | Out-Null
    [NativePointer]::mouse_event(0x0002, 0, 0, 0, [UIntPtr]::Zero)
    [NativePointer]::mouse_event(0x0004, 0, 0, 0, [UIntPtr]::Zero)
}

function Run-Sample([string]$Name, [scriptblock]$Test) {
    if ($Sample -and $Name -ne $Sample) {
        return
    }
    $executable = Join-Path $root "target\$Profile\examples\$Name.exe"
    $process = Start-Process -FilePath $executable -PassThru
    $window = $null
    try {
        $window = Wait-Window $process
        & $Test $window $process
    } finally {
        if ($null -ne $window) {
            Stop-Sample $process $window
        } elseif (!$process.HasExited) {
            Stop-Process -Id $process.Id
        }
    }
}

Run-Sample "command_bar" {
    param($window)
    $add = Wait-Control $window (Name-Condition "Add") "CommandBar Add button"
    $add.GetCurrentPattern(
        [System.Windows.Automation.InvokePattern]::Pattern
    ).Invoke()
    $null = Wait-Control $window (
        Name-Condition "Last clicked: Add; pinned: false"
    ) "CommandBar callback status"
}

Run-Sample "command_bar_flyout" {
    param($window, $process)
    $show = Wait-Control $window (Name-Condition "Show Commands") "CommandBarFlyout owner"
    Click-Control $process $show
    $paste = Wait-Process-Control $process (Name-Condition "Paste") "CommandBarFlyout Paste command"
    $paste.GetCurrentPattern(
        [System.Windows.Automation.InvokePattern]::Pattern
    ).Invoke()
    $null = Wait-Control $window (
        Name-Condition "Last action: Paste"
    ) "CommandBarFlyout callback"
}

Run-Sample "card" {
    param($window)
    $null = Wait-Control $window (Name-Condition "Sharp") "sharp card"
    $null = Wait-Control $window (Name-Condition "Rounded") "rounded card"
    $null = Wait-Control $window (Name-Condition "Pill") "pill card"
}

Run-Sample "theme_brush" {
    param($window)
    $null = Wait-Control $window (Name-Condition "Accent / AccentText") "accent theme brush"
    $null = Wait-Control $window (Name-Condition "Card / Primary text") "card theme brush"
}

Run-Sample "lightweight_resources" {
    param($window)
    $clear = Wait-Control $window (Name-Condition "Clear resources") "resource clear button"
    $clear.GetCurrentPattern(
        [System.Windows.Automation.InvokePattern]::Pattern
    ).Invoke()
    $null = Wait-Control $window (Name-Condition "Apply resources") "resource apply button"
}

Run-Sample "use_color_scheme" {
    param($window)
    $null = Wait-Control $window (
        New-Object System.Windows.Automation.OrCondition(
            (Name-Condition "color_scheme  = Light"),
            (Name-Condition "color_scheme  = Dark")
        )
    ) "resolved color scheme"
}

Run-Sample "opacity_transition" {
    param($window)
    $fade = Wait-Control $window (Name-Condition "Fade out") "opacity transition button"
    $fade.GetCurrentPattern(
        [System.Windows.Automation.InvokePattern]::Pattern
    ).Invoke()
    $null = Wait-Control $window (Name-Condition "Fade in") "updated opacity state"
}

Run-Sample "scale_transition" {
    param($window)
    $scale = Wait-Control $window (Name-Condition "Scale up") "scale transition button"
    $scale.GetCurrentPattern(
        [System.Windows.Automation.InvokePattern]::Pattern
    ).Invoke()
    $null = Wait-Control $window (Name-Condition "Scale down") "updated scale state"
}

Run-Sample "exit_transition" {
    param($window)
    $remove = Wait-Control $window (Name-Condition "Remove") "exit transition button"
    $remove.GetCurrentPattern(
        [System.Windows.Automation.InvokePattern]::Pattern
    ).Invoke()
    $restore = Wait-Control $window (Name-Condition "Restore") "exit transition restore button"
    $null = Wait-Until {
        $content = $window.FindFirst(
            [System.Windows.Automation.TreeScope]::Descendants,
            (Name-Condition "This visual remains visible while its exit animation completes.")
        )
        if ($null -eq $content) {
            return $restore
        }
        return $null
    } "exit transition completion"
    $restore.GetCurrentPattern(
        [System.Windows.Automation.InvokePattern]::Pattern
    ).Invoke()
    $null = Wait-Control $window (
        Name-Condition "This visual remains visible while its exit animation completes."
    ) "restored exit transition content"
}

Run-Sample "calculator" {
    param($window)
    $seven = Wait-Control $window (Name-Condition "7") "calculator seven button"
    $beforeWidth = $seven.Current.BoundingRectangle.Width
    $windowBounds = $window.Current.BoundingRectangle
    $transform = $window.GetCurrentPattern(
        [System.Windows.Automation.TransformPattern]::Pattern
    )
    if (!$transform.Current.CanResize) {
        throw "calculator window is not resizable"
    }
    $transform.Resize($windowBounds.Width + 120, $windowBounds.Height + 100)
    $null = Wait-Until {
        if ($seven.Current.BoundingRectangle.Width -gt $beforeWidth + 20) {
            return $seven
        }
        return $null
    } "calculator buttons to resize with the window"
    foreach ($button in @(
        $seven,
        (Wait-Control $window (Name-Condition "+") "calculator plus button"),
        (Wait-Control $window (Name-Condition "8") "calculator eight button")
    )) {
        $button.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
    }
    $equals = Wait-Control $window (
        Name-Condition "="
    ) "calculator equals button"
    $equals.GetCurrentPattern(
        [System.Windows.Automation.InvokePattern]::Pattern
    ).Invoke()
    $null = Wait-Control $window (Name-Condition "15") "calculator result"
}

Run-Sample "title_bar" {
    param($window)
    $back = Wait-Control $window (Id-Condition "PART_BackButton") "title-bar back button"
    $back.GetCurrentPattern(
        [System.Windows.Automation.InvokePattern]::Pattern
    ).Invoke()
    $null = Wait-Control $window (
        Name-Condition "back_clicks = 1, pane_toggle_clicks = 0"
    ) "title-bar back callback"

    $pane = Wait-Control $window (
        Id-Condition "PART_PaneToggleButton"
    ) "title-bar pane-toggle button"
    $pane.GetCurrentPattern(
        [System.Windows.Automation.InvokePattern]::Pattern
    ).Invoke()
    $null = Wait-Control $window (
        Name-Condition "back_clicks = 1, pane_toggle_clicks = 1"
    ) "title-bar pane callback"

    $system = Wait-Control $window (
        Name-Condition "Use system title bar"
    ) "system title-bar switch"
    $system.GetCurrentPattern(
        [System.Windows.Automation.InvokePattern]::Pattern
    ).Invoke()
    $custom = Wait-Control $window (
        Name-Condition "Use custom title bar"
    ) "custom title-bar switch"
    $custom.GetCurrentPattern(
        [System.Windows.Automation.InvokePattern]::Pattern
    ).Invoke()
    $null = Wait-Control $window (
        Id-Condition "PART_BackButton"
    ) "remounted title-bar back button"
}

Run-Sample "content_dialog" {
    param($window, $process)
    $open = Wait-Control $window (Name-Condition "Open dialog") "open dialog button"
    $open.GetCurrentPattern(
        [System.Windows.Automation.InvokePattern]::Pattern
    ).Invoke()
    $delete = Wait-Process-Control $process (Name-Condition "Delete") "dialog primary button"
    $delete.GetCurrentPattern(
        [System.Windows.Automation.InvokePattern]::Pattern
    ).Invoke()
    $null = Wait-Control $window (
        Name-Condition "You picked: Delete"
    ) "ContentDialog callback result"
}

Run-Sample "pointer_position" {
    param($window, $process)
    $target = Wait-Control $window (
        Name-Condition "Click to read the pointer position"
    ) "pointer position target"
    $status = Wait-Control $window (
        Name-Condition "Click anywhere in the box"
    ) "pointer position status"
    Click-Control $process $target
    $null = Wait-Until {
        if ($status.Current.Name.StartsWith("Pressed at (")) {
            return $status
        }
        return $null
    } "pointer position callback"
}

Run-Sample "pivot" {
    param($window, $process)
    $second = Wait-Control $window (
        Name-Condition "Pivot - second tab"
    ) "Pivot second item"
    $second.GetCurrentPattern(
        [System.Windows.Automation.SelectionItemPattern]::Pattern
    ).Select()
    $null = Wait-Control $window (
        Name-Condition "selected_index = 1"
    ) "Pivot selection callback"
}

Run-Sample "flip_view" {
    param($window, $process)
    $first = Wait-Control $window (Name-Condition "Red") "FlipView first page"
    $next = Wait-Control $window (Id-Condition "next-page") "FlipView next button"
    $next.GetCurrentPattern(
        [System.Windows.Automation.InvokePattern]::Pattern
    ).Invoke()
    $null = Wait-Control $window (Name-Condition "page = 1") "FlipView controlled selection"
    $null = Wait-Control $window (Name-Condition "Green") "FlipView second page"
    $first.GetCurrentPattern(
        [System.Windows.Automation.SelectionItemPattern]::Pattern
    ).Select()
    $null = Wait-Control $window (Name-Condition "page = 0") "FlipView selection callback"
}

Run-Sample "tab_view" {
    param($window, $process)
    $badges = Wait-Control $window (
        Name-Condition "Tab content - Badges"
    ) "TabView Badges tab"
    $badges.GetCurrentPattern(
        [System.Windows.Automation.SelectionItemPattern]::Pattern
    ).Select()
    $null = Wait-Control $window (
        Name-Condition "selected_index = 1, tabs remaining = 3"
    ) "TabView selection callback"
    $close = Wait-Control $window (Id-Condition "CloseButton") "TabView close button"
    $close.GetCurrentPattern(
        [System.Windows.Automation.InvokePattern]::Pattern
    ).Invoke()
    $null = Wait-Control $window (
        Name-Condition "selected_index = 1, tabs remaining = 2"
    ) "TabView close callback"
}

Run-Sample "tab_view_add_button" {
    param($window, $process)
    $add = Wait-Control $window (Id-Condition "AddButton") "TabView add button"
    $add.GetCurrentPattern(
        [System.Windows.Automation.InvokePattern]::Pattern
    ).Invoke()
    $null = Wait-Control $window (
        Name-Condition "selected = 2, total tabs = 3"
    ) "TabView add callback"
    $null = Wait-Control $window (Name-Condition "Content for Tab 3") "added TabView item"
}

Run-Sample "tab_view_item_key" {
    param($window, $process)
    $rename = Wait-Control $window (Name-Condition "Rename tab") "TabView rename button"
    $rename.GetCurrentPattern(
        [System.Windows.Automation.InvokePattern]::Pattern
    ).Invoke()
    $null = Wait-Control $window (Name-Condition "Renamed document") "renamed TabView item"
    $close = Wait-Control $window (Id-Condition "CloseButton") "TabView keyed close button"
    $close.GetCurrentPattern(
        [System.Windows.Automation.InvokePattern]::Pattern
    ).Invoke()
    $null = Wait-Control $window (
        Name-Condition "configured key: 42; last close request: 42"
    ) "TabView stable close key"
}

Run-Sample "counter" {
    param($window)
    $increment = Wait-Control $window (Id-Condition "increment-button") "counter increment button"
    $increment.GetCurrentPattern(
        [System.Windows.Automation.InvokePattern]::Pattern
    ).Invoke()
    $null = Wait-Control $window (Name-Condition "Count: 1") "updated counter value"
}

Run-Sample "calendar_view" {
    param($window)
    $null = Wait-Control $window (
        Name-Condition "Selection changed 0 time(s)"
    ) "CalendarView status"
}

Run-Sample "icon_elements" {
    param($window)
    $null = Wait-Control $window (
        Name-Condition "Symbol icon (SymbolIcon)."
    ) "NavigationView initial content"
    $path = Wait-Control $window (Name-Condition "Path") "NavigationView Path item"
    $path.GetCurrentPattern(
        [System.Windows.Automation.SelectionItemPattern]::Pattern
    ).Select()
    $null = Wait-Control $window (
        Name-Condition "Vector path data (PathIcon)."
    ) "NavigationView controlled selection"
    $settings = Wait-Control $window (Name-Condition "Settings") "NavigationView Settings item"
    $settings.GetCurrentPattern(
        [System.Windows.Automation.SelectionItemPattern]::Pattern
    ).Select()
    Start-Sleep -Milliseconds 100
    if ($settings.GetCurrentPattern(
            [System.Windows.Automation.SelectionItemPattern]::Pattern
        ).Current.IsSelected) {
        throw "NavigationView Settings item escaped controlled selection"
    }
    if (!$path.GetCurrentPattern(
            [System.Windows.Automation.SelectionItemPattern]::Pattern
        ).Current.IsSelected) {
        throw "NavigationView controlled item was not restored after Settings"
    }
}

Run-Sample "navigation" {
    param($window)
    $null = Wait-Control $window (Name-Condition "Welcome Home") "Navigation initial page"
    $dashboard = Wait-Control $window (Name-Condition "Dashboard") "Navigation Dashboard item"
    $dashboard.GetCurrentPattern(
        [System.Windows.Automation.SelectionItemPattern]::Pattern
    ).Select()
    $null = Wait-Control $window (Name-Condition "Users online: 1,234") "Navigation async dashboard"
    $settings = Wait-Control $window (Name-Condition "Settings") "Navigation Settings item"
    $settings.GetCurrentPattern(
        [System.Windows.Automation.SelectionItemPattern]::Pattern
    ).Select()
    $null = Wait-Control $window (
        Name-Condition "Dark: off | Notifications: on"
    ) "Navigation settings state"
    $homeItem = Wait-Control $window (Name-Condition "Home") "Navigation Home item"
    $homeItem.GetCurrentPattern(
        [System.Windows.Automation.SelectionItemPattern]::Pattern
    ).Select()
    $null = Wait-Control $window (Name-Condition "Welcome Home") "Navigation controlled return"
}

Run-Sample "navigation_view" {
    param($window)
    $null = Wait-Control $window (Name-Condition "page: home") "NavigationView initial header"
    $null = Wait-Control $window (Name-Condition "Home page") "NavigationView initial content"
    $about = Wait-Control $window (Name-Condition "About") "NavigationView About item"
    $about.GetCurrentPattern(
        [System.Windows.Automation.SelectionItemPattern]::Pattern
    ).Select()
    $null = Wait-Control $window (Name-Condition "page: about") "NavigationView updated header"
    $null = Wait-Control $window (Name-Condition "About page") "NavigationView updated content"
    $settings = Wait-Control $window (Name-Condition "Settings") "NavigationView Settings item"
    $settings.GetCurrentPattern(
        [System.Windows.Automation.SelectionItemPattern]::Pattern
    ).Select()
    $null = Wait-Control $window (Name-Condition "page: settings") "NavigationView Settings header"
    $null = Wait-Control $window (Name-Condition "Settings page") "NavigationView Settings content"
}

Run-Sample "navigation_view_icons" {
    param($window)
    $null = Wait-Control $window (Name-Condition "Welcome home!") "NavigationViewIcons initial page"
    $mail = Wait-Control $window (Name-Condition "Mail") "NavigationViewIcons Mail item"
    $mail.GetCurrentPattern(
        [System.Windows.Automation.SelectionItemPattern]::Pattern
    ).Select()
    $null = Wait-Control $window (Name-Condition "Mail inbox") "NavigationViewIcons Mail page"
    $people = Wait-Control $window (Name-Condition "People") "NavigationViewIcons People item"
    $people.GetCurrentPattern(
        [System.Windows.Automation.SelectionItemPattern]::Pattern
    ).Select()
    $null = Wait-Control $window (Name-Condition "Contacts") "NavigationViewIcons People page"
}

Run-Sample "navigation_view_pane" {
    param($window)
    $null = Wait-Control $window (Name-Condition "Home page") "NavigationViewPane initial page"
    $documents = Wait-Control $window (Name-Condition "Documents") "NavigationViewPane Documents item"
    $documents.GetCurrentPattern(
        [System.Windows.Automation.SelectionItemPattern]::Pattern
    ).Select()
    $null = Wait-Control $window (Name-Condition "Documents page") "NavigationViewPane Documents page"
    $signOut = Wait-Control $window (Name-Condition "Sign out") "NavigationViewPane footer"
    $signOut.GetCurrentPattern(
        [System.Windows.Automation.InvokePattern]::Pattern
    ).Invoke()
    $null = Wait-Control $window (Name-Condition "Signed out") "NavigationViewPane footer callback"
}

Run-Sample "responsive_navigation" {
    param($window)
    $null = Wait-Control $window (Name-Condition "Pane is open") "responsive pane initial state"
    $toggle = Wait-Control $window (Name-Condition "Toggle pane") "responsive pane toggle"
    $toggle.GetCurrentPattern(
        [System.Windows.Automation.InvokePattern]::Pattern
    ).Invoke()
    $null = Wait-Control $window (Name-Condition "Pane is closed") "responsive pane controlled state"
    $bounds = $window.Current.BoundingRectangle
    $transform = $window.GetCurrentPattern(
        [System.Windows.Automation.TransformPattern]::Pattern
    )
    if (!$transform.Current.CanResize) {
        throw "responsive navigation window is not resizable"
    }
    $transform.Resize(500, $bounds.Height)
    $modeCondition = New-Object System.Windows.Automation.OrCondition(
        (Name-Condition "Actual display mode: compact"),
        (Name-Condition "Actual display mode: minimal")
    )
    $null = Wait-Control $window $modeCondition "responsive display-mode callback"
    $null = Wait-Control $window (Name-Condition "AD") "responsive footer"
}

Run-Sample "rich_edit_box" {
    param($window, $process)
    $editor = Wait-Control $window (Id-Condition "editor") "RichEditBox editor"
    Click-Control $process $editor
    [System.Windows.Forms.SendKeys]::SendWait("Hello Reactor")
    $status = Wait-Control $window (
        Id-Condition "plain-text"
    ) "RichEditBox controlled text status"
    $null = Wait-Until {
        if ($status.Current.Name -eq "Plain text: Hello Reactor") {
            return $status
        }
        $null
    } "RichEditBox controlled text callback"
    $readOnly = Wait-Control $window (
        Id-Condition "read-only-editor"
    ) "read-only RichEditBox"
}

Run-Sample "rich_text" {
    param($window)
    $null = Wait-Control $window (Id-Condition "mixed-text") "mixed RichTextBlock"
    $null = Wait-Control $window (Id-Condition "multi-text") "multi-paragraph RichTextBlock"
}

Run-Sample "tree_view" {
    param($window, $process)
    $documents = Wait-Control $window (Name-Condition "Documents") "TreeView Documents node"
    Click-Control $process $documents
    $null = Wait-Control $window (
        Name-Condition "Last invoked: Documents"
    ) "TreeView item callback"
    $null = Wait-Control $window (Name-Condition "Work") "TreeView child node"
    $null = Wait-Control $window (Name-Condition "Personal") "TreeView sibling node"
}

Run-Sample "keyed_list_reorder" {
    param($window)
    $increment = Wait-Control $window (
        Name-Condition "Increment Alpha"
    ) "Alpha row increment button"
    $increment.GetCurrentPattern(
        [System.Windows.Automation.InvokePattern]::Pattern
    ).Invoke()
    $null = Wait-Control $window (Name-Condition "Alpha: 1") "updated Alpha row state"
    $rotate = Wait-Control $window (Name-Condition "Rotate rows") "row rotation button"
    $rotate.GetCurrentPattern(
        [System.Windows.Automation.InvokePattern]::Pattern
    ).Invoke()
    $null = Wait-Control $window (
        Name-Condition "Order: Beta, Gamma, Alpha"
    ) "rotated row order"
    $null = Wait-Control $window (
        Name-Condition "Alpha: 1"
    ) "Alpha state retained after rotation"
}

Run-Sample "info_badge" {
    param($window)
    $null = Wait-Control $window (Name-Condition "Attention indicator") "dot InfoBadge"
    $null = Wait-Control $window (Name-Condition "Count 42") "numeric InfoBadge"
    $null = Wait-Control $window (Name-Condition "Count 999") "large numeric InfoBadge"
}

Run-Sample "info_bar" {
    param($window)
    $null = Wait-Control $window (Id-Condition "controlled-info-bar") "controlled InfoBar"
    $close = Wait-Control $window (Name-Condition "Close") "InfoBar close button"
    $close.GetCurrentPattern(
        [System.Windows.Automation.InvokePattern]::Pattern
    ).Invoke()
    $null = Wait-Control $window (
        Name-Condition "Status: Close requested"
    ) "controlled InfoBar close request"
    $show = Wait-Control $window (Id-Condition "show-info-bar") "InfoBar show button"
    $show.GetCurrentPattern(
        [System.Windows.Automation.InvokePattern]::Pattern
    ).Invoke()
    $null = Wait-Control $window (Name-Condition "Status: Open") "controlled InfoBar reopen"
}

Run-Sample "person_picture" {
    param($window)
    $null = Wait-Control $window (
        Name-Condition "Display name (initials derived by WinUI)"
    ) "PersonPicture display-name section"
    $null = Wait-Control $window (
        Name-Condition "Explicit initials"
    ) "PersonPicture initials section"
}

Run-Sample "button_icon" {
    param($window, $process)
    $add = Wait-Control $window (Name-Condition "Add Item") "icon button"
    Click-Control $process $add
    $null = Wait-Control $window (Name-Condition "Count: 1") "icon button count"
}

Run-Sample "button_icon_dynamic" {
    param($window, $process)
    $button = Wait-Control $window (Name-Condition "Clicked 0 times") "dynamic icon button"
    Click-Control $process $button
    $null = Wait-Control $window (Name-Condition "Clicked 1 times") "updated icon button label"
    $null = Wait-Control $window (Name-Condition "Saved!") "retained save icon button"
}

Run-Sample "button_icon_glyph_change" {
    param($window, $process)
    $button = Wait-Control $window (Name-Condition "Toggle Icon") "icon toggle button"
    Click-Control $process $button
    $null = Wait-Control $window (Name-Condition "Current icon: Save") "updated icon status"
}

Run-Sample "image_icon_size" {
    param($window)
    $null = Wait-Control $window (Name-Condition "SVG image icon") "image icon button"
    $null = Wait-Control $window (
        Name-Condition "The same source in an Image control:"
    ) "image icon comparison"
}

Run-Sample "shape" {
    param($window)
    $null = Wait-Control $window (
        Name-Condition "Rectangle (fill + corner radius)"
    ) "filled rectangle section"
    $null = Wait-Control $window (
        Name-Condition "Line (stroke + stroke thickness)"
    ) "line section"
    $null = Wait-Control $window (
        Name-Condition "Rectangle outline (stroke, no fill)"
    ) "outlined rectangle section"
}

Run-Sample "text_box_border" {
    param($window)
    $null = Wait-Control $window (Name-Condition "Default style") "default TextBox"
    $null = Wait-Control $window (Name-Condition "Thick blue border") "bordered TextBox"
    $null = Wait-Control $window (Name-Condition "Type a message...") "borderless TextBox"
}

# Tooltip geometry is verified by native property readback. UI Automation only checks the owners
# because popup placement cannot be inferred reliably from the tooltip automation bounds.
Run-Sample "tooltip_placement" {
    param($window)
    foreach ($name in @("Top", "Bottom", "Left", "Right", "Mouse")) {
        $null = Wait-Control $window (Name-Condition $name) "$name tooltip owner"
    }

    Run-Sample "time_picker" {
        param($window)
        $null = Wait-Control $window (Name-Condition "Pick a time") "TimePicker"
        $null = Wait-Control $window (Name-Condition "No time picked") "TimePicker status"
    }
}

# TODO: Add ListView drag automation when the input harness can initiate WinUI item drags.

Run-Sample "memo_widget_descendant" {
    param($window)
    $increment = Wait-Control $window (
        Name-Condition "Increment child"
    ) "memoized child increment button"
    $increment.GetCurrentPattern(
        [System.Windows.Automation.InvokePattern]::Pattern
    ).Invoke()
    $null = Wait-Control $window (
        Name-Condition "Child count: 1"
    ) "updated memoized child"
    $rerender = Wait-Control $window (
        Name-Condition "Rerender parent"
    ) "parent rerender button"
    $rerender.GetCurrentPattern(
        [System.Windows.Automation.InvokePattern]::Pattern
    ).Invoke()
    $null = Wait-Control $window (
        Name-Condition "Parent renders: 1"
    ) "updated parent render count"
    $null = Wait-Control $window (
        Name-Condition "Child count: 1"
    ) "child state retained through parent render"
}

Run-Sample "radio_button" {
    param($window)
    $medium = Wait-Control $window (Name-Condition "Medium") "individual RadioButton"
    $medium.GetCurrentPattern(
        [System.Windows.Automation.SelectionItemPattern]::Pattern
    ).Select()
    $null = Wait-Control $window (Name-Condition "size = Medium") "updated radio selection"
}

Run-Sample "split_view" {
    param($window)
    $toggle = Wait-Control $window (Name-Condition "Toggle Pane") "SplitView toggle button"
    $invoke = $toggle.GetCurrentPattern(
        [System.Windows.Automation.InvokePattern]::Pattern
    )
    $invoke.Invoke()
    $null = Wait-Control $window (Name-Condition "Pane is closed") "closed SplitView status"
    $invoke.Invoke()
    $null = Wait-Control $window (Name-Condition "Pane is open") "open SplitView status"
}

Run-Sample "split_button" {
    param($window, $process)
    $primary = Wait-Control $window (Name-Condition "Primary action (0)") "SplitButton primary action"
    $primary.GetCurrentPattern(
        [System.Windows.Automation.InvokePattern]::Pattern
    ).Invoke()
    $null = Wait-Control $window (
        Name-Condition "Primary action (1)"
    ) "updated SplitButton action"
    $updated = Wait-Control $window (
        Name-Condition "Primary action (1)"
    ) "updated SplitButton flyout owner"
    $updated.GetCurrentPattern(
        [System.Windows.Automation.ExpandCollapsePattern]::Pattern
    ).Expand()
    $null = Wait-Process-Control $process (
        Name-Condition "Secondary action (1)"
    ) "SplitButton secondary flyout"
}

Run-Sample "flyout" {
    param($window, $process)
    $increment = Wait-Control $window (Name-Condition "Increment") "flyout increment button"
    $increment.GetCurrentPattern(
        [System.Windows.Automation.InvokePattern]::Pattern
    ).Invoke()
    Start-Sleep -Milliseconds 100
    $open = Wait-Control $window (Name-Condition "Bottom Flyout") "flyout owner button"
    Click-Control $process $open
    $null = Wait-Process-Control $process (
        Name-Condition "Clicked 1 times"
    ) "updated flyout content"
}

Run-Sample "menu_flyout" {
    param($window, $process)
    $open = Wait-Control $window (Name-Condition "Open Menu") "menu flyout owner"
    Click-Control $process $open
    $paste = Wait-Process-Control $process (Name-Condition "Paste") "menu flyout Paste item"
    $paste.GetCurrentPattern(
        [System.Windows.Automation.InvokePattern]::Pattern
    ).Invoke()
    $null = Wait-Control $window (
        Name-Condition "Last action: Paste"
    ) "menu flyout callback"
}

Run-Sample "menu_bar" {
    param($window, $process)
    $file = Wait-Control $window (Name-Condition "File") "MenuBar File item"
    $file.GetCurrentPattern(
        [System.Windows.Automation.ExpandCollapsePattern]::Pattern
    ).Expand()
    $open = Wait-Process-Control $process (Name-Condition "Open") "MenuBar Open item"
    $open.GetCurrentPattern(
        [System.Windows.Automation.InvokePattern]::Pattern
    ).Invoke()
    $null = Wait-Control $window (
        Name-Condition "Last clicked: Open"
    ) "MenuBar callback"
}

Run-Sample "expander" {
    param($window)
    $more = Wait-Control $window (Name-Condition "More") "collapsed Expander"
    $more.GetCurrentPattern(
        [System.Windows.Automation.ExpandCollapsePattern]::Pattern
    ).Expand()
    $null = Wait-Control $window (Name-Condition "Collapsed by default.") "expanded content"
}

Run-Sample "use_resource" {
    param($window)
    $next = Wait-Control $window (Name-Condition "Next") "resource next button"
    $invoke = $next.GetCurrentPattern(
        [System.Windows.Automation.InvokePattern]::Pattern
    )
    $invoke.Invoke()
    $invoke.Invoke()
    $null = Wait-Control $window (Name-Condition "Item 11 (page 3)") "latest resource page"
}

Run-Sample "use_resource_retry" {
    param($window)
    $null = Wait-Control $window (
        Name-Condition "72 F and sunny (attempt #1)"
    ) "initial weather result"
    $refresh = Wait-Control $window (Name-Condition "Refresh") "weather refresh button"
    $refresh.GetCurrentPattern(
        [System.Windows.Automation.InvokePattern]::Pattern
    ).Invoke()
    $null = Wait-Control $window (
        Name-Condition "72 F and sunny (attempt #2)"
    ) "refreshed weather result"
    $refresh.GetCurrentPattern(
        [System.Windows.Automation.InvokePattern]::Pattern
    ).Invoke()
    $null = Wait-Control $window (
        Name-Condition "Error: network timeout - server unreachable (0x80004005)"
    ) "failed weather result"
    $retry = Wait-Control $window (Name-Condition "Retry") "weather retry button"
    $retry.GetCurrentPattern(
        [System.Windows.Automation.InvokePattern]::Pattern
    ).Invoke()
    $null = Wait-Control $window (
        Name-Condition "72 F and sunny (attempt #4)"
    ) "recovered weather result"
}

Run-Sample "tictactoe" {
    param($window)
    foreach ($move in @(
        @{ Cell = 0; Status = "Turn: O" },
        @{ Cell = 3; Status = "Turn: X" },
        @{ Cell = 1; Status = "Turn: O" },
        @{ Cell = 4; Status = "Turn: X" },
        @{ Cell = 2; Status = "X wins!" }
    )) {
        $cell = Wait-Control $window (Id-Condition "cell-$($move.Cell)") "tic-tac-toe cell"
        $cell.GetCurrentPattern(
            [System.Windows.Automation.InvokePattern]::Pattern
        ).Invoke()
        $null = Wait-Control $window (Name-Condition $move.Status) "tic-tac-toe status"
    }
    $reset = Wait-Control $window (Name-Condition "New Game") "tic-tac-toe reset button"
    $reset.GetCurrentPattern(
        [System.Windows.Automation.InvokePattern]::Pattern
    ).Invoke()
    $null = Wait-Control $window (Name-Condition "Turn: X") "reset tic-tac-toe status"
}

Write-Output "windows-reactor sample smoke tests passed"
