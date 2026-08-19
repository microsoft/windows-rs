#![windows_subsystem = "windows"]

use windows_reactor::{
    Application, Element, Grid, HorizontalAlignment, RenderCx, TextBlock, Thickness,
    VerticalAlignment, Window, WindowBackdrop, WindowConstraints, WindowSize,
};

fn app(cx: &mut RenderCx<'_>) -> Element {
    let open = cx.use_state(|| true);
    let size = cx.use_state(|| WindowSize {
        width: 800.0,
        height: 600.0,
    });
    let current = size.value();
    let set_size = size;
    let close = open.clone();

    let content = Grid::new([
        arrow(
            "Up",
            HorizontalAlignment::Center,
            VerticalAlignment::Top,
            Thickness::xy(0.0, 8.0),
        ),
        arrow(
            "Down",
            HorizontalAlignment::Center,
            VerticalAlignment::Bottom,
            Thickness::xy(0.0, 8.0),
        ),
        arrow(
            "Left",
            HorizontalAlignment::Left,
            VerticalAlignment::Center,
            Thickness::xy(8.0, 0.0),
        ),
        arrow(
            "Right",
            HorizontalAlignment::Right,
            VerticalAlignment::Center,
            Thickness::xy(8.0, 0.0),
        ),
        TextBlock::new(format!("({:.0}, {:.0})", current.width, current.height))
            .font_size(24.0)
            .horizontal_alignment(HorizontalAlignment::Center)
            .vertical_alignment(VerticalAlignment::Center)
            .build(),
    ])
    .build();

    let windows = if open.value() {
        vec![
            Window::new("Window Size", content, move || {
                close.set(false);
            })
            .backdrop(WindowBackdrop::Mica)
            .client_size(800.0, 600.0)
            .client_constraints(WindowConstraints {
                min_width: Some(400.0),
                min_height: Some(300.0),
                max_width: Some(1200.0),
                max_height: Some(900.0),
            })
            .on_size_changed(move |size| {
                set_size.set(size);
            })
            .build(),
        ]
    } else {
        Vec::new()
    };
    Application::new(windows).build()
}

fn arrow(
    label: &'static str,
    horizontal: HorizontalAlignment,
    vertical: VerticalAlignment,
    margin: Thickness,
) -> Element {
    TextBlock::new(label)
        .font_size(24.0)
        .horizontal_alignment(horizontal)
        .vertical_alignment(vertical)
        .margin(margin)
        .build()
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run_application(app)
}
