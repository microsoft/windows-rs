use super::*;
use std::cell::Cell;
use windows_canvas::{ColorF, GpuDevice, animated_canvas_with_device};

pub(super) struct CanvasFixture {
    surface: Option<View>,
    generation: u32,
    resized: bool,
    frame: String,
    dropped: Vec<u32>,
}

#[derive(Clone)]
pub(super) enum Message {
    Resize,
    Remove,
    Attach,
    Frame(String),
    Dropped(u32),
}

struct DrawLifetime {
    generation: u32,
    dropped: Callback<u32>,
}

impl Drop for DrawLifetime {
    fn drop(&mut self) {
        self.dropped.call(self.generation);
    }
}

fn surface(generation: u32, context: &ComponentContext<CanvasFixture>) -> View {
    let lifetime = DrawLifetime {
        generation,
        dropped: context.callback(Message::Dropped),
    };
    let observed = Cell::new(None);
    let drawn = context.callback(Message::Frame);
    animated_canvas_with_device(GpuDevice::new_warp().unwrap(), move |frame| {
        let _keep_alive = &lifetime;
        frame.clear(ColorF::CORNFLOWER_BLUE);
        let size = (frame.width, frame.height);
        if observed.replace(Some(size)) != Some(size) {
            drawn.call(format!(
                "Canvas {generation}: {:.0} x {:.0}; changed {}",
                frame.width,
                frame.height,
                frame.device_changed()
            ));
        }
        Ok(())
    })
}

impl Component for CanvasFixture {
    type Input = ();
    type Message = Message;

    fn create(_: &(), context: &ComponentContext<Self>) -> Self {
        Self {
            surface: Some(surface(0, context)),
            generation: 0,
            resized: false,
            frame: String::new(),
            dropped: Vec::new(),
        }
    }

    fn update(&mut self, message: Message, context: &ComponentContext<Self>) {
        match message {
            Message::Resize => self.resized = true,
            Message::Remove => self.surface = None,
            Message::Attach => {
                self.generation += 1;
                self.resized = false;
                self.surface = Some(surface(self.generation, context));
            }
            Message::Frame(frame) => self.frame = frame,
            Message::Dropped(generation) => self.dropped.push(generation),
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        let surface = self.surface.as_ref().map(|surface| {
            keyed(
                self.generation,
                Border::new()
                    .width(if self.resized { 320.0 } else { 240.0 })
                    .height(if self.resized { 200.0 } else { 160.0 })
                    .content(surface.clone()),
            )
        });
        context.window_title("Reactor UI fixture: canvas");
        context.window_visuals(WindowVisuals::new().client_size(600.0, 500.0));
        StackPanel::new()
            .margin(20.0)
            .spacing(8.0)
            .children((
                Button::new()
                    .automation_id("resize")
                    .content("Resize canvas")
                    .on_click(context.message(Message::Resize)),
                Button::new()
                    .automation_id("remove")
                    .content("Remove canvas")
                    .on_click(context.message(Message::Remove)),
                Button::new()
                    .automation_id("attach")
                    .content("Attach canvas")
                    .on_click(context.message(Message::Attach)),
                StackPanel::new().keyed_children(surface),
                TextBlock::new().automation_id("frame").text(&self.frame),
                TextBlock::new()
                    .automation_id("dropped")
                    .text(format!("Dropped {:?}", self.dropped)),
            ))
            .into()
    }
}
