#![windows_subsystem = "windows"]

use windows_reactor::*;

#[derive(Clone, Copy)]
enum Message {
    Show(ContentDialogButton, bool),
    Closed(ContentDialogResult),
}

struct DialogSample {
    open: bool,
    default: ContentDialogButton,
    editor: bool,
    result: String,
}

impl Component for DialogSample {
    type Input = ();
    type Message = Message;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            open: false,
            default: ContentDialogButton::None,
            editor: false,
            result: "No result yet".into(),
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Show(default, editor) => {
                self.default = default;
                self.editor = editor;
                self.open = true;
            }
            Message::Closed(result) => {
                self.open = false;
                self.result = format!("Result: {result:?}");
            }
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("ContentDialog default button");
        context.window_visuals(WindowVisuals::new().client_size(480.0, 400.0));
        let show = |label, default, editor| {
            Button::new()
                .content(label)
                .on_click(context.message(Message::Show(default, editor)))
        };
        let content: View = if self.editor {
            TextBox::new()
                .accepts_return(true)
                .placeholder_text("Enter adds a line here. Tab moves to the dialog buttons.")
                .into()
        } else {
            TextBlock::new()
                .text("Press Enter or choose a button.")
                .into()
        };
        Border::new()
            .padding(Thickness::uniform(16.0))
            .content(StackPanel::new().spacing(8.0).children((
                show("Primary default", ContentDialogButton::Primary, false),
                show("Secondary default", ContentDialogButton::Secondary, false),
                show("Close default", ContentDialogButton::Close, false),
                show("No default", ContentDialogButton::None, false),
                show(
                    "Editor with primary default",
                    ContentDialogButton::Primary,
                    true,
                ),
                self.result.clone(),
            )))
            .content_dialog(
                ContentDialog::new()
                    .title("Save changes?")
                    .primary_button_text("Save")
                    .secondary_button_text("Discard")
                    .close_button_text("Cancel")
                    .default_button(self.default)
                    .content(content)
                    .is_open(self.open)
                    .on_closed(context.callback(Message::Closed)),
            )
    }
}

fn main() {
    App::run_component::<DialogSample>(()).unwrap();
}
