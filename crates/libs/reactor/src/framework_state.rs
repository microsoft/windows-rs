use crate::element::{
    FontStretch, FontStyle, FontWeight, HorizontalAlignment, TextTrimming, TextWrapping, Thickness,
    VerticalAlignment, Visibility,
};

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct TextBlockStyleProps {
    pub text_wrapping: u8,
    pub text_trimming: u8,
    pub text_selection_enabled: u8,
}

#[derive(Clone, Copy)]
pub struct SizeProps {
    pub width: f64,
    pub height: f64,
    pub min_width: f64,
    pub max_width: f64,
    pub min_height: f64,
    pub max_height: f64,
}

#[derive(Clone, Copy, Default, PartialEq)]
pub struct LayoutProps {
    pub margin: Option<Thickness>,
    pub horizontal_alignment: Option<HorizontalAlignment>,
    pub vertical_alignment: Option<VerticalAlignment>,
}

#[derive(Clone, Copy)]
pub struct VisualProps {
    pub opacity: f32,
    pub visibility: Option<Visibility>,
}

#[derive(Clone, Copy)]
pub struct TextStyleProps {
    pub font_size: f32,
    pub character_spacing: Option<i32>,
    pub font_weight: u16,
    pub font_style: u8,
    pub font_stretch: u8,
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct ControlProps {
    pub enabled: Option<bool>,
}

impl VisualProps {
    pub fn opacity(self) -> Option<f32> {
        (!self.opacity.is_nan()).then_some(self.opacity)
    }

    pub fn visibility(self) -> Option<Visibility> {
        self.visibility
    }
}

impl Default for VisualProps {
    fn default() -> Self {
        Self {
            opacity: f32::NAN,
            visibility: None,
        }
    }
}

impl PartialEq for VisualProps {
    fn eq(&self, other: &Self) -> bool {
        self.opacity() == other.opacity() && self.visibility == other.visibility
    }
}

impl TextStyleProps {
    pub fn font_size(self) -> Option<f32> {
        (!self.font_size.is_nan()).then_some(self.font_size)
    }

    pub fn character_spacing(self) -> Option<i32> {
        self.character_spacing
    }

    pub fn font_weight(self) -> Option<FontWeight> {
        (self.font_weight != 0).then_some(FontWeight::from_raw(self.font_weight))
    }

    pub fn font_style(self) -> Option<FontStyle> {
        match self.font_style {
            0 => None,
            1 => Some(FontStyle::Normal),
            2 => Some(FontStyle::Oblique),
            3 => Some(FontStyle::Italic),
            _ => unreachable!(),
        }
    }

    pub fn font_stretch(self) -> Option<FontStretch> {
        match self.font_stretch {
            0 => None,
            1 => Some(FontStretch::Undefined),
            2 => Some(FontStretch::UltraCondensed),
            3 => Some(FontStretch::ExtraCondensed),
            4 => Some(FontStretch::Condensed),
            5 => Some(FontStretch::SemiCondensed),
            6 => Some(FontStretch::Normal),
            7 => Some(FontStretch::SemiExpanded),
            8 => Some(FontStretch::Expanded),
            9 => Some(FontStretch::ExtraExpanded),
            10 => Some(FontStretch::UltraExpanded),
            _ => unreachable!(),
        }
    }
}

impl Default for TextStyleProps {
    fn default() -> Self {
        Self {
            font_size: f32::NAN,
            character_spacing: None,
            font_weight: 0,
            font_style: 0,
            font_stretch: 0,
        }
    }
}

impl PartialEq for TextStyleProps {
    fn eq(&self, other: &Self) -> bool {
        self.font_size() == other.font_size()
            && self.character_spacing == other.character_spacing
            && self.font_weight == other.font_weight
            && self.font_style == other.font_style
            && self.font_stretch == other.font_stretch
    }
}

impl TextBlockStyleProps {
    pub fn text_wrapping(self) -> Option<TextWrapping> {
        match self.text_wrapping {
            0 => None,
            1 => Some(TextWrapping::NoWrap),
            2 => Some(TextWrapping::Wrap),
            3 => Some(TextWrapping::WrapWholeWords),
            _ => unreachable!(),
        }
    }

    pub fn text_trimming(self) -> Option<TextTrimming> {
        match self.text_trimming {
            0 => None,
            1 => Some(TextTrimming::None),
            2 => Some(TextTrimming::CharacterEllipsis),
            3 => Some(TextTrimming::WordEllipsis),
            4 => Some(TextTrimming::Clip),
            _ => unreachable!(),
        }
    }

    pub fn text_selection_enabled(self) -> Option<bool> {
        match self.text_selection_enabled {
            0 => None,
            1 => Some(false),
            2 => Some(true),
            _ => unreachable!(),
        }
    }
}

impl ControlProps {
    pub fn enabled(self) -> Option<bool> {
        self.enabled
    }
}

impl LayoutProps {
    pub fn margin(self) -> Option<Thickness> {
        self.margin
    }

    pub fn horizontal_alignment(self) -> Option<HorizontalAlignment> {
        self.horizontal_alignment
    }

    pub fn vertical_alignment(self) -> Option<VerticalAlignment> {
        self.vertical_alignment
    }
}

impl Default for SizeProps {
    fn default() -> Self {
        Self {
            width: f64::NAN,
            height: f64::NAN,
            min_width: f64::NAN,
            max_width: f64::NAN,
            min_height: f64::NAN,
            max_height: f64::NAN,
        }
    }
}

impl SizeProps {
    pub fn width(self) -> Option<f64> {
        value(self.width)
    }

    pub fn height(self) -> Option<f64> {
        value(self.height)
    }

    pub fn min_width(self) -> Option<f64> {
        value(self.min_width)
    }

    pub fn max_width(self) -> Option<f64> {
        value(self.max_width)
    }

    pub fn min_height(self) -> Option<f64> {
        value(self.min_height)
    }

    pub fn max_height(self) -> Option<f64> {
        value(self.max_height)
    }

    pub fn is_default(&self) -> bool {
        self.width.is_nan()
            && self.height.is_nan()
            && self.min_width.is_nan()
            && self.max_width.is_nan()
            && self.min_height.is_nan()
            && self.max_height.is_nan()
    }
}

impl PartialEq for SizeProps {
    fn eq(&self, other: &Self) -> bool {
        self.width() == other.width()
            && self.height() == other.height()
            && self.min_width() == other.min_width()
            && self.max_width() == other.max_width()
            && self.min_height() == other.min_height()
            && self.max_height() == other.max_height()
    }
}

fn value(value: f64) -> Option<f64> {
    (!value.is_nan()).then_some(value)
}
