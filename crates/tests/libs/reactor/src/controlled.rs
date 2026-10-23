use super::*;

/// Controlled toggles and ranged values. Every callback is logged; while rejecting, the state
/// ignores the reported values so the next render must restore the declared ones. The range
/// buttons change the bounds of the declared values.
pub(super) struct ControlledFixture {
    check: Option<bool>,
    toggle: Option<bool>,
    switch: bool,
    expanded: bool,
    slider: f64,
    number: Option<f64>,
    rating: Option<f64>,
    maximum: f64,
    max_rating: i32,
    accept: bool,
    calls: Vec<String>,
}

#[derive(Clone)]
pub(super) enum ControlledMessage {
    Check(Option<bool>),
    Toggle(Option<bool>),
    Switch(bool),
    Expanded(bool),
    Slider(f64),
    Number(Option<f64>),
    Rating(Option<f64>),
    Program,
    Reject,
    Shrink,
    Widen,
    Grow,
}

impl Component for ControlledFixture {
    type Input = ();
    type Message = ControlledMessage;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            check: Some(false),
            toggle: Some(false),
            switch: false,
            expanded: false,
            slider: 150.0,
            number: Some(150.0),
            rating: Some(4.0),
            maximum: 200.0,
            max_rating: 5,
            accept: true,
            calls: Vec::new(),
        }
    }

    fn update(&mut self, message: ControlledMessage, _: &ComponentContext<Self>) {
        use ControlledMessage as M;
        let call = match &message {
            M::Check(value) => format!("check {value:?}"),
            M::Toggle(value) => format!("toggle {value:?}"),
            M::Switch(value) => format!("switch {value}"),
            M::Expanded(value) => format!("expanded {value}"),
            M::Slider(value) => format!("slider {value}"),
            M::Number(value) => format!("number {value:?}"),
            M::Rating(value) => format!("rating {value:?}"),
            M::Program | M::Reject | M::Shrink | M::Widen | M::Grow => String::new(),
        };
        if !call.is_empty() {
            self.calls.push(call);
            if !self.accept {
                return;
            }
        }
        match message {
            M::Check(value) => self.check = value,
            M::Toggle(value) => self.toggle = value,
            M::Switch(value) => self.switch = value,
            M::Expanded(value) => self.expanded = value,
            M::Slider(value) => self.slider = value,
            M::Number(value) => self.number = value,
            M::Rating(value) => self.rating = value,
            M::Program => {
                self.check = Some(false);
                self.toggle = Some(false);
                self.switch = false;
                self.expanded = false;
                self.slider = 40.0;
                self.number = Some(40.0);
                self.rating = Some(4.0);
            }
            M::Reject => self.accept = false,
            M::Shrink => {
                self.maximum = 20.0;
                self.max_rating = 2;
            }
            M::Widen => {
                self.maximum = 200.0;
                self.max_rating = 5;
            }
            M::Grow => {
                self.maximum = 300.0;
                self.max_rating = 8;
                self.slider = 250.0;
                self.number = Some(250.0);
                self.rating = Some(7.0);
            }
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        use ControlledMessage as M;
        let controls: [View; 7] = [
            CheckBox::new()
                .automation_id("check")
                .content("Check")
                .is_checked(self.check)
                .on_is_checked_changed(context.callback(M::Check))
                .into(),
            ToggleButton::new()
                .automation_id("toggle")
                .content("Toggle")
                .is_checked(self.toggle)
                .on_is_checked_changed(context.callback(M::Toggle))
                .into(),
            ToggleSwitch::new()
                .automation_id("switch")
                .is_on(self.switch)
                .on_toggled(context.callback(M::Switch))
                .into(),
            Expander::new()
                .automation_id("expander")
                .header("Expander")
                .content(TextBlock::new().text("Content"))
                .is_expanded(self.expanded)
                .on_is_expanded_changed(context.callback(M::Expanded))
                .into(),
            // The value is declared before its range to show that declaration order does not
            // decide the native write order.
            Slider::new()
                .automation_id("slider")
                .value(self.slider)
                .maximum(self.maximum)
                .on_value_changed(context.callback(M::Slider))
                .into(),
            NumberBox::new()
                .automation_id("number")
                .value(self.number)
                .maximum(self.maximum)
                .on_value_changed(context.callback(M::Number))
                .into(),
            RatingControl::new()
                .automation_id("rating")
                .value(self.rating)
                .max_rating(self.max_rating)
                .on_value_changed(context.callback(M::Rating))
                .into(),
        ];
        let commands: [View; 7] = [
            Button::new()
                .automation_id("program")
                .content("Program")
                .on_click(context.message(M::Program))
                .into(),
            Button::new()
                .automation_id("reject")
                .content("Reject")
                .on_click(context.message(M::Reject))
                .into(),
            Button::new()
                .automation_id("shrink")
                .content("Shrink")
                .on_click(context.message(M::Shrink))
                .into(),
            Button::new()
                .automation_id("widen")
                .content("Widen")
                .on_click(context.message(M::Widen))
                .into(),
            Button::new()
                .automation_id("grow")
                .content("Grow")
                .on_click(context.message(M::Grow))
                .into(),
            TextBlock::new()
                .automation_id("state")
                .text(format!(
                    "{:?} {:?} {} {} {} {:?} {:?}",
                    self.check,
                    self.toggle,
                    self.switch,
                    self.expanded,
                    self.slider,
                    self.number,
                    self.rating
                ))
                .into(),
            TextBlock::new()
                .automation_id("calls")
                .text(self.calls.join("; "))
                .into(),
        ];
        context.window_title("Reactor UI fixture: controlled");
        context.window_visuals(WindowVisuals::new().client_size(900.0, 800.0));
        StackPanel::new()
            .orientation(Orientation::Horizontal)
            .margin(20.0)
            .spacing(20.0)
            .children((
                StackPanel::new()
                    .width(400.0)
                    .spacing(8.0)
                    .children(controls),
                StackPanel::new()
                    .width(400.0)
                    .spacing(8.0)
                    .children(commands),
            ))
            .into()
    }
}

