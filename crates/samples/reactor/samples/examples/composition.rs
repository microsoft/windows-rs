#![windows_subsystem = "windows"]

use windows_reactor::*;

fn labeled_row(label: &str, value: impl Into<View>) -> View {
    View::fragment((TextBlock::new().text(label), value.into()))
}

fn badge_button(label: &str, count: u32) -> View {
    Button::new().content(TextBlock::new().text(format!("{label} ({count})")))
}

#[derive(Clone, Copy)]
enum Message {
    IncrementDrafts,
    IncrementInbox,
}

struct CompositionSample {
    drafts: u32,
    inbox: u32,
}

impl Component for CompositionSample {
    type Message = Message;
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            drafts: 1,
            inbox: 3,
        }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::IncrementDrafts => self.drafts += 1,
            Message::IncrementInbox => self.inbox += 1,
        }
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("Composition");
        Border::new().padding(16.0).content(
            StackPanel::new().spacing(12.0).children((
                TextBlock::new()
                    .text("Settings (labeled_row returns a Fragment)")
                    .font_size(20.0),
                StackPanel::new().spacing(6.0).children((
                    labeled_row("Username", TextBlock::new().text("alice")),
                    labeled_row("Theme", TextBlock::new().text("Dark")),
                    labeled_row("Notifications", TextBlock::new().text("Enabled")),
                )),
                TextBlock::new()
                    .text("Reusable widgets (badge_button composes a Button)")
                    .font_size(20.0),
                StackPanel::new()
                    .orientation(Orientation::Horizontal)
                    .spacing(8.0)
                    .children((
                        badge_button("Inbox", self.inbox),
                        badge_button("Drafts", self.drafts),
                        Button::new()
                            .on_click(context.message(Message::IncrementInbox))
                            .content(TextBlock::new().text("+ Inbox")),
                        Button::new()
                            .on_click(context.message(Message::IncrementDrafts))
                            .content(TextBlock::new().text("+ Drafts")),
                    )),
            )),
        )
    }
}

fn main() {
    App::run_component::<CompositionSample>(()).unwrap();
}
