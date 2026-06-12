use std::borrow::Cow;
use std::cell::Cell;

use super::*;

/// Effective color scheme reported by
/// [`crate::imp::RenderCx::use_color_scheme`].
#[derive(Copy, Clone, Debug, Default, Hash, PartialEq, Eq)]
pub enum ColorScheme {
    #[default]
    Light,
    Dark,
}

thread_local! {
    static CURRENT_COLOR_SCHEME: Cell<ColorScheme> = const { Cell::new(ColorScheme::Light) };
}

/// Read the host's last-known [`ColorScheme`] for the current UI thread.
pub fn current_color_scheme() -> ColorScheme {
    CURRENT_COLOR_SCHEME.with(|c| c.get())
}

/// Update the per-thread [`ColorScheme`]; called by the host on
/// `ActualThemeChanged` (and once during initial attach).
pub fn set_current_color_scheme(scheme: ColorScheme) {
    CURRENT_COLOR_SCHEME.with(|c| c.set(scheme));
}

/// Symbolic reference to a WinUI XAML theme resource (resolved at apply
/// time so the binding tracks light/dark switches).
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum ThemeRef {
    Accent,
    AccentSecondary,
    AccentTertiary,
    AccentDisabled,
    PrimaryText,
    SecondaryText,
    TertiaryText,
    DisabledText,
    AccentText,
    SolidBackground,
    CardBackground,
    SmokeFill,
    SubtleFill,
    LayerFill,
    ControlFill,
    ControlFillSecondary,
    ControlFillTertiary,
    ControlFillDisabled,
    ControlFillInputActive,
    CardStroke,
    SurfaceStroke,
    DividerStroke,
    ControlStroke,
    ControlStrokeSecondary,
    SystemAttention,
    SystemSuccess,
    SystemCaution,
    SystemCritical,
    SystemNeutral,
    SystemSolidNeutral,
    SystemAttentionBackground,
    SystemSuccessBackground,
    SystemCautionBackground,
    SystemCriticalBackground,
    SystemNeutralBackground,
    SystemSolidAttention,
    Custom(Cow<'static, str>),
}

impl ThemeRef {
    pub fn custom(key: impl Into<Cow<'static, str>>) -> Self {
        ThemeRef::Custom(key.into())
    }

    pub fn resource_key(&self) -> &str {
        match self {
            ThemeRef::Accent => "AccentFillColorDefaultBrush",
            ThemeRef::AccentSecondary => "AccentFillColorSecondaryBrush",
            ThemeRef::AccentTertiary => "AccentFillColorTertiaryBrush",
            ThemeRef::AccentDisabled => "AccentFillColorDisabledBrush",

            ThemeRef::PrimaryText => "TextFillColorPrimaryBrush",
            ThemeRef::SecondaryText => "TextFillColorSecondaryBrush",
            ThemeRef::TertiaryText => "TextFillColorTertiaryBrush",
            ThemeRef::DisabledText => "TextFillColorDisabledBrush",
            ThemeRef::AccentText => "AccentTextFillColorPrimaryBrush",

            ThemeRef::SolidBackground => "SolidBackgroundFillColorBaseBrush",
            ThemeRef::CardBackground => "CardBackgroundFillColorDefaultBrush",
            ThemeRef::SmokeFill => "SmokeFillColorDefaultBrush",
            ThemeRef::SubtleFill => "SubtleFillColorSecondaryBrush",
            ThemeRef::LayerFill => "LayerFillColorDefaultBrush",

            ThemeRef::ControlFill => "ControlFillColorDefaultBrush",
            ThemeRef::ControlFillSecondary => "ControlFillColorSecondaryBrush",
            ThemeRef::ControlFillTertiary => "ControlFillColorTertiaryBrush",
            ThemeRef::ControlFillDisabled => "ControlFillColorDisabledBrush",
            ThemeRef::ControlFillInputActive => "ControlFillColorInputActiveBrush",

            ThemeRef::CardStroke => "CardStrokeColorDefaultBrush",
            ThemeRef::SurfaceStroke => "SurfaceStrokeColorDefaultBrush",
            ThemeRef::DividerStroke => "DividerStrokeColorDefaultBrush",
            ThemeRef::ControlStroke => "ControlStrokeColorDefaultBrush",
            ThemeRef::ControlStrokeSecondary => "ControlStrokeColorSecondaryBrush",

            ThemeRef::SystemAttention => "SystemFillColorAttentionBrush",
            ThemeRef::SystemSuccess => "SystemFillColorSuccessBrush",
            ThemeRef::SystemCaution => "SystemFillColorCautionBrush",
            ThemeRef::SystemCritical => "SystemFillColorCriticalBrush",
            ThemeRef::SystemNeutral => "SystemFillColorNeutralBrush",
            ThemeRef::SystemSolidNeutral => "SystemFillColorSolidNeutralBrush",
            ThemeRef::SystemAttentionBackground => "SystemFillColorAttentionBackgroundBrush",
            ThemeRef::SystemSuccessBackground => "SystemFillColorSuccessBackgroundBrush",
            ThemeRef::SystemCautionBackground => "SystemFillColorCautionBackgroundBrush",
            ThemeRef::SystemCriticalBackground => "SystemFillColorCriticalBackgroundBrush",
            ThemeRef::SystemNeutralBackground => "SystemFillColorNeutralBackgroundBrush",
            ThemeRef::SystemSolidAttention => "SystemFillColorSolidAttentionBackgroundBrush",

            ThemeRef::Custom(s) => s.as_ref(),
        }
    }
}

/// Brush slot that can be either a literal [`Brush`]
/// or a [`ThemeRef`]; used for `background` / `foreground` modifiers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BrushBinding {
    Direct(Brush),
    Theme(ThemeRef),
}

