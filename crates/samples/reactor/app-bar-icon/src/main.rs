use windows_reactor::*;

fn main() {
    App::run(
        CommandBar::new()
            .primary_commands([
                Keyed::new(
                    "symbol",
                    AppBarButton::new().label("Symbol").icon(Symbol::Like),
                ),
                Keyed::new("separator", AppBarSeparator::new()),
                Keyed::new(
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
