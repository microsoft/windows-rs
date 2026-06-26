//! Interactive shape editor — composes the reactor pointer events (phases 1 & 2)
//! with the `windows-canvas` geometry queries (phase 4) into a small map-style
//! editor.
//!
//! - Click an empty spot to drop a shape of the current tool kind.
//! - Click a shape to select it, then drag (pointer-moved while pressed) to move it.
//! - Right-click a shape to delete it.
//! - The toolbar picks the active shape kind or clears the canvas.
//!
//! Selection hit-tests against each shape's *actual filled polygon* via
//! `Path::fill_contains_point`, not its bounding box; the selection outline comes
//! from `Path::compute_bounds`. Pointer coordinates and the drawing surface are
//! both in device-independent pixels, so positions feed straight into the
//! geometry queries with no conversion.
//!
//! The whole editor model lives in a single `use_ref`, so high-frequency pointer
//! handlers mutate it in place without triggering reconcile churn during a drag;
//! the per-frame draw loop simply reflects the current model.

#![windows_subsystem = "windows"]

use windows_canvas::*;
use windows_reactor::*;

#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Rectangle,
    Triangle,
    Star,
}

impl Kind {
    fn label(self) -> &'static str {
        match self {
            Self::Rectangle => "Rectangle",
            Self::Triangle => "Triangle",
            Self::Star => "Star",
        }
    }
}

struct Shape {
    kind: Kind,
    x: f32,
    y: f32,
    color: ColorF,
    // Geometry cached in absolute coordinates, used for both drawing and
    // `fill_contains_point`. Rebuilt only when the shape moves or the device is
    // lost — see the draw loop.
    path: Option<Path>,
    built_at: Option<(f32, f32)>,
}

impl Shape {
    fn new(kind: Kind, x: f32, y: f32, color: ColorF) -> Self {
        Self {
            kind,
            x,
            y,
            color,
            path: None,
            built_at: None,
        }
    }
}

struct Model {
    shapes: Vec<Shape>,
    kind: Kind,
    selected: Option<usize>,
    drag_offset: Option<(f32, f32)>,
    next_color: usize,
}

impl Model {
    fn new() -> Self {
        Self {
            shapes: Vec::new(),
            kind: Kind::Star,
            selected: None,
            drag_offset: None,
            next_color: 0,
        }
    }

    // Topmost shape whose filled geometry contains the point, if any.
    fn hit(&self, x: f32, y: f32) -> Option<usize> {
        self.shapes
            .iter()
            .enumerate()
            .rev()
            .find(|(_, s)| {
                s.path
                    .as_ref()
                    .is_some_and(|p| p.fill_contains_point(Vector2::new(x, y)))
            })
            .map(|(i, _)| i)
    }
}

fn app(cx: &mut RenderCx) -> Element {
    let model = cx.use_ref(Model::new());

    let on_pressed = cx.use_callback((), {
        let model = model.clone();
        move |info: PointerEventInfo| {
            let (x, y) = (info.x as f32, info.y as f32);
            let mut m = model.borrow_mut();
            let hit = m.hit(x, y);

            if info.is_right_button_pressed {
                if let Some(i) = hit {
                    m.shapes.remove(i);
                    m.selected = None;
                    m.drag_offset = None;
                }
                return;
            }

            if let Some(i) = hit {
                let (sx, sy) = (m.shapes[i].x, m.shapes[i].y);
                m.selected = Some(i);
                m.drag_offset = Some((x - sx, y - sy));
            } else {
                let kind = m.kind;
                let color = palette(m.next_color);
                m.next_color += 1;
                m.shapes.push(Shape::new(kind, x, y, color));
                m.selected = Some(m.shapes.len() - 1);
                m.drag_offset = Some((0.0, 0.0));
            }
        }
    });

    let on_moved = cx.use_callback((), {
        let model = model.clone();
        move |info: PointerEventInfo| {
            if !info.is_left_button_pressed {
                return;
            }
            let mut m = model.borrow_mut();
            if let (Some(i), Some((ox, oy))) = (m.selected, m.drag_offset)
                && let Some(s) = m.shapes.get_mut(i)
            {
                s.x = info.x as f32 - ox;
                s.y = info.y as f32 - oy;
            }
        }
    });

    let on_released = cx.use_callback((), {
        let model = model.clone();
        move |_: PointerEventInfo| model.borrow_mut().drag_offset = None
    });

    let margin = 16.0;

    grid((
        Element::from(
            animated_canvas({
                let model = model.clone();
                move |ctx| draw(ctx, &model)
            })
            .on_pointer_pressed(on_pressed)
            .on_pointer_moved(on_moved)
            .on_pointer_released(on_released)
            .margin(Thickness {
                left: margin,
                top: margin,
                right: margin,
                bottom: 0.0,
            }),
        )
        .grid_row(0),
        Element::from(
            hstack((
                tool_button(&model, Kind::Rectangle),
                tool_button(&model, Kind::Triangle),
                tool_button(&model, Kind::Star),
                button("Clear").on_click({
                    let model = model.clone();
                    move || {
                        let mut m = model.borrow_mut();
                        m.shapes.clear();
                        m.selected = None;
                        m.drag_offset = None;
                    }
                }),
            ))
            .spacing(8.0)
            .margin(Thickness::uniform(margin)),
        )
        .grid_row(1),
    ))
    .rows([GridLength::STAR, GridLength::Auto])
    .into()
}