impl From<Brush> for BrushBinding {
    fn from(v: Brush) -> Self {
        BrushBinding::Direct(v)
    }
}

impl From<Color> for BrushBinding {
    fn from(c: Color) -> Self {
        BrushBinding::Direct(Brush::Solid(c))
    }
}

impl From<ThemeRef> for BrushBinding {
    fn from(v: ThemeRef) -> Self {
        BrushBinding::Theme(v)
    }
}

#[allow(non_upper_case_globals)]
pub mod tokens {
    use super::ThemeRef;

    pub const Accent: ThemeRef = ThemeRef::Accent;

    pub const AccentSecondary: ThemeRef = ThemeRef::AccentSecondary;

    pub const AccentTertiary: ThemeRef = ThemeRef::AccentTertiary;

    pub const AccentDisabled: ThemeRef = ThemeRef::AccentDisabled;

    pub const PrimaryText: ThemeRef = ThemeRef::PrimaryText;

    pub const SecondaryText: ThemeRef = ThemeRef::SecondaryText;

    pub const TertiaryText: ThemeRef = ThemeRef::TertiaryText;

    pub const DisabledText: ThemeRef = ThemeRef::DisabledText;

    pub const AccentText: ThemeRef = ThemeRef::AccentText;

    pub const SolidBackground: ThemeRef = ThemeRef::SolidBackground;

    pub const CardBackground: ThemeRef = ThemeRef::CardBackground;

    pub const SmokeFill: ThemeRef = ThemeRef::SmokeFill;

    pub const SubtleFill: ThemeRef = ThemeRef::SubtleFill;

    pub const LayerFill: ThemeRef = ThemeRef::LayerFill;

    pub const ControlFill: ThemeRef = ThemeRef::ControlFill;

    pub const ControlFillSecondary: ThemeRef = ThemeRef::ControlFillSecondary;

    pub const ControlFillTertiary: ThemeRef = ThemeRef::ControlFillTertiary;

    pub const ControlFillDisabled: ThemeRef = ThemeRef::ControlFillDisabled;

    pub const ControlFillInputActive: ThemeRef = ThemeRef::ControlFillInputActive;

    pub const CardStroke: ThemeRef = ThemeRef::CardStroke;

    pub const SurfaceStroke: ThemeRef = ThemeRef::SurfaceStroke;

    pub const DividerStroke: ThemeRef = ThemeRef::DividerStroke;

    pub const ControlStroke: ThemeRef = ThemeRef::ControlStroke;

    pub const ControlStrokeSecondary: ThemeRef = ThemeRef::ControlStrokeSecondary;

    pub const SystemAttention: ThemeRef = ThemeRef::SystemAttention;

    pub const SystemSuccess: ThemeRef = ThemeRef::SystemSuccess;

    pub const SystemCaution: ThemeRef = ThemeRef::SystemCaution;

    pub const SystemCritical: ThemeRef = ThemeRef::SystemCritical;

    pub const SystemNeutral: ThemeRef = ThemeRef::SystemNeutral;

    pub const SystemSolidNeutral: ThemeRef = ThemeRef::SystemSolidNeutral;
}
