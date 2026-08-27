#![windows_subsystem = "windows"]

use std::time::Duration;
use windows_reactor::*;

struct Form {
    amount: f64,
    amount_ref: ElementRef<NumberBox>,
    name: String,
    name_ref: ElementRef<TextBox>,
    status: Status,
}

#[derive(Clone, PartialEq)]
struct SummaryInput {
    amount: f64,
    name: String,
}

struct Summary;

#[derive(Clone)]
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
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            amount: 1.0,
            amount_ref: ElementRef::new(),
            name: String::new(),
            name_ref: ElementRef::new(),
            status: Status::Editing,
        }
    }

    fn update(&mut self, message: Message, context: &ComponentContext<Self>) {
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
            Message::Submit if self.name.trim().is_empty() => {
                _ = self.name_ref.request_focus();
            }
            Message::Submit => {
                _ = self.amount_ref.request_focus();
            }
        }
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
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

        Grid::new()
            .row_spacing(8.0)
            .column_spacing(12.0)
            .rows([
                GridLength::Auto,
                GridLength::Auto,
                GridLength::Auto,
                GridLength::Auto,
                GridLength::Auto,
                GridLength::Auto,
                GridLength::Auto,
            ])
            .columns([GridLength::Pixel(100.0), GridLength::STAR])
            .children((
                TextBlock::new()
                    .text("Payment")
                    .text_wrapping(TextWrapping::Wrap)
                    .grid_column_span(2),
                TextBlock::new().text("Name").grid_row(1),
                TextBox::new()
                    .element_ref(&self.name_ref)
                    .text(self.name.clone())
                    .placeholder_text("Name")
                    .is_enabled(self.status != Status::Submitting)
                    .on_text_changed(context.callback(Message::NameChanged))
                    .grid_row(1)
                    .grid_column(1),
                TextBlock::new().text("Amount").grid_row(2),
                NumberBox::new()
                    .element_ref(&self.amount_ref)
                    .minimum(0.0)
                    .maximum(10_000.0)
                    .value(self.amount)
                    .is_enabled(self.status != Status::Submitting)
                    .on_value_changed(context.callback(Message::AmountChanged))
                    .grid_row(2)
                    .grid_column(1),
                TextBlock::new()
                    .text(status)
                    .grid_row(3)
                    .grid_column_span(2),
                ProgressBar::new()
                    .minimum(0.0)
                    .maximum(1.0)
                    .value(if self.status == Status::Submitted {
                        1.0
                    } else {
                        0.0
                    })
                    .is_indeterminate(self.status == Status::Submitting)
                    .grid_row(4)
                    .grid_column_span(2),
                StackPanel::new()
                    .grid_row(5)
                    .grid_column_span(2)
                    .children((View::component::<Summary>(SummaryInput {
                        amount: self.amount,
                        name: self.name.clone(),
                    }),)),
                Button::new()
                    .is_enabled(self.status == Status::Editing)
                    .on_click(context.message(Message::Submit))
                    .grid_row(6)
                    .grid_column(1)
                    .content(TextBlock::new().text("Submit")),
            ))
    }
}

impl Component for Summary {
    type Message = ();
    type Input = SummaryInput;

    fn create(_input: &SummaryInput, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {}

    fn view(&self, input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        TextBlock::new()
            .text(format!("{}: {:.2}", input.name.trim(), input.amount))
            .into()
    }
}

fn main() {
    App::run_component::<Form>(()).unwrap();
}
