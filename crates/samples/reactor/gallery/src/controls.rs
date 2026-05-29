//! Reusable UI building blocks for gallery pages.

use windows_reactor::*;

/// Renders a page header with title and description (matches C# PageHeader).
pub fn page_header(title: &str, description: &str) -> Element {
    vstack((
        text_block(title).font_size(28.0).bold(),
        text_block(description)
            .foreground(ThemeRef::SecondaryText)
            .horizontal_alignment(HorizontalAlignment::Left)
            .max_width(800.0)
            .margin(Thickness {
                left: 0.0,
                top: 0.0,
                right: 0.0,
                bottom: 12.0,
            }),
    ))
    .spacing(4.0)
    .margin(Thickness {
        left: 0.0,
        top: 0.0,
        right: 0.0,
        bottom: 8.0,
    })
    .into()
}

/// Renders a sample card matching the C# GalleryControls.SampleCard layout:
/// title → bordered card (sample area + source expander).
pub fn sample_card(title: &str, sample: impl Into<Element>, source: &str) -> Element {
    let sample_area: Element = border(sample.into())
        .corner_radius(8.0)
        .padding(Thickness::uniform(24.0))
        .background(ThemeRef::SolidBackground)
        .into();

    let source_area: Element = Expander::new(text_block(source).font_size(13.0))
        .header("Source code")
        .horizontal_alignment(HorizontalAlignment::Stretch)
        .font_family("Consolas")
        .into();

    let inner: Element = vstack((sample_area, source_area)).spacing(0.0).into();
    let card: Element = border(inner)
        .corner_radius(8.0)
        .border_brush(ThemeRef::CardStroke)
        .border_thickness(Thickness::uniform(1.0))
        .background(ThemeRef::CardBackground)
        .into();

    vstack((
        text_block(title).font_size(14.0).bold().margin(Thickness {
            left: 0.0,
            top: 0.0,
            right: 0.0,
            bottom: 12.0,
        }),
        card,
    ))
    .spacing(0.0)
    .into()
}

/// Data for a single tile in a [`card_grid`].
#[derive(Clone, PartialEq)]
pub struct CardItem {
    pub title: String,
    pub subtitle: String,
    pub image_file: String,
    pub key: String,
}

/// Renders a virtualized grid of icon + text tiles, one per [`CardItem`].
/// `on_click` receives the `key` of the tapped tile.
pub fn card_grid(items: &[CardItem], on_click: impl Fn(String) + 'static) -> Element {
    let items_owned = items.to_vec();
    let items_for_handler = items_owned.clone();

    let assets_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("assets");

    grid_view(items_owned, move |item, _idx| {
        let image_uri = if item.image_file.is_empty() {
            String::new()
        } else {
            format!(
                "file:///{}",
                assets_dir
                    .join(&item.image_file)
                    .to_string_lossy()
                    .replace('\\', "/")
            )
        };
        border(
            hstack((
                Image::new(image_uri.as_str())
                    .width(32.0)
                    .height(32.0)
                    .horizontal_alignment(HorizontalAlignment::Left)
                    .vertical_alignment(VerticalAlignment::Top),
                vstack((
                    text_block(item.title.clone()).font_size(14.0).semibold(),
                    text_block(item.subtitle.clone()).opacity(0.6),
                ))
                .spacing(4.0),
            ))
            .spacing(12.0),
        )
        .background(ThemeRef::CardBackground)
        .border_brush(Color {
            a: 15,
            r: 0,
            g: 0,
            b: 0,
        })
        .border_thickness(Thickness::uniform(1.0))
        .padding(Thickness::uniform(16.0))
        .corner_radius(8.0)
        .width(300.0)
        .height(80.0)
    })
    .selection_mode(SelectionMode::Single)
    .on_selection_changed(move |idx| {
        if idx >= 0
            && let Some(item) = items_for_handler.get(idx as usize)
        {
            on_click(item.key.clone());
        }
    })
    .with_key_selector(|item| item.key.clone())
    .build()
}

/// Wraps page content in a ScrollView with header + sample cards (matches C# PageContent).
pub fn page_content(title: &str, description: &str, cards: Vec<Element>) -> Element {
    let mut items: Vec<Element> = vec![page_header(title, description)];
    items.extend(cards);
    let content: Element = vstack(items)
        .spacing(16.0)
        .margin(Thickness {
            left: 36.0,
            top: 24.0,
            right: 36.0,
            bottom: 36.0,
        })
        .horizontal_alignment(HorizontalAlignment::Stretch)
        .into();
    scroll_view(content).into()
}
