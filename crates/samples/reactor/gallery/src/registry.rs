/// Metadata for a single control in the gallery.
#[derive(Clone)]
pub struct ControlInfo {
    pub title: &'static str,
    pub description: &'static str,
    pub category: &'static str,
    pub tag: &'static str,
    pub image: &'static str,
}

pub const CATEGORIES: &[&str] = &[
    "Basic Input",
    "Collections",
    "Date and Time",
    "Design Guidance",
    "Dialogs and Flyouts",
    "Layout",
    "Media",
    "Menus and Toolbars",
    "Navigation",
    "Status and Info",
    "Text",
];

pub const ALL_CONTROLS: &[ControlInfo] = &[
    // ── Basic Input ─────────────────────────────────────────────────
    ControlInfo {
        title: "Button",
        description: "A button that responds to user clicks.",
        category: "Basic Input",
        tag: "button",
        image: "Button.png",
    },
    ControlInfo {
        title: "CheckBox",
        description: "A control that a user can select or clear.",
        category: "Basic Input",
        tag: "check-box",
        image: "Checkbox.png",
    },
    ControlInfo {
        title: "ColorPicker",
        description: "A control that lets a user pick a color.",
        category: "Basic Input",
        tag: "color-picker",
        image: "ColorPicker.png",
    },
    ControlInfo {
        title: "ComboBox",
        description: "A drop-down list of items a user can select from.",
        category: "Basic Input",
        tag: "combo-box",
        image: "ComboBox.png",
    },
    ControlInfo {
        title: "DropDownButton",
        description: "A button that displays a flyout of choices.",
        category: "Basic Input",
        tag: "drop-down-button",
        image: "DropDownButton.png",
    },
    ControlInfo {
        title: "HyperlinkButton",
        description: "A button that appears as a hyperlink.",
        category: "Basic Input",
        tag: "hyperlink-button",
        image: "HyperlinkButton.png",
    },
    ControlInfo {
        title: "NumberBox",
        description: "A text control for entering numeric values.",
        category: "Basic Input",
        tag: "number-box",
        image: "NumberBox.png",
    },
    ControlInfo {
        title: "PasswordBox",
        description: "A text input that conceals typed characters.",
        category: "Basic Input",
        tag: "password-box",
        image: "PasswordBox.png",
    },
    ControlInfo {
        title: "RadioButton",
        description: "Select one option from a group.",
        category: "Basic Input",
        tag: "radio-button",
        image: "RadioButton.png",
    },
    ControlInfo {
        title: "RatingControl",
        description: "A control that lets users provide a star rating.",
        category: "Basic Input",
        tag: "rating-control",
        image: "RatingControl.png",
    },
    ControlInfo {
        title: "RepeatButton",
        description: "A button that raises click events while pressed.",
        category: "Basic Input",
        tag: "repeat-button",
        image: "RepeatButton.png",
    },
    ControlInfo {
        title: "Slider",
        description: "Select from a range of values by moving a thumb.",
        category: "Basic Input",
        tag: "slider",
        image: "Slider.png",
    },
    ControlInfo {
        title: "SplitButton",
        description: "A button with a primary action and flyout menu.",
        category: "Basic Input",
        tag: "split-button",
        image: "SplitButton.png",
    },
    ControlInfo {
        title: "TextBox",
        description: "A single-line or multi-line text input field.",
        category: "Basic Input",
        tag: "text-box",
        image: "TextBox.png",
    },
    ControlInfo {
        title: "ToggleButton",
        description: "A button that toggles between two states.",
        category: "Basic Input",
        tag: "toggle-button",
        image: "ToggleButton.png",
    },
    ControlInfo {
        title: "ToggleSwitch",
        description: "A switch between two mutually exclusive states.",
        category: "Basic Input",
        tag: "toggle-switch",
        image: "ToggleSwitch.png",
    },
    // ── Collections ─────────────────────────────────────────────────
    ControlInfo {
        title: "FlipView",
        description: "Presents one item at a time with flipping navigation.",
        category: "Collections",
        tag: "flip-view",
        image: "FlipView.png",
    },
    ControlInfo {
        title: "GridView",
        description: "Displays items in a horizontally wrapping grid.",
        category: "Collections",
        tag: "grid-view",
        image: "GridView.png",
    },
    ControlInfo {
        title: "ListBox",
        description: "A list of selectable items presented inline.",
        category: "Collections",
        tag: "list-box",
        image: "ListBox.png",
    },
    ControlInfo {
        title: "ListView",
        description: "Displays items in a vertical scrolling list.",
        category: "Collections",
        tag: "list-view",
        image: "ListView.png",
    },
    ControlInfo {
        title: "TreeView",
        description: "A hierarchical list with expanding and collapsing nodes.",
        category: "Collections",
        tag: "tree-view",
        image: "TreeView.png",
    },
    // ── Date and Time ───────────────────────────────────────────────
    ControlInfo {
        title: "CalendarDatePicker",
        description: "Pick a date from a calendar dropdown.",
        category: "Date and Time",
        tag: "calendar-date-picker",
        image: "CalendarDatePicker.png",
    },
    ControlInfo {
        title: "CalendarView",
        description: "A calendar display for selecting a date.",
        category: "Date and Time",
        tag: "calendar-view",
        image: "CalendarView.png",
    },
    ControlInfo {
        title: "DatePicker",
        description: "Pick a date using spinners.",
        category: "Date and Time",
        tag: "date-picker",
        image: "DatePicker.png",
    },
    ControlInfo {
        title: "TimePicker",
        description: "Pick a time using spinners.",
        category: "Date and Time",
        tag: "time-picker",
        image: "TimePicker.png",
    },
    // ── Design Guidance ─────────────────────────────────────────────
    ControlInfo {
        title: "Typography",
        description: "The WinUI 3 type ramp and text styles.",
        category: "Design Guidance",
        tag: "typography",
        image: "TextBlock.png",
    },
    ControlInfo {
        title: "Color",
        description: "System accent and semantic color palette.",
        category: "Design Guidance",
        tag: "color",
        image: "ColorPaletteResources.png",
    },
    ControlInfo {
        title: "Spacing",
        description: "Standard spacing values for consistent layouts.",
        category: "Design Guidance",
        tag: "spacing",
        image: "",
    },
    ControlInfo {
        title: "Iconography",
        description: "Segoe Fluent Icons symbol set.",
        category: "Design Guidance",
        tag: "iconography",
        image: "",
    },
    ControlInfo {
        title: "Geometry",
        description: "Corner radius resources for consistent shape language.",
        category: "Design Guidance",
        tag: "geometry",
        image: "",
    },
    ControlInfo {
        title: "Theme",
        description: "Light, dark, and high-contrast theme guidance.",
        category: "Design Guidance",
        tag: "theme",
        image: "ThemeTransition.png",
    },
    ControlInfo {
        title: "Materials",
        description: "Window backdrop materials (Mica, Acrylic) for depth and hierarchy.",
        category: "Design Guidance",
        tag: "materials",
        image: "Acrylic.png",
    },
    // ── Dialogs and Flyouts ─────────────────────────────────────────
    ControlInfo {
        title: "CommandBarFlyout",
        description: "A flyout that provides quick access to common commands.",
        category: "Dialogs and Flyouts",
        tag: "command-bar-flyout",
        image: "CommandBarFlyout.png",
    },
    ControlInfo {
        title: "ContentDialog",
        description: "A modal dialog box with content and actions.",
        category: "Dialogs and Flyouts",
        tag: "content-dialog",
        image: "ContentDialog.png",
    },
    ControlInfo {
        title: "Flyout",
        description: "A lightweight popup for contextual info.",
        category: "Dialogs and Flyouts",
        tag: "flyout",
        image: "Flyout.png",
    },
    ControlInfo {
        title: "TeachingTip",
        description: "A notification flyout for guiding users.",
        category: "Dialogs and Flyouts",
        tag: "teaching-tip",
        image: "TeachingTip.png",
    },
    ControlInfo {
        title: "MenuFlyout",
        description: "A flyout that displays a list of menu commands.",
        category: "Dialogs and Flyouts",
        tag: "menu-flyout",
        image: "MenuFlyout.png",
    },
    // ── Layout ──────────────────────────────────────────────────────
    ControlInfo {
        title: "Border",
        description: "A container that draws a border around its child.",
        category: "Layout",
        tag: "border",
        image: "Border.png",
    },
    ControlInfo {
        title: "Canvas",
        description: "Absolute positioning of child elements.",
        category: "Layout",
        tag: "canvas",
        image: "Canvas.png",
    },
    ControlInfo {
        title: "Expander",
        description: "Expands and collapses to show or hide content.",
        category: "Layout",
        tag: "expander",
        image: "Expander.png",
    },
    ControlInfo {
        title: "Grid",
        description: "Arranges children in rows and columns.",
        category: "Layout",
        tag: "grid",
        image: "Grid.png",
    },
    ControlInfo {
        title: "RelativePanel",
        description: "Positions children relative to each other.",
        category: "Layout",
        tag: "relative-panel",
        image: "RelativePanel.png",
    },
    ControlInfo {
        title: "ScrollView",
        description: "A scrollable container for overflowing content.",
        category: "Layout",
        tag: "scroll-view",
        image: "ScrollView.png",
    },
    ControlInfo {
        title: "SplitView",
        description: "A collapsible pane and content area.",
        category: "Layout",
        tag: "split-view",
        image: "SplitView.png",
    },
    ControlInfo {
        title: "StackPanel",
        description: "Arranges children in a single line.",
        category: "Layout",
        tag: "stack-panel",
        image: "StackPanel.png",
    },
    ControlInfo {
        title: "Viewbox",
        description: "Scales its child to fill available space.",
        category: "Layout",
        tag: "viewbox",
        image: "Viewbox.png",
    },
    // ── Media ───────────────────────────────────────────────────────
    ControlInfo {
        title: "Image",
        description: "Displays an image from a file or URI.",
        category: "Media",
        tag: "image",
        image: "Image.png",
    },
    ControlInfo {
        title: "PersonPicture",
        description: "A circular avatar for a person.",
        category: "Media",
        tag: "person-picture",
        image: "PersonPicture.png",
    },
    // ── Menus and Toolbars ──────────────────────────────────────────
    ControlInfo {
        title: "CommandBar",
        description: "A toolbar for app commands and actions.",
        category: "Menus and Toolbars",
        tag: "command-bar",
        image: "CommandBar.png",
    },
    ControlInfo {
        title: "MenuBar",
        description: "A horizontal bar hosting drop-down menus.",
        category: "Menus and Toolbars",
        tag: "menu-bar",
        image: "MenuBar.png",
    },
    ControlInfo {
        title: "SelectorBar",
        description: "Switch between different views or modes.",
        category: "Menus and Toolbars",
        tag: "selector-bar",
        image: "",
    },
    // ── Navigation ──────────────────────────────────────────────────
    ControlInfo {
        title: "BreadcrumbBar",
        description: "A trail showing the navigation path.",
        category: "Navigation",
        tag: "breadcrumb-bar",
        image: "BreadcrumbBar.png",
    },
    ControlInfo {
        title: "NavigationView",
        description: "A side or top navigation pane.",
        category: "Navigation",
        tag: "navigation-view",
        image: "NavigationView.png",
    },
    ControlInfo {
        title: "Pivot",
        description: "A tabbed interface for switching sections.",
        category: "Navigation",
        tag: "pivot",
        image: "Pivot.png",
    },
    ControlInfo {
        title: "TabView",
        description: "Closable, rearrangeable tabs.",
        category: "Navigation",
        tag: "tab-view",
        image: "TabView.png",
    },
    ControlInfo {
        title: "TitleBar",
        description: "A customizable application title bar.",
        category: "Navigation",
        tag: "title-bar",
        image: "TitleBar.png",
    },
    // ── Status and Info ─────────────────────────────────────────────
    ControlInfo {
        title: "InfoBadge",
        description: "A small indicator conveying status.",
        category: "Status and Info",
        tag: "info-badge",
        image: "InfoBadge.png",
    },
    ControlInfo {
        title: "InfoBar",
        description: "A dismissible bar for app-level messages.",
        category: "Status and Info",
        tag: "info-bar",
        image: "InfoBar.png",
    },
    ControlInfo {
        title: "ProgressBar",
        description: "A horizontal bar showing progress.",
        category: "Status and Info",
        tag: "progress-bar",
        image: "ProgressBar.png",
    },
    ControlInfo {
        title: "ProgressRing",
        description: "A circular indicator of ongoing progress.",
        category: "Status and Info",
        tag: "progress-ring",
        image: "ProgressRing.png",
    },
    ControlInfo {
        title: "ToolTip",
        description: "A popup with helpful text on hover.",
        category: "Status and Info",
        tag: "tool-tip",
        image: "ToolTip.png",
    },
    // ── Text ────────────────────────────────────────────────────────
    ControlInfo {
        title: "AutoSuggestBox",
        description: "Text input that shows suggestions as you type.",
        category: "Text",
        tag: "auto-suggest-box",
        image: "AutoSuggestBox.png",
    },
    ControlInfo {
        title: "RichEditBox",
        description: "A rich text editing control.",
        category: "Text",
        tag: "rich-edit-box",
        image: "RichEditBox.png",
    },
    ControlInfo {
        title: "RichTextBlock",
        description: "Displays formatted, read-only rich text.",
        category: "Text",
        tag: "rich-text-block",
        image: "RichTextBlock.png",
    },
    ControlInfo {
        title: "Type Ramp",
        description: "Named text factories for the WinUI 3 type ramp.",
        category: "Text",
        tag: "type-ramp",
        image: "TextBlock.png",
    },
];

pub fn controls_in_category(category: &str) -> Vec<&'static ControlInfo> {
    ALL_CONTROLS
        .iter()
        .filter(|c| c.category == category)
        .collect()
}

pub fn search(query: &str) -> Vec<&'static ControlInfo> {
    let q = query.to_lowercase();
    ALL_CONTROLS
        .iter()
        .filter(|c| {
            c.title.to_lowercase().contains(&q) || c.description.to_lowercase().contains(&q)
        })
        .collect()
}

pub fn category_tag(category: &str) -> String {
    category.to_lowercase().replace(' ', "-")
}
