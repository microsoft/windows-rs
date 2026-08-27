#![windows_subsystem = "windows"]

use windows_reactor::*;

struct UseMemoSample {
    factorial: i64,
    n: i32,
    recomputes: u32,
    show_hint: bool,
}

#[derive(Clone, Copy)]
enum Message {
    Decrement,
    Increment,
    ToggleHint,
}

impl UseMemoSample {
    fn set_n(&mut self, n: i32) {
        if self.n != n {
            self.n = n;
            self.factorial = (1..=i64::from(n)).product();
            self.recomputes += 1;
        }
    }
}

impl Component for UseMemoSample {
    type Message = Message;
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            factorial: 6,
            n: 3,
            recomputes: 1,
            show_hint: false,
        }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::Decrement => self.set_n((self.n - 1).max(0)),
            Message::Increment => self.set_n((self.n + 1).min(20)),
            Message::ToggleHint => self.show_hint = !self.show_hint,
        }
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("UseMemo");
        StackPanel::new().spacing(8.0).children((
            TextBlock::new()
                .text(format!(
                    "n = {},  factorial(n) = {}",
                    self.n, self.factorial
                ))
                .font_size(18.0),
            TextBlock::new()
                .text(format!(
                    "memo factory ran {} time(s) so far",
                    self.recomputes
                ))
                .font_size(12.0)
                .opacity(0.7),
            StackPanel::new()
                .orientation(Orientation::Horizontal)
                .spacing(8.0)
                .children((
                    Button::new()
                        .on_click(context.message(Message::Decrement))
                        .content(TextBlock::new().text("-")),
                    Button::new()
                        .on_click(context.message(Message::Increment))
                        .content(TextBlock::new().text("+")),
                    Button::new()
                        .on_click(context.message(Message::ToggleHint))
                        .content(TextBlock::new().text("toggle unrelated state")),
                )),
            if self.show_hint {
                TextBlock::new()
                    .text("Toggling this state rerenders, but the memo was skipped - same deps.")
                    .opacity(0.7)
                    .into()
            } else {
                View::empty()
            },
        ))
    }
}

fn main() {
    App::run_component::<UseMemoSample>(()).unwrap();
}
