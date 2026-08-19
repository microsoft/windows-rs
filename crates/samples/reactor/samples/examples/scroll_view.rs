#![windows_subsystem = "windows"]

use windows_reactor::{
    Element, FontWeight, RenderCx, ScrollOrientation, ScrollView, ScrollViewBarVisibility,
    StackPanel, TextBlock, vstack,
};

pub fn app(_cx: &mut RenderCx<'_>) -> Element {
    let tall_body = StackPanel::new((1..=30).map(|line| {
        TextBlock::new(format!("Line {line}"))
            .font_size(13.0)
            .build()
    }))
    .spacing(4.0)
    .build();
    let wide_body = TextBlock::new(
        "This line is intentionally long so ScrollView can reveal the full horizontal content.",
    )
    .font_size(13.0)
    .build();

    vstack(
        8.0,
        [
            TextBlock::new("Default vertical scrolling")
                .font_weight(FontWeight::BOLD)
                .build(),
            ScrollView::new(tall_body).max_height(120.0).build(),
            TextBlock::new("Both axes with visible scroll bars")
                .font_weight(FontWeight::BOLD)
                .build(),
            ScrollView::new(wide_body)
                .content_orientation(ScrollOrientation::Both)
                .horizontal_scroll_bar_visibility(ScrollViewBarVisibility::Visible)
                .vertical_scroll_bar_visibility(ScrollViewBarVisibility::Visible)
                .max_width(280.0)
                .max_height(80.0)
                .build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("ScrollView", app)
}
