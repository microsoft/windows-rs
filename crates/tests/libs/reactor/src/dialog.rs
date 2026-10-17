use super::*;

pub(super) struct DialogFixture {
    open: bool,
    closed: u32,
    result: ContentDialogResult,
}

#[derive(Clone, Copy)]
pub(super) enum Message {
    Open,
    Closed(ContentDialogResult),
}

impl Component for DialogFixture {
    type Input = ();
    type Message = Message;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            open: true,
            closed: 0,
            result: ContentDialogResult::None,
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Open => self.open = true,
            Message::Closed(result) => {
                self.result = result;
                self.open = false;
                self.closed += 1;
            }
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Reactor UI fixture: dialog");
        context.window_visuals(WindowVisuals::new().client_size(500.0, 400.0));
        StackPanel::new()
            .margin(20.0)
            .spacing(12.0)
            .children((
                Button::new()
                    .automation_id("open")
                    .content("Open dialog")
                    .on_click(context.message(Message::Open))
                    .content_dialog(
                        ContentDialog::new()
                            .title("Lifecycle")
                            .primary_button_text("Accept")
                            .close_button_text("Close")
                            .is_open(self.open)
                            .on_closed(context.callback(Message::Closed))
                            .content(
                                TextBlock::new()
                                    .automation_id("dialog-content")
                                    .text("Requested at mount"),
                            ),
                    ),
                TextBlock::new()
                    .automation_id("closed")
                    .text(format!("Closed {}; result {:?}", self.closed, self.result)),
            ))
            .into()
    }
}
