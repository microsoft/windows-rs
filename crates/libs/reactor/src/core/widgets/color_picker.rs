use super::*;

/// ARGB color value for the `ColorPicker` widget.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ColorArgb {
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl ColorArgb {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { a: 255, r, g, b }
    }

    pub fn with_alpha(a: u8, r: u8, g: u8, b: u8) -> Self {
        Self { a, r, g, b }
    }
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct ColorPickerWidget {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub color: ColorArgb,
    pub is_alpha_enabled: Option<bool>,
    pub is_hex_input_visible: Option<bool>,
    pub is_color_slider_visible: Option<bool>,
    pub is_color_channel_text_input_visible: Option<bool>,
    pub on_changed: Option<Callback<(u8, u8, u8, u8)>>,
}

impl ColorPickerWidget {
    pub fn new(color: ColorArgb) -> Self {
        Self {
            color,
            ..Default::default()
        }
    }

    pub fn alpha_enabled(mut self, v: bool) -> Self {
        self.is_alpha_enabled = Some(v);
        self
    }

    pub fn hex_input_visible(mut self, v: bool) -> Self {
        self.is_hex_input_visible = Some(v);
        self
    }

    pub fn color_slider_visible(mut self, v: bool) -> Self {
        self.is_color_slider_visible = Some(v);
        self
    }

    pub fn color_channel_text_input_visible(mut self, v: bool) -> Self {
        self.is_color_channel_text_input_visible = Some(v);
        self
    }

    pub fn on_changed(mut self, f: impl IntoCallback<(u8, u8, u8, u8)>) -> Self {
        self.on_changed = Some(f.into_callback());
        self
    }
}

impl Widget for ColorPickerWidget {
    widget_header!(ControlKind::ColorPicker);
    fn bindings(&self) -> PropBindings {
        let mut out = Vec::with_capacity(6);
        out.push(Binding::Prop(
            Prop::ColorValue,
            PropValue::Color {
                a: self.color.a,
                r: self.color.r,
                g: self.color.g,
                b: self.color.b,
            },
        ));
        if let Some(v) = self.is_alpha_enabled {
            out.push(Binding::Prop(Prop::IsAlphaEnabled, PropValue::Bool(v)));
        }
        if let Some(v) = self.is_hex_input_visible {
            out.push(Binding::Prop(Prop::IsHexInputVisible, PropValue::Bool(v)));
        }
        if let Some(v) = self.is_color_slider_visible {
            out.push(Binding::Prop(
                Prop::IsColorSliderVisible,
                PropValue::Bool(v),
            ));
        }
        if let Some(v) = self.is_color_channel_text_input_visible {
            out.push(Binding::Prop(
                Prop::IsColorChannelTextInputVisible,
                PropValue::Bool(v),
            ));
        }
        out.push(Binding::Event(
            Event::ColorChanged,
            self.on_changed
                .as_ref()
                .map(|cb| EventHandler::ColorChanged(cb.clone())),
        ));
        out
    }
}

pub fn color_picker(color: ColorArgb) -> ColorPickerWidget {
    ColorPickerWidget::new(color)
}
