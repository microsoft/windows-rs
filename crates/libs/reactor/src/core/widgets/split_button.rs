use super::*;

/// `Microsoft.UI.Xaml.Controls.SplitButton`. A button with a primary
/// action and a secondary dropdown action.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct SplitButtonWidget {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub content: Option<String>,
    pub is_enabled: bool,
    pub on_click: Option<Callback<()>>,
}

impl SplitButtonWidget {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: Some(content.into()),
            is_enabled: true,
            ..Default::default()
        }
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.is_enabled = enabled;
        self
    }

    pub fn on_click<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.on_click = Some(Callback::new(move |()| f()));
        self
    }
}

impl Widget for SplitButtonWidget {
    widget_header!(ControlKind::SplitButton);
    fn bindings(&self) -> PropBindings {
        let mut out = Vec::with_capacity(3);
        if let Some(s) = &self.content {
            out.push(Binding::Prop(
                Prop::ButtonContent,
                PropValue::Str(s.clone()),
            ));
        }
        if !self.is_enabled {
            out.push(Binding::Prop(Prop::IsEnabled, PropValue::Bool(false)));
        }
        out.push(Binding::Event(
            Event::SplitButtonClick,
            self.on_click
                .as_ref()
                .map(|cb| EventHandler::Click(cb.clone())),
        ));
        out
    }
}

pub fn split_button(content: impl Into<String>) -> SplitButtonWidget {
    SplitButtonWidget::new(content)
}
