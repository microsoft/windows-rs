#![windows_subsystem = "windows"]

use std::cell::RefCell;
use std::rc::Rc;
use windows_canvas::FontWeight as CanvasFontWeight;
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

struct Sample {
    model: Rc<RefCell<Model>>,
    invalidator: Invalidator,
}

#[derive(Clone)]
enum Message {
    Pressed(PointerEventInfo),
    Moved(PointerEventInfo),
    Released,
    Select(Kind),
    Clear,
}

impl Component for Sample {
    type Input = ();
    type Message = Message;

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            model: Rc::new(RefCell::new(Model::new())),
            invalidator: Invalidator::new(),
        }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        let mut model = self.model.borrow_mut();
        match message {
            Message::Pressed(info) => {
                let (x, y) = (info.x as f32, info.y as f32);
                let hit = model.hit(x, y);
                if info.is_right_button_pressed {
                    if let Some(index) = hit {
                        model.shapes.remove(index);
                        model.selected = None;
                        model.drag_offset = None;
                    }
                } else if let Some(index) = hit {
                    let shape = &model.shapes[index];
                    let (shape_x, shape_y) = (shape.x, shape.y);
                    model.selected = Some(index);
                    model.drag_offset = Some((x - shape_x, y - shape_y));
                } else {
                    let kind = model.kind;
                    let color = palette(model.next_color);
                    model.next_color += 1;
                    model.shapes.push(Shape::new(kind, x, y, color));
                    model.selected = Some(model.shapes.len() - 1);
                    model.drag_offset = Some((0.0, 0.0));
                }
            }
            Message::Moved(info) if info.is_left_button_pressed => {
                if let (Some(index), Some((offset_x, offset_y))) =
                    (model.selected, model.drag_offset)
                    && let Some(shape) = model.shapes.get_mut(index)
                {
                    shape.x = info.x as f32 - offset_x;
                    shape.y = info.y as f32 - offset_y;
                }
            }
            Message::Moved(_) => {}
            Message::Released => model.drag_offset = None,
            Message::Select(kind) => model.kind = kind,
            Message::Clear => {
                model.shapes.clear();
                model.selected = None;
                model.drag_offset = None;
            }
        }
        drop(model);
        self.invalidator.invalidate();
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Canvas editor");
        context.window_visuals(WindowVisuals::new().backdrop(WindowBackdrop::Mica));
        let model = Rc::clone(&self.model);
        let surface = Border::new()
            .on_pointer_pressed(context.callback(Message::Pressed))
            .on_pointer_moved(context.callback(Message::Moved))
            .on_pointer_released(context.callback(|_| Message::Released))
            .margin(Thickness::new(16.0, 16.0, 16.0, 0.0))
            .grid_row(0)
            .content(canvas_invalidated(&self.invalidator, move |ctx| {
                draw(ctx, &model)
            }));
        let tool = |kind| {
            Button::new()
                .on_click(context.message(Message::Select(kind)))
                .content(kind.label())
        };
        Grid::new()
            .rows([GridLength::STAR, GridLength::Auto])
            .children((
                surface,
                StackPanel::new()
                    .orientation(Orientation::Horizontal)
                    .spacing(8.0)
                    .margin(Thickness::uniform(16.0))
                    .grid_row(1)
                    .children((
                        tool(Kind::Rectangle),
                        tool(Kind::Triangle),
                        tool(Kind::Star),
                        Button::new()
                            .on_click(context.message(Message::Clear))
                            .content("Clear"),
                    )),
            ))
    }
}

fn draw(ctx: &DrawContext<'_>, model: &RefCell<Model>) -> Result<()> {
    ctx.clear(ColorF::new(0.11, 0.12, 0.16, 1.0));

    let grid_brush = ctx.create_solid_brush(ColorF::new(1.0, 1.0, 1.0, 0.06))?;
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

        let brush = ctx.create_solid_brush(s.color)?;
        ctx.fill_path(path, &brush);

        if Some(i) == selected {
            let brush = ctx.create_solid_brush(ColorF::WHITE)?;
            let b = path.compute_bounds();
            let pad = 4.0;
            ctx.draw_rect(
                &Rect::new(b.left - pad, b.top - pad, b.right + pad, b.bottom + pad),
                &brush,
                1.5,
            );
        }
    }

    let format = TextFormat::with_weight("Segoe UI", 16.0, CanvasFontWeight::BOLD)?;
    let brush = ctx.create_solid_brush(ColorF::WHITE)?;
    let label = format!(
        "{} shape(s)  ·  tool: {}  ·  click to add, left-drag to move, right-click to delete",
        m.shapes.len(),
        m.kind.label()
    );
    let rect = Rect::new(12.0, ctx.height - 30.0, ctx.width, ctx.height);
    ctx.draw_text(&label, &format, &rect, &brush);
    Ok(())
}

const SIZE: f32 = 38.0;

fn build_path(device: &GpuDevice, kind: Kind, x: f32, y: f32) -> Result<Path> {
    PathBuilder::new(device)?.polygon(polygon(kind, x, y))
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
    App::run_component::<Sample>(())
}
