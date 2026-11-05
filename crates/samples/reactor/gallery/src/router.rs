//! Maps control destination tags to stateful components or stateless page functions.
//! Home, Settings, Materials, and category listings are resolved by [`crate::shell`].

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

        "typography" => design::typography_page(),
        "color" => design::color_page(),
        "spacing" => design::spacing_page(),
        "iconography" => design::iconography_page(),
        "geometry" => design::geometry_page(),
        "theme" => design::theme_page(),

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
        "title-bar" => navigation::title_bar_page(),

        "info-badge" => View::component::<status::InfoBadgePage>(()),
        "info-bar" => View::component::<status::InfoBarPage>(()),
        "progress-bar" => View::component::<status::ProgressBarPage>(()),
        "progress-ring" => View::component::<status::ProgressRingPage>(()),
        "tool-tip" => View::component::<status::ToolTipPage>(()),

        "auto-suggest-box" => View::component::<text::AutoSuggestBoxPage>(()),
        "rich-edit-box" => View::component::<text::RichEditBoxPage>(()),
        "rich-text-block" => View::component::<text::RichTextBlockPage>(()),

        _ => TextBlock::new()
            .text(format!("Page not found: {tag}"))
            .opacity(0.6)
            .into(),
    }
}
