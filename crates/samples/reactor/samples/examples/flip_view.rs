#![windows_subsystem = "windows"]

use windows_reactor::{
    Border, Button, Color, Element, FlipView, FlipViewItem, FontWeight, RenderCx, TextBlock,
    Thickness, hstack, vstack,
};

pub fn app(cx: &mut RenderCx<'_>) -> Element {
    let page = cx.use_state(|| Some(0usize));
    let current = page.value();
    let current_text = current.map_or_else(|| "none".to_string(), |index| index.to_string());
    let previous_page = page.clone();
    let next_page = page.clone();
    let selected_page = page;

    vstack(
        8.0,
        [
            FlipView::new(
                [
                    page_item(1, "Red"),
                    page_item(2, "Green"),
                    page_item(3, "Blue"),
                ],
                move |index| {
                    selected_page.set(index);
                },
            )
            .selected_index(current)
            .height(180.0)
            .build(),
            hstack(
                8.0,
                [
                    Button::new("Prev")
                        .on_click(move || {
                            previous_page.set(Some(current.unwrap_or(0).saturating_sub(1)));
                        })
                        .automation_id("previous-page")
                        .build(),
                    Button::new("Next")
                        .on_click(move || {
                            next_page.set(Some(current.unwrap_or(0).saturating_add(1).min(2)));
                        })
                        .automation_id("next-page")
                        .build(),
                    TextBlock::new(format!("page = {current_text}"))
                        .opacity(0.7)
                        .build(),
                ],
            ),
        ],
    )
}

fn page_item(key: u64, name: &str) -> FlipViewItem {
    FlipViewItem::new(
        key,
        Border::new(
            TextBlock::new(name)
                .font_size(20.0)
                .font_weight(FontWeight::BOLD)
                .build(),
        )
        .background(Color::rgb(245, 230, 220))
        .padding(Thickness::uniform(24.0))
        .build(),
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("FlipView", app)
}
