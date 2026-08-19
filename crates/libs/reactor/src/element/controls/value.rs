use std::rc::Rc;

use windows_time::{DateTime, TimeSpan};

use crate::element::props::*;
use crate::element::tree::*;
use crate::element::values::*;
use crate::element::{
    ColorEventFn, DatesEventFn, Element, FloatEventFn, Framework, OptionalDateEventFn,
    OptionalFloatEventFn, OptionalTimeEventFn, enforce_display_only,
};
use crate::framework_properties::FrameworkProps;
pub struct ProgressBar {
    props: ProgressBarProps,
}

impl ProgressBar {
    pub fn new(value: f64) -> Framework<Self> {
        Framework::new({
            Self {
                props: ProgressBarProps {
                    value,
                    minimum: 0.0,
                    maximum: 100.0,
                    indeterminate: false,
                    framework: FrameworkProps::default(),
                },
            }
        })
    }

    pub fn indeterminate() -> Framework<Self> {
        Framework::new({
            Self {
                props: ProgressBarProps {
                    value: 0.0,
                    minimum: 0.0,
                    maximum: 100.0,
                    indeterminate: true,
                    framework: FrameworkProps::default(),
                },
            }
        })
    }

    pub(crate) fn build_with_framework(mut self, framework: FrameworkProps) -> Element {
        validate_range_state(self.props.value, self.props.minimum, self.props.maximum);
        self.props.framework = framework;
        Element::new(ElementKind::ProgressBar(self.props))
    }
}

pub struct ProgressRing {
    props: ProgressRingProps,
}

impl ProgressRing {
    pub fn new(value: f64) -> Framework<Self> {
        Framework::new({
            Self {
                props: ProgressRingProps {
                    value,
                    minimum: 0.0,
                    maximum: 100.0,
                    active: true,
                    indeterminate: false,
                    framework: FrameworkProps::default(),
                },
            }
        })
    }

    pub fn indeterminate() -> Framework<Self> {
        Framework::new({
            Self {
                props: ProgressRingProps {
                    value: 0.0,
                    minimum: 0.0,
                    maximum: 100.0,
                    active: true,
                    indeterminate: true,
                    framework: FrameworkProps::default(),
                },
            }
        })
    }

    pub(crate) fn build_with_framework(mut self, framework: FrameworkProps) -> Element {
        validate_range_state(self.props.value, self.props.minimum, self.props.maximum);
        self.props.framework = framework;
        Element::new(ElementKind::ProgressRing(self.props))
    }
}

pub struct Slider {
    props: SliderProps,
}

impl Slider {
    pub fn new(value: f64, on_change: impl Fn(f64) + 'static) -> Framework<Self> {
        Framework::new(Self::with_handler(value, Some(Rc::new(on_change))))
    }

    pub fn display(value: f64) -> Framework<Self> {
        Framework::new(Self::with_handler(value, None))
    }

    fn with_handler(value: f64, on_change: Option<FloatEventFn>) -> Self {
        Self {
            props: SliderProps {
                value,
                minimum: 0.0,
                maximum: 100.0,
                step: 1.0,
                header: None,
                orientation: Orientation::Horizontal,
                on_change,
                framework: FrameworkProps::default(),
            },
        }
    }
}

impl Framework<Slider> {
    pub fn range(mut self, minimum: f64, maximum: f64) -> Self {
        self.control.props.minimum = minimum;
        self.control.props.maximum = maximum;
        self
    }

    pub fn step(mut self, value: f64) -> Self {
        assert!(
            value.is_finite() && value > 0.0,
            "Slider step must be finite and positive"
        );
        self.control.props.step = value;
        self
    }

    pub fn header(mut self, value: impl Into<String>) -> Self {
        self.control.props.header = Some(value.into());
        self
    }

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.control.props.orientation = orientation;
        self
    }
}

impl Slider {
    pub(crate) fn build_with_framework(mut self, framework: FrameworkProps) -> Element {
        validate_range_state(self.props.value, self.props.minimum, self.props.maximum);
        let mut framework = framework;
        if self.props.on_change.is_none() {
            enforce_display_only(&mut framework);
        }
        self.props.framework = framework;
        Element::new(ElementKind::Slider(self.props))
    }
}

pub struct NumberBox {
    props: NumberBoxProps,
}

impl NumberBox {
    pub fn new(
        value: impl Into<Option<f64>>,
        on_change: impl Fn(Option<f64>) + 'static,
    ) -> Framework<Self> {
        Framework::new(Self::with_handler(value.into(), Some(Rc::new(on_change))))
    }

    pub fn display(value: impl Into<Option<f64>>) -> Framework<Self> {
        Framework::new(Self::with_handler(value.into(), None))
    }

