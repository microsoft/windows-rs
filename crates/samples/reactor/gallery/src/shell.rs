use crate::controls::{card_grid, CardItem};
use crate::registry::{self, ControlInfo, CATEGORIES};
use crate::router;
use windows_reactor::*;

pub fn gallery_shell(cx: &mut RenderCx) -> Element {
    let (nav, set_nav) = cx.use_state((String::from("home"), Vec::<String>::new()));
    let (is_pane_open, set_pane_open) = cx.use_state(true);
    let (search_text, set_search_text) = cx.use_state(String::new());

    let selected_tag = nav.0;
    let history = nav.1;
    let is_dark = matches!(cx.use_color_scheme(), ColorScheme::Dark);

    let category_tags: Vec<String> = CATEGORIES
        .iter()
        .map(|c| registry::category_tag(c))
        .collect();

    let suggestions: Vec<String> = if search_text.is_empty() {
        Vec::new()
    } else {
        registry::search(&search_text)
            .iter()
            .map(|c| c.title.to_string())
            .collect()
    };

    let mut nav_items = vec![NavViewItem::new("Home").tag("home").icon(SymbolGlyph::Home)];
    for &cat in CATEGORIES {
        let controls = registry::controls_in_category(cat);
        let mut item = NavViewItem::new(cat)
            .tag(registry::category_tag(cat))
            .icon(category_icon(cat));
        for c in &controls {
            item = item.child(NavViewItem::new(c.title).tag(c.tag));
        }
        nav_items.push(item);
    }

    let content: Element = if selected_tag == "home" {
        crate::pages::home::home_page({
            let (set_nav, tag, hist) = (set_nav.clone(), selected_tag.clone(), history.clone());
            move |key| {
                let mut h = hist.clone();
                h.push(tag.clone());
                set_nav.call((key, h));
            }
        })
        .into()
    } else if selected_tag.eq_ignore_ascii_case("settings") {
        component(crate::pages::settings::settings_page, ())
    } else if category_tags.contains(&selected_tag) {
        let cat = CATEGORIES
            .iter()
            .find(|c| registry::category_tag(c) == selected_tag)
            .unwrap_or(&"");
        let controls = registry::controls_in_category(cat);
        render_category_page(cat, controls, {
            let (set_nav, tag, hist) = (set_nav.clone(), selected_tag.clone(), history.clone());
            move |key| {
                let mut h = hist.clone();
                h.push(tag.clone());
                set_nav.call((key, h));
            }
        })
    } else {
        router::route(&selected_tag)
    };

    let search_box: Element = auto_suggest_box(&*search_text)
        .placeholder("Search controls and samples...")
        .items(suggestions)
        .on_text_changed(move |text| set_search_text.call(text))
        .on_query_submitted({
            let (set_nav, tag, hist) = (set_nav.clone(), selected_tag.clone(), history.clone());
            move |query: String| {
                if let Some(info) = registry::ALL_CONTROLS
                    .iter()
                    .find(|c| c.title.eq_ignore_ascii_case(&query))
                {
                    let mut h = hist.clone();
                    h.push(tag.clone());
                    set_nav.call((info.tag.to_string(), h));
                }
            }
        })
        .on_suggestion_chosen({
            let (set_nav, tag, hist) = (set_nav.clone(), selected_tag.clone(), history.clone());
            move |chosen: String| {
                if let Some(info) = registry::ALL_CONTROLS
                    .iter()
                    .find(|c| c.title.eq_ignore_ascii_case(&chosen))
                {
                    let mut h = hist.clone();
                    h.push(tag.clone());
                    set_nav.call((info.tag.to_string(), h));
                }
            }
        })
        .into();
    let search_box = search_box.width(320.0);

    let title_bar = TitleBar::new("Reactor WinUI Gallery")
        .pane_toggle_button_visible(true)
        .back_button_visible(true)
        .back_button_enabled(!history.is_empty())
        .on_back_requested({
            let (set_nav, hist) = (set_nav.clone(), history.clone());
            move || {
                let mut h = hist.clone();
                if let Some(prev) = h.pop() {
                    set_nav.call((prev, h));
                }
            }
        })
        .on_pane_toggle_requested(move || set_pane_open.call(!is_pane_open))
        .content(search_box)
        .footer({
            let glyph = if is_dark { "\u{E706}" } else { "\u{E708}" };
            button(glyph)
                .on_click(move || {
                    set_requested_theme(if is_dark {
                        RequestedTheme::Light
                    } else {
                        RequestedTheme::Dark
                    });
                })
                .font_family("Segoe MDL2 Assets")
                .font_size(14.0)
                .width(40.0)
                .height(36.0)
                .padding(0.0)
        })
        .tall(true);

    let nav_view = NavigationView::new(nav_items, content)
        .selected_tag(&selected_tag)
        .on_selection_changed({
            // Last uses of set_nav, selected_tag, history — move directly.
            let (set_nav, tag, hist) = (set_nav, selected_tag, history);
            move |new_tag: String| {
                let effective = if new_tag.is_empty() || new_tag.eq_ignore_ascii_case("settings") {
                    "settings".to_string()
                } else {
                    new_tag
                };
                if effective != tag {
                    let mut h = hist.clone();
                    h.push(tag.clone());
                    set_nav.call((effective, h));
                }
            }
        })
        .pane_display_mode(NavViewPaneDisplayMode::Left)
        .pane_open(is_pane_open)
        .pane_toggle_button_visible(false)
        .back_button_visible(false)
        .font_family("Segoe UI Variable");

    grid((title_bar.grid_row(0), nav_view.grid_row(1)))
        .rows([GridLength::Auto, GridLength::Star(1.0)])
        .columns([GridLength::Star(1.0)])
        .into()
}

fn category_icon(category: &str) -> SymbolGlyph {
    match category {
        "Basic Input" | "Text" => SymbolGlyph::Edit,
        "Collections" | "Menus and Toolbars" => SymbolGlyph::More,
        "Date and Time" => SymbolGlyph::Favorite,
        "Design Guidance" => SymbolGlyph::People,
        "Dialogs and Flyouts" => SymbolGlyph::Mail,
        "Layout" => SymbolGlyph::Find,
        "Media" => SymbolGlyph::Camera,
        "Navigation" => SymbolGlyph::World,
        "Status and Info" => SymbolGlyph::Flag,
        _ => SymbolGlyph::Help,
    }
}

fn render_category_page(
    category: &'static str,
    controls: Vec<&'static ControlInfo>,
    on_navigate: impl Fn(String) + 'static,
) -> Element {
    let count = controls.len();
    let items: Vec<CardItem> = controls
        .iter()
        .map(|c| CardItem {
            title: c.title.to_string(),
            subtitle: c.description.to_string(),
            image_file: c.image.to_string(),
            key: c.tag.to_string(),
        })
        .collect();

    let root = grid((
        vstack((
            text_block(category).font_size(28.0).bold(),
            text_block(format!("{count} controls")).opacity(0.6),
        ))
        .spacing(4.0)
        .grid_row(0),
        card_grid(&items, on_navigate).grid_row(1),
    ))
    .rows([GridLength::Auto, GridLength::Star(1.0)])
    .columns([GridLength::Star(1.0)])
    .row_spacing(24.0);

    border(root)
        .padding(Thickness {
            left: 36.0,
            top: 40.0,
            right: 36.0,
            bottom: 36.0,
        })
        .into()
}
