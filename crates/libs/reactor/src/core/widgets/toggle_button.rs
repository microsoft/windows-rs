use super::*;

/// `Microsoft.UI.Xaml.Controls.Primitives.ToggleButton`. A button that
/// toggles between checked and unchecked states.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct ToggleButtonWidget {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub label: String,
    pub is_checked: bool,
    pub on_changed: Option<Callback<bool>>,
    pub is_enabled: bool,
}

impl ToggleButtonWidget {
    pub fn new(label: impl Into<String>, is_checked: bool) -> Self {
        Self {
            label: label.into(),
            is_checked,
            is_enabled: true,
            ..Default::default()
        }
    }

    pub fn on_changed(mut self, f: impl IntoCallback<bool>) -> Self {
        self.on_changed = Some(f.into_callback());
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.is_enabled = enabled;
        self
    }
}

impl Widget for ToggleButtonWidget {
    widget_header!(ControlKind::ToggleButton);
    fn bindings(&self) -> PropBindings {
        let mut out = Vec::with_capacity(4);
        out.push(Binding::Prop(
            Prop::IsChecked,
            PropValue::Bool(self.is_checked),
        ));
        out.push(Binding::Prop(
            Prop::CheckBoxLabel,
            PropValue::Str(self.label.clone()),
        ));
        if !self.is_enabled {
            out.push(Binding::Prop(Prop::IsEnabled, PropValue::Bool(false)));
        }
        out.push(Binding::Event(
            Event::CheckedChanged,
            self.on_changed
                .as_ref()
                .map(|cb| EventHandler::CheckedChanged(cb.clone())),
        ));
        out
    }
}

/// Creates a [`ToggleButtonWidget`] with a label and initial checked state.
pub fn toggle_button(label: impl Into<String>, is_checked: bool) -> ToggleButtonWidget {
    ToggleButtonWidget::new(label, is_checked)
}
