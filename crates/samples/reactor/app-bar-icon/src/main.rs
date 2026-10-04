use windows_reactor::*;

fn main() {
    App::run(
        CommandBar::new().primary_commands((
            AppBarButton::new().label("Symbol").icon(Symbol::Like),
            AppBarSeparator::new(),
            AppBarButton::new()
                .label("Search")
                .icon(Icon::font("\u{E721}")),
        )),
    )
    .unwrap();
}