    fn with_handler(value: Option<f64>, on_change: Option<OptionalFloatEventFn>) -> Self {
        Self {
            props: NumberBoxProps {
                value,
                minimum: f64::MIN,
                maximum: f64::MAX,
                header: None,
                on_change,
                framework: FrameworkProps::default(),
            },
        }
    }

    pub(crate) fn build_with_framework(mut self, framework: FrameworkProps) -> Element {
        validate_optional_range_state(self.props.value, self.props.minimum, self.props.maximum);
        let mut framework = framework;
        if self.props.on_change.is_none() {
            enforce_display_only(&mut framework);
        }
        self.props.framework = framework;
        Element::new(ElementKind::NumberBox(self.props))
    }
}

pub struct RatingControl {
    props: RatingControlProps,
}

impl RatingControl {
    pub fn new(
        value: impl Into<Option<f64>>,
        on_change: impl Fn(Option<f64>) + 'static,
    ) -> Framework<Self> {
        Framework::new(Self::with_handler(value.into(), Some(Rc::new(on_change))))
    }

    pub fn display(value: impl Into<Option<f64>>) -> Framework<Self> {
        Framework::new(Self::with_handler(value.into(), None))
    }

    fn with_handler(value: Option<f64>, on_change: Option<OptionalFloatEventFn>) -> Self {
        Self {
            props: RatingControlProps {
                value,
                max_rating: 5,
                placeholder: None,
                caption: String::new(),
                read_only: false,
                on_change,
                framework: FrameworkProps::default(),
            },
        }
    }

    pub(crate) fn build_with_framework(mut self, framework: FrameworkProps) -> Element {
        validate_rating_state(
            self.props.value,
            self.props.placeholder,
            self.props.max_rating,
        );
        if self.props.on_change.is_none() {
            self.props.read_only = true;
        }
        self.props.framework = framework;
        Element::new(ElementKind::RatingControl(self.props))
    }
}

pub struct ColorPicker {
    props: ColorPickerProps,
}

impl ColorPicker {
    pub fn new(color: Color, on_change: impl Fn(Color) + 'static) -> Framework<Self> {
        Framework::new(Self::with_handler(color, Some(Rc::new(on_change))))
    }

    pub fn display(color: Color) -> Framework<Self> {
        Framework::new(Self::with_handler(color, None))
    }

    fn with_handler(color: Color, on_change: Option<ColorEventFn>) -> Self {
        Self {
            props: ColorPickerProps {
                color,
                alpha_enabled: true,
                hex_input_visible: true,
                color_slider_visible: true,
                color_channel_text_input_visible: true,
                on_change,
                framework: FrameworkProps::default(),
            },
        }
    }

    pub(crate) fn build_with_framework(mut self, framework: FrameworkProps) -> Element {
        let mut framework = framework;
        if self.props.on_change.is_none() {
            enforce_display_only(&mut framework);
        }
        self.props.framework = framework;
        Element::new(ElementKind::ColorPicker(self.props))
    }
}

pub struct DatePicker {
    props: DatePickerProps,
}

pub struct CalendarDatePicker {
    props: CalendarDatePickerProps,
}

pub struct TimePicker {
    props: TimePickerProps,
}

pub struct CalendarView {
    props: CalendarViewProps,
}

impl DatePicker {
    pub fn new(
        date: impl Into<Option<DateTime>>,
        on_change: impl Fn(Option<DateTime>) + 'static,
    ) -> Framework<Self> {
        Framework::new(Self::with_handler(date.into(), Some(Rc::new(on_change))))
    }

    pub fn display(date: impl Into<Option<DateTime>>) -> Framework<Self> {
        Framework::new(Self::with_handler(date.into(), None))
    }

    fn with_handler(date: Option<DateTime>, on_change: Option<OptionalDateEventFn>) -> Self {
        Self {
            props: DatePickerProps {
                date,
                header: None,
                day_visible: true,
                month_visible: true,
                year_visible: true,
                on_change,
                framework: FrameworkProps::default(),
            },
        }
    }

    pub(crate) fn build_with_framework(mut self, framework: FrameworkProps) -> Element {
        let mut framework = framework;
        if self.props.on_change.is_none() {
            enforce_display_only(&mut framework);
        }
        self.props.framework = framework;
        Element::new(ElementKind::DatePicker(self.props))
    }
}

impl CalendarDatePicker {
    pub fn new(
        date: impl Into<Option<DateTime>>,
        on_change: impl Fn(Option<DateTime>) + 'static,
    ) -> Framework<Self> {
        Framework::new(Self::with_handler(date.into(), Some(Rc::new(on_change))))
    }

