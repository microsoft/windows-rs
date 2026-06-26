//! Sample for the `cx.use_color_scheme` hook.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let scheme = cx.use_color_scheme();
    let is_dark = matches!(scheme, ColorScheme::Dark);

    let scheme_label = match scheme {
        ColorScheme::Light => "Light",
        ColorScheme::Dark => "Dark",
    };

    vstack((
        text_block(format!("is_dark_theme = {is_dark}"))
            .font_size(20.0)
            .bold(),
        text_block(format!("color_scheme  = {scheme_label}")).font_size(16.0),
        text_block(if is_dark {
            "🌙 dark branch"
        } else {
            "☀ light branch"
        })
        .font_size(14.0)
        .foreground(ThemeRef::PrimaryText),
    ))
    .spacing(8.0)
    .into()
}

fn main() -> Result<()> {
    reactor_samples::run("UseColorScheme", app)
}
