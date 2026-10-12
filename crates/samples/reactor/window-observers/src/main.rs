#![windows_subsystem = "windows"]

use windows_reactor::*;

struct ObserverSample {
    revision: u32,
    dark: bool,
    subscribed: [bool; 3],
    counts: [u32; 3],
    last: [String; 3],
}

#[derive(Clone)]
enum Message {
    Observed(usize, u32, String),
    ReplaceCallbacks,
    ToggleTheme,
    ToggleSubscription(usize),
}

impl Component for ObserverSample {
    type Input = ();
    type Message = Message;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            revision: 0,
            dark: false,
            subscribed: [true; 3],
            counts: [0; 3],
            last: std::array::from_fn(|_| "Waiting for a notification".to_string()),
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Observed(index, revision, value) => {
                self.counts[index] += 1;
                self.last[index] = format!("Callback {revision}: {value}");
            }
            Message::ReplaceCallbacks => self.revision += 1,
            Message::ToggleTheme => self.dark = !self.dark,
            Message::ToggleSubscription(index) => {
                self.subscribed[index] = !self.subscribed[index];
            }
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        let revision = self.revision;
        if self.subscribed[0] {
            context.on_color_scheme(
                context.callback(move |value| Message::Observed(0, revision, format!("{value:?}"))),
            );
        }
        if self.subscribed[1] {
            context.on_window_size(
                context.callback(move |value| Message::Observed(1, revision, format!("{value:?}"))),
            );
        }
        if self.subscribed[2] {
            context.on_window_placement(
                context.callback(move |value| Message::Observed(2, revision, format!("{value:?}"))),
            );
        }
        context.window_visuals(WindowVisuals::new().client_size(800.0, 600.0).theme(
            if self.dark {
                WindowTheme::Dark
            } else {
                WindowTheme::Light
            },
        ));

        let observers = StackPanel::new().spacing(16.0).children(
            ["Scheme", "Size", "Placement"]
                .into_iter()
                .enumerate()
                .map(|(index, name)| {
                    StackPanel::new()
                        .spacing(4.0)
                        .children((
                            TextBlock::new().text(format!(
                                "{name}: {} notifications, {}",
                                self.counts[index],
                                if self.subscribed[index] {
                                    "subscribed"
                                } else {
                                    "unsubscribed"
                                },
                            )),
                            TextBlock::new()
                                .text(&self.last[index])
                                .text_wrapping(TextWrapping::Wrap),
                            Button::new()
                                .content(if self.subscribed[index] {
                                    "Unsubscribe"
                                } else {
                                    "Subscribe"
                                })
                                .on_click(context.message(Message::ToggleSubscription(index))),
                        ))
                        .into()
                })
                .collect::<Vec<View>>(),
        );
        context.window_frame(
            "Window observers",
            StackPanel::new().margin(24.0).spacing(12.0).children((
                TextBlock::new().text(format!("Current callback revision: {revision}")),
                "Replace callbacks: counts stay unchanged. The next event uses the new revision.",
                "Toggle theme, move or resize the window to produce real notifications.",
                "Re-subscribe: scheme and placement report current values; size waits for a change.",
                Button::new()
                    .content("Replace callbacks")
                    .on_click(context.message(Message::ReplaceCallbacks)),
                Button::new()
                    .content("Toggle theme")
                    .on_click(context.message(Message::ToggleTheme)),
                observers,
            )),
        )
    }
}

fn main() {
    App::run_component::<ObserverSample>(()).unwrap();
}
