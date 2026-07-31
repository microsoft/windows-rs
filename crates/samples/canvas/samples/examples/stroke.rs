#![windows_subsystem = "windows"]

use windows_canvas::*;
use windows_reactor::DrawContext;

fn draw(ctx: &DrawContext) -> Result<()> {
    ctx.clear(ColorF::BLACK);

    let brush = ctx.create_solid_brush(ColorF::WHITE)?;

    let device = ctx.device();
    let margin = 30.0;
    let spacing = (ctx.height - margin * 2.0) / 4.0;

    let styles: Vec<(&str, StrokeStyle)> = [
        (
            "Dash + Round caps",
            StrokeStyleBuilder::new()
                .dash_style(DashStyle::Dash)
                .caps(CapStyle::Round),
        ),
        (
            "Dot + Square caps",
            StrokeStyleBuilder::new()
                .dash_style(DashStyle::Dot)
                .caps(CapStyle::Square),
        ),
        (
            "DashDot + Triangle caps",
            StrokeStyleBuilder::new()
                .dash_style(DashStyle::DashDot)
                .caps(CapStyle::Triangle),
        ),
        (
            "Solid + Bevel join",
            StrokeStyleBuilder::new().line_join(LineJoin::Bevel),
        ),
    ]
    .into_iter()
    .filter_map(|(name, builder)| device.create_stroke_style(&builder).ok().map(|s| (name, s)))
    .collect();

    let label_fmt = TextFormat::new("Segoe UI", 14.0)?;

    for (i, (name, style)) in styles.iter().enumerate() {
        let y = margin + i as f32 * spacing + spacing / 2.0;

        brush.set_color(ColorF::WHITE);
        ctx.draw_text(
            name,
            &label_fmt,
            &Rect::new(margin, y - 20.0, ctx.width - margin, y - 2.0),
            &brush,
        );

        brush.set_color(ColorF::CORNFLOWER_BLUE);
        ctx.draw_line_styled(
            Vector2::new(margin, y),
            Vector2::new(ctx.width - margin, y),
            &brush,
            6.0,
            style,
        );
    }
    Ok(())
}

fn main() -> Result<()> {
    canvas_samples::run("Stroke Styles", draw)
}
