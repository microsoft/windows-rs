#![windows_subsystem = "windows"]

use windows_reactor::*;

#[derive(Clone, Copy)]
enum CounterMessage {
    Decrement,
    Increment,
}

struct CounterWindow {
    count: i32,
}

impl Component for CounterWindow {
    type Message = CounterMessage;
    type Input = u32;

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self { count: 0 }
    }

    fn update(&mut self, message: CounterMessage, _context: &ComponentContext<Self>) {
        match message {
            CounterMessage::Decrement => self.count -= 1,
            CounterMessage::Increment => self.count += 1,
        }
    }

    fn view(&self, number: &u32, context: &mut ViewContext<Self>) -> View {
        context.window_title(format!("Counter window #{number}"));
        context.window_visuals(WindowVisuals::new().client_size(320.0, 220.0));
        Border::new().padding(24.0).content(
            StackPanel::new().spacing(12.0).children((
                TextBlock::new().text("Independent counter").font_size(20.0),
                TextBlock::new()
                    .text(format!("Count: {}", self.count))
                    .font_size(28.0),
                StackPanel::new()
                    .orientation(Orientation::Horizontal)
                    .spacing(8.0)
                    .children((
                        Button::new()
                            .on_click(context.message(CounterMessage::Decrement))
                            .content(TextBlock::new().text("-")),
                        Button::new()
                            .on_click(context.message(CounterMessage::Increment))
                            .content(TextBlock::new().text("+")),
                    )),
            )),
        )
    }
}

struct SecondaryWindowSample {
    opened: u32,
}

impl Component for SecondaryWindowSample {
    type Message = ();
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self { opened: 0 }
    }

    fn update(&mut self, _message: (), context: &ComponentContext<Self>) {
        let number = self.opened + 1;
        if context.open_window(View::component::<CounterWindow>(number)) {
            self.opened = number;
        }
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("Secondary windows");
        Border::new().padding(24.0).content(
            StackPanel::new().spacing(12.0).children((
                TextBlock::new().text("Each opened window hosts its own independent counter."),
                TextBlock::new().text("Closing the last remaining window exits the app."),
                Button::new()
                    .on_click(context.message(()))
                    .content(TextBlock::new().text("Open counter window")),
                TextBlock::new().text(format!("Windows opened: {}", self.opened)),
            )),
        )
    }
}

fn main() {
    App::run_component::<SecondaryWindowSample>(()).unwrap();
}
