use super::*;

/// Preferred placement for a [`TeachingTipWidget`].
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum TeachingTipPlacement {
    #[default]
    Auto,
    Top,
    Bottom,
    Left,
    Right,
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
    LeftTop,
    LeftBottom,
    RightTop,
    RightBottom,
    Center,
}

/// `Microsoft.UI.Xaml.Controls.TeachingTip`. A contextual teaching popup.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct TeachingTipWidget {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub title: String,
    pub subtitle: Option<String>,
    pub is_open: bool,
    pub is_light_dismiss_enabled: bool,
    pub preferred_placement: TeachingTipPlacement,
    pub action_button_text: Option<String>,
    pub close_button_text: Option<String>,
    pub on_closed: Option<Callback<()>>,
    pub on_action_click: Option<Callback<()>>,
}

impl TeachingTipWidget {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..Default::default()
        }
    }

    pub fn subtitle(mut self, s: impl Into<String>) -> Self {
        self.subtitle = Some(s.into());
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

    pub fn preferred_placement(mut self, p: TeachingTipPlacement) -> Self {
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

    pub fn on_closed<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.on_closed = Some(Callback::new(move |()| f()));
        self
    }

    pub fn on_action_click<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.on_action_click = Some(Callback::new(move |()| f()));
        self
    }
}

impl Widget for TeachingTipWidget {
    widget_header!(ControlKind::TeachingTip);
    fn bindings(&self) -> PropBindings {
        let mut out = Vec::with_capacity(8);
        out.push(Binding::Prop(
            Prop::TeachingTipTitle,
            PropValue::Str(self.title.clone()),
        ));
        if let Some(sub) = &self.subtitle {
            out.push(Binding::Prop(
                Prop::TeachingTipSubtitle,
                PropValue::Str(sub.clone()),
            ));
        }
        out.push(Binding::Prop(
            Prop::TeachingTipIsOpen,
            PropValue::Bool(self.is_open),
        ));
        if self.is_light_dismiss_enabled {
            out.push(Binding::Prop(
                Prop::TeachingTipIsLightDismiss,
                PropValue::Bool(true),
            ));
        }
        out.push(Binding::Prop(
            Prop::TeachingTipPlacement,
            PropValue::TeachingTipPlacement(self.preferred_placement),
        ));
        if let Some(text) = &self.action_button_text {
            out.push(Binding::Prop(
                Prop::TeachingTipActionButton,
                PropValue::Str(text.clone()),
            ));
        }
        if let Some(text) = &self.close_button_text {
            out.push(Binding::Prop(
                Prop::TeachingTipCloseButton,
                PropValue::Str(text.clone()),
            ));
        }
        out.push(Binding::Event(
            Event::TeachingTipClosed,
            self.on_closed
                .as_ref()
                .map(|cb| EventHandler::Click(cb.clone())),
        ));
        out.push(Binding::Event(
            Event::TeachingTipActionClick,
            self.on_action_click
                .as_ref()
                .map(|cb| EventHandler::Click(cb.clone())),
        ));
        out
    }
}

pub fn teaching_tip(title: impl Into<String>) -> TeachingTipWidget {
    TeachingTipWidget::new(title)
}
