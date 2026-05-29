use windows_reactor::*;

use crate::pages::{
    basic_input, collections, date_time, design, dialogs, layout, media, menus, navigation, status,
    text,
};

/// Routes a tag string to the appropriate page component.
pub fn route(tag: &str) -> Element {
    match tag {
        // Basic Input
        "button" => component(basic_input::button_page, ()),
        "check-box" => component(basic_input::check_box_page, ()),
        "color-picker" => component(basic_input::color_picker_page, ()),
        "combo-box" => component(basic_input::combo_box_page, ()),
        "drop-down-button" => component(basic_input::drop_down_button_page, ()),
        "hyperlink-button" => component(basic_input::hyperlink_button_page, ()),
        "number-box" => component(basic_input::number_box_page, ()),
        "password-box" => component(basic_input::password_box_page, ()),
        "radio-button" => component(basic_input::radio_button_page, ()),
        "rating-control" => component(basic_input::rating_control_page, ()),
        "repeat-button" => component(basic_input::repeat_button_page, ()),
        "slider" => component(basic_input::slider_page, ()),
        "split-button" => component(basic_input::split_button_page, ()),
        "text-box" => component(basic_input::text_box_page, ()),
        "toggle-button" => component(basic_input::toggle_button_page, ()),
        "toggle-switch" => component(basic_input::toggle_switch_page, ()),

        // Collections
        "flip-view" => component(collections::flip_view_page, ()),
        "grid-view" => component(collections::grid_view_page, ()),
        "list-box" => component(collections::list_box_page, ()),
        "list-view" => component(collections::list_view_page, ()),
        "tree-view" => component(collections::tree_view_page, ()),

        // Date and Time
        "calendar-date-picker" => component(date_time::calendar_date_picker_page, ()),
        "calendar-view" => component(date_time::calendar_view_page, ()),
        "date-picker" => component(date_time::date_picker_page, ()),
        "time-picker" => component(date_time::time_picker_page, ()),

        // Dialogs and Flyouts
        "command-bar-flyout" => component(dialogs::command_bar_flyout_page, ()),
        "content-dialog" => component(dialogs::content_dialog_page, ()),
        "flyout" => component(dialogs::flyout_page, ()),
        "menu-flyout" => component(dialogs::menu_flyout_page, ()),
        "teaching-tip" => component(dialogs::teaching_tip_page, ()),

        // Layout
        "border" => component(layout::border_page, ()),
        "canvas" => component(layout::canvas_page, ()),
        "expander" => component(layout::expander_page, ()),
        "grid" => component(layout::grid_page, ()),
        "relative-panel" => component(layout::relative_panel_page, ()),
        "scroll-view" => component(layout::scroll_view_page, ()),
        "split-view" => component(layout::split_view_page, ()),
        "stack-panel" => component(layout::stack_panel_page, ()),
        "viewbox" => component(layout::viewbox_page, ()),

        // Media
        "image" => component(media::image_page, ()),
        "person-picture" => component(media::person_picture_page, ()),

        // Menus and Toolbars
        "command-bar" => component(menus::command_bar_page, ()),
        "menu-bar" => component(menus::menu_bar_page, ()),
        "selector-bar" => component(menus::selector_bar_page, ()),

        // Navigation
        "breadcrumb-bar" => component(navigation::breadcrumb_bar_page, ()),
        "navigation-view" => component(navigation::navigation_view_page, ()),
        "pivot" => component(navigation::pivot_page, ()),
        "tab-view" => component(navigation::tab_view_page, ()),
        "title-bar" => component(navigation::title_bar_page, ()),

        // Status and Info
        "info-badge" => component(status::info_badge_page, ()),
        "info-bar" => component(status::info_bar_page, ()),
        "progress-bar" => component(status::progress_bar_page, ()),
        "progress-ring" => component(status::progress_ring_page, ()),
        "tool-tip" => component(status::tool_tip_page, ()),

        // Text
        "auto-suggest-box" => component(text::auto_suggest_box_page, ()),
        "rich-edit-box" => component(text::rich_edit_box_page, ()),
        "rich-text-block" => component(text::rich_text_block_page, ()),
        "type-ramp" => component(text::type_ramp_page, ()),

        // Design Guidance
        "typography" => component(design::typography_page, ()),
        "color" => component(design::color_page, ()),
        "spacing" => component(design::spacing_page, ()),
        "iconography" => component(design::iconography_page, ()),
        "geometry" => component(design::geometry_page, ()),
        "theme" => component(design::theme_page, ()),
        "materials" => component(design::materials_page, ()),

        _ => text_block(format!("Page not found: {tag}"))
            .opacity(0.6)
            .into(),
    }
    .into()
}