/// A `RadioButton` group controlled by one selected index, the usual app pattern. WinUI unchecks
/// the rest of the group when one is checked, and a rejected choice restores the whole group.
pub(super) struct RadioFixture {
    selected: usize,
    accept: bool,
    calls: Vec<String>,
}

#[derive(Clone)]
pub(super) enum RadioMessage {
    First(Option<bool>),
    Second(Option<bool>),
    Reject,
}

impl Component for RadioFixture {
    type Input = ();
    type Message = RadioMessage;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            selected: 0,
            accept: true,
            calls: Vec::new(),
        }
    }

    fn update(&mut self, message: RadioMessage, _: &ComponentContext<Self>) {
        let (call, value, index) = match message {
            RadioMessage::First(value) => ("first", value, 0),
            RadioMessage::Second(value) => ("second", value, 1),
            RadioMessage::Reject => {
                self.accept = false;
                return;
            }
        };
        self.calls.push(format!("{call} {value:?}"));
        if self.accept && value == Some(true) {
            self.selected = index;
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Reactor UI fixture: radio");
        context.window_visuals(WindowVisuals::new().client_size(480.0, 320.0));
        StackPanel::new()
            .margin(20.0)
            .spacing(8.0)
            .children((
                RadioButton::new()
                    .automation_id("first")
                    .content("First")
                    .group_name("group")
                    .is_checked(self.selected == 0)
                    .on_checked(context.callback(RadioMessage::First)),
                RadioButton::new()
                    .automation_id("second")
                    .content("Second")
                    .group_name("group")
                    .is_checked(self.selected == 1)
                    .on_checked(context.callback(RadioMessage::Second)),
                Button::new()
                    .automation_id("reject")
                    .content("Reject")
                    .on_click(context.message(RadioMessage::Reject)),
                TextBlock::new()
                    .automation_id("calls")
                    .text(self.calls.join("; ")),
            ))
            .into()
    }
}

/// Local noon on a day in March 2024. `to_local` applies today's UTC offset, which may differ
/// from March's by a DST hour, so noon keeps the local day correct in every time zone. The picker
/// keeps this time of day when the user picks another date.
pub fn date(day: u32) -> DateTime {
    let utc = DateTime::from_unix_secs(1_709_294_400 + (i64::from(day) - 1) * 86_400);
    DateTime::from_ticks(2 * utc.ticks() - utc.to_local().ticks())
}

