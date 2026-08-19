use std::borrow::Cow;
use std::collections::BTreeSet;
use std::rc::Rc;
use std::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShapeKind {
    Rectangle,
    Ellipse,
    Line,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TooltipPlacement {
    Top,
    Bottom,
    Left,
    Right,
    Mouse,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CalendarSelectionMode {
    None,
    Single,
    Multiple,
}

impl ShapeKind {
    pub(crate) const fn native_kind(self) -> crate::runtime::NativeKind {
        match self {
            Self::Rectangle => crate::runtime::NativeKind::Rectangle,
            Self::Ellipse => crate::runtime::NativeKind::Ellipse,
            Self::Line => crate::runtime::NativeKind::Line,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
pub enum TextWrapping {
    #[default]
    NoWrap = 1,
    Wrap = 2,
    WrapWholeWords = 3,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
pub enum PasswordRevealMode {
    #[default]
    Peek,
    Hidden,
    Visible,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
pub enum TextTrimming {
    #[default]
    None,
    CharacterEllipsis,
    WordEllipsis,
    Clip,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
pub enum Stretch {
    None,
    Fill,
    #[default]
    Uniform,
    UniformToFill,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ImageSource(ImageSourceKind);

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) enum ImageSourceKind {
    #[default]
    None,
    Bitmap(Rc<str>),
    Svg(Rc<str>),
}

impl ImageSource {
    pub const fn none() -> Self {
        Self(ImageSourceKind::None)
    }

    pub fn bitmap(uri: impl Into<Rc<str>>) -> Self {
        Self(ImageSourceKind::Bitmap(uri.into()))
    }

    pub fn svg(uri: impl Into<Rc<str>>) -> Self {
        Self(ImageSourceKind::Svg(uri.into()))
    }

    pub(crate) fn kind(&self) -> &ImageSourceKind {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IconSymbol(i32);

impl IconSymbol {
    pub const PREVIOUS: Self = Self(57600);
    pub const NEXT: Self = Self(57601);
    pub const PLAY: Self = Self(57602);
    pub const PAUSE: Self = Self(57603);
    pub const EDIT: Self = Self(57604);
    pub const SAVE: Self = Self(57605);
    pub const CLEAR: Self = Self(57606);
    pub const DELETE: Self = Self(57607);
    pub const REMOVE: Self = Self(57608);
    pub const ADD: Self = Self(57609);
    pub const CANCEL: Self = Self(57610);
    pub const ACCEPT: Self = Self(57611);
    pub const MORE: Self = Self(57612);
    pub const HOME: Self = Self(57615);
    pub const BACK: Self = Self(57618);
    pub const FAVORITE: Self = Self(57619);
    pub const SETTINGS: Self = Self(57621);
    pub const DOWNLOAD: Self = Self(57624);
    pub const MAIL: Self = Self(57625);
    pub const FIND: Self = Self(57626);
    pub const PEOPLE: Self = Self(57637);
    pub const WORLD: Self = Self(57640);
    pub const DOCUMENT: Self = Self(57648);
    pub const REFRESH: Self = Self(57673);

    pub const fn value(self) -> i32 {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Icon(IconKind);

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum IconKind {
    Symbol(IconSymbol),
    Font { glyph: Rc<str>, family: Rc<str> },
    Bitmap { uri: Rc<str>, monochrome: bool },
    Image(ImageSource),
    Path(Rc<str>),
}

impl Icon {
    pub const fn symbol(symbol: IconSymbol) -> Self {
        Self(IconKind::Symbol(symbol))
    }

    pub fn font(glyph: impl Into<Rc<str>>, family: impl Into<Rc<str>>) -> Self {
        Self(IconKind::Font {
            glyph: glyph.into(),
            family: family.into(),
        })
    }

    pub fn bitmap(uri: impl Into<Rc<str>>, monochrome: bool) -> Self {
        Self(IconKind::Bitmap {
            uri: uri.into(),
            monochrome,
        })
    }

    pub fn image(source: ImageSource) -> Self {
        Self(IconKind::Image(source))
    }

    pub fn path(data: impl Into<Rc<str>>) -> Self {
        Self(IconKind::Path(data.into()))
    }

    pub(crate) fn kind(&self) -> &IconKind {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum NavigationPaneDisplayMode {
    #[default]
    Auto,
    Left,
    Top,
    LeftCompact,
    LeftMinimal,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum NavigationDisplayMode {
    Minimal,
    Compact,
    #[default]
    Expanded,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
pub enum ScrollBarVisibility {
    #[default]
    Disabled,
    Auto,
    Hidden,
    Visible,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
pub enum ScrollViewBarVisibility {
    #[default]
    Auto,
    Visible,
    Hidden,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
pub enum ScrollOrientation {
    #[default]
    Vertical,
    Horizontal,
    None,
    Both,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
pub enum SelectionMode {
    None,
    #[default]
    Single,
    Multiple,
    Extended,
}

#[derive(Clone, Debug)]
pub struct VirtualItemKeys(Rc<[u64]>);

impl VirtualItemKeys {
    pub fn new(keys: impl IntoIterator<Item = u64>) -> Self {
        let keys = keys.into_iter().collect::<Vec<_>>();
        assert!(
            keys.iter().copied().collect::<BTreeSet<_>>().len() == keys.len(),
            "VirtualList item keys must be unique"
        );
        Self(keys.into())
    }

    pub(crate) fn values(&self) -> Rc<[u64]> {
        Rc::clone(&self.0)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn as_slice(&self) -> &[u64] {
        &self.0
    }
}

impl PartialEq for VirtualItemKeys {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0) || self.0 == other.0
    }
}

impl Eq for VirtualItemKeys {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CollectionSelection(Rc<[u64]>);

impl CollectionSelection {
    pub fn new(keys: impl IntoIterator<Item = u64>) -> Self {
        let mut keys = keys.into_iter().collect::<Vec<_>>();
        keys.sort_unstable();
        keys.dedup();
        Self(keys.into())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn as_slice(&self) -> &[u64] {
        &self.0
    }
}

impl Default for CollectionSelection {
    fn default() -> Self {
        Self(Rc::from([]))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelectorItem {
    key: u64,
    text: String,
}

impl SelectorItem {
    pub fn new(key: u64, text: impl Into<String>) -> Self {
        Self {
            key,
            text: text.into(),
        }
    }

    pub fn key(&self) -> u64 {
        self.key
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

impl<T: Into<String>> From<(u64, T)> for SelectorItem {
    fn from((key, text): (u64, T)) -> Self {
        Self::new(key, text)
    }
}

#[derive(Clone, Debug)]
pub struct SelectorItems(Rc<[SelectorItem]>);

impl SelectorItems {
    pub fn new<T: Into<SelectorItem>>(items: impl IntoIterator<Item = T>) -> Self {
        let items = items.into_iter().map(Into::into).collect::<Vec<_>>();
        assert!(
            items
                .iter()
                .map(SelectorItem::key)
                .collect::<BTreeSet<_>>()
                .len()
                == items.len(),
            "selector item keys must be unique"
        );
        Self(items.into())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn as_slice(&self) -> &[SelectorItem] {
        &self.0
    }

    pub(crate) fn values(&self) -> Rc<[SelectorItem]> {
        Rc::clone(&self.0)
    }
}

impl PartialEq for SelectorItems {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0) || self.0 == other.0
    }
}

impl Eq for SelectorItems {}

pub type ListBoxItem = SelectorItem;
pub type ListBoxItems = SelectorItems;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
pub enum ScrollActivity {
    #[default]
    Idle,
    Interaction,
    Inertia,
    Animation,
    Intermediate,
    Unknown,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
pub enum SplitViewDisplayMode {
    Overlay,
    #[default]
    Inline,
    CompactOverlay,
    CompactInline,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ScrollEvent {
    pub horizontal_offset: f64,
    pub vertical_offset: f64,
    pub zoom_factor: f32,
    pub activity: ScrollActivity,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonEmphasis {
    #[default]
    Standard,
    Accent,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Color {
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Brush {
    Solid(Color),
    Theme(ThemeBrush),
}

impl From<Color> for Brush {
    fn from(value: Color) -> Self {
        Self::Solid(value)
    }
}

impl From<ThemeBrush> for Brush {
    fn from(value: ThemeBrush) -> Self {
        Self::Theme(value)
    }
}

pub trait IntoBrushOption {
    fn into_brush_option(self) -> Option<Brush>;
}

impl IntoBrushOption for Color {
    fn into_brush_option(self) -> Option<Brush> {
        Some(self.into())
    }
}

impl IntoBrushOption for Brush {
    fn into_brush_option(self) -> Option<Brush> {
        Some(self)
    }
}

impl IntoBrushOption for ThemeBrush {
    fn into_brush_option(self) -> Option<Brush> {
        Some(self.into())
    }
}

impl IntoBrushOption for Option<Color> {
    fn into_brush_option(self) -> Option<Brush> {
        self.map(Into::into)
    }
}

impl IntoBrushOption for Option<Brush> {
    fn into_brush_option(self) -> Option<Brush> {
        self
    }
}

impl IntoBrushOption for Option<ThemeBrush> {
    fn into_brush_option(self) -> Option<Brush> {
        self.map(Into::into)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ThemeBrush {
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ImplicitTransitions {
    pub opacity: Option<Duration>,
    pub scale: Option<Duration>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FlyoutPlacement {
    Top,
    Bottom,
    Left,
    Right,
    Full,
    TopEdgeAlignedLeft,
    TopEdgeAlignedRight,
    BottomEdgeAlignedLeft,
    BottomEdgeAlignedRight,
    LeftEdgeAlignedTop,
    LeftEdgeAlignedBottom,
    RightEdgeAlignedTop,
    RightEdgeAlignedBottom,
    #[default]
    Auto,
}

impl ImplicitTransitions {
    pub fn is_empty(self) -> bool {
        self == Self::default()
    }
}

impl ThemeBrush {
    pub fn custom(key: impl Into<Cow<'static, str>>) -> Self {
        Self::Custom(key.into())
    }

    pub fn resource_key(&self) -> &str {
        match self {
            Self::Accent => "AccentFillColorDefaultBrush",
            Self::AccentSecondary => "AccentFillColorSecondaryBrush",
            Self::AccentTertiary => "AccentFillColorTertiaryBrush",
            Self::AccentDisabled => "AccentFillColorDisabledBrush",
            Self::PrimaryText => "TextFillColorPrimaryBrush",
            Self::SecondaryText => "TextFillColorSecondaryBrush",
            Self::TertiaryText => "TextFillColorTertiaryBrush",
            Self::DisabledText => "TextFillColorDisabledBrush",
            Self::AccentText => "AccentTextFillColorPrimaryBrush",
            Self::SolidBackground => "SolidBackgroundFillColorBaseBrush",
            Self::CardBackground => "CardBackgroundFillColorDefaultBrush",
            Self::SmokeFill => "SmokeFillColorDefaultBrush",
            Self::SubtleFill => "SubtleFillColorSecondaryBrush",
            Self::LayerFill => "LayerFillColorDefaultBrush",
            Self::ControlFill => "ControlFillColorDefaultBrush",
            Self::ControlFillSecondary => "ControlFillColorSecondaryBrush",
            Self::ControlFillTertiary => "ControlFillColorTertiaryBrush",
            Self::ControlFillDisabled => "ControlFillColorDisabledBrush",
            Self::ControlFillInputActive => "ControlFillColorInputActiveBrush",
            Self::CardStroke => "CardStrokeColorDefaultBrush",
            Self::SurfaceStroke => "SurfaceStrokeColorDefaultBrush",
            Self::DividerStroke => "DividerStrokeColorDefaultBrush",
            Self::ControlStroke => "ControlStrokeColorDefaultBrush",
            Self::ControlStrokeSecondary => "ControlStrokeColorSecondaryBrush",
            Self::SystemAttention => "SystemFillColorAttentionBrush",
            Self::SystemSuccess => "SystemFillColorSuccessBrush",
            Self::SystemCaution => "SystemFillColorCautionBrush",
            Self::SystemCritical => "SystemFillColorCriticalBrush",
            Self::SystemNeutral => "SystemFillColorNeutralBrush",
            Self::SystemSolidNeutral => "SystemFillColorSolidNeutralBrush",
            Self::SystemAttentionBackground => "SystemFillColorAttentionBackgroundBrush",
            Self::SystemSuccessBackground => "SystemFillColorSuccessBackgroundBrush",
            Self::SystemCautionBackground => "SystemFillColorCautionBackgroundBrush",
            Self::SystemCriticalBackground => "SystemFillColorCriticalBackgroundBrush",
            Self::SystemNeutralBackground => "SystemFillColorNeutralBackgroundBrush",
            Self::SystemSolidAttention => "SystemFillColorSolidAttentionBackgroundBrush",
            Self::Custom(key) => key,
        }
    }
}

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { a: 255, r, g, b }
    }

    pub const fn argb(a: u8, r: u8, g: u8, b: u8) -> Self {
        Self { a, r, g, b }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Thickness {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CornerRadius {
    pub top_left: f64,
    pub top_right: f64,
    pub bottom_right: f64,
    pub bottom_left: f64,
}

impl CornerRadius {
    pub const fn uniform(value: f64) -> Self {
        Self {
            top_left: value,
            top_right: value,
            bottom_right: value,
            bottom_left: value,
        }
    }
}

impl Thickness {
    pub const fn uniform(value: f64) -> Self {
        Self {
            left: value,
            top: value,
            right: value,
            bottom: value,
        }
    }

    pub const fn xy(horizontal: f64, vertical: f64) -> Self {
        Self {
            left: horizontal,
            top: vertical,
            right: horizontal,
            bottom: vertical,
        }
    }
}

impl From<f64> for Thickness {
    fn from(value: f64) -> Self {
        Self::uniform(value)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HorizontalAlignment {
    Left,
    Center,
    Right,
    #[default]
    Stretch,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum VerticalAlignment {
    Top,
    Center,
    Bottom,
    #[default]
    Stretch,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Visibility {
    #[default]
    Visible,
    Collapsed,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
pub enum FontStyle {
    #[default]
    Normal,
    Oblique,
    Italic,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
pub enum FontStretch {
    Undefined,
    UltraCondensed,
    ExtraCondensed,
    Condensed,
    SemiCondensed,
    #[default]
    Normal,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UltraExpanded,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FontWeight(u16);

impl FontWeight {
    pub const THIN: Self = Self(100);
    pub const EXTRA_LIGHT: Self = Self(200);
    pub const LIGHT: Self = Self(300);
    pub const SEMI_LIGHT: Self = Self(350);
    pub const NORMAL: Self = Self(400);
    pub const MEDIUM: Self = Self(500);
    pub const SEMI_BOLD: Self = Self(600);
    pub const BOLD: Self = Self(700);
    pub const EXTRA_BOLD: Self = Self(800);
    pub const BLACK: Self = Self(900);
    pub const EXTRA_BLACK: Self = Self(950);

    pub const fn from_weight(weight: u16) -> Option<Self> {
        if weight >= 1 && weight <= 999 {
            Some(Self(weight))
        } else {
            None
        }
    }

    pub const fn weight(self) -> u16 {
        self.0
    }

    pub(crate) const fn from_raw(weight: u16) -> Self {
        Self(weight)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GridLength {
    Auto,
    Pixel(f64),
    Star(f64),
}

impl GridLength {
    pub const STAR: Self = Self::Star(1.0);
}
