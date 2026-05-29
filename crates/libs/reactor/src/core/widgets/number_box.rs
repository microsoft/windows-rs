use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct NumberBox {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub value: f64,
    pub minimum: Option<f64>,
    pub maximum: Option<f64>,
    pub on_value_changed: Option<Callback<f64>>,
    pub header: Option<String>,
    pub is_enabled: bool,
}
impl Default for NumberBox {
    fn default() -> Self {
        Self {
            key: None,
            modifiers: Modifiers::default(),
            value: 0.0,
            minimum: None,
            maximum: None,
            on_value_changed: None,
            header: None,
            is_enabled: true,
        }
    }
}
impl NumberBox {
    pub fn new(value: f64) -> Self {
        Self {
            value,
            ..Default::default()
        }
    }
    pub fn range(mut self, min: f64, max: f64) -> Self {
        self.minimum = Some(min);
        self.maximum = Some(max);
        self
    }
    pub fn on_value_changed<F: Fn(f64) + 'static>(mut self, f: F) -> Self {
        self.on_value_changed = Some(Callback::new(f));
        self
    }
    pub fn header(mut self, s: impl Into<String>) -> Self {
        self.header = Some(s.into());
        self
    }
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.is_enabled = enabled;
        self
    }
}

impl Widget for NumberBox {
    widget_header!(ControlKind::NumberBox);
    fn bindings(&self) -> PropBindings {
        let mut out = Vec::with_capacity(6);
        out.push(Binding::Prop(
            Prop::NumericValue,
            PropValue::F64(self.value),
        ));
        if let Some(min) = self.minimum {
            out.push(Binding::Prop(Prop::Minimum, PropValue::F64(min)));
        }
        if let Some(max) = self.maximum {
            out.push(Binding::Prop(Prop::Maximum, PropValue::F64(max)));
        }
        if let Some(h) = &self.header {
            out.push(Binding::Prop(Prop::Header, PropValue::Str(h.clone())));
        }
        if !self.is_enabled {
            out.push(Binding::Prop(Prop::IsEnabled, PropValue::Bool(false)));
        }
        out.push(Binding::Event(
            Event::ValueChanged,
            self.on_value_changed
                .as_ref()
                .map(|cb| EventHandler::ValueChanged(cb.clone())),
        ));
        out
    }
}
