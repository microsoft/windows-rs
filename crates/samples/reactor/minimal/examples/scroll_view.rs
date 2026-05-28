//! Minimal sample for the `ScrollView` element (modern replacement for `ScrollViewer`).

use windows_reactor::*;

fn app(_cx: &mut RenderCx) -> impl Into<Element> {
    let tall_body = vstack(
        (1..=30)
            .map(|i| text_block(format!("Line {i}")).font_size(13.0).into())
            .collect::<Vec<Element>>(),
    )
    .spacing(4.0);

    let wide_body = text_block(
        "This line is intentionally long so the ScrollView must scroll \
             horizontally to reveal the full sentence end-to-end.",
    )
    .font_size(13.0);

    vstack((
        text_block("Default (vertical-only, auto)").bold(),
        scroll_view(tall_body).max_height(120.0),
        text_block("Both axes, always visible").bold(),
        scroll_view(wide_body)
            .horizontal_scroll_bar_visibility(ScrollViewScrollBarVisibility::Visible)
            .vertical_scroll_bar_visibility(ScrollViewScrollBarVisibility::Visible)
            .max_width(280.0)
            .max_height(80.0),
    ))
    .spacing(8.0)
}

fn main() -> Result<()> {
    App::new().title("ScrollView Sample").render(app)
}
