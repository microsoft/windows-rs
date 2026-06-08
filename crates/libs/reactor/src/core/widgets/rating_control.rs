use super::*;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct RatingControl {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub value: f64,
    pub max_rating: Option<i32>,
    pub caption: Option<String>,
    pub placeholder_value: Option<f64>,
    pub is_read_only: bool,
    pub on_value_changed: Option<Callback<f64>>,
}

impl RatingControl {
    pub fn new(value: f64) -> Self {
        Self {
            value,
            ..Default::default()
        }
    }

    pub fn max_rating(mut self, max: i32) -> Self {
        self.max_rating = Some(max);
        self
    }

    pub fn caption(mut self, s: impl Into<String>) -> Self {
        self.caption = Some(s.into());
        self
    }

    pub fn placeholder_value(mut self, v: f64) -> Self {
        self.placeholder_value = Some(v);
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
        crate::core::generated_bindings::rating_control_bindings(self)
    }
}

pub fn rating_control(value: f64) -> RatingControl {
    RatingControl::new(value)
}
