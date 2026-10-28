use super::*;
use std::cell::RefCell;
use windows_canvas::{CanvasImageSource, ColorF, GpuDevice, Rect};

pub(super) struct CanvasImageFixture {
    image: ElementRef<Image>,
    surface: Rc<RefCell<Option<CanvasImageSource>>>,
    drawn: Option<bool>,
    attached: Option<bool>,
}

#[derive(Clone, Copy)]
pub(super) enum Message {
    Drawn(bool),
    Attached(bool),
}

impl Component for CanvasImageFixture {
    type Input = ();
    type Message = Message;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            image: ElementRef::new(),
            surface: Rc::new(RefCell::new(None)),
            drawn: None,
            attached: None,
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Drawn(drawn) => self.drawn = Some(drawn),
            Message::Attached(attached) => self.attached = Some(attached),
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Reactor UI fixture: canvas-image");
        context.window_visuals(WindowVisuals::new().client_size(480.0, 320.0));

        let image = self.image.clone();
        let surface = Rc::clone(&self.surface);
        let sender = context.sender();
        context.use_effect("draw", (), move || {
            let source =
                CanvasImageSource::new(&GpuDevice::new_warp().unwrap(), 64.0, 48.0, 1.5).unwrap();
            let attached = sender.clone();
            assert!(source.attach_result(&image, move |result| {
                _ = attached.send(Message::Attached(result.is_ok()));
            }));
            let drawn = source
                .draw(ColorF::CORNFLOWER_BLUE, |session| {
                    let brush = session.create_solid_brush(ColorF::WHITE)?;
                    session.fill_rect(&Rect::new(8.0, 8.0, 24.0, 24.0), &brush);
                    Ok(())
                })
                .unwrap();
            _ = sender.send(Message::Drawn(drawn));
            *surface.borrow_mut() = Some(source);
            None
        });

        StackPanel::new()
            .margin(20.0)
            .spacing(12.0)
            .children((
                Image::new()
                    .element_ref(&self.image)
                    .width(64.0)
                    .height(48.0),
                TextBlock::new().automation_id("status").text(format!(
                    "Drawn {:?}; attached {:?}",
                    self.drawn, self.attached
                )),
            ))
            .into()
    }
}
