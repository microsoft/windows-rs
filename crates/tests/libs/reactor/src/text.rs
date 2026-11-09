use super::*;

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
