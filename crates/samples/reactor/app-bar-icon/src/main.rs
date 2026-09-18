use windows_reactor::*;

fn main() {
    App::run(
        CommandBar::new()
            .primary_commands([KeyedView::new(
                "like",
                AppBarButton::new().label("Like").icon(Symbol::Like),
            )])
            .into(),
    )
    .unwrap();
}
