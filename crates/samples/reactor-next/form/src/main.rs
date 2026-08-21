#![windows_subsystem = "windows"]

use std::time::Duration;
use windows_reactor_next::*;

struct Form {
    amount: f64,
    name: String,
    status: Status,
}

#[derive(Clone, PartialEq)]
struct SummaryProps {
    amount: f64,
    name: String,
}

struct Summary;

enum Message {
    AmountChanged(f64),
    Cancelled,
    NameChanged(String),
    Submit,
    Submitted,
}

#[derive(Clone, Copy, PartialEq)]
enum Status {
    Editing,
    Submitting,
    Submitted,
}

impl Form {
    fn is_valid(&self) -> bool {
        !self.name.trim().is_empty() && self.amount.is_finite() && self.amount > 0.0
    }
}

impl Component for Form {
    type Message = Message;
    type Props = ();

    fn create(_props: &(), _context: &mut ComponentContext<Self>) -> Self {
        Self {
            amount: 1.0,
            name: String::new(),
            status: Status::Editing,
        }
    }

    fn update(&mut self, message: Message, context: &mut ComponentContext<Self>) {
        match message {
            Message::AmountChanged(value) => {
                self.amount = value;
                self.status = Status::Editing;
            }
            Message::NameChanged(value) => {
                self.name = value;
                self.status = Status::Editing;
            }
            Message::Submit if self.is_valid() && self.status != Status::Submitting => {
                self.status = Status::Submitting;
                context.spawn_background(|cancellation| {
                    std::thread::sleep(Duration::from_millis(500));
                    if cancellation.is_cancelled() {
                        return Message::Cancelled;
                    }
                    Message::Submitted
                });
            }
            Message::Cancelled => self.status = Status::Editing,
            Message::Submitted => self.status = Status::Submitted,
            Message::Submit => {}
        }
    }

    fn view(&self, _props: &Self::Props, context: &mut ViewContext<Self>) -> View {
        let name_changed = context.sender();
        let amount_changed = context.sender();
        let submit = context.sender();
        let validation = if self.is_valid() {
            "Ready to submit"
        } else {
            "Enter a name and an amount greater than zero"
        };
        let status = match self.status {
            Status::Editing => validation,
            Status::Submitting => "Submitting...",
            Status::Submitted => "Submitted",
        };

        StackPanel::new().spacing(8.0).children([
            TextBlock::new()
                .text("Payment")
                .text_wrapping(TextWrapping::Wrap)
                .into(),
            TextBox::new()
                .text(self.name.clone())
                .placeholder_text("Name")
                .is_enabled(self.status != Status::Submitting)
                .on_text_changed(move |value| {
                    _ = name_changed.send(Message::NameChanged(value));
                })
                .into(),
            NumberBox::new()
                .minimum(0.0)
                .maximum(10_000.0)
                .value(self.amount)
                .is_enabled(self.status != Status::Submitting)
                .on_value_changed(move |value| {
                    _ = amount_changed.send(Message::AmountChanged(value));
                })
                .into(),
            TextBlock::new().text(status).into(),
            ProgressBar::new()
                .minimum(0.0)
                .maximum(1.0)
                .value(if self.status == Status::Submitted {
                    1.0
                } else {
                    0.0
                })
                .is_indeterminate(self.status == Status::Submitting)
                .into(),
            View::component::<Summary>(SummaryProps {
                amount: self.amount,
                name: self.name.clone(),
            }),
            Button::new()
                .is_enabled(self.is_valid() && self.status == Status::Editing)
                .on_click(move || {
                    _ = submit.send(Message::Submit);
                })
                .content(TextBlock::new().text("Submit")),
        ])
    }
}

impl Component for Summary {
    type Message = ();
    type Props = SummaryProps;

    fn create(_props: &SummaryProps, _context: &mut ComponentContext<Self>) -> Self {
        Self
    }

    fn update(&mut self, _message: (), _context: &mut ComponentContext<Self>) {}

    fn view(&self, props: &Self::Props, _context: &mut ViewContext<Self>) -> View {
        TextBlock::new()
            .text(format!("{}: {:.2}", props.name.trim(), props.amount))
            .into()
    }
}

fn main() {
    bootstrap().unwrap();
    App::run_component::<Form>(()).unwrap();
}
