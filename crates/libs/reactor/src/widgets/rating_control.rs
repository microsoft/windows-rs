use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct RatingControl {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub value: f64,
    pub max_rating: i32,
    pub caption: String,
    pub placeholder_value: f64,
    pub is_read_only: bool,
    pub on_value_changed: Option<Callback<f64>>,
}

impl Default for RatingControl {
    fn default() -> Self {
        Self {
            key: None,
            modifiers: Modifiers::default(),
            value: 0.0,
            max_rating: 5,
            caption: String::new(),
            placeholder_value: -1.0,
            is_read_only: false,
            on_value_changed: None,
        }
    }
}

impl RatingControl {
    pub fn new(value: f64) -> Self {
        Self {
            value,
            ..Default::default()
        }
    }

    pub fn max_rating(mut self, max: i32) -> Self {
        self.max_rating = max;
        self
    }

    pub fn caption(mut self, s: impl Into<String>) -> Self {
        self.caption = s.into();
        self
    }

    pub fn placeholder_value(mut self, v: f64) -> Self {
        self.placeholder_value = v;
        self
    }

    pub fn read_only(mut self) -> Self {
        self.is_read_only = true;
        self
    }

    pub fn on_value_changed(mut self, f: impl IntoCallback<f64>) -> Self {
        self.on_value_changed = Some(f.into_callback());
        self
    }
}

impl Widget for RatingControl {
    widget_header!(ControlKind::RatingControl);
    fn bindings(&self) -> PropBindings {
        generated::rating_control_bindings(self)
    }
}

pub fn rating_control(value: f64) -> RatingControl {
    RatingControl::new(value)
}
