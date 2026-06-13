#![windows_subsystem = "windows"]

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (display, set_display) = cx.use_state(String::from("0"));
    let (operand, set_operand) = cx.use_state::<Option<f64>>(None);
    let (op, set_op) = cx.use_state::<Option<&'static str>>(None);
    let (reset_next, set_reset_next) = cx.use_state(false);

    let press_digit = {
        let display = display.clone();
        let set_display = set_display.clone();
        let set_reset_next = set_reset_next.clone();
        move |digit: &'static str| {
            if reset_next || display == "0" {
                set_display.call(digit.to_string());
                set_reset_next.call(false);
            } else {
                set_display.call(format!("{display}{digit}"));
            }
        }
    };

    let press_op = {
        let display = display.clone();
        let set_display = set_display.clone();
        let set_operand = set_operand.clone();
        let set_op = set_op.clone();
        let set_reset_next = set_reset_next.clone();
        move |next_op: &'static str| {
            let current = display.parse::<f64>().unwrap_or(0.0);
            if let (Some(a), Some(prev_op)) = (operand, op) {
                let result = calculate(a, current, prev_op);
                set_display.call(format_result(result));
                set_operand.call(Some(result));
            } else {
                set_operand.call(Some(current));
            }
            set_op.call(Some(next_op));
            set_reset_next.call(true);
        }
    };

    let make_num = |digit: &'static str| -> Element {
        let pd = press_digit.clone();
        button(digit)
            .on_click(move || pd(digit))
            .horizontal_alignment(HorizontalAlignment::Stretch)
            .vertical_alignment(VerticalAlignment::Stretch)
            .into()
    };

    let make_op_btn = |label: &'static str, op_code: &'static str| -> Element {
        let po = press_op.clone();
        button(label)
            .on_click(move || po(op_code))
            .horizontal_alignment(HorizontalAlignment::Stretch)
            .vertical_alignment(VerticalAlignment::Stretch)
            .into()
    };

    fn make_fn_btn(label: &'static str, handler: impl Fn() + 'static) -> Element {
        button(label)
            .on_click(handler)
            .horizontal_alignment(HorizontalAlignment::Stretch)
            .vertical_alignment(VerticalAlignment::Stretch)
            .into()
    }

    let display_text = text_block(format_display(&display))
        .font_size(48.0)
        .horizontal_alignment(HorizontalAlignment::Right)
        .vertical_alignment(VerticalAlignment::Bottom)
        .margin(Thickness::uniform(16.0));

    let percent_button = make_fn_btn("%", {
        let display = display.clone();
        let set_display = set_display.clone();
        let set_reset_next = set_reset_next.clone();
        move || {
            let current = display.parse::<f64>().unwrap_or(0.0);
            let base = operand.unwrap_or(0.0);
            set_display.call(format_result(base * current / 100.0));
            set_reset_next.call(true);
        }
    });

    let ce_button = make_fn_btn("CE", {
        let set_display = set_display.clone();
        let set_reset_next = set_reset_next.clone();
        move || {
            set_display.call(String::from("0"));
            set_reset_next.call(false);
        }
    });

    let clear_button = make_fn_btn("C", {
        let set_display = set_display.clone();
        let set_operand = set_operand.clone();
        let set_op = set_op.clone();
        let set_reset_next = set_reset_next.clone();
        move || {
            set_display.call(String::from("0"));
            set_operand.call(None);
            set_op.call(None);
            set_reset_next.call(false);
        }
    });

    let backspace_button = make_fn_btn("\u{232B}", {
        let display = display.clone();
        let set_display = set_display.clone();
        move || {
            if display.len() <= 1 || (display.len() == 2 && display.starts_with('-')) {
                set_display.call(String::from("0"));
            } else {
                set_display.call(display[..display.len() - 1].to_string());
            }
        }
    });

    let reciprocal_button = make_fn_btn("\u{00B9}\u{2044}\u{2093}", {
        let display = display.clone();
        let set_display = set_display.clone();
        let set_reset_next = set_reset_next.clone();
        move || {
            let current = display.parse::<f64>().unwrap_or(0.0);
            if current != 0.0 {
                set_display.call(format_result(1.0 / current));
                set_reset_next.call(true);
            }
        }
    });

    let square_button = make_fn_btn("x\u{00B2}", {
        let display = display.clone();
        let set_display = set_display.clone();
        let set_reset_next = set_reset_next.clone();
        move || {
            let current = display.parse::<f64>().unwrap_or(0.0);
            set_display.call(format_result(current * current));
            set_reset_next.call(true);
        }
    });

    let sqrt_button = make_fn_btn("\u{221A}x", {
        let display = display.clone();
        let set_display = set_display.clone();
        let set_reset_next = set_reset_next.clone();
        move || {
            let current = display.parse::<f64>().unwrap_or(0.0);
            if current >= 0.0 {
                set_display.call(format_result(current.sqrt()));
                set_reset_next.call(true);
            }
        }
    });

    let negate_button = make_fn_btn("\u{00B1}", {
        let display = display.clone();
        let set_display = set_display.clone();
        move || {
            if display != "0" {
                if let Some(stripped) = display.strip_prefix('-') {
                    set_display.call(stripped.to_string());
                } else {
                    set_display.call(format!("-{display}"));
                }
            }
        }
    });

    let decimal_button = make_fn_btn(".", {
        let display = display.clone();
        let set_display = set_display.clone();
        let set_reset_next = set_reset_next.clone();
        move || {
            if reset_next {
                set_display.call(String::from("0."));
                set_reset_next.call(false);
            } else if !display.contains('.') {
                set_display.call(format!("{display}."));
            }
        }
    });

    let equals_handler = {
        let display = display.clone();
        let set_display = set_display.clone();
        let set_operand = set_operand;
        let set_op = set_op.clone();
        let set_reset_next = set_reset_next.clone();
        std::rc::Rc::new(move || {
            if let (Some(a), Some(prev_op)) = (operand, op) {
                let current = display.parse::<f64>().unwrap_or(0.0);
                let result = calculate(a, current, prev_op);
                set_display.call(format_result(result));
                set_operand.call(None);
                set_op.call(None);
                set_reset_next.call(true);
            }
        })
    };

    let equals_button: Element = button("=")
        .on_click({
            let h = equals_handler.clone();
            move || h()
        })
        .horizontal_alignment(HorizontalAlignment::Stretch)
        .vertical_alignment(VerticalAlignment::Stretch)
        .accent()
        .into();

    // Button grid matching Windows Calculator Standard layout:
    // Row 0: %    CE   C    ⌫
    // Row 1: 1/x  x²   √x   ÷
    // Row 2: 7    8    9    ×
    // Row 3: 4    5    6    −
    // Row 4: 1    2    3    +
    // Row 5: ±    0    .    =
    let button_grid = grid(vec![
        percent_button.grid_row(0).grid_column(0),
        ce_button.grid_row(0).grid_column(1),
        clear_button.grid_row(0).grid_column(2),
        backspace_button.grid_row(0).grid_column(3),
        reciprocal_button.grid_row(1).grid_column(0),
        square_button.grid_row(1).grid_column(1),
        sqrt_button.grid_row(1).grid_column(2),
        make_op_btn("\u{00F7}", "/").grid_row(1).grid_column(3),
        make_num("7").grid_row(2).grid_column(0),
        make_num("8").grid_row(2).grid_column(1),
        make_num("9").grid_row(2).grid_column(2),
        make_op_btn("\u{00D7}", "*").grid_row(2).grid_column(3),
        make_num("4").grid_row(3).grid_column(0),
        make_num("5").grid_row(3).grid_column(1),
        make_num("6").grid_row(3).grid_column(2),
        make_op_btn("\u{2212}", "-").grid_row(3).grid_column(3),
        make_num("1").grid_row(4).grid_column(0),
        make_num("2").grid_row(4).grid_column(1),
        make_num("3").grid_row(4).grid_column(2),
        make_op_btn("+", "+").grid_row(4).grid_column(3),
        negate_button.grid_row(5).grid_column(0),
        make_num("0").grid_row(5).grid_column(1),
        decimal_button.grid_row(5).grid_column(2),
        equals_button.grid_row(5).grid_column(3),
    ])
    .rows([GridLength::Star(1.0); 6])
    .columns([GridLength::Star(1.0); 4])
    .row_spacing(4.0)
    .column_spacing(4.0)
    .horizontal_alignment(HorizontalAlignment::Stretch)
    .vertical_alignment(VerticalAlignment::Stretch)
    .margin(Thickness {
        left: 4.0,
        top: 2.0,
        right: 4.0,
        bottom: 4.0,
    });

    // Numpad keyboard accelerators
    let numpad_digits: [(KeyboardKey, &str); 10] = [
        (KeyboardKey::NumPad0, "0"),
        (KeyboardKey::NumPad1, "1"),
        (KeyboardKey::NumPad2, "2"),
        (KeyboardKey::NumPad3, "3"),
        (KeyboardKey::NumPad4, "4"),
        (KeyboardKey::NumPad5, "5"),
        (KeyboardKey::NumPad6, "6"),
        (KeyboardKey::NumPad7, "7"),
        (KeyboardKey::NumPad8, "8"),
        (KeyboardKey::NumPad9, "9"),
    ];

    let mut root: Element = grid((
        TitleBar::new("Calculator")
            .tall(true)
            .grid_row(0)
            .grid_column(0),
        display_text.grid_row(1).grid_column(0),
        button_grid.grid_row(2).grid_column(0),
    ))
    .rows([GridLength::Auto, GridLength::Auto, GridLength::Star(1.0)])
    .columns([GridLength::Star(1.0)])
    .horizontal_alignment(HorizontalAlignment::Stretch)
    .vertical_alignment(VerticalAlignment::Stretch)
    .into();

    for (key, digit) in numpad_digits {
        let pd = press_digit.clone();
        root = root.keyboard_accelerator(KeyboardAccelerator::new(
            key,
            KeyModifiers::NONE,
            move || pd(digit),
        ));
    }

    let po = press_op.clone();
    root = root
        .keyboard_accelerator(KeyboardAccelerator::new(
            KeyboardKey::NumPadDivide,
            KeyModifiers::NONE,
            {
                let po = po.clone();
                move || po("/")
            },
        ))
        .keyboard_accelerator(KeyboardAccelerator::new(
            KeyboardKey::NumPadMultiply,
            KeyModifiers::NONE,
            {
                let po = po.clone();
                move || po("*")
            },
        ))
        .keyboard_accelerator(KeyboardAccelerator::new(
            KeyboardKey::NumPadSubtract,
            KeyModifiers::NONE,
            {
                let po = po.clone();
                move || po("-")
            },
        ))
        .keyboard_accelerator(KeyboardAccelerator::new(
            KeyboardKey::NumPadAdd,
            KeyModifiers::NONE,
            move || po("+"),
        ))
        .keyboard_accelerator(KeyboardAccelerator::new(
            KeyboardKey::NumPadDecimal,
            KeyModifiers::NONE,
            {
                move || {
                    if reset_next {
                        set_display.call(String::from("0."));
                        set_reset_next.call(false);
                    } else if !display.contains('.') {
                        set_display.call(format!("{display}."));
                    }
                }
            },
        ))
        .keyboard_accelerator(KeyboardAccelerator::new(
            KeyboardKey::Enter,
            KeyModifiers::NONE,
            move || equals_handler(),
        ));

    root
}

