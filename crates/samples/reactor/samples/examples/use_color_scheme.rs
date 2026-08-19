#![windows_subsystem = "windows"]

use windows_reactor::{
    Application, ColorScheme, Element, RenderCx, TextBlock, ThemeBrush, Window, vstack,
};

fn app(cx: &mut RenderCx<'_>) -> Element {
    let open = cx.use_state(|| true);
    let scheme = cx.use_state(|| ColorScheme::Light);
    let current = scheme.value();
    let set_scheme = scheme;
    let close = open.clone();
    let is_dark = current == ColorScheme::Dark;

    let content = vstack(
        8.0,
        [
            TextBlock::new(format!("is_dark_theme = {is_dark}"))
                .font_size(20.0)
                .font_weight(windows_reactor::FontWeight::BOLD)
                .build(),
            TextBlock::new(format!(
                "color_scheme  = {}",
                if is_dark { "Dark" } else { "Light" }
            ))
            .font_size(16.0)
            .build(),
            TextBlock::new(if is_dark {
                "dark color branch"
            } else {
                "light color branch"
            })
            .font_size(14.0)
            .foreground(ThemeBrush::PrimaryText)
            .build(),
        ],
    );

    let windows = if open.value() {
        vec![
            Window::new("Color Scheme", content, move || {
                close.set(false);
            })
            .on_color_scheme_changed(move |scheme| {
                set_scheme.set(scheme);
            })
            .build(),
        ]
    } else {
        Vec::new()
    };
    Application::new(windows).build()
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run_application(app)
}
