use super::*;

/// `Microsoft.UI.Xaml.Controls.TeachingTip`. A contextual teaching popup.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct TeachingTip {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub title: String,
    pub subtitle: String,
    pub is_open: bool,
    pub is_light_dismiss_enabled: bool,
    pub preferred_placement: TeachingTipPlacementMode,
    pub action_button_text: Option<String>,
    pub close_button_text: Option<String>,
    pub on_closed: Option<Callback<()>>,
    pub on_action_button_click: Option<Callback<()>>,
}

impl TeachingTip {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..Default::default()
        }
    }

    pub fn subtitle(mut self, s: impl Into<String>) -> Self {
        self.subtitle = s.into();
        self
    }

    pub fn is_open(mut self, v: bool) -> Self {
        self.is_open = v;
        self
    }

    pub fn light_dismiss(mut self) -> Self {
        self.is_light_dismiss_enabled = true;
        self
    }

    pub fn preferred_placement(mut self, p: TeachingTipPlacementMode) -> Self {
        self.preferred_placement = p;
        self
    }

    pub fn action_button(mut self, text: impl Into<String>) -> Self {
        self.action_button_text = Some(text.into());
        self
    }

    pub fn close_button(mut self, text: impl Into<String>) -> Self {
        self.close_button_text = Some(text.into());
        self
    }

    pub fn on_closed(mut self, f: impl IntoUnitCallback) -> Self {
        self.on_closed = Some(f.into_unit_callback());
        self
    }

    pub fn on_action_button_click(mut self, f: impl IntoUnitCallback) -> Self {
        self.on_action_button_click = Some(f.into_unit_callback());
        self
    }
}

impl Widget for TeachingTip {
    widget_header!(ControlKind::TeachingTip);
    fn bindings(&self) -> PropBindings {
        let mut out = generated::teaching_tip_bindings(self);
        if let Some(v) = &self.action_button_text {
            out.push(Binding::Prop(
                Prop::ActionButtonText,
                PropValue::Str(v.clone()),
            ));
        }
        if let Some(v) = &self.close_button_text {
            out.push(Binding::Prop(
                Prop::CloseButtonText,
                PropValue::Str(v.clone()),
            ));
        }
        out
    }
}

pub fn teaching_tip(title: impl Into<String>) -> TeachingTip {
    TeachingTip::new(title)
}
