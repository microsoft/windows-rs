use super::*;

const PNG: &[u8] = &[
    0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48, 0x44, 0x52,
    0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x06, 0x00, 0x00, 0x00, 0x1f, 0x15, 0xc4,
    0x89, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x44, 0x41, 0x54, 0x08, 0xd7, 0x63, 0xf8, 0xcf, 0xc0, 0xf0,
    0x1f, 0x00, 0x05, 0x00, 0x01, 0xff, 0x89, 0x99, 0x3d, 0x1d, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45,
    0x4e, 0x44, 0xae, 0x42, 0x60, 0x82,
];

#[derive(Clone, Copy)]
pub(super) enum Source {
    Valid,
    Invalid,
}

pub(super) struct ImageFixture {
    source: Source,
    opened: u32,
    failed: u32,
}

#[derive(Clone, Copy)]
pub(super) enum Message {
    Source(Source),
    Opened,
    Failed,
}

impl Component for ImageFixture {
    type Input = ();
    type Message = Message;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            source: Source::Valid,
            opened: 0,
            failed: 0,
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Source(source) => self.source = source,
            Message::Opened => self.opened += 1,
            Message::Failed => self.failed += 1,
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Reactor UI fixture: image");
        context.window_visuals(WindowVisuals::new().client_size(480.0, 320.0));
        let image = Image::new()
            .width(32.0)
            .height(32.0)
            .on_opened(context.message(Message::Opened))
            .on_failed(context.message(Message::Failed));
        let data = match self.source {
            Source::Valid => PNG,
            Source::Invalid => &[0, 1, 2],
        };
        let image = image.source_data(EncodedImage::from_static(data));
        StackPanel::new()
            .margin(20.0)
            .spacing(12.0)
            .children((
                image,
                Button::new()
                    .automation_id("invalid")
                    .content("Invalid image")
                    .on_click(context.message(Message::Source(Source::Invalid))),
                Button::new()
                    .automation_id("valid")
                    .content("Valid image")
                    .on_click(context.message(Message::Source(Source::Valid))),
                TextBlock::new()
                    .automation_id("events")
                    .text(format!("Opened {}; failed {}", self.opened, self.failed)),
            ))
            .into()
    }
}
