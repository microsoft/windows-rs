use super::*;

pub(super) struct AppearanceFixture {
    style: u8,
    hit_test: Option<bool>,
    entered: bool,
    presses: u32,
    text: Rc<str>,
    sizing_text: Rc<str>,
}

#[derive(Clone)]
pub(super) enum AppearanceMessage {
    Style(u8),
    HitTest(Option<bool>),
    Entered,
    Pressed,
    Text(Rc<str>),
    SizingText(Rc<str>),
}

impl Component for AppearanceFixture {
    type Input = ();
    type Message = AppearanceMessage;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            style: 0,
            hit_test: Some(false),
            entered: false,
            presses: 0,
            text: Rc::from(""),
            sizing_text: Rc::from("MMMMMMMMMMMM"),
        }
    }

    fn update(&mut self, message: AppearanceMessage, _: &ComponentContext<Self>) {
        match message {
            AppearanceMessage::Style(style) => self.style = style,
            AppearanceMessage::HitTest(value) => self.hit_test = value,
            AppearanceMessage::Entered => self.entered = true,
            AppearanceMessage::Pressed => self.presses += 1,
            AppearanceMessage::Text(text) => self.text = text,
            AppearanceMessage::SizingText(text) => self.sizing_text = text,
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Reactor UI fixture: text-appearance");
        context.window_visuals(
            WindowVisuals::new()
                .client_size(720.0, 500.0)
                .theme(WindowTheme::Light),
        );
        let action = |id: &str, message| {
            Button::new()
                .automation_id(id)
                .content(id)
                .on_click(context.message(message))
        };
        let mut sizing = TextBox::new()
            .automation_id("sizing")
            .text(self.sizing_text.clone())
            .on_text_changed(context.callback(AppearanceMessage::SizingText))
            .background(Color::rgb(255, 0, 0))
            .border_brush(Color::rgb(255, 0, 0))
            .border_thickness(0.0);
        sizing = match self.style {
            1 => sizing
                .padding(Thickness::new(40.0, 20.0, 40.0, 20.0))
                .corner_radius(CornerRadius::uniform(24.0)),
            2 => sizing
                .padding(Thickness::new(12.0, 8.0, 12.0, 8.0))
                .corner_radius(CornerRadius::uniform(0.0)),
            _ => sizing,
        };
        let mut decoration = Border::new()
            .width(24.0)
            .height(24.0)
            .margin(Thickness::new(16.0, 0.0, 0.0, 0.0))
            .horizontal_alignment(HorizontalAlignment::Left)
            .vertical_alignment(VerticalAlignment::Center)
            .background(Color::rgb(255, 255, 255))
            .on_pointer_pressed(context.callback(|_| AppearanceMessage::Pressed))
            .content(SymbolIcon::new().symbol(Symbol::Find));
        if let Some(value) = self.hit_test {
            decoration = decoration.is_hit_test_visible(value);
        }
        let content = Border::new()
            .background(Color::rgb(255, 255, 255))
            .padding(20.0)
            .content(
                StackPanel::new().spacing(16.0).children((
                    action("anchor", AppearanceMessage::Style(self.style)),
                    StackPanel::new()
                        .orientation(Orientation::Horizontal)
                        .children((sizing,)),
                    Border::new()
                        .horizontal_alignment(HorizontalAlignment::Left)
                        .on_pointer_entered(context.callback(|_| AppearanceMessage::Entered))
                        .content(
                            Grid::new().width(320.0).height(64.0).children((
                                TextBox::new()
                                    .automation_id("input")
                                    .automation_name("Search")
                                    .placeholder_text("Search")
                                    .padding(Thickness::new(48.0, 8.0, 12.0, 8.0))
                                    .corner_radius(CornerRadius::uniform(12.0))
                                    .text(self.text.clone())
                                    .on_text_changed(context.callback(AppearanceMessage::Text)),
                                decoration,
                            )),
                        ),
                    StackPanel::new()
                        .orientation(Orientation::Horizontal)
                        .spacing(8.0)
                        .children((
                            action("apply", AppearanceMessage::Style(1)),
                            action("change", AppearanceMessage::Style(2)),
                            action("reset", AppearanceMessage::Style(0)),
                            action("block", AppearanceMessage::HitTest(Some(true))),
                            action("pass", AppearanceMessage::HitTest(Some(false))),
                            action("omit-hit", AppearanceMessage::HitTest(None)),
                        )),
                    TextBlock::new()
                        .automation_id("entered")
                        .text(self.entered.to_string()),
                    TextBlock::new()
                        .automation_id("presses")
                        .text(self.presses.to_string()),
                    TextBlock::new()
                        .automation_id("echo")
                        .text(self.text.clone()),
                    TextBlock::new()
                        .automation_id("style")
                        .text(self.style.to_string()),
                )),
            );
        Grid::new()
            .key_accelerators(KeyAccelerators::new([KeyAccelerator::new(
                AcceleratorKey::N,
                AcceleratorModifiers::Control,
                context.message(AppearanceMessage::Style((self.style + 1) % 3)),
            )]))
            .children((content,))
            .into()
    }
}

