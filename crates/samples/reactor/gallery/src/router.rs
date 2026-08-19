use std::rc::Rc;

use windows_reactor::{
    Border, Button, Color, CornerRadius, Element, FontWeight, Image, ImageSource, RenderCx,
    ScrollView, TextBlock, ThemeBrush, Thickness, component, hstack, vstack,
};

use crate::registry::{self, ControlInfo};

pub const HOME_KEY: u64 = 0;
pub const SETTINGS_KEY: u64 = 1;
const CATEGORY_BASE: u64 = 10;
const CONTROL_BASE: u64 = 100;

type PageRender = for<'a> fn(&mut RenderCx<'a>) -> Element;

pub const fn category_key(index: usize) -> u64 {
    CATEGORY_BASE + index as u64
}

pub const fn control_key(index: usize) -> u64 {
    CONTROL_BASE + index as u64
}

pub fn control_info(key: u64) -> Option<&'static ControlInfo> {
    key.checked_sub(CONTROL_BASE)
        .and_then(|index| registry::ALL_CONTROLS.get(index as usize))
}

pub fn route(key: u64, navigate: Rc<dyn Fn(u64)>) -> Element {
    if key == HOME_KEY {
        return home_page(navigate);
    }
    if key == SETTINGS_KEY {
        return settings_page();
    }
    if let Some(index) = key
        .checked_sub(CATEGORY_BASE)
        .filter(|index| (*index as usize) < registry::CATEGORIES.len())
    {
        return category_page(index as usize, navigate);
    }
    if let Some(info) = control_info(key) {
        return control_page(info);
    }
    page_frame(
        "Page not found",
        "The selected gallery route does not exist.",
        TextBlock::new(format!("Unknown route key: {key}")).build(),
    )
}

fn home_page(navigate: Rc<dyn Fn(u64)>) -> Element {
    let categories = registry::CATEGORIES
        .iter()
        .enumerate()
        .map(|(index, category)| {
            let count = registry::controls_in_category(category).len();
            let navigate = Rc::clone(&navigate);
            Button::new(format!("{category} ({count})"))
                .on_click(move || navigate(category_key(index)))
                .horizontal_alignment(windows_reactor::HorizontalAlignment::Stretch)
                .build()
        })
        .collect::<Vec<_>>();

    page_frame(
        "Reactor WinUI Gallery",
        "A 65-page WinUI control gallery built on the windows-reactor public API.",
        vstack(8.0, categories),
    )
}

fn category_page(index: usize, navigate: Rc<dyn Fn(u64)>) -> Element {
    let category = registry::CATEGORIES[index];
    let controls = registry::ALL_CONTROLS
        .iter()
        .enumerate()
        .filter(|(_, info)| info.category == category)
        .map(|(control_index, info)| {
            let navigate = Rc::clone(&navigate);
            let button = Button::new(format!("{} - {}", info.title, info.description))
                .on_click(move || navigate(control_key(control_index)))
                .horizontal_alignment(windows_reactor::HorizontalAlignment::Stretch)
                .build();
            if info.image.is_empty() {
                button
            } else {
                let source = format!(
                    "file:///{}/assets/{}",
                    env!("CARGO_MANIFEST_DIR").replace('\\', "/"),
                    info.image
                );
                hstack(
                    8.0,
                    [
                        Image::new(ImageSource::bitmap(source))
                            .width(32.0)
                            .height(32.0)
                            .build(),
                        button,
                    ],
                )
            }
        })
        .collect::<Vec<_>>();

    page_frame(
        category,
        &format!("{} controls", controls.len()),
        vstack(8.0, controls),
    )
}

fn settings_page() -> Element {
    page_frame(
        "Settings",
        "About this sample.",
        vstack(
            8.0,
            [
                TextBlock::new("WinUI Gallery (Reactor)")
                    .font_weight(FontWeight::BOLD)
                    .build(),
                TextBlock::new("Framework: windows-reactor").build(),
                TextBlock::new("Platform: WinUI 3 / Windows App SDK").build(),
                TextBlock::new("State: reconciler-owned hooks").build(),
            ],
        ),
    )
}

