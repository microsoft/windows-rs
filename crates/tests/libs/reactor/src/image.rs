use super::*;

pub(super) const PNG: &[u8] = &[
    0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48, 0x44, 0x52,
    0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x06, 0x00, 0x00, 0x00, 0x1f, 0x15, 0xc4,
    0x89, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x44, 0x41, 0x54, 0x08, 0xd7, 0x63, 0xf8, 0xcf, 0xc0, 0xf0,
    0x1f, 0x00, 0x05, 0x00, 0x01, 0xff, 0x89, 0x99, 0x3d, 0x1d, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45,
    0x4e, 0x44, 0xae, 0x42, 0x60, 0x82,
];

const SVG: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 8 8"><circle cx="4" cy="4" r="3"/></svg>"#;

#[derive(Clone, Copy, PartialEq)]
pub(super) enum Source {
    Valid,
    Invalid,
    Svg,
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
    type Input = bool;
    type Message = Message;

    fn create(_: &Self::Input, _: &ComponentContext<Self>) -> Self {
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

    fn view(&self, transparent: &bool, context: &mut ViewContext<Self>) -> View {
        let name = if *transparent {
            "image-transparent"
        } else {
            "image"
        };
        context.window_title(format!("Reactor UI fixture: {name}"));
        context.window_visuals(WindowVisuals::new().client_size(480.0, 320.0));
        let image = Image::new()
            .width(32.0)
            .height(32.0)
            .opacity(if *transparent { 0.0 } else { 1.0 })
            .on_opened(context.message(Message::Opened))
            .on_failed(context.message(Message::Failed));
        let image = match self.source {
            Source::Valid => image.source_data(EncodedImage::from_static(PNG)),
            Source::Invalid => image.source_data(EncodedImage::from_static(&[0, 1, 2])),
            Source::Svg => image.source_svg(EncodedImage::from_static(SVG)),
        };
        let mut images: Vec<View> = vec![image.into()];
        // The second SVG image mounts with the update, so its load starts from `Loaded` while the
        // first image is already live.
        if self.source == Source::Svg {
            images.push(
                Image::new()
                    .width(64.0)
                    .height(64.0)
                    .on_opened(context.message(Message::Opened))
                    .on_failed(context.message(Message::Failed))
                    .source_svg(EncodedImage::from_static(SVG))
                    .into(),
            );
        }
        StackPanel::new()
            .margin(20.0)
            .spacing(12.0)
            .children((
                StackPanel::new()
                    .orientation(Orientation::Horizontal)
                    .spacing(12.0)
                    .children(images),
                Button::new()
                    .automation_id("invalid")
                    .content("Invalid image")
                    .on_click(context.message(Message::Source(Source::Invalid))),
                Button::new()
                    .automation_id("valid")
                    .content("Valid image")
                    .on_click(context.message(Message::Source(Source::Valid))),
                Button::new()
                    .automation_id("svg")
                    .content("SVG images")
                    .on_click(context.message(Message::Source(Source::Svg))),
                TextBlock::new()
                    .automation_id("events")
                    .text(format!("Opened {}; failed {}", self.opened, self.failed)),
            ))
            .into()
    }
}

pub(super) struct BitmapFixture {
    images: Vec<EncodedImage>,
    bytes: Vec<std::sync::Weak<[u8]>>,
    probes: u32,
    live_bytes: usize,
    shown: bool,
    large: bool,
    opened: u32,
}

#[derive(Clone, Copy)]
pub(super) enum BitmapMessage {
    Load,
    Release,
    Probe,
    Share,
    Resize,
    Show(bool),
    Opened,
}

impl Component for BitmapFixture {
    type Input = ();
    type Message = BitmapMessage;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            images: Vec::new(),
            bytes: Vec::new(),
            probes: 0,
            live_bytes: 0,
            shown: false,
            large: false,
            opened: 0,
        }
    }

    fn update(&mut self, message: BitmapMessage, _: &ComponentContext<Self>) {
        match message {
            BitmapMessage::Load => {
                self.images = (0..8)
                    .map(|_| {
                        let bytes: std::sync::Arc<[u8]> =
                            include_bytes!("large-image.png").as_slice().into();
                        self.bytes.push(std::sync::Arc::downgrade(&bytes));
                        EncodedImage::new(bytes)
                    })
                    .collect();
                self.shown = true;
            }
            BitmapMessage::Release => {
                self.shown = false;
                self.images.clear();
            }
            BitmapMessage::Probe => {
                self.bytes.retain(|bytes| bytes.strong_count() != 0);
                self.probes += 1;
                self.live_bytes = self.bytes.len();
            }
            BitmapMessage::Share => {
                let bytes = self.images[0].clone();
                self.images.fill(bytes);
            }
            BitmapMessage::Resize => self.large = true,
            BitmapMessage::Show(shown) => self.shown = shown,
            BitmapMessage::Opened => self.opened += 1,
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Reactor UI fixture: image-bitmap");
        context.window_visuals(WindowVisuals::new().client_size(680.0, 360.0));
        let images: Vec<View> = self
            .images
            .iter()
            .filter(|_| self.shown)
            .enumerate()
            .map(|(index, bytes)| {
                let size = if self.large && index == 0 {
                    192.0
                } else {
                    48.0
                };
                Image::new()
                    .automation_id(format!("bitmap-{index}"))
                    .width(size)
                    .height(size)
                    .source_data(bytes.clone())
                    .on_opened(context.message(BitmapMessage::Opened))
                    .on_failed(Callback::new(|()| panic!("bitmap failed to load")))
                    .into()
            })
            .collect();
        StackPanel::new()
            .margin(16.0)
            .spacing(8.0)
            .children((
                StackPanel::new()
                    .automation_id("images")
                    .orientation(Orientation::Horizontal)
                    .spacing(8.0)
                    .children(images),
                StackPanel::new()
                    .orientation(Orientation::Horizontal)
                    .spacing(8.0)
                    .children((
                        Button::new()
                            .automation_id("load")
                            .content("Load")
                            .on_click(context.message(BitmapMessage::Load)),
                        Button::new()
                            .automation_id("share")
                            .content("Share bytes")
                            .on_click(context.message(BitmapMessage::Share)),
                        Button::new()
                            .automation_id("resize")
                            .content("Enlarge first")
                            .on_click(context.message(BitmapMessage::Resize)),
                        Button::new()
                            .automation_id("remove")
                            .content("Remove")
                            .on_click(context.message(BitmapMessage::Show(false))),
                        Button::new()
                            .automation_id("remount")
                            .content("Remount")
                            .on_click(context.message(BitmapMessage::Show(true))),
                        Button::new()
                            .automation_id("release")
                            .content("Release")
                            .on_click(context.message(BitmapMessage::Release)),
                        Button::new()
                            .automation_id("probe")
                            .content("Probe")
                            .on_click(context.message(BitmapMessage::Probe)),
                    )),
                TextBlock::new()
                    .automation_id("events")
                    .text(format!("Opened {}", self.opened)),
                TextBlock::new()
                    .automation_id("sample")
                    .text(format!("Sample {}; bytes {}", self.probes, self.live_bytes)),
            ))
            .into()
    }
}
