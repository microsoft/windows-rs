#![windows_subsystem = "windows"]

use windows_reactor::*;

#[derive(Clone, Copy, PartialEq)]
enum HoverKind {
    None,
    File,
    Text,
    Rejected,
}

fn app(cx: &mut RenderCx) -> Element {
    let (hover, set_hover) = cx.use_async_state(HoverKind::None);
    let (dropped, set_dropped) = cx.use_async_state(Option::<String>::None);

    let background_color = match hover {
        HoverKind::None => Color::rgb(255, 255, 255),
        HoverKind::File => Color::rgb(92, 202, 221),
        HoverKind::Text => Color::rgb(155, 219, 90),
        HoverKind::Rejected => Color::rgb(255, 180, 180),
    };

    let label = dropped.as_deref().unwrap_or("Drop files or some text here");

    border(
        text_block(label)
            .wrap()
            .font_size(24.0)
            .horizontal_alignment(HorizontalAlignment::Center)
            .vertical_alignment(VerticalAlignment::Center),
    )
    .background(background_color)
    .border_brush(Color::rgb(224, 224, 224))
    .border_thickness(Thickness::uniform(1.5))
    .corner_radius(12.0)
    .padding(Thickness::uniform(20.0))
    .horizontal_alignment(HorizontalAlignment::Stretch)
    .vertical_alignment(VerticalAlignment::Stretch)
    .margin(Thickness::uniform(40.0))
    // 1. Enables drag-and-drop on the target element.
    .allow_drop(true)
    // 2a. Fires once when the drag enters the target. Inspect and accept or reject.
    .drag_enter({
        let set_hover = set_hover.clone();
        move |ctx| accept(ctx, &set_hover)
    })
    // 2b. Fires once when the drag leaves the target. Clear all hover feedback.
    .drag_leave({
        let set_hover = set_hover.clone();
        move |_ctx| set_hover.call(HoverKind::None)
    })
    // 3. Fires continuously while hovering (including modifier-key changes). Re-run accept logic.
    .drag_over({
        let set_hover = set_hover.clone();
        move |ctx| accept(ctx, &set_hover)
    })
    // 4. Fires once on release. Read the dropped data and update state.
    .drag_drop({
        move |ctx| {
            set_hover.call(HoverKind::None);
            let items = ctx.storage_items();
            if !items.is_empty() {
                let label = if items.len() == 1 {
                    items[0].path.clone()
                } else {
                    let names: Vec<_> = items.iter().map(|i| i.name.clone()).collect();
                    format!("{} files dropped: {}", items.len(), names.join(", "))
                };
                set_dropped.call(Some(label));
                DragOperation::Link
            } else if let Some(text) = ctx.text() {
                set_dropped.call(Some(text));
                DragOperation::Copy
            } else {
                DragOperation::None
            }
        }
    })
    .into()
}

fn accept(ctx: &mut DragContext, set_hover: &AsyncSetState<HoverKind>) -> DragOperation {
    if ctx.has_storage_items {
        set_hover.call(HoverKind::File);
        ctx.caption = Some("Drop to link file(s)".into());
        DragOperation::Link
    } else if ctx.has_text {
        set_hover.call(HoverKind::Text);
        ctx.caption = Some("Drop to paste text".into());
        DragOperation::Copy
    } else {
        set_hover.call(HoverKind::Rejected);
        ctx.caption = Some("Not supported".into());
        DragOperation::None
    }
}

fn main() -> Result<()> {
    bootstrap()?;

    App::new()
        .title("Drag & Drop Example")
        .inner_size(800.0, 600.0)
        .render(app)
}