fn control_page(info: &'static ControlInfo) -> Element {
    let body = if let Some(render) = renderer(info.tag) {
        component(render)
    } else {
        design_page(info.tag)
    };
    page_frame(info.title, info.description, body)
}

fn renderer(tag: &str) -> Option<PageRender> {
    Some(match tag {
        "auto-suggest-box" => reactor_samples::auto_suggest_box::app,
        "border" => reactor_samples::border::app,
        "breadcrumb-bar" => reactor_samples::breadcrumb_bar::app,
        "button" => reactor_samples::button::app,
        "calendar-date-picker" => reactor_samples::calendar_date_picker::app,
        "calendar-view" => reactor_samples::calendar_view::app,
        "canvas" => reactor_samples::canvas::app,
        "check-box" => reactor_samples::check_box::app,
        "color-picker" => reactor_samples::color_picker::app,
        "combo-box" => reactor_samples::combo_box::app,
        "command-bar" => reactor_samples::command_bar::app,
        "command-bar-flyout" => reactor_samples::command_bar_flyout::app,
        "content-dialog" => reactor_samples::content_dialog::app,
        "date-picker" => reactor_samples::date_picker::app,
        "drop-down-button" => reactor_samples::drop_down_button::app,
        "expander" => reactor_samples::expander::app,
        "flip-view" => reactor_samples::flip_view::app,
        "flyout" => reactor_samples::flyout::app,
        "grid" => reactor_samples::grid::app,
        "grid-view" => reactor_samples::grid_view::app,
        "hyperlink-button" => reactor_samples::hyperlink_button::app,
        "image" => reactor_samples::image::app,
        "info-badge" => reactor_samples::info_badge::app,
        "info-bar" => reactor_samples::info_bar::app,
        "list-box" => reactor_samples::list_box::app,
        "list-view" => reactor_samples::list_view::app,
        "menu-bar" => reactor_samples::menu_bar::app,
        "menu-flyout" => reactor_samples::menu_flyout::app,
        "navigation-view" => reactor_samples::navigation_view::app,
        "number-box" => reactor_samples::number_box::app,
        "password-box" => reactor_samples::password_box::app,
        "person-picture" => reactor_samples::person_picture::app,
        "pivot" => reactor_samples::pivot::app,
        "progress-bar" => reactor_samples::progress_bar::app,
        "progress-ring" => reactor_samples::progress_ring::app,
        "radio-button" => reactor_samples::radio_button::app,
        "rating-control" => reactor_samples::rating_control::app,
        "relative-panel" => reactor_samples::relative_panel::app,
        "repeat-button" => reactor_samples::repeat_button::app,
        "rich-edit-box" => reactor_samples::rich_edit_box::app,
        "rich-text-block" => reactor_samples::rich_text::app,
        "scroll-view" => reactor_samples::scroll_view::app,
        "selector-bar" => reactor_samples::selector_bar::app,
        "slider" => reactor_samples::slider::app,
        "split-button" => reactor_samples::split_button::app,
        "split-view" => reactor_samples::split_view::app,
        "stack-panel" => reactor_samples::stack::app,
        "tab-view" => reactor_samples::tab_view::app,
        "teaching-tip" => reactor_samples::teaching_tip::app,
        "text-box" => reactor_samples::text_box::app,
        "time-picker" => reactor_samples::time_picker::app,
        "toggle-button" => reactor_samples::toggle_button::app,
        "toggle-switch" => reactor_samples::toggle_switch::app,
        "tool-tip" => reactor_samples::tooltip::app,
        "tree-view" => reactor_samples::tree_view::app,
        "type-ramp" => reactor_samples::type_ramp::app,
        "viewbox" => reactor_samples::viewbox::app,
        "title-bar" | "typography" | "color" | "spacing" | "iconography" | "geometry" | "theme"
        | "materials" => return None,
        _ => return None,
    })
}

fn design_page(tag: &str) -> Element {
    match tag {
        "typography" => component(reactor_samples::type_ramp::app),
        "color" => color_page(),
        "spacing" => spacing_page(),
        "iconography" => iconography_page(),
        "geometry" => geometry_page(),
        "theme" => theme_page(),
        "materials" => materials_page(),
        "title-bar" => title_bar_page(),
        _ => TextBlock::new("This page has no sample.").build(),
    }
}

