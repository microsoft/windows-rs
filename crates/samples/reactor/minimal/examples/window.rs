use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let window_size = cx.use_inner_size();

    let mut children = create_arrows();
    children.push(
        TextBlock::new(format!(
            "({}, {})",
            window_size.width as i32, window_size.height as i32
        ))
        .font_size(24.0)
        .horizontal_alignment(HorizontalAlignment::Center)
        .vertical_alignment(VerticalAlignment::Center)
        .into(),
    );

    grid(children)
        .vertical_alignment(VerticalAlignment::Stretch)
        .horizontal_alignment(HorizontalAlignment::Stretch)
        .into()
}

fn create_arrows() -> Vec<Element> {
    [
        (
            "🢁",
            HorizontalAlignment::Center,
            VerticalAlignment::Top,
            Thickness::xy(0.0, -8.0),
        ),
        (
            "🢃",
            HorizontalAlignment::Center,
            VerticalAlignment::Bottom,
            Thickness::xy(0.0, -5.0),
        ),
        (
            "🢀",
            HorizontalAlignment::Left,
            VerticalAlignment::Center,
            Thickness::xy(-2.0, 0.0),
        ),
        (
            "🢂",
            HorizontalAlignment::Right,
            VerticalAlignment::Center,
            Thickness::xy(-2.0, 0.0),
        ),
    ]
    .into_iter()
    .map(|(arrow, halign, valign, margin)| {
        TextBlock::new(arrow)
            .font_size(24.0)
            .horizontal_alignment(halign)
            .vertical_alignment(valign)
            .margin(margin)
            .into()
    })
    .collect()
}

fn main() -> Result<()> {
    let _bootstrap_handle = bootstrap::initialize()?;
    App::new()
        .title("Sample")
        .backdrop(Backdrop::Mica)
        .inner_size(800.0, 600.0)
        .inner_constraints(InnerConstraints {
            min_width: Some(400.0),
            min_height: Some(300.0),
            max_height: Some(900.0),
            max_width: Some(1200.0),
        })
        .render(app)
}