    pub fn display(date: impl Into<Option<DateTime>>) -> Framework<Self> {
        Framework::new(Self::with_handler(date.into(), None))
    }

    fn with_handler(date: Option<DateTime>, on_change: Option<OptionalDateEventFn>) -> Self {
        Self {
            props: CalendarDatePickerProps {
                date,
                header: None,
                placeholder: None,
                today_highlighted: true,
                on_change,
                framework: FrameworkProps::default(),
            },
        }
    }

    pub(crate) fn build_with_framework(mut self, framework: FrameworkProps) -> Element {
        let mut framework = framework;
        if self.props.on_change.is_none() {
            enforce_display_only(&mut framework);
        }
        self.props.framework = framework;
        Element::new(ElementKind::CalendarDatePicker(self.props))
    }
}

impl TimePicker {
    pub fn new(
        time: impl Into<Option<TimeSpan>>,
        on_change: impl Fn(Option<TimeSpan>) + 'static,
    ) -> Framework<Self> {
        Framework::new(Self::with_handler(time.into(), Some(Rc::new(on_change))))
    }

    pub fn display(time: impl Into<Option<TimeSpan>>) -> Framework<Self> {
        Framework::new(Self::with_handler(time.into(), None))
    }

    fn with_handler(time: Option<TimeSpan>, on_change: Option<OptionalTimeEventFn>) -> Self {
        Self {
            props: TimePickerProps {
                time,
                header: None,
                minute_increment: 1,
                on_change,
                framework: FrameworkProps::default(),
            },
        }
    }

    pub(crate) fn build_with_framework(mut self, framework: FrameworkProps) -> Element {
        let mut framework = framework;
        if self.props.on_change.is_none() {
            enforce_display_only(&mut framework);
        }
        self.props.framework = framework;
        Element::new(ElementKind::TimePicker(self.props))
    }
}

impl CalendarView {
    pub fn new(
        selected_dates: impl IntoIterator<Item = DateTime>,
        on_change: impl Fn(Vec<DateTime>) + 'static,
    ) -> Framework<Self> {
        Framework::new(Self::with_handler(selected_dates, Some(Rc::new(on_change))))
    }

    pub fn display(selected_dates: impl IntoIterator<Item = DateTime>) -> Framework<Self> {
        Framework::new(Self::with_handler(selected_dates, None))
    }

    fn with_handler(
        selected_dates: impl IntoIterator<Item = DateTime>,
        on_change: Option<DatesEventFn>,
    ) -> Self {
        Self {
            props: CalendarViewProps {
                selected_dates: normalize_dates(selected_dates),
                selection_mode: CalendarSelectionMode::Single,
                today_highlighted: true,
                group_label_visible: false,
                on_change,
                framework: FrameworkProps::default(),
            },
        }
    }

    pub(crate) fn build_with_framework(mut self, framework: FrameworkProps) -> Element {
        validate_calendar_selection(self.props.selection_mode, &self.props.selected_dates);
        let mut framework = framework;
        if self.props.on_change.is_none() {
            enforce_display_only(&mut framework);
        }
        self.props.framework = framework;
        Element::new(ElementKind::CalendarView(self.props))
    }
}

fn normalize_dates(values: impl IntoIterator<Item = DateTime>) -> Rc<[DateTime]> {
    let mut values: Vec<_> = values.into_iter().collect();
    values.sort_unstable();
    values.dedup();
    values.into()
}

fn validate_calendar_selection(mode: CalendarSelectionMode, dates: &[DateTime]) {
    assert!(
        mode != CalendarSelectionMode::None || dates.is_empty(),
        "CalendarView selection must be empty in None mode"
    );
    assert!(
        mode != CalendarSelectionMode::Single || dates.len() <= 1,
        "CalendarView Single mode accepts at most one selected date"
    );
}

fn validate_range_state(value: f64, minimum: f64, maximum: f64) {
    validate_range_bounds(minimum, maximum);
    assert!(value.is_finite(), "range value and bounds must be finite");
    assert!(
        value >= minimum && value <= maximum,
        "value must be within the configured range"
    );
}

fn validate_optional_range_state(value: Option<f64>, minimum: f64, maximum: f64) {
    validate_range_bounds(minimum, maximum);
    if let Some(value) = value {
        assert!(value.is_finite(), "NumberBox value must be finite");
        assert!(
            value >= minimum && value <= maximum,
            "value must be within the configured range"
        );
    }
}

fn validate_range_bounds(minimum: f64, maximum: f64) {
    assert!(
        minimum.is_finite() && maximum.is_finite(),
        "range value and bounds must be finite"
    );
    assert!(minimum <= maximum, "range minimum must not exceed maximum");
}

