#![windows_subsystem = "windows"]

use windows_canvas::*;
use windows_reactor::{
    Application, Button, CanvasInvalidator, Grid, GridChild, GridLength, HookRef, Orientation,
    RenderCx, StackPanel, Thickness, Window, WindowBackdrop, component, run_reactor_winui_app,
    swap_chain_canvas_invalidated,
};

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

fn app(cx: &mut RenderCx<'_>) -> windows_reactor::Element {
    let model = cx.use_ref(Model::new);
    let invalidator = cx.use_canvas_invalidator();

    let pressed_model = model.clone();
    let pressed_invalidator = invalidator.clone();
    let on_pressed = move |event: windows_reactor::PointerEvent| {
        let (x, y) = (event.x, event.y);
        pressed_model.with_mut(|model| {
            let hit = model.hit(x, y);

            if event.is_right_button_pressed {
                if let Some(index) = hit {
                    model.shapes.remove(index);
                    model.selected = None;
                    model.drag_offset = None;
                }
                return;
            }

            if !event.is_left_button_pressed {
                return;
            }

            if let Some(index) = hit {
                let shape = &model.shapes[index];
                model.selected = Some(index);
                model.drag_offset = Some((x - shape.x, y - shape.y));
            } else {
                let color = palette(model.next_color);
                model.next_color += 1;
                model.shapes.push(Shape::new(model.kind, x, y, color));
                model.selected = Some(model.shapes.len() - 1);
                model.drag_offset = Some((0.0, 0.0));
            }
        });
        pressed_invalidator.invalidate();
    };

    let moved_model = model.clone();
    let moved_invalidator = invalidator.clone();
    let on_moved = move |event: windows_reactor::PointerEvent| {
        if !event.is_left_button_pressed {
            moved_model.with_mut(|model| model.drag_offset = None);
            return;
        }
        let moved = moved_model
            .with_mut(|model| {
                if let (Some(index), Some((offset_x, offset_y))) =
                    (model.selected, model.drag_offset)
                    && let Some(shape) = model.shapes.get_mut(index)
                {
                    shape.x = event.x - offset_x;
                    shape.y = event.y - offset_y;
                    true
                } else {
                    false
                }
            })
            .unwrap_or(false);
        if moved {
            moved_invalidator.invalidate();
        }
    };

    let released_model = model.clone();
    let on_released = move |_: windows_reactor::PointerEvent| {
        released_model.with_mut(|model| model.drag_offset = None);
    };
    let lost_model = model.clone();
    let on_capture_lost = move |_: windows_reactor::PointerEvent| {
        lost_model.with_mut(|model| model.drag_offset = None);
    };
    let canceled_model = model.clone();
    let on_canceled = move |_: windows_reactor::PointerEvent| {
        canceled_model.with_mut(|model| model.drag_offset = None);
    };

    let margin = 16.0;
    let draw_model = model.clone();
    let drawing_surface =
        swap_chain_canvas_invalidated(&invalidator, move |ctx| draw(ctx, &draw_model))
            .on_pointer_pressed(on_pressed)
            .on_pointer_moved(on_moved)
            .on_pointer_released(on_released)
            .on_pointer_capture_lost(on_capture_lost)
            .on_pointer_canceled(on_canceled)
            .capture_pointer_on_press()
            .margin(Thickness {
                left: margin,
                top: margin,
                right: margin,
                bottom: 0.0,
            })
            .build();

    Grid::new([
        GridChild::new(drawing_surface).row(0),
        GridChild::new(
            StackPanel::new([
                tool_button(&model, &invalidator, Kind::Rectangle),
                tool_button(&model, &invalidator, Kind::Triangle),
                tool_button(&model, &invalidator, Kind::Star),
                Button::new("Clear")
                    .on_click({
                        let model = model.clone();
                        let invalidator = invalidator.clone();
                        move || {
                            model.with_mut(|model| {
                                model.shapes.clear();
                                model.selected = None;
                                model.drag_offset = None;
                            });
                            invalidator.invalidate();
                        }
                    })
                    .build(),
            ])
            .orientation(Orientation::Horizontal)
            .spacing(8.0)
            .margin(Thickness::uniform(margin))
            .build(),
        )
        .row(1),
    ])
    .rows([GridLength::STAR, GridLength::Auto])
    .build()
}

fn tool_button(
    model: &HookRef<Model>,
    invalidator: &CanvasInvalidator,
    kind: Kind,
) -> windows_reactor::Element {
    let model = model.clone();
    let invalidator = invalidator.clone();
    Button::new(kind.label())
        .on_click(move || {
            model.with_mut(|model| model.kind = kind);
            invalidator.invalidate();
        })
        .build()
}

fn draw(ctx: &windows_reactor::CanvasDrawContext<'_>, model: &HookRef<Model>) -> Result<()> {
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
    model
        .with_mut(|model| {
            let selected = model.selected;

            for (index, shape) in model.shapes.iter_mut().enumerate() {
                if device_changed || shape.built_at != Some((shape.x, shape.y)) {
                    shape.path = build_path(ctx.device(), shape.kind, shape.x, shape.y).ok();
                    shape.built_at = Some((shape.x, shape.y));
                }
                let Some(path) = &shape.path else {
                    continue;
                };

                let brush = ctx.create_solid_brush(shape.color)?;
                ctx.fill_path(path, &brush);

                if Some(index) == selected {
                    let brush = ctx.create_solid_brush(ColorF::WHITE)?;
                    let bounds = path.compute_bounds();
                    let pad = 4.0;
                    ctx.draw_rect(
                        &Rect::new(
                            bounds.left - pad,
                            bounds.top - pad,
                            bounds.right + pad,
                            bounds.bottom + pad,
                        ),
                        &brush,
                        1.5,
                    );
                }
            }

            let format = TextFormat::with_weight("Segoe UI", 16.0, FontWeight::BOLD)?;
            let brush = ctx.create_solid_brush(ColorF::WHITE)?;
            let label = format!(
                "{} shape(s) - tool: {} - click to add, left-drag to move, right-click to delete",
                model.shapes.len(),
                model.kind.label()
            );
            let rect = Rect::new(12.0, ctx.height - 30.0, ctx.width, ctx.height);
            ctx.draw_text(&label, &format, &rect, &brush);
            Ok(())
        })
        .unwrap()
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
    let root = component(|cx| {
        let open = cx.use_state(|| true);
        let windows = if open.value() {
            vec![
                Window::new("Canvas editor", component(app), move || {
                    open.set(false);
                })
                .backdrop(WindowBackdrop::Mica)
                .build()
                .key(0),
            ]
        } else {
            Vec::new()
        };
        Application::new(windows).build()
    });
    run_reactor_winui_app(root)
}