pub(super) struct MultilineFixture {
    multiline: Option<bool>,
    calls: u32,
}

#[derive(Clone)]
pub(super) enum MultilineMessage {
    Toggle,
    OmitMode,
    Changed,
}

impl Component for MultilineFixture {
    type Input = ();
    type Message = MultilineMessage;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            multiline: Some(false),
            calls: 0,
        }
    }

    fn update(&mut self, message: MultilineMessage, _: &ComponentContext<Self>) {
        match message {
            MultilineMessage::Toggle => {
                self.multiline = Some(!self.multiline.unwrap_or(false));
            }
            MultilineMessage::OmitMode => self.multiline = None,
            MultilineMessage::Changed => self.calls += 1,
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        let changed = context.callback(|_: Rc<str>| MultilineMessage::Changed);
        let mut changing = TextBox::new()
            .automation_id("changing")
            .text(if self.multiline == Some(true) {
                "First\rSecond"
            } else {
                "First"
            })
            .on_text_changed(changed.clone());
        let mut retained = TextBox::new()
            .automation_id("retained")
            .text("First\rSecond")
            .on_text_changed(changed.clone());
        if let Some(multiline) = self.multiline {
            changing = changing.accepts_return(multiline);
            retained = retained.accepts_return(multiline);
        }
        context.window_frame(
            "Reactor UI fixture: text-multiline",
            StackPanel::new().margin(20.0).spacing(12.0).children((
                TextBox::new()
                    .automation_id("initial")
                    .text("First\rSecond")
                    .accepts_return(true)
                    .on_text_changed(changed),
                changing,
                retained,
                Button::new()
                    .automation_id("toggle")
                    .content("Toggle multiline")
                    .on_click(context.message(MultilineMessage::Toggle)),
                Button::new()
                    .automation_id("omit-mode")
                    .content("Omit multiline mode")
                    .on_click(context.message(MultilineMessage::OmitMode)),
                TextBlock::new()
                    .automation_id("calls")
                    .text(format!("Calls {}", self.calls)),
            )),
        )
    }
}

#[derive(Clone, Copy, PartialEq)]
pub(super) enum TextControl {
    Text,
    Suggest,
    Rich,
}

/// Uppercases every edit so the native control receives a different value than the user typed.
/// The header is editable so the caret tracking must find the input's editor, not the header's.
pub(super) struct TransformFixture {
    text: Rc<str>,
    calls: u32,
    header: bool,
}

#[derive(Clone)]
pub(super) enum TransformMessage {
    Changed(Rc<str>),
    ToggleHeader,
}

impl Component for TransformFixture {
    type Input = TextControl;
    type Message = TransformMessage;

    fn create(_: &TextControl, _: &ComponentContext<Self>) -> Self {
        Self {
            text: Rc::from(""),
            calls: 0,
            header: true,
        }
    }

    fn update(&mut self, message: TransformMessage, _: &ComponentContext<Self>) {
        match message {
            TransformMessage::Changed(text) => {
                self.text = Rc::from(text.to_uppercase());
                self.calls += 1;
            }
            TransformMessage::ToggleHeader => self.header = !self.header,
        }
    }

    fn view(&self, control: &TextControl, context: &mut ViewContext<Self>) -> View {
        let header = self
            .header
            .then(|| TextBox::new().automation_id("header").text("Header"));
        let input: View = match control {
            TextControl::Text => {
                let mut input = TextBox::new()
                    .automation_id("input")
                    .text(self.text.clone())
                    .on_text_changed(context.callback(TransformMessage::Changed));
                if let Some(header) = header {
                    input = input.header(header);
                }
                input.into()
            }
            TextControl::Suggest => {
                let mut input = AutoSuggestBox::new()
                    .automation_id("input")
                    .text(self.text.clone())
                    .on_text_changed(context.callback(TransformMessage::Changed));
                if let Some(header) = header {
                    input = input.header(header);
                }
                input.into()
            }
            TextControl::Rich => {
                let mut input = RichEditBox::new()
                    .automation_id("input")
                    .text(self.text.clone())
                    .on_text_changed(context.callback(TransformMessage::Changed));
                if let Some(header) = header {
                    input = input.header(header);
                }
                input.into()
            }
        };
        let title = match control {
            TextControl::Text => "Reactor UI fixture: text-transform",
            TextControl::Suggest => "Reactor UI fixture: suggest-transform",
            TextControl::Rich => "Reactor UI fixture: rich-transform",
        };
        context.window_frame(
            title,
            StackPanel::new().margin(20.0).spacing(12.0).children((
                input,
                TextBlock::new()
                    .automation_id("echo")
                    .text(format!("Text {}; calls {}", self.text, self.calls)),
                Button::new()
                    .automation_id("toggle-header")
                    .content("Toggle header")
                    .on_click(context.message(TransformMessage::ToggleHeader)),
            )),
        )
    }
}