fn format_display(s: &str) -> String {
    let (integer_part, decimal_part) = match s.find('.') {
        Some(pos) => (&s[..pos], Some(&s[pos..])),
        None => (s, None),
    };

    let (sign, digits) = if let Some(stripped) = integer_part.strip_prefix('-') {
        ("-", stripped)
    } else {
        ("", integer_part)
    };

    // Insert thousands separators
    let mut result = String::new();
    for (i, ch) in digits.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(ch);
    }
    let with_commas: String = result.chars().rev().collect();

    match decimal_part {
        Some(dec) => format!("{sign}{with_commas}{dec}"),
        None => format!("{sign}{with_commas}"),
    }
}

fn calculate(a: f64, b: f64, op: &str) -> f64 {
    match op {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => {
            if b == 0.0 {
                0.0
            } else {
                a / b
            }
        }
        _ => b,
    }
}

fn format_result(value: f64) -> String {
    if value == value.floor() && value.is_finite() {
        format!("{value:.0}")
    } else {
        let s = format!("{value:.10}");
        let trimmed = s.trim_end_matches('0').trim_end_matches('.');
        trimmed.to_string()
    }
}

fn main() -> Result<()> {
    let _bootstrap_handle = bootstrap()?;
    App::new()
        .title("Calculator")
        .backdrop(Backdrop::Mica)
        .inner_size(350.0, 500.0)
        .inner_constraints(InnerConstraints {
            min_width: Some(350.0),
            min_height: Some(500.0),
            ..Default::default()
        })
        .render(app)
}
