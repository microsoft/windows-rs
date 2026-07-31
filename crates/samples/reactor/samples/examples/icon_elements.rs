use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (page, set_page) = cx.use_state(String::from("home"));
    let image = format!(
        "file:///{}/examples/image.svg",
        env!("CARGO_MANIFEST_DIR").replace('\\', "/"),
    );

    let content = match page.as_str() {
        "home" => text_block("Symbol icon (SymbolIcon)."),
        "starred" => text_block("Font-glyph icon (FontIcon)."),
        "repo" => text_block("SVG image icon (ImageIcon)."),
        _ => text_block("Unknown page"),
    };

    NavigationView::new(
        [
            NavViewItem::new("Home").tag("home").icon(Symbol::Home),
            NavViewItem::new("Starred")
                .tag("starred")
                .icon(Icon::font("\u{E734}")),
            NavViewItem::new("Repository")
                .tag("repo")
                .icon(Icon::image(image)),
        ],
        content,
    )
    .selected_tag(&*page)
    .on_selection_changed(move |tag: String| set_page.call(tag))
    .into()
}

fn main() -> Result<()> {
    reactor_samples::run("IconElements", app)
}