fn validate_rating_state(value: Option<f64>, placeholder: Option<f64>, max_rating: i32) {
    assert!(max_rating > 0, "maximum rating must be positive");
    let maximum = f64::from(max_rating);
    for value in [value, placeholder].into_iter().flatten() {
        assert!(value.is_finite(), "rating values must be finite");
        assert!(
            value >= 0.0 && value <= maximum,
            "rating values must be within the configured maximum"
        );
    }
}

impl Framework<ProgressBar> {
    pub fn range(mut self, minimum: f64, maximum: f64) -> Self {
        self.control.props.minimum = minimum;
        self.control.props.maximum = maximum;
        self
    }

    pub fn is_indeterminate(mut self, indeterminate: bool) -> Self {
        self.control.props.indeterminate = indeterminate;
        self
    }
}

impl Framework<ProgressRing> {
    pub fn range(mut self, minimum: f64, maximum: f64) -> Self {
        self.control.props.minimum = minimum;
        self.control.props.maximum = maximum;
        self
    }

    pub fn active(mut self, active: bool) -> Self {
        self.control.props.active = active;
        self
    }

    pub fn is_indeterminate(mut self, indeterminate: bool) -> Self {
        self.control.props.indeterminate = indeterminate;
        self
    }
}

impl Framework<NumberBox> {
    pub fn range(mut self, minimum: f64, maximum: f64) -> Self {
        self.control.props.minimum = minimum;
        self.control.props.maximum = maximum;
        self
    }

    pub fn header(mut self, value: impl Into<String>) -> Self {
        self.control.props.header = Some(value.into());
        self
    }
}

impl Framework<RatingControl> {
    pub fn max_rating(mut self, max_rating: i32) -> Self {
        self.control.props.max_rating = max_rating;
        self
    }

    pub fn placeholder(mut self, value: impl Into<Option<f64>>) -> Self {
        self.control.props.placeholder = value.into();
        self
    }

    pub fn caption(mut self, caption: impl Into<String>) -> Self {
        self.control.props.caption = caption.into();
        self
    }

    pub fn read_only(mut self, read_only: bool) -> Self {
        self.control.props.read_only = read_only;
        self
    }
}

impl Framework<ColorPicker> {
    pub fn alpha_enabled(mut self, value: bool) -> Self {
        self.control.props.alpha_enabled = value;
        self
    }

    pub fn hex_input_visible(mut self, value: bool) -> Self {
        self.control.props.hex_input_visible = value;
        self
    }

    pub fn color_slider_visible(mut self, value: bool) -> Self {
        self.control.props.color_slider_visible = value;
        self
    }

    pub fn color_channel_text_input_visible(mut self, value: bool) -> Self {
        self.control.props.color_channel_text_input_visible = value;
        self
    }
}

impl Framework<DatePicker> {
    pub fn header(mut self, value: impl Into<String>) -> Self {
        self.control.props.header = Some(value.into());
        self
    }

    pub fn day_visible(mut self, value: bool) -> Self {
        self.control.props.day_visible = value;
        self
    }

    pub fn month_visible(mut self, value: bool) -> Self {
        self.control.props.month_visible = value;
        self
    }

    pub fn year_visible(mut self, value: bool) -> Self {
        self.control.props.year_visible = value;
        self
    }
}

impl Framework<CalendarDatePicker> {
    pub fn header(mut self, value: impl Into<String>) -> Self {
        self.control.props.header = Some(value.into());
        self
    }

    pub fn placeholder_text(mut self, value: impl Into<String>) -> Self {
        self.control.props.placeholder = Some(value.into());
        self
    }

    pub fn today_highlighted(mut self, value: bool) -> Self {
        self.control.props.today_highlighted = value;
        self
    }
}

impl Framework<TimePicker> {
    pub fn header(mut self, value: impl Into<String>) -> Self {
        self.control.props.header = Some(value.into());
        self
    }

    pub fn minute_increment(mut self, value: i32) -> Self {
        assert!(
            (1..=59).contains(&value),
            "minute increment must be between 1 and 59"
        );
        self.control.props.minute_increment = value;
        self
    }
}

impl Framework<CalendarView> {
    pub fn selection_mode(mut self, value: CalendarSelectionMode) -> Self {
        validate_calendar_selection(value, &self.control.props.selected_dates);
        self.control.props.selection_mode = value;
        self
    }

    pub fn today_highlighted(mut self, value: bool) -> Self {
        self.control.props.today_highlighted = value;
        self
    }

    pub fn group_label_visible(mut self, value: bool) -> Self {
        self.control.props.group_label_visible = value;
        self
    }
}
