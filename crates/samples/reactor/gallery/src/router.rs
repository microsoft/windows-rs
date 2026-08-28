//! Maps a leaf destination tag to its page component. Home, Settings, the Materials design page
//! (which needs shell-owned backdrop state), and the 11 category listing pages are resolved
//! directly by [`crate::shell`] because they need shell input or have no fixed control identity;
//! every other one of the 65 control destinations is a self-contained component routed here.

use crate::pages::{
    basic_input, collections, date_time, design, dialogs, layout, media, menus, navigation, status,
    text,
};
use windows_reactor::*;

pub fn route(tag: &str) -> View {
    match tag {
        "button" => View::component::<basic_input::ButtonPage>(()),
        "check-box" => View::component::<basic_input::CheckBoxPage>(()),
        "color-picker" => View::component::<basic_input::ColorPickerPage>(()),
        "combo-box" => View::component::<basic_input::ComboBoxPage>(()),
        "drop-down-button" => View::component::<basic_input::DropDownButtonPage>(()),
        "hyperlink-button" => View::component::<basic_input::HyperlinkButtonPage>(()),
        "number-box" => View::component::<basic_input::NumberBoxPage>(()),
        "password-box" => View::component::<basic_input::PasswordBoxPage>(()),
        "radio-button" => View::component::<basic_input::RadioButtonPage>(()),
        "rating-control" => View::component::<basic_input::RatingControlPage>(()),
        "repeat-button" => View::component::<basic_input::RepeatButtonPage>(()),
        "slider" => View::component::<basic_input::SliderPage>(()),
        "split-button" => View::component::<basic_input::SplitButtonPage>(()),
        "text-box" => View::component::<basic_input::TextBoxPage>(()),
        "toggle-button" => View::component::<basic_input::ToggleButtonPage>(()),
        "toggle-switch" => View::component::<basic_input::ToggleSwitchPage>(()),

        "flip-view" => View::component::<collections::FlipViewPage>(()),
        "grid-view" => View::component::<collections::GridViewPage>(()),
        "list-box" => View::component::<collections::ListBoxPage>(()),
        "list-view" => View::component::<collections::ListViewPage>(()),
        "tree-view" => View::component::<collections::TreeViewPage>(()),

        "calendar-date-picker" => View::component::<date_time::CalendarDatePickerPage>(()),
        "calendar-view" => View::component::<date_time::CalendarViewPage>(()),
        "date-picker" => View::component::<date_time::DatePickerPage>(()),
        "time-picker" => View::component::<date_time::TimePickerPage>(()),

        "typography" => View::component::<design::TypographyPage>(()),
        "color" => View::component::<design::ColorPage>(()),
        "spacing" => View::component::<design::SpacingPage>(()),
        "iconography" => View::component::<design::IconographyPage>(()),
        "geometry" => View::component::<design::GeometryPage>(()),
        "theme" => View::component::<design::ThemePage>(()),

        "command-bar-flyout" => View::component::<dialogs::CommandBarFlyoutPage>(()),
        "content-dialog" => View::component::<dialogs::ContentDialogPage>(()),
        "flyout" => View::component::<dialogs::FlyoutPage>(()),
        "menu-flyout" => View::component::<dialogs::MenuFlyoutPage>(()),
        "teaching-tip" => View::component::<dialogs::TeachingTipPage>(()),

        "border" => View::component::<layout::BorderPage>(()),
        "canvas" => View::component::<layout::CanvasPage>(()),
        "expander" => View::component::<layout::ExpanderPage>(()),
        "grid" => View::component::<layout::GridPage>(()),
        "relative-panel" => View::component::<layout::RelativePanelPage>(()),
        "scroll-view" => View::component::<layout::ScrollViewPage>(()),
        "split-view" => View::component::<layout::SplitViewPage>(()),
        "stack-panel" => View::component::<layout::StackPanelPage>(()),
        "viewbox" => View::component::<layout::ViewboxPage>(()),

        "image" => View::component::<media::ImagePage>(()),
        "person-picture" => View::component::<media::PersonPicturePage>(()),

        "command-bar" => View::component::<menus::CommandBarPage>(()),
        "menu-bar" => View::component::<menus::MenuBarPage>(()),
        "selector-bar" => View::component::<menus::SelectorBarPage>(()),

        "breadcrumb-bar" => View::component::<navigation::BreadcrumbBarPage>(()),
        "navigation-view" => View::component::<navigation::NavigationViewPage>(()),
        "pivot" => View::component::<navigation::PivotPage>(()),
        "tab-view" => View::component::<navigation::TabViewPage>(()),
        "title-bar" => View::component::<navigation::TitleBarPage>(()),

        "info-badge" => View::component::<status::InfoBadgePage>(()),
        "info-bar" => View::component::<status::InfoBarPage>(()),
        "progress-bar" => View::component::<status::ProgressBarPage>(()),
        "progress-ring" => View::component::<status::ProgressRingPage>(()),
        "tool-tip" => View::component::<status::ToolTipPage>(()),

        "auto-suggest-box" => View::component::<text::AutoSuggestBoxPage>(()),
        "rich-edit-box" => View::component::<text::RichEditBoxPage>(()),
        "rich-text-block" => View::component::<text::RichTextBlockPage>(()),
        "type-ramp" => View::component::<text::TypeRampPage>(()),

        _ => TextBlock::new()
            .text(format!("Page not found: {tag}"))
            .opacity(0.6)
            .into(),
    }
}
