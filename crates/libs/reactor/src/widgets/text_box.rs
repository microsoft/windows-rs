use super::*;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct TextBox {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub value: String,
    pub on_text_changed: Option<Callback<String>>,
    pub placeholder_text: String,
    pub header: Option<String>,
    pub is_enabled: bool,
    pub accepts_return: bool,
    pub text_wrapping: TextWrapping,
    pub border_brush: Option<BrushBinding>,
    pub border_thickness: Option<Thickness>,
}
impl TextBox {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            is_enabled: true,
            ..Default::default()
        }
    }
}

impl Widget for TextBox {
    widget_header!(ControlKind::TextBox);
    fn bindings(&self) -> PropBindings {
        let mut out = generated::text_box_bindings(self);
        out.push(Binding::Prop(
            Prop::Value,
            PropValue::Str(self.value.clone()),
        ));
        if let Some(BrushBinding::Direct(br)) = &self.border_brush {
            out.push(Binding::Prop(Prop::BorderBrush, PropValue::Color(*br)));
        }
        if let Some(v) = self.border_thickness {
            out.push(Binding::Prop(
                Prop::BorderThickness,
                PropValue::Thickness(v),
            ));
        }
        out
    }
}

impl TextBox {
    pub fn on_text_changed(mut self, f: impl IntoCallback<String>) -> Self {
        self.on_text_changed = Some(f.into_callback());
        self
    }

    pub fn placeholder_text(mut self, s: impl Into<String>) -> Self {
        self.placeholder_text = s.into();
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

    pub fn accepts_return(mut self, v: bool) -> Self {
        self.accepts_return = v;
        self
    }

    pub fn text_wrapping(mut self, v: TextWrapping) -> Self {
        self.text_wrapping = v;
        self
    }

    pub fn multiline(mut self) -> Self {
        self.accepts_return = true;
        self.text_wrapping = TextWrapping::Wrap;
        self
    }

    /// Brush used to paint the border stroke. Maps to WinUI
    /// `Control.BorderBrush`. Accepts a direct [`Color`] or a [`ThemeRef`].
    pub fn border_brush(mut self, v: impl Into<BrushBinding>) -> Self {
        apply_widget_brush_binding(
            &mut self.border_brush,
            &mut self.modifiers,
            Prop::BorderBrush,
            v.into(),
        );
        self
    }

    /// Border stroke thickness, in DIPs, on each side. Maps to WinUI
    /// `Control.BorderThickness`.
    pub fn border_thickness(mut self, v: Thickness) -> Self {
        self.border_thickness = Some(v);
        self
    }
}

pub fn text_box(value: impl Into<String>) -> TextBox {
    TextBox::new(value)
}
