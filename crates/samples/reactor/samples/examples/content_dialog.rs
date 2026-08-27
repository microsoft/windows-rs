#![windows_subsystem = "windows"]

use windows_reactor::*;

#[derive(Clone)]
enum Message {
    Open,
    Closed(ContentDialogResult),
}

struct ContentDialogSample {
    open: bool,
    result: Option<ContentDialogResult>,
}

impl Component for ContentDialogSample {
    type Message = Message;
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            open: false,
            result: None,
        }
    }

    fn update(&mut self, message: Self::Message, _context: &ComponentContext<Self>) {
        match message {
            Message::Open => self.open = true,
            Message::Closed(result) => {
                self.result = Some(result);
                self.open = false;
            }
        }
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("ContentDialog");
        let label = match self.result {
            None => "No choice yet.".to_string(),
            Some(ContentDialogResult::Primary) => "You picked: Delete".to_string(),
            Some(ContentDialogResult::Secondary) => "You picked: Archive".to_string(),
            Some(ContentDialogResult::None) => "You picked: Cancel".to_string(),
        };

        StackPanel::new().spacing(8.0).max_width(320.0).children((
            TextBlock::new().text(label),
            Button::new()
                .on_click(context.message(Message::Open))
                .content(TextBlock::new().text("Open dialog")),
            ContentDialog::new()
                .title("Delete this item?")
                .primary_button_text("Delete")
                .secondary_button_text("Archive")
                .close_button_text("Cancel")
                .is_primary_button_enabled(true)
                .is_secondary_button_enabled(true)
                .is_open(self.open)
                .on_closed(context.callback(Message::Closed))
                .content(TextBlock::new().text("This action cannot be undone.")),
        ))
    }
}

fn main() {
    App::run_component::<ContentDialogSample>(()).unwrap();
}
