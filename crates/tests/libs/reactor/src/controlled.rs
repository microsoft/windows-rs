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
