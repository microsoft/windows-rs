use crate::controls::*;
use windows_reactor::*;

#[derive(Clone)]
pub enum Message {
    Open,
    Closed(ContentDialogResult),
    OpenThree,
    ClosedThree,
}

pub struct ContentDialogPage {
    open: bool,
    result: Option<ContentDialogResult>,
    open_three: bool,
}

impl Component for ContentDialogPage {
    type Message = Message;
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            open: false,
            result: None,
            open_three: false,
        }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::Open => self.open = true,
            Message::Closed(result) => {
                self.result = Some(result);
                self.open = false;
            }
            Message::OpenThree => self.open_three = true,
            Message::ClosedThree => self.open_three = false,
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        let result_text = match self.result {
            None => "(none)".to_string(),
            Some(ContentDialogResult::Primary) => "Primary (Yes)".to_string(),
            Some(ContentDialogResult::Secondary) => "Secondary".to_string(),
            Some(ContentDialogResult::None) => "Closed (No)".to_string(),
        };

        page_content(
            "ContentDialog",
            "A modal dialog box with content and actions.",
            [
                KeyedView::new(
                    "basic",
                    sample_card(
                        "Basic Confirmation Dialog",
                        StackPanel::new().spacing(8.0).children((
                            Button::new()
                                .on_click(context.message(Message::Open))
                                .content("Show Dialog"),
                            ContentDialog::new()
                                .title("Confirm Action")
                                .primary_button_text("Yes")
                                .close_button_text("No")
                                .is_open(self.open)
                                .on_closed(context.callback(Message::Closed))
                                .content("Are you sure you want to proceed?"),
                            TextBlock::new()
                                .text(format!("Last result: {result_text}"))
                                .opacity(0.6),
                        )),
                        r#"ContentDialog::new()
    .title("Confirm Action")
    .content("Are you sure you want to proceed?")
    .primary_button_text("Yes")
    .close_button_text("No")
    .is_open(open)
    .on_closed(context.callback(Message::Closed))"#,
                    ),
                ),
                KeyedView::new(
                    "three-button",
                    sample_card(
                        "Three-Button Dialog",
                        StackPanel::new().spacing(8.0).children((
                            Button::new()
                                .on_click(context.message(Message::OpenThree))
                                .content("Show Three-Button"),
                            ContentDialog::new()
                                .title("Save Changes?")
                                .primary_button_text("Save")
                                .secondary_button_text("Don't Save")
                                .close_button_text("Cancel")
                                .is_open(self.open_three)
                                .on_closed(context.callback(|_| Message::ClosedThree))
                                .content("You have unsaved changes. What would you like to do?"),
                        )),
                        r#"ContentDialog::new()
    .title("Save Changes?")
    .primary_button_text("Save")
    .secondary_button_text("Don't Save")
    .close_button_text("Cancel")
    .is_open(open)
    .on_closed(context.callback(Message::ClosedThree))"#,
                    ),
                ),
            ],
        )
    }
}
