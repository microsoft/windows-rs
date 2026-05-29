//! Minimal sample for the context API.
//!
//! A parent provides a value via `Element::provide(&CTX, value)`. A
//! nested component reads it via `cx.use_context(&CTX)` without
//! prop-drilling through ancestors.

use std::sync::LazyLock;

use windows_reactor::*;

static THEME: LazyLock<Context<String>> = LazyLock::new(|| Context::new("light".to_string()));

fn leaf(_: &(), cx: &mut RenderCx) -> Element {
    let theme = cx.use_context(&THEME);
    let (bg, fg) = match theme.as_str() {
        "dark" => (Color::rgb(30, 30, 30), Color::rgb(255, 255, 255)),
        "neon" => (Color::rgb(50, 200, 150), Color::rgb(0, 0, 0)),
        _ => (Color::rgb(240, 240, 240), Color::rgb(0, 0, 0)),
    };
    border(
        text_block(format!("Leaf sees theme = {theme}"))
            .font_size(16.0)
            .foreground(fg)
            .padding(Thickness::uniform(16.0)),
    )
    .background(bg)
    .into()
}

fn app(cx: &mut RenderCx) -> Element {
    let (theme, set_theme) = cx.use_state("light".to_string());

    let pick = |name: &'static str| {
        let set_theme = set_theme.clone();
        button(name).on_click(move || set_theme.call(name.to_string()))
    };

    let tree = vstack((
        text_block("Pick a theme; the leaf below reads it via cx.use_context.").font_size(12.0),
        hstack((pick("light"), pick("dark"), pick("neon"))).spacing(8.0),
        border(component(leaf, ())).padding(Thickness::uniform(8.0)),
    ))
    .spacing(12.0)
    .padding(Thickness::uniform(16.0));

    tree.provide(&THEME, theme)
}

fn main() -> Result<()> {
    App::new().title("Sample").render(app)
}
