use crate::controls::{card_grid, CardItem};
use crate::registry::{self, CATEGORIES};
use windows_reactor::*;

pub fn home_page(on_navigate: impl Fn(String) + Clone + 'static) -> Element {
    let items: Vec<CardItem> = CATEGORIES
        .iter()
        .map(|&category| {
            let controls = registry::controls_in_category(category);
            CardItem {
                title: category.to_string(),
                subtitle: format!("{} controls", controls.len()),
                image_file: controls
                    .first()
                    .map(|c| c.image.to_string())
                    .unwrap_or_default(),
                key: registry::category_tag(category),
            }
        })
        .collect();

    let root = grid((
        vstack((
            text_block("Reactor WinUI Gallery").font_size(28.0).bold(),
            text_block("A showcase of WinUI controls built entirely with windows-reactor — a declarative, component-based UI framework for WinUI 3.")
                .horizontal_alignment(HorizontalAlignment::Left)
                .max_width(500.0)
                .opacity(0.7)
                .wrap(),
        ))
        .spacing(8.0)
        .grid_row(0),
        grid((
            text_block("Browse by Category")
                .font_size(14.0)
                .semibold()
                .grid_row(0),
            card_grid(&items, on_navigate).grid_row(1),
        ))
        .rows([GridLength::Auto, GridLength::Star(1.0)])
        .columns([GridLength::Star(1.0)])
        .row_spacing(16.0)
        .grid_row(1),
    ))
    .rows([GridLength::Auto, GridLength::Star(1.0)])
    .columns([GridLength::Star(1.0)])
    .row_spacing(32.0);

    border(root)
        .padding(Thickness {
            left: 36.0,
            top: 40.0,
            right: 36.0,
            bottom: 36.0,
        })
        .into()
}
