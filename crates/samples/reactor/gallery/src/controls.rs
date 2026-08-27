//! Shared page-building blocks used by every gallery destination: page headers, sample cards
//! with a collapsible source panel, scrollable page content, and the card grid used by the home
//! and category pages to link to their child destinations.

use windows_reactor::*;

/// Resolves a bundled asset file relative to this crate's `assets` directory.
pub fn asset_path(name: &str) -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("assets")
        .join(name)
}

/// Loads a bundled asset as an `Image`, or an empty view when `name` is blank (some design
/// destinations have no representative screenshot).
pub fn asset_image(name: &str, width: f64, height: f64) -> View {
    if name.is_empty() {
        return View::empty();
    }
    Image::new()
        .source_file(asset_path(name))
        .unwrap()
        .width(width)
        .height(height)
        .stretch(Stretch::Uniform)
        .into()
}

pub fn page_header(title: &str, description: &str) -> View {
    StackPanel::new().spacing(4.0).children((
        TextBlock::new()
            .text(title)
            .font_size(28.0)
            .font_weight(700),
        description,
    ))
}

/// Wraps a live sample in a card with its source snippet shown underneath.
pub fn sample_card(title: &str, sample: impl Into<View>, source: &str) -> View {
    StackPanel::new().spacing(8.0).children((
        TextBlock::new()
            .text(title)
            .font_size(14.0)
            .font_weight(600),
        Border::new()
            .border_thickness(1.0)
            .corner_radius(8.0)
            .background(ThemeBrush::CardBackground)
            .border_brush(ThemeBrush::CardStroke)
            .content(
                StackPanel::new().children((
                    Border::new()
                        .padding(24.0)
                        .background(ThemeBrush::SolidBackground)
                        .content(sample),
                    Border::new()
                        .padding(Thickness::new(12.0, 8.0, 12.0, 8.0))
                        .content(TextBlock::new().text(source).font_size(13.0)),
                )),
            ),
    ))
}

/// Lays out a page title, description, and a list of keyed sample cards inside a scroll viewer.
pub fn page_content(
    title: &str,
    description: &str,
    cards: impl IntoIterator<Item = KeyedView>,
) -> View {
    let children = std::iter::once(KeyedView::new("header", page_header(title, description)))
        .chain(cards)
        .collect::<Vec<_>>();
    ScrollViewer::new().content(
        Border::new()
            .padding(Thickness::new(36.0, 24.0, 36.0, 36.0))
            .content(StackPanel::new().spacing(16.0).keyed_children(children)),
    )
}

/// One card in a home or category grid: a destination title, subtitle, bundled screenshot, and
/// the navigation tag to route to when the card is activated.
#[derive(Clone, PartialEq)]
pub struct CardItem {
    pub title: String,
    pub subtitle: String,
    pub image_file: String,
    pub key: String,
}

const CARD_COLUMNS: usize = 3;
const CARD_WIDTH: f64 = 300.0;
const CARD_HEIGHT: f64 = 88.0;

/// Arranges `items` in a fixed-column grid of clickable cards. Reactor has no wrap panel, so
/// the grid dimensions are computed from the item count, matching the manual grid layout already
/// used by the design pages.
pub fn card_grid(items: &[CardItem], on_click: impl Fn(String) + Clone + 'static) -> View {
    let rows = items.len().div_ceil(CARD_COLUMNS).max(1);
    let children = items.iter().enumerate().map(|(index, item)| {
        let on_click = on_click.clone();
        let key = item.key.clone();
        KeyedView::new(
            item.key.clone(),
            Button::new()
                .style(ButtonStyle::Subtle)
                .width(CARD_WIDTH)
                .height(CARD_HEIGHT)
                .grid_row((index / CARD_COLUMNS) as i32)
                .grid_column((index % CARD_COLUMNS) as i32)
                .on_click(move || on_click(key.clone()))
                .content(
                    Border::new()
                        .background(ThemeBrush::CardBackground)
                        .border_brush(Color::argb(38, 0, 0, 0))
                        .border_thickness(1.0)
                        .corner_radius(8.0)
                        .padding(16.0)
                        .width(CARD_WIDTH)
                        .height(CARD_HEIGHT)
                        .content(
                            StackPanel::new()
                                .orientation(Orientation::Horizontal)
                                .spacing(12.0)
                                .children((
                                    asset_image(&item.image_file, 32.0, 32.0),
                                    StackPanel::new().spacing(4.0).children((
                                        TextBlock::new()
                                            .text(item.title.clone())
                                            .font_size(14.0)
                                            .font_weight(600),
                                        TextBlock::new().text(item.subtitle.clone()).opacity(0.6),
                                    )),
                                )),
                        ),
                ),
        )
    });

    Grid::new()
        .rows(vec![GridLength::Auto; rows])
        .columns(vec![GridLength::Star(1.0); CARD_COLUMNS])
        .row_spacing(12.0)
        .column_spacing(12.0)
        .keyed_children(children)
}

/// Picks a representative pane icon for a category, mirroring the incumbent gallery's mapping.
pub fn category_icon(category: &str) -> Symbol {
    match category {
        "Basic Input" | "Text" => Symbol::Edit,
        "Collections" | "Menus and Toolbars" => Symbol::More,
        "Date and Time" => Symbol::Favorite,
        "Design Guidance" => Symbol::People,
        "Dialogs and Flyouts" => Symbol::Mail,
        "Layout" => Symbol::Find,
        "Media" => Symbol::Camera,
        "Navigation" => Symbol::World,
        "Status and Info" => Symbol::Flag,
        _ => Symbol::Help,
    }
}