fn show_date(value: Option<DateTime>) -> String {
    value.map(DateTime::to_local).map_or_else(
        || "none".to_string(),
        |value| format!("{}-{:02}-{:02}", value.year(), value.month(), value.day()),
    )
}

/// Controlled text and picker values, logged and rejected like `ControlledFixture`. Narrow moves
/// the date range past the programmed date so writing it is coerced.
pub(super) struct InputFixture {
    password: Rc<str>,
    color: Color,
    date: Option<DateTime>,
    narrow: bool,
    accept: bool,
    calls: Vec<String>,
}

#[derive(Clone)]
pub(super) enum InputMessage {
    Password(Rc<str>),
    Color(Color),
    Date(Option<DateTime>),
    Program,
    Reject,
    Narrow,
    Widen,
}

impl Component for InputFixture {
    type Input = ();
    type Message = InputMessage;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            password: Rc::from("secret"),
            color: Color::rgb(0x11, 0x22, 0x33),
            date: Some(date(15)),
            narrow: false,
            accept: true,
            calls: Vec::new(),
        }
    }

    fn update(&mut self, message: InputMessage, _: &ComponentContext<Self>) {
        use InputMessage as M;
        let call = match &message {
            M::Password(value) => format!("password {value}"),
            M::Color(value) => format!("color {:02x}{:02x}{:02x}", value.r, value.g, value.b),
            M::Date(value) => format!("date {}", show_date(*value)),
            M::Program | M::Reject | M::Narrow | M::Widen => String::new(),
        };
        if !call.is_empty() {
            self.calls.push(call);
            if !self.accept {
                return;
            }
        }
        match message {
            M::Password(value) => self.password = value,
            M::Color(value) => self.color = value,
            M::Date(value) => self.date = value,
            M::Program => {
                self.password = Rc::from("program");
                self.color = Color::rgb(0x44, 0x55, 0x66);
                self.date = Some(date(10));
            }
            M::Reject => self.accept = false,
            M::Narrow => self.narrow = true,
            M::Widen => self.narrow = false,
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        use InputMessage as M;
        let (min, max) = if self.narrow {
            (date(20), date(25))
        } else {
            (date(1), date(28))
        };
        let controls: [View; 3] = [
            PasswordBox::new()
                .automation_id("password")
                .password(&*self.password)
                .password_reveal_mode(PasswordRevealMode::Visible)
                .on_password_changed(context.callback(M::Password))
                .into(),
            ColorPicker::new()
                .automation_id("color")
                .color(self.color)
                .on_color_changed(context.callback(M::Color))
                .into(),
            CalendarDatePicker::new()
                .automation_id("date")
                .date(self.date)
                .min_date(min)
                .max_date(max)
                .on_date_changed(context.callback(M::Date))
                .into(),
        ];
        let commands: [View; 6] = [
            Button::new()
                .automation_id("program")
                .content("Program")
                .on_click(context.message(M::Program))
                .into(),
            Button::new()
                .automation_id("reject")
                .content("Reject")
                .on_click(context.message(M::Reject))
                .into(),
            Button::new()
                .automation_id("narrow")
                .content("Narrow")
                .on_click(context.message(M::Narrow))
                .into(),
            Button::new()
                .automation_id("widen")
                .content("Widen")
                .on_click(context.message(M::Widen))
                .into(),
            TextBlock::new()
                .automation_id("state")
                .text(format!(
                    "{} {:02x}{:02x}{:02x} {}",
                    self.password,
                    self.color.r,
                    self.color.g,
                    self.color.b,
                    show_date(self.date)
                ))
                .into(),
            TextBlock::new()
                .automation_id("calls")
                .text(self.calls.join("; "))
                .into(),
        ];
        context.window_title("Reactor UI fixture: controlled-input");
        context.window_visuals(WindowVisuals::new().client_size(900.0, 800.0));
        StackPanel::new()
            .orientation(Orientation::Horizontal)
            .margin(20.0)
            .spacing(20.0)
            .children((
                StackPanel::new()
                    .width(400.0)
                    .spacing(8.0)
                    .children(controls),
                StackPanel::new()
                    .width(400.0)
                    .spacing(8.0)
                    .children(commands),
            ))
            .into()
    }
}
