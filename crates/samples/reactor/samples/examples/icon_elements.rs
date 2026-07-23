//! Sample for the [`Icon`] kinds beyond [`Symbol`]: `Icon::bitmap` (an image
//! from a URI, e.g. a brand mark) and `Icon::font` (a glyph from an icon font).
//!
//! This mirrors the common need to show a custom/brand icon — such as a GitHub
//! repository link — in a `NavigationView`, which `Symbol` alone cannot express.

use windows_reactor::*;

/// The GitHub mark, served as a PNG. Any `http(s)` URL or `ms-appx:///` package
/// path works as a `BitmapIcon` source.
const GITHUB_MARK: &str =
    "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png";

fn app(cx: &mut RenderCx) -> Element {
    let (page, set_page) = cx.use_state(String::from("home"));

    let content = match page.as_str() {
        "home" => text_block("Symbol icon (SymbolIcon)."),
        "starred" => text_block("Font-glyph icon (FontIcon)."),
        "repo" => text_block("Bitmap brand icon (BitmapIcon)."),
        _ => text_block("Unknown page"),
    };

    NavigationView::new(
        [
            NavViewItem::new("Home").tag("home").icon(Symbol::Home),
            // A glyph from the default icon font.
            NavViewItem::new("Starred")
                .tag("starred")
                .icon(Icon::font("\u{E734}")),
            // A brand image loaded from a URI.
            NavViewItem::new("Repository")
                .tag("repo")
                .icon(Icon::bitmap(GITHUB_MARK)),
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
