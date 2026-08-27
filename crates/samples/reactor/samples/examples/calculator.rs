#![windows_subsystem = "windows"]

use windows_reactor::*;

#[derive(Clone)]
enum Message {
    Digit(&'static str),
    Operation(&'static str),
    Percent,
    ClearEntry,
    Clear,
    Backspace,
    Reciprocal,
    Square,
    SquareRoot,
    Negate,
    Decimal,
    Equals,
}

struct Calculator {
    display: String,
    operand: Option<f64>,
    operation: Option<&'static str>,
    reset_next: bool,
}

impl Calculator {
    fn current(&self) -> f64 {
        self.display.parse().unwrap_or(0.0)
    }

    fn apply_unary(&mut self, value: f64) {
        self.display = format_result(value);
        self.reset_next = true;
    }
}

impl Component for Calculator {
    type Message = Message;
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            display: "0".to_string(),
            operand: None,
            operation: None,
            reset_next: false,
        }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::Digit(digit) => {
                if self.reset_next || self.display == "0" {
                    self.display = digit.to_string();
                    self.reset_next = false;
                } else {
                    self.display.push_str(digit);
                }
            }
            Message::Operation(next) => {
                let current = self.current();
                if let (Some(left), Some(previous)) = (self.operand, self.operation) {
                    let result = calculate(left, current, previous);
                    self.display = format_result(result);
                    self.operand = Some(result);
                } else {
                    self.operand = Some(current);
                }
                self.operation = Some(next);
                self.reset_next = true;
            }
            Message::Percent => {
                self.apply_unary(self.operand.unwrap_or(0.0) * self.current() / 100.0);
            }
            Message::ClearEntry => {
                self.display = "0".to_string();
                self.reset_next = false;
            }
            Message::Clear => {
                self.display = "0".to_string();
                self.operand = None;
                self.operation = None;
                self.reset_next = false;
            }
            Message::Backspace => {
                if self.display.len() <= 1
                    || self.display.len() == 2 && self.display.starts_with('-')
                {
                    self.display = "0".to_string();
                } else {
                    self.display.pop();
                }
            }
            Message::Reciprocal => {
                let current = self.current();
                if current != 0.0 {
                    self.apply_unary(1.0 / current);
                }
            }
            Message::Square => self.apply_unary(self.current() * self.current()),
            Message::SquareRoot => {
                let current = self.current();
                if current >= 0.0 {
                    self.apply_unary(current.sqrt());
                }
            }
            Message::Negate => {
                if self.display != "0" {
                    if self.display.starts_with('-') {
                        self.display.remove(0);
                    } else {
                        self.display.insert(0, '-');
                    }
                }
            }
            Message::Decimal => {
                if self.reset_next {
                    self.display = "0.".to_string();
                    self.reset_next = false;
                } else if !self.display.contains('.') {
                    self.display.push('.');
                }
            }
            Message::Equals => {
                if let (Some(left), Some(operation)) = (self.operand, self.operation) {
                    self.display = format_result(calculate(left, self.current(), operation));
                    self.operand = None;
                    self.operation = None;
                    self.reset_next = true;
                }
            }
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Calculator");
        context.window_visuals(
            WindowVisuals::new()
                .backdrop(WindowBackdrop::Mica)
                .client_size(350.0, 500.0),
        );
        let button = |label, message, row, column, accent| {
            let button = Button::new()
                .on_click(context.message(message))
                .horizontal_alignment(HorizontalAlignment::Stretch)
                .vertical_alignment(VerticalAlignment::Stretch)
                .grid_row(row)
                .grid_column(column);
            let button = if accent {
                button.style(ButtonStyle::Accent)
            } else {
                button
            };
            button.content(TextBlock::new().text(label))
        };
        let buttons = [
            button("%", Message::Percent, 0, 0, false),
            button("CE", Message::ClearEntry, 0, 1, false),
            button("C", Message::Clear, 0, 2, false),
            button("\u{232B}", Message::Backspace, 0, 3, false),
            button("\u{00B9}\u{2044}\u{2093}", Message::Reciprocal, 1, 0, false),
            button("x\u{00B2}", Message::Square, 1, 1, false),
            button("\u{221A}x", Message::SquareRoot, 1, 2, false),
            button("\u{00F7}", Message::Operation("/"), 1, 3, false),
            button("7", Message::Digit("7"), 2, 0, false),
            button("8", Message::Digit("8"), 2, 1, false),
            button("9", Message::Digit("9"), 2, 2, false),
            button("\u{00D7}", Message::Operation("*"), 2, 3, false),
            button("4", Message::Digit("4"), 3, 0, false),
            button("5", Message::Digit("5"), 3, 1, false),
            button("6", Message::Digit("6"), 3, 2, false),
            button("\u{2212}", Message::Operation("-"), 3, 3, false),
            button("1", Message::Digit("1"), 4, 0, false),
            button("2", Message::Digit("2"), 4, 1, false),
            button("3", Message::Digit("3"), 4, 2, false),
            button("+", Message::Operation("+"), 4, 3, false),
            button("\u{00B1}", Message::Negate, 5, 0, false),
            button("0", Message::Digit("0"), 5, 1, false),
            button(".", Message::Decimal, 5, 2, false),
            button("=", Message::Equals, 5, 3, true),
        ];
        let mut accelerators = [
            (AcceleratorKey::NumberPad0, Message::Digit("0")),
            (AcceleratorKey::NumberPad1, Message::Digit("1")),
            (AcceleratorKey::NumberPad2, Message::Digit("2")),
            (AcceleratorKey::NumberPad3, Message::Digit("3")),
            (AcceleratorKey::NumberPad4, Message::Digit("4")),
            (AcceleratorKey::NumberPad5, Message::Digit("5")),
            (AcceleratorKey::NumberPad6, Message::Digit("6")),
            (AcceleratorKey::NumberPad7, Message::Digit("7")),
            (AcceleratorKey::NumberPad8, Message::Digit("8")),
            (AcceleratorKey::NumberPad9, Message::Digit("9")),
            (AcceleratorKey::Divide, Message::Operation("/")),
            (AcceleratorKey::Multiply, Message::Operation("*")),
            (AcceleratorKey::Subtract, Message::Operation("-")),
            (AcceleratorKey::Add, Message::Operation("+")),
            (AcceleratorKey::Decimal, Message::Decimal),
            (AcceleratorKey::Enter, Message::Equals),
        ]
        .into_iter()
        .map(|(key, message)| {
            KeyAccelerator::new(key, AcceleratorModifiers::None, context.message(message))
        })
        .collect::<Vec<_>>();

        Grid::new()
            .rows([GridLength::Auto, GridLength::Auto, GridLength::STAR])
            .columns([GridLength::STAR])
            .key_accelerators(KeyAccelerators::new(accelerators.drain(..)))
            .children((
                TextBlock::new()
                    .text("Calculator")
                    .font_size(20.0)
                    .grid_row(0),
                TextBlock::new()
                    .text(format_display(&self.display))
                    .font_size(48.0)
                    .horizontal_alignment(HorizontalAlignment::Right)
                    .vertical_alignment(VerticalAlignment::Bottom)
                    .margin(Thickness::uniform(16.0))
                    .grid_row(1),
                Border::new()
                    .padding(Thickness::uniform(4.0))
                    .grid_row(2)
                    .content(
                        Grid::new()
                            .rows([GridLength::STAR; 6])
                            .columns([GridLength::STAR; 4])
                            .row_spacing(4.0)
                            .column_spacing(4.0)
                            .children(buttons),
                    ),
            ))
    }
}

fn format_display(value: &str) -> String {
    let (integer, decimal) = value.find('.').map_or((value, None), |position| {
        (&value[..position], Some(&value[position..]))
    });
    let (sign, digits) = integer
        .strip_prefix('-')
        .map_or(("", integer), |digits| ("-", digits));
    let mut grouped = String::new();
    for (index, character) in digits.chars().rev().enumerate() {
        if index > 0 && index % 3 == 0 {
            grouped.push(',');
        }
        grouped.push(character);
    }
    let grouped: String = grouped.chars().rev().collect();
    format!("{sign}{grouped}{}", decimal.unwrap_or_default())
}

fn calculate(left: f64, right: f64, operation: &str) -> f64 {
    match operation {
        "+" => left + right,
        "-" => left - right,
        "*" => left * right,
        "/" if right != 0.0 => left / right,
        "/" => 0.0,
        _ => right,
    }
}

fn format_result(value: f64) -> String {
    if value == value.floor() && value.is_finite() {
        format!("{value:.0}")
    } else {
        format!("{value:.10}")
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    }
}

fn main() {
    App::run_component::<Calculator>(()).unwrap();
}
