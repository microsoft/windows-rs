#![windows_subsystem = "windows"]

use std::cell::Cell;
use std::rc::Rc;
use windows_canvas::*;
use windows_canvas::{Ellipse as CanvasEllipse, Rect as CanvasRect};
use windows_reactor::*;

struct Sample {
    count: u32,
    frame: Rc<Cell<u64>>,
}

#[derive(Clone)]
enum Message {
    Add,
    Remove,
}

impl Component for Sample {
    type Input = ();
    type Message = Message;

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            count: 5,
            frame: Rc::new(Cell::new(0)),
        }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::Add => self.count += 1,
            Message::Remove => self.count = self.count.saturating_sub(1),
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Canvas + Reactor");
        context.window_visuals(WindowVisuals::new().backdrop(WindowBackdrop::Mica));
        let count = self.count;
        let frame = Rc::clone(&self.frame);
        let surface = animated_canvas(move |ctx| {
            let current = frame.get() + 1;
            frame.set(current);
            let time = current as f32 * 0.02;
            let center_x = ctx.width / 2.0;
            let center_y = ctx.height / 2.0;
            let orbit = center_x.min(center_y) * 0.5;
            ctx.clear(ColorF::TRANSPARENT);
            let brush = ctx.create_solid_brush(ColorF::CORNFLOWER_BLUE)?;
            for index in 0..count {
                let phase = time + index as f32 * 1.2;
                let x = center_x + phase.cos() * orbit;
                let y = center_y + phase.sin() * (orbit * 0.7);
                let radius = 20.0 + (phase * 0.7).sin().abs() * 30.0;
                brush.set_color(ColorF::new(
                    0.3 + (phase * 0.3).sin().abs() * 0.7,
                    0.4 + (phase * 0.5).cos().abs() * 0.5,
                    0.8,
                    0.85,
                ));
                ctx.fill_ellipse(&CanvasEllipse::circle(Vector2::new(x, y), radius), &brush);
            }
            let format = TextFormat::with_weight("Segoe UI", 20.0, CanvasFontWeight::BOLD)?
                .with_alignment(TextAlignment::Center);
            brush.set_color(ColorF::WHITE);
            ctx.draw_text(
                &format!("{count} circles"),
                &format,
                &CanvasRect::new(0.0, ctx.height - 40.0, ctx.width, ctx.height),
                &brush,
            );
            Ok(())
        });
        Grid::new()
            .rows([GridLength::STAR, GridLength::Auto])
            .children((
                Border::new()
                    .margin(Thickness::new(16.0, 16.0, 16.0, 0.0))
                    .grid_row(0)
                    .content(surface),
                StackPanel::new()
                    .orientation(Orientation::Horizontal)
                    .spacing(8.0)
                    .margin(Thickness::uniform(16.0))
                    .grid_row(1)
                    .children((
                        Button::new()
                            .on_click(context.message(Message::Add))
                            .content("Add circle"),
                        Button::new()
                            .on_click(context.message(Message::Remove))
                            .content("Remove circle"),
                    )),
            ))
    }
}

fn main() -> Result<()> {
    App::run_component::<Sample>(())
}
