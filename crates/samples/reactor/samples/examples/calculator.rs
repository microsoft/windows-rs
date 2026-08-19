#![windows_subsystem = "windows"]

use windows_reactor::{
    AutomationHeadingLevel, Button, ButtonEmphasis, Callback, Element, Grid, GridChild, GridLength,
    HorizontalAlignment, KeyboardAccelerator, RenderCx, TextBlock, Thickness, TitleBar,
    VerticalAlignment, VirtualKey, VirtualKeyModifiers, WindowBackdrop, WindowConstraints,
};

#[derive(Clone, Copy)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Clone)]
struct Calculator {
    display: String,
    operand: Option<f64>,
    operation: Option<Operation>,
    replace_display: bool,
}

impl Default for Calculator {
    fn default() -> Self {
        Self {
            display: "0".to_string(),
            operand: None,
            operation: None,
            replace_display: false,
        }
    }
}

#[derive(Clone)]
enum Action {
    Digit(char),
    Decimal,
    Operator(Operation),
    Equals,
    Percent,
    ClearEntry,
    Clear,
    Backspace,
    Reciprocal,
    Square,
    SquareRoot,
    Negate,
}

fn parse_display(display: &str) -> f64 {
    display.parse().unwrap_or(0.0)
}

fn calculate(left: f64, right: f64, operation: Operation) -> f64 {
    match operation {
        Operation::Add => left + right,
        Operation::Subtract => left - right,
        Operation::Multiply => left * right,
        Operation::Divide if right != 0.0 => left / right,
        Operation::Divide => 0.0,
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

fn format_display(display: &str) -> String {
    let (integer, decimal) = display.find('.').map_or((display, None), |index| {
        (&display[..index], Some(&display[index..]))
    });
    let (sign, digits) = integer
        .strip_prefix('-')
        .map_or(("", integer), |digits| ("-", digits));
    let mut reversed = String::new();
    for (index, digit) in digits.chars().rev().enumerate() {
        if index > 0 && index % 3 == 0 {
            reversed.push(',');
        }
        reversed.push(digit);
    }
    let grouped = reversed.chars().rev().collect::<String>();
    format!("{sign}{grouped}{}", decimal.unwrap_or_default())
}

fn reducer(mut state: Calculator, action: Action) -> Calculator {
    match action {
        Action::Digit(digit) => {
            if state.replace_display || state.display == "0" {
                state.display = digit.to_string();
                state.replace_display = false;
            } else {
                state.display.push(digit);
            }
        }
        Action::Decimal => {
            if state.replace_display {
                state.display = "0.".to_string();
                state.replace_display = false;
            } else if !state.display.contains('.') {
                state.display.push('.');
            }
        }
        Action::Operator(next) => {
            let current = parse_display(&state.display);
            let operand = match (state.operand, state.operation) {
                (Some(left), Some(operation)) => calculate(left, current, operation),
                _ => current,
            };
            state.display = format_result(operand);
            state.operand = Some(operand);
            state.operation = Some(next);
            state.replace_display = true;
        }
        Action::Equals => {
            if let (Some(left), Some(operation)) = (state.operand, state.operation) {
                let right = parse_display(&state.display);
                state.display = format_result(calculate(left, right, operation));
                state.operand = None;
                state.operation = None;
                state.replace_display = true;
            }
        }
        Action::Percent => {
            let current = parse_display(&state.display);
            let base = state.operand.unwrap_or(0.0);
            state.display = format_result(base * current / 100.0);
            state.replace_display = true;
        }
        Action::ClearEntry => {
            state.display = "0".to_string();
            state.replace_display = false;
        }
        Action::Clear => state = Calculator::default(),
        Action::Backspace => {
            if !state.replace_display {
                state.display.pop();
                if state.display.is_empty() || state.display == "-" {
                    state.display = "0".to_string();
                }
            }
        }
        Action::Reciprocal => {
            let current = parse_display(&state.display);
            if current != 0.0 {
                state.display = format_result(1.0 / current);
                state.replace_display = true;
            }
        }
        Action::Square => {
            let current = parse_display(&state.display);
            state.display = format_result(current * current);
            state.replace_display = true;
        }
        Action::SquareRoot => {
            let current = parse_display(&state.display);
            if current >= 0.0 {
                state.display = format_result(current.sqrt());
                state.replace_display = true;
            }
        }
        Action::Negate => {
            if state.display != "0" {
                if let Some(value) = state.display.strip_prefix('-') {
                    state.display = value.to_string();
                } else {
                    state.display.insert(0, '-');
                }
            }
        }
    }
    state
}

fn calculator_button(
    label: &'static str,
    action: Action,
    dispatch: Callback<Action>,
    accent: bool,
) -> Element {
    let button = Button::new(label);
    let button = if accent {
        button.emphasis(ButtonEmphasis::Accent)
    } else {
        button
    };
    button
        .on_click(move || dispatch.call(action.clone()))
        .horizontal_alignment(HorizontalAlignment::Stretch)
        .vertical_alignment(VerticalAlignment::Stretch)
        .build()
}

fn app(cx: &mut RenderCx<'_>) -> Element {
    let (state, dispatch) = cx.use_reducer(Calculator::default, reducer);
    let mut children = vec![
        GridChild::new(
            TextBlock::new(format_display(&state.display))
                .font_size(48.0)
                .horizontal_alignment(HorizontalAlignment::Right)
                .vertical_alignment(VerticalAlignment::Bottom)
                .margin(Thickness::uniform(16.0))
                .automation_id("calculator-display")
                .heading_level(AutomationHeadingLevel::Level1)
                .build(),
        )
        .row(0)
        .column_span(4),
    ];
    let buttons = [
        ("%", Action::Percent, 1, 0, false),
        ("CE", Action::ClearEntry, 1, 1, false),
        ("C", Action::Clear, 1, 2, false),
        ("Back", Action::Backspace, 1, 3, false),
        ("1/x", Action::Reciprocal, 2, 0, false),
        ("x^2", Action::Square, 2, 1, false),
        ("sqrt", Action::SquareRoot, 2, 2, false),
        ("/", Action::Operator(Operation::Divide), 2, 3, false),
        ("7", Action::Digit('7'), 3, 0, false),
        ("8", Action::Digit('8'), 3, 1, false),
        ("9", Action::Digit('9'), 3, 2, false),
        ("*", Action::Operator(Operation::Multiply), 3, 3, false),
        ("4", Action::Digit('4'), 4, 0, false),
        ("5", Action::Digit('5'), 4, 1, false),
        ("6", Action::Digit('6'), 4, 2, false),
        ("-", Action::Operator(Operation::Subtract), 4, 3, false),
        ("1", Action::Digit('1'), 5, 0, false),
        ("2", Action::Digit('2'), 5, 1, false),
        ("3", Action::Digit('3'), 5, 2, false),
        ("+", Action::Operator(Operation::Add), 5, 3, false),
        ("+/-", Action::Negate, 6, 0, false),
        ("0", Action::Digit('0'), 6, 1, false),
        (".", Action::Decimal, 6, 2, false),
        ("=", Action::Equals, 6, 3, true),
    ];
    children.extend(
        buttons
            .into_iter()
            .map(|(label, action, row, column, accent)| {
                GridChild::new(calculator_button(label, action, dispatch.clone(), accent))
                    .row(row)
                    .column(column)
            }),
    );

    let mut root = Grid::new(children)
        .rows([
            GridLength::Auto,
            GridLength::STAR,
            GridLength::STAR,
            GridLength::STAR,
            GridLength::STAR,
            GridLength::STAR,
            GridLength::STAR,
        ])
        .columns([GridLength::STAR; 4])
        .row_spacing(4.0)
        .column_spacing(4.0)
        .margin(Thickness {
            left: 4.0,
            top: 2.0,
            right: 4.0,
            bottom: 4.0,
        })
        .horizontal_alignment(HorizontalAlignment::Stretch)
        .vertical_alignment(VerticalAlignment::Stretch);

    for (key, digit) in [
        (VirtualKey::NUMBER_PAD_0, '0'),
        (VirtualKey::NUMBER_PAD_1, '1'),
        (VirtualKey::NUMBER_PAD_2, '2'),
        (VirtualKey::NUMBER_PAD_3, '3'),
        (VirtualKey::NUMBER_PAD_4, '4'),
        (VirtualKey::NUMBER_PAD_5, '5'),
        (VirtualKey::NUMBER_PAD_6, '6'),
        (VirtualKey::NUMBER_PAD_7, '7'),
        (VirtualKey::NUMBER_PAD_8, '8'),
        (VirtualKey::NUMBER_PAD_9, '9'),
    ] {
        let digit_dispatch = dispatch.clone();
        root = root.keyboard_accelerator(KeyboardAccelerator::new(
            key,
            VirtualKeyModifiers::NONE,
            move || digit_dispatch.call(Action::Digit(digit)),
        ));
    }
    for (key, operation) in [
        (VirtualKey::DIVIDE, Operation::Divide),
        (VirtualKey::MULTIPLY, Operation::Multiply),
        (VirtualKey::SUBTRACT, Operation::Subtract),
        (VirtualKey::ADD, Operation::Add),
    ] {
        let operation_dispatch = dispatch.clone();
        root = root.keyboard_accelerator(KeyboardAccelerator::new(
            key,
            VirtualKeyModifiers::NONE,
            move || operation_dispatch.call(Action::Operator(operation)),
        ));
    }
    let decimal_dispatch = dispatch.clone();
    root = root.keyboard_accelerator(KeyboardAccelerator::new(
        VirtualKey::DECIMAL,
        VirtualKeyModifiers::NONE,
        move || decimal_dispatch.call(Action::Decimal),
    ));
    root.keyboard_accelerator(KeyboardAccelerator::new(
        VirtualKey::ENTER,
        VirtualKeyModifiers::NONE,
        move || dispatch.call(Action::Equals),
    ))
    .build()
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run_with_window(
        "Calculator",
        |window| {
            window
                .backdrop(WindowBackdrop::Mica)
                .title_bar(TitleBar::custom("Calculator"))
                .client_size(350.0, 500.0)
                .client_constraints(WindowConstraints {
                    min_width: Some(350.0),
                    min_height: Some(500.0),
                    ..WindowConstraints::default()
                })
        },
        app,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chained_operations_use_the_displayed_result() {
        let mut state = Calculator::default();
        for action in [
            Action::Digit('7'),
            Action::Operator(Operation::Add),
            Action::Digit('8'),
            Action::Equals,
            Action::Operator(Operation::Multiply),
            Action::Digit('2'),
            Action::Equals,
        ] {
            state = reducer(state, action);
        }
        assert_eq!(state.display, "30");
    }

    #[test]
    fn display_groups_the_integer_part() {
        assert_eq!(format_display("-1234567.5"), "-1,234,567.5");
    }
}
