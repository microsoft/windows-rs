#![windows_subsystem = "windows"]

use std::cell::Cell;
use std::rc::Rc;
use windows_canvas::*;
use windows_reactor::{
    App, Border, Component, ComponentContext, PointerEventInfo, View, ViewContext, WindowBackdrop,
    WindowVisuals,
};
use windows_text::{HitTest, TextFormat, TextLayout};

const TEXT: &str = "Point at the text to find a cluster, and drag to select. Carets and \
selections follow the reading direction: \u{5e9}\u{5dc}\u{5d5}\u{5dd} \u{5e2}\u{5d5}\u{5dc}\u{5dd} and \
\u{645}\u{631}\u{62d}\u{628}\u{627} \u{628}\u{627}\u{644}\u{639}\u{627}\u{644}\u{645}, with an emoji \
\u{1f600} and a combining e\u{301}.";

const MARGIN: f32 = 32.0;

// The pointer, and where a drag started, in DIPs relative to the canvas.
#[derive(Clone, Copy, Default)]
struct Pointer {
    position: Option<Vector2>,
    anchor: Option<Vector2>,
}

struct Sample {
    pointer: Rc<Cell<Pointer>>,
    invalidator: Invalidator,
}

enum Message {
    Pressed(PointerEventInfo),
    Moved(PointerEventInfo),
    Released,
    Exited,
}

impl Component for Sample {
    type Input = ();
    type Message = Message;

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            pointer: Rc::default(),
            invalidator: Invalidator::new(),
        }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        let mut pointer = self.pointer.get();

        match message {
            Message::Pressed(info) => {
                pointer.position = Some(Vector2::new(info.x as f32, info.y as f32));
                pointer.anchor = pointer.position;
            }
            Message::Moved(info) => {
                pointer.position = Some(Vector2::new(info.x as f32, info.y as f32));
            }
            Message::Released => pointer.anchor = None,
            Message::Exited => pointer = Pointer::default(),
        }

        self.pointer.set(pointer);
        self.invalidator.invalidate();
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Text hit-testing");
        context.window_visuals(WindowVisuals::new().backdrop(WindowBackdrop::Mica));
        let pointer = Rc::clone(&self.pointer);

        Border::new()
            .on_pointer_pressed(context.callback(Message::Pressed))
            .on_pointer_moved(context.callback(Message::Moved))
            .on_pointer_released(context.callback(|_| Message::Released))
            .on_pointer_exited(context.callback(|_| Message::Exited))
            .content(canvas_invalidated(&self.invalidator, move |ctx| {
                draw(ctx, pointer.get())
            }))
            .into()
    }
}

// The caret position of a hit: before the cluster, or after it for a trailing hit.
fn caret(hit: &HitTest) -> usize {
    if hit.is_trailing_hit {
        hit.text_position + hit.length
    } else {
        hit.text_position
    }
}

fn draw(ctx: &DrawContext, pointer: Pointer) -> Result<()> {
    ctx.clear(ColorF::from_rgb8(16, 20, 28));
    let white = ctx.create_solid_brush(ColorF::WHITE)?;
    let selection = ctx.create_solid_brush(ColorF::new(0.4, 0.6, 1.0, 0.4))?;
    let cluster = ctx.create_solid_brush(ColorF::new(0.3, 0.8, 0.4, 1.0))?;
    let accent = ctx.create_solid_brush(ColorF::CORNFLOWER_BLUE)?;

    let format = TextFormat::new("Segoe UI", 28.0)?;
    let layout = TextLayout::new(
        TEXT,
        &format,
        (ctx.width - 2.0 * MARGIN).max(0.0),
        (ctx.height - 3.0 * MARGIN).max(0.0),
    )?;
    let origin = Vector2::new(MARGIN, MARGIN);

    let Some(position) = pointer.position else {
        ctx.draw_text_layout(origin, &layout, &white);
        return Ok(());
    };

    // Hit-testing takes a point relative to the layout box.
    let hit = layout.hit_test_point(position - origin);

    if let Some(anchor) = pointer.anchor {
        let start = caret(&layout.hit_test_point(anchor - origin));
        let end = caret(&hit);

        for rect in layout.range_bounds(start.min(end)..start.max(end)) {
            ctx.fill_rect(&rect.offset(MARGIN, MARGIN), &selection);
        }
    }

    if hit.is_inside {
        ctx.draw_rect(&hit.bounds.offset(MARGIN, MARGIN), &cluster, 1.0);
    }

    ctx.draw_text_layout(origin, &layout, &white);

    let bounds = layout
        .caret_bounds(hit.text_position, hit.is_trailing_hit)
        .offset(MARGIN, MARGIN);
    ctx.fill_rect(
        &Rect::new(
            bounds.left - 1.0,
            bounds.top,
            bounds.left + 1.0,
            bounds.bottom,
        ),
        &accent,
    );

    let cluster_text = &TEXT[hit.text_position..hit.text_position + hit.length];
    let summary = format!(
        "Cluster {cluster_text:?} at byte {}, trailing: {}, inside: {}",
        hit.text_position, hit.is_trailing_hit, hit.is_inside
    );
    let format = TextFormat::new("Consolas", 15.0)?;
    let label = TextLayout::new(
        &summary,
        &format,
        (ctx.width - 2.0 * MARGIN).max(0.0),
        MARGIN,
    )?;
    ctx.draw_text_layout(Vector2::new(MARGIN, ctx.height - MARGIN), &label, &accent);
    Ok(())
}

fn main() -> Result<()> {
    App::run_component::<Sample>(())
}
