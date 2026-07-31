#![windows_subsystem = "windows"]

use windows_canvas::*;
use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let points = cx.use_ref(Vec::<Vector2>::new());
    let inv = cx.use_invalidator();

    canvas_invalidated(&inv, {
        let points = points.clone();
        move |ctx| draw(ctx, &points.borrow())
    })
    .on_pointer_pressed(move |info: PointerEventInfo| {
        points
            .borrow_mut()
            .push(Vector2::new(info.x as f32, info.y as f32));
        inv.invalidate();
    })
    .into()
}

fn draw(ctx: &DrawContext, points: &[Vector2]) -> Result<()> {
    ctx.clear(ColorF::from_rgb8(0x10, 0x12, 0x18));

    let brush = ctx.create_solid_brush(ColorF::CORNFLOWER_BLUE)?;

    for pair in points.windows(2) {
        ctx.draw_line(pair[0], pair[1], &brush, 2.0);
    }
    for &p in points {
        ctx.fill_ellipse(&Ellipse::circle(p, 4.0), &brush);
    }
    Ok(())
}

fn main() -> Result<()> {
    bootstrap()?;
    App::new()
        .title("Invalidate")
        .backdrop(Backdrop::Mica)
        .render(app)
}
