#![windows_subsystem = "windows"]

use windows_reactor::{
    Element, Icon, IconSymbol, ImageSource, NavigationItem, NavigationView, RenderCx, TextBlock,
};

fn app(cx: &mut RenderCx<'_>) -> Element {
    let page = cx.use_state(|| 0_u64);
    let set_page = page.clone();
    let image = format!(
        "file:///{}/examples/image.svg",
        env!("CARGO_MANIFEST_DIR").replace('\\', "/"),
    );
    let bitmap = format!(
        "file:///{}/examples/image.png",
        env!("CARGO_MANIFEST_DIR").replace('\\', "/"),
    );

    let content = TextBlock::new(match page.value() {
        0 => "Symbol icon (SymbolIcon).",
        1 => "Font-glyph icon (FontIcon).",
        2 => "SVG image icon (ImageIcon).",
        3 => "Foreground-tinted bitmap mask (BitmapIcon).",
        4 => "Vector path data (PathIcon).",
        _ => "Unknown page",
    })
    .build();

    NavigationView::new(
        [
            NavigationItem::new(0, "Home").icon(Icon::symbol(IconSymbol::HOME)),
            NavigationItem::new(1, "Starred").icon(Icon::font("\u{E734}", "Segoe MDL2 Assets")),
            NavigationItem::new(2, "Repository").icon(Icon::image(ImageSource::svg(image))),
            NavigationItem::new(3, "Bitmap mask").icon(Icon::bitmap(bitmap, true)),
            NavigationItem::new(4, "Path")
                .icon(Icon::path("F1 M 0,8 L 6,14 L 16,2 L 14,0 L 6,10 L 2,6 Z")),
        ],
        content,
        move |key| {
            if let Some(key) = key {
                set_page.set(key);
            }
        },
    )
    .selected_key(Some(page.value()))
    .build()
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("IconElements", app)
}
