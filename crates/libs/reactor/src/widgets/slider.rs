use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Slider {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub value: f64,
    pub minimum: f64,
    pub maximum: f64,
    pub step: Option<f64>,
    pub on_value_changed: Option<Callback<f64>>,
    pub header: Option<String>,
    pub orientation: Orientation,
    pub is_enabled: bool,
}
impl Default for Slider {
    fn default() -> Self {
        Self {
            key: None,
            modifiers: Modifiers::default(),
            value: 0.0,
            minimum: 0.0,
            maximum: 100.0,
            step: None,
            on_value_changed: None,
            header: None,
            orientation: Orientation::Horizontal,
            is_enabled: true,
        }
    }
}
impl Slider {
    pub fn new(value: f64) -> Self {
        Self {
            value,
            ..Default::default()
        }
    }
    pub fn range(mut self, min: f64, max: f64) -> Self {
        self.minimum = min;
        self.maximum = max;
        self
    }
    pub fn step(mut self, v: f64) -> Self {
        self.step = Some(v);
        self
    }
    pub fn on_value_changed(mut self, f: impl IntoCallback<f64>) -> Self {
        self.on_value_changed = Some(f.into_callback());
        self
    }
    pub fn header(mut self, s: impl Into<String>) -> Self {
        self.header = Some(s.into());
        self
    }
    /// Switch to a vertical slider (`ISlider::Orientation = Vertical`).
    pub fn vertical(mut self) -> Self {
        self.orientation = Orientation::Vertical;
        self
    }
    /// Switch to a horizontal slider (default).
    pub fn horizontal(mut self) -> Self {
        self.orientation = Orientation::Horizontal;
        self
    }
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.is_enabled = enabled;
        self
    }
}

impl Widget for Slider {
    widget_header!(ControlKind::Slider);
    fn bindings(&self) -> PropBindings {
        let mut out = generated::slider_bindings(self);
        if let Some(v) = self.step {
            out.push(Binding::Prop(Prop::Step, PropValue::F64(v)));
        }
        out
    }
}
