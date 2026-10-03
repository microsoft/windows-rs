use windows_reactor::*;

fn main() {
    let icon = Icon::font("\u{E7C3}");
    App::run(
        StackPanel::new().children((
            TitleBar::new()
                .title("Title bar icon")
                .icon(icon.clone())
                .left_header(TextBlock::new().text("Left header")),
            AppBarButton::new().label("Same icon").icon(icon),
        )),
    )
    .unwrap();
}
