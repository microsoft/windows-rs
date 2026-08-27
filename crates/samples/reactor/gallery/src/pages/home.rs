use crate::controls::{CardItem, card_grid, page_header};
use crate::registry::{self, CATEGORIES};
use windows_reactor::*;

#[derive(Clone, PartialEq)]
pub struct HomeInput {
    pub on_navigate: Callback<String>,
}

pub struct HomePage;

impl Component for HomePage {
    type Message = ();
    type Input = HomeInput;

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {}

    fn view(&self, input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
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

        let on_navigate = input.on_navigate.clone();
        ScrollViewer::new().content(
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
                                .font_weight(600),
                            card_grid(&items, move |tag| {
                                let _ = on_navigate.call(tag);
                            }),
                        )),
                    )),
                ),
        )
    }
}
