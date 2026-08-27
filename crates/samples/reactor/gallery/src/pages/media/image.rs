use crate::controls::*;
use windows_reactor::*;

pub struct ImagePage {
    stretch: Stretch,
}

#[derive(Clone)]
pub enum Message {
    SetStretch(Stretch),
}

impl Component for ImagePage {
    type Message = Message;
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            stretch: Stretch::Uniform,
        }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::SetStretch(stretch) => self.stretch = stretch,
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        let stretch_name = match self.stretch {
            Stretch::Uniform => "Uniform",
            Stretch::UniformToFill => "UniformToFill",
            Stretch::Fill => "Fill",
            Stretch::None => "None",
        };

        page_content(
            "Image",
            "Displays an image from a file or URI.",
            [
                KeyedView::new(
                    "stretch-modes",
                    sample_card(
                        "Stretch Modes",
                        StackPanel::new().spacing(8.0).children((
                            Border::new()
                                .border_brush(Color::rgb(200, 200, 200))
                                .border_thickness(1.0)
                                .content(
                                    Image::new()
                                        .source_file(asset_path("Image.png"))
                                        .unwrap()
                                        .stretch(self.stretch)
                                        .width(300.0)
                                        .height(150.0),
                                ),
                            StackPanel::new()
                                .orientation(Orientation::Horizontal)
                                .spacing(4.0)
                                .children((
                                    Button::new()
                                        .on_click(
                                            context.message(Message::SetStretch(Stretch::Uniform)),
                                        )
                                        .content(TextBlock::new().text("Uniform")),
                                    Button::new()
                                        .on_click(
                                            context.message(Message::SetStretch(
                                                Stretch::UniformToFill,
                                            )),
                                        )
                                        .content(TextBlock::new().text("UniformToFill")),
                                    Button::new()
                                        .on_click(
                                            context.message(Message::SetStretch(Stretch::Fill)),
                                        )
                                        .content(TextBlock::new().text("Fill")),
                                    Button::new()
                                        .on_click(
                                            context.message(Message::SetStretch(Stretch::None)),
                                        )
                                        .content(TextBlock::new().text("None")),
                                )),
                            TextBlock::new()
                                .text(format!("Current: {stretch_name}"))
                                .opacity(0.6),
                        )),
                        r#"Image::new().source_file(path).unwrap().stretch(Stretch::Uniform).width(300.0).height(150.0)"#,
                    ),
                ),
                KeyedView::new(
                    "fixed-size",
                    sample_card(
                        "Fixed Dimensions",
                        Image::new()
                            .source_file(asset_path("Image.png"))
                            .unwrap()
                            .width(64.0)
                            .height(64.0),
                        r#"Image::new().source_file(path).unwrap().width(64.0).height(64.0)"#,
                    ),
                ),
            ],
        )
    }
}
