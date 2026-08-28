#![windows_subsystem = "windows"]

use windows_reactor::*;

fn main() {
    sample_reactor_controls::run("ScrollViewer", || {
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
                "This line is intentionally long so the ScrollViewer scrolls horizontally to \
                 reveal the full content.",
            )
            .font_size(13.0);

        StackPanel::new().spacing(8.0).children((
            "Default (vertical-only, auto)",
            ScrollViewer::new().max_height(120.0).content(tall_body),
            "Both axes, always visible",
            ScrollViewer::new()
                .horizontal_scroll_bar_visibility(ScrollBarVisibility::Visible)
                .vertical_scroll_bar_visibility(ScrollBarVisibility::Visible)
                .max_width(280.0)
                .max_height(80.0)
                .content(wide_body),
        ))
    })
    .unwrap();
}