fn tool_button(model: &HookRef<Model>, kind: Kind) -> Button {
    let model = model.clone();
    button(kind.label()).on_click(move || model.borrow_mut().kind = kind)
}

fn draw(ctx: &DrawContext<'_>, model: &HookRef<Model>) {
    ctx.clear(ColorF::new(0.11, 0.12, 0.16, 1.0));

    // Faint grid lines for a map-like backdrop.
    if let Ok(grid_brush) = ctx.create_solid_brush(ColorF::new(1.0, 1.0, 1.0, 0.06)) {
        let step = 40.0;
        let mut gx = step;
        while gx < ctx.width {
            ctx.draw_line(
                Vector2::new(gx, 0.0),
                Vector2::new(gx, ctx.height),
                &grid_brush,
                1.0,
            );
            gx += step;
        }
        let mut gy = step;
        while gy < ctx.height {
            ctx.draw_line(
                Vector2::new(0.0, gy),
                Vector2::new(ctx.width, gy),
                &grid_brush,
                1.0,
            );
            gy += step;
        }
    }

    let device_changed = ctx.device_changed();
    let mut m = model.borrow_mut();
    let selected = m.selected;

    for (i, s) in m.shapes.iter_mut().enumerate() {
        if device_changed || s.built_at != Some((s.x, s.y)) {
            s.path = build_path(ctx.device(), s.kind, s.x, s.y).ok();
            s.built_at = Some((s.x, s.y));
        }
        let Some(path) = &s.path else {
            continue;
        };

        if let Ok(brush) = ctx.create_solid_brush(s.color) {
            ctx.fill_path(path, &brush);
        }

        if Some(i) == selected
            && let Ok(brush) = ctx.create_solid_brush(ColorF::WHITE)
        {
            let b = path.compute_bounds();
            let pad = 4.0;
            ctx.draw_rect(
                &Rect::new(b.left - pad, b.top - pad, b.right + pad, b.bottom + pad),
                &brush,
                1.5,
            );
        }
    }

    if let Ok(format) = TextFormat::with_weight("Segoe UI", 16.0, FontWeight::BOLD)
        && let Ok(brush) = ctx.create_solid_brush(ColorF::WHITE)
    {
        let label = format!(
            "{} shape(s)  ·  tool: {}  ·  click to add, left-drag to move, right-click to delete",
            m.shapes.len(),
            m.kind.label()
        );
        let rect = Rect::new(12.0, ctx.height - 30.0, ctx.width, ctx.height);
        ctx.draw_text(&label, &format, &rect, &brush);
    }
}

const SIZE: f32 = 38.0;

fn build_path(device: &GpuDevice, kind: Kind, x: f32, y: f32) -> Result<Path> {
    let points = polygon(kind, x, y);
    let mut figure = PathBuilder::new(device)?.begin(points[0]);
    for p in &points[1..] {
        figure = figure.line_to(*p);
    }
    figure.close().build()
}

fn polygon(kind: Kind, x: f32, y: f32) -> Vec<Vector2> {
    match kind {
        Kind::Rectangle => {
            let (hw, hh) = (SIZE, SIZE * 0.72);
            vec![
                Vector2::new(x - hw, y - hh),
                Vector2::new(x + hw, y - hh),
                Vector2::new(x + hw, y + hh),
                Vector2::new(x - hw, y + hh),
            ]
        }
        Kind::Triangle => vec![
            Vector2::new(x, y - SIZE),
            Vector2::new(x + SIZE, y + SIZE),
            Vector2::new(x - SIZE, y + SIZE),
        ],
        Kind::Star => (0..10)
            .map(|i| {
                let r = if i % 2 == 0 { SIZE } else { SIZE * 0.45 };
                let angle = std::f32::consts::PI / 5.0 * i as f32 - std::f32::consts::FRAC_PI_2;
                Vector2::new(x + r * angle.cos(), y + r * angle.sin())
            })
            .collect(),
    }
}

fn palette(i: usize) -> ColorF {
    const COLORS: [(f32, f32, f32); 6] = [
        (0.26, 0.47, 0.78),
        (0.86, 0.31, 0.47),
        (0.30, 0.69, 0.40),
        (0.95, 0.61, 0.18),
        (0.55, 0.40, 0.78),
        (0.20, 0.68, 0.71),
    ];
    let (r, g, b) = COLORS[i % COLORS.len()];
    ColorF::new(r, g, b, 1.0)
}

fn main() -> Result<()> {
    App::new()
        .title("Canvas editor")
        .backdrop(Backdrop::Mica)
        .render(app)
}
