use crate::controls::{CardItem, card_grid, page_header};
use crate::registry::{self, CATEGORIES};
use windows_reactor::*;

pub fn home_page(on_navigate: Callback<String>) -> View {
    let items: Vec<CardItem> = CATEGORIES
        .iter()
        .map(|&category| {
            let controls = registry::controls_in_category(category);
            CardItem {
                title: category.to_string(),
                subtitle: format!("{} controls", controls.len()),
                image_file: controls
                    .first()
                    .map_or_else(String::new, |control| control.image.to_string()),
                key: registry::category_tag(category),
            }
        })
        .collect();

    ScrollViewer::new()
        .content(
            Border::new()
                .padding(Thickness::new(36.0, 24.0, 36.0, 36.0))
                .content(
                    StackPanel::new().spacing(24.0).children((
                        page_header(
                            "Reactor gallery",
                            "A showcase of WinUI controls built entirely with windows-reactor \
                         - a declarative, component-based UI framework for WinUI 3.",
                        ),
                        StackPanel::new().spacing(12.0).children((
                            TextBlock::new()
                                .text("Browse by category")
                                .font_size(14.0)
                                .font_weight(FontWeight::SEMI_BOLD),
                            card_grid(&items, on_navigate),
                        )),
                    )),
                ),
        )
        .into()
}
