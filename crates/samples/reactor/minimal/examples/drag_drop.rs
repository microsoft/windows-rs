#![windows_subsystem = "windows"]

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (hovering, set_hovering) = cx.use_async_state(false);
    let (dropped_path, set_dropped_path) = cx.use_async_state(Option::<String>::None);

    let bg = if hovering {
        Color::rgb(92, 202, 221)
    } else {
        Color::rgb(255, 255, 255)
    };

    let label = dropped_path.as_deref().unwrap_or("Drop a file here");

    border(
        text_block(label)
            .font_size(24.0)
            .horizontal_alignment(HorizontalAlignment::Center)
            .vertical_alignment(VerticalAlignment::Center),
    )
    .background(bg)
    .border_brush(Color::rgb(224, 224, 224))
    .border_thickness(Thickness::uniform(1.5))
    .corner_radius(12.0)
    .padding(Thickness::uniform(20.0))
    .horizontal_alignment(HorizontalAlignment::Stretch)
    .vertical_alignment(VerticalAlignment::Stretch)
    .margin(Thickness::uniform(40.0))
    .allow_drop(true)
    .drag_enter({
        let set_hovering = set_hovering.clone();
        move |ctx| {
            if ctx.has_storage_items {
                set_hovering.call(true);
                DragOperation::Link
            } else {
                DragOperation::None
            }
        }
    })
    .drag_leave({
        let set_hovering = set_hovering.clone();
        move |_ctx| set_hovering.call(false)
    })
    .drag_over(|ctx| {
        if ctx.has_storage_items {
            DragOperation::Link
        } else {
            DragOperation::None
        }
    })
    .drag_drop({
        move |ctx| {
            set_hovering.call(false);
            if let Some(item) = ctx.get_storage_items().into_iter().next() {
                set_dropped_path.call(Some(item.path));
                DragOperation::Link
            } else {
                DragOperation::None
            }
        }
    })
    .into()
}

fn main() -> Result<()> {
    bootstrap()?;

    App::new()
        .title("Drag & Drop")
        .inner_size(800.0, 600.0)
        .render(app)
}
