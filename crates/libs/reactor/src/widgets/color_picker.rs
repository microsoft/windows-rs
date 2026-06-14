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
pub struct ColorPicker {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub color: ColorArgb,
    pub is_alpha_enabled: bool,
    pub is_hex_input_visible: bool,
    pub is_color_slider_visible: bool,
    pub is_color_channel_text_input_visible: bool,
    pub on_color_changed: Option<Callback<(u8, u8, u8, u8)>>,
}

impl ColorPicker {
    pub fn new(color: ColorArgb) -> Self {
        Self {
            color,
            is_alpha_enabled: true,
            is_hex_input_visible: true,
            is_color_slider_visible: true,
            is_color_channel_text_input_visible: true,
            ..Default::default()
        }
    }

    pub fn alpha_enabled(mut self, v: bool) -> Self {
        self.is_alpha_enabled = v;
        self
    }

    pub fn hex_input_visible(mut self, v: bool) -> Self {
        self.is_hex_input_visible = v;
        self
    }

    pub fn color_slider_visible(mut self, v: bool) -> Self {
        self.is_color_slider_visible = v;
        self
    }

    pub fn color_channel_text_input_visible(mut self, v: bool) -> Self {
        self.is_color_channel_text_input_visible = v;
        self
    }

    pub fn on_color_changed(mut self, f: impl IntoCallback<(u8, u8, u8, u8)>) -> Self {
        self.on_color_changed = Some(f.into_callback());
        self
    }
}

impl Widget for ColorPicker {
    widget_header!(ControlKind::ColorPicker);
    fn bindings(&self) -> PropBindings {
        let mut out = generated::color_picker_bindings(self);
        // ColorValue is a compound ARGB type not expressible in TOML.
        out.push(Binding::Prop(
            Prop::ColorValue,
            PropValue::Color(Color {
                a: self.color.a,
                r: self.color.r,
                g: self.color.g,
                b: self.color.b,
            }),
        ));
        out
    }
}

pub fn color_picker(color: ColorArgb) -> ColorPicker {
    ColorPicker::new(color)
}
