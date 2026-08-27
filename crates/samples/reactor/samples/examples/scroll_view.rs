#![windows_subsystem = "windows"]

use windows_reactor::*;

fn main() {
    sample_reactor_controls::run("ScrollView", || {
        let tall_body = StackPanel::new()
            .spacing(4.0)
            .keyed_children((1_u32..=30).map(|index| {
                KeyedView::new(
                    index,
                    TextBlock::new()
                        .text(format!("Line {index}"))
                        .font_size(13.0),
                )
            }));
        let wide_body = TextBlock::new()
            .text(
                "This line is intentionally long so the ScrollView scrolls horizontally to reveal \
                 the full content.",
            )
            .font_size(13.0);

        StackPanel::new().spacing(8.0).children((
            TextBlock::new().text("Default (vertical-only, auto)"),
            ScrollView::new().max_height(120.0).content(tall_body),
            TextBlock::new().text("Both axes, always visible"),
            ScrollView::new()
                .horizontal_scroll_bar_visibility(ScrollingScrollBarVisibility::Visible)
                .vertical_scroll_bar_visibility(ScrollingScrollBarVisibility::Visible)
                .max_width(280.0)
                .max_height(80.0)
                .content(wide_body),
        ))
    })
    .unwrap();
}
