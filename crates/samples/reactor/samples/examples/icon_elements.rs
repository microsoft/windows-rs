use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (page, set_page) = cx.use_state(String::from("home"));
    // The sample runner is unpackaged, so absolute file URIs point at the example assets.
    // Packaged apps should use ms-appx:/// URIs for files included in the package.
    let image = format!(
        "file:///{}/examples/image.svg",
        env!("CARGO_MANIFEST_DIR").replace('\\', "/"),
    );
    let bitmap = format!(
        "file:///{}/examples/image.png",
        env!("CARGO_MANIFEST_DIR").replace('\\', "/"),
    );

    let content = match page.as_str() {
        "home" => text_block("Symbol icon (SymbolIcon)."),
        "starred" => text_block("Font-glyph icon (FontIcon)."),
        "repo" => text_block("SVG image icon (ImageIcon)."),
        "bitmap" => text_block("Foreground-tinted bitmap mask (BitmapIcon)."),
        "path" => text_block("Vector path data (PathIcon)."),
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
            NavViewItem::new("Bitmap mask")
                .tag("bitmap")
                .icon(Icon::bitmap_icon(bitmap, true)),
            NavViewItem::new("Path")
                .tag("path")
                .icon(Icon::path("F1 M 0,8 L 6,14 L 16,2 L 14,0 L 6,10 L 2,6 Z")),
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