fn color_page() -> Element {
    hstack(
        8.0,
        [
            color_swatch("Accent", Color::rgb(0, 120, 212)),
            color_swatch("Success", Color::rgb(15, 123, 15)),
            color_swatch("Caution", Color::rgb(157, 93, 0)),
            color_swatch("Critical", Color::rgb(196, 43, 28)),
        ],
    )
}

fn color_swatch(name: &str, color: Color) -> Element {
    Border::new(
        TextBlock::new(name)
            .foreground(Color::rgb(255, 255, 255))
            .build(),
    )
    .background(color)
    .padding(Thickness::uniform(20.0))
    .corner_radius(CornerRadius::uniform(8.0))
    .build()
}

fn spacing_page() -> Element {
    vstack(
        8.0,
        [4.0, 8.0, 12.0, 16.0, 24.0]
            .into_iter()
            .map(|spacing| {
                Border::new(TextBlock::new(format!("{spacing} DIPs")).build())
                    .background(ThemeBrush::CardBackground)
                    .padding(Thickness::uniform(spacing))
                    .build()
            })
            .collect::<Vec<_>>(),
    )
}

fn iconography_page() -> Element {
    hstack(
        12.0,
        ["Home", "Edit", "Search", "Settings", "Mail"]
            .into_iter()
            .map(|name| Button::new(name).build())
            .collect::<Vec<_>>(),
    )
}

fn geometry_page() -> Element {
    hstack(
        12.0,
        [0.0, 4.0, 8.0, 16.0]
            .into_iter()
            .map(|radius| {
                Border::new(TextBlock::new(format!("Radius {radius}")).build())
                    .background(ThemeBrush::CardBackground)
                    .padding(Thickness::uniform(20.0))
                    .corner_radius(CornerRadius::uniform(radius))
                    .build()
            })
            .collect::<Vec<_>>(),
    )
}

fn theme_page() -> Element {
    vstack(
        8.0,
        [
            TextBlock::new("Primary text")
                .foreground(ThemeBrush::PrimaryText)
                .build(),
            Border::new(TextBlock::new("Card background").build())
                .background(ThemeBrush::CardBackground)
                .padding(Thickness::uniform(16.0))
                .build(),
            Border::new(TextBlock::new("Accent").build())
                .background(ThemeBrush::Accent)
                .padding(Thickness::uniform(16.0))
                .build(),
        ],
    )
}

fn materials_page() -> Element {
    vstack(
        8.0,
        [
            TextBlock::new("This window uses the Mica backdrop.").build(),
            TextBlock::new("Use the title-bar theme button to compare light and dark materials.")
                .build(),
        ],
    )
}

fn title_bar_page() -> Element {
    vstack(
        8.0,
        [
            TextBlock::new("The gallery window uses a custom TitleBar.")
                .font_weight(FontWeight::BOLD)
                .build(),
            TextBlock::new(
                "Its back button, pane toggle, search content, theme action, and tall height are \
                 reconciled through the public window API.",
            )
            .build(),
        ],
    )
}

fn page_frame(title: &str, description: &str, body: Element) -> Element {
    ScrollView::new(
        Border::new(vstack(
            16.0,
            [
                TextBlock::new(title)
                    .font_size(28.0)
                    .font_weight(FontWeight::BOLD)
                    .build(),
                TextBlock::new(description).opacity(0.7).build(),
                body,
            ],
        ))
        .padding(Thickness::uniform(32.0))
        .build(),
    )
    .build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gallery_keeps_all_routes() {
        assert_eq!(registry::ALL_CONTROLS.len(), 65);
        for info in registry::ALL_CONTROLS {
            assert!(
                renderer(info.tag).is_some()
                    || matches!(
                        info.tag,
                        "title-bar"
                            | "typography"
                            | "color"
                            | "spacing"
                            | "iconography"
                            | "geometry"
                            | "theme"
                            | "materials"
                    ),
                "missing gallery route {}",
                info.tag
            );
        }
    }
}
