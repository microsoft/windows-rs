use windows_reactor::*;

fn main() {
    App::run(
        CommandBar::new()
            .primary_commands([
                KeyedView::new(
                    "symbol",
                    AppBarButton::new().label("Symbol").icon(Symbol::Like),
                ),
                KeyedView::new(
                    "search",
                    AppBarButton::new()
                        .label("Search")
                        .icon(FontIcon::new().glyph("\u{E721}")),
                ),
            ])
            .into(),
    )
    .unwrap();
}
