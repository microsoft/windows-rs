use super::*;

fn validate_uri(value: &str) -> windows_core::Result<()> {
    native::validate_native_uri(value)
}

#[derive(Clone)]
pub struct EncodedImage(EncodedImageBytes);

#[derive(Clone)]
enum EncodedImageBytes {
    Shared(Arc<[u8]>),
    Static(&'static [u8]),
}

impl EncodedImage {
    pub fn new(bytes: impl Into<Arc<[u8]>>) -> Self {
        let bytes = bytes.into();
        assert!(
            bytes.len() <= u32::MAX as usize,
            "encoded image data cannot exceed 4 GiB"
        );
        Self(EncodedImageBytes::Shared(bytes))
    }

    pub fn from_static(bytes: &'static [u8]) -> Self {
        assert!(
            bytes.len() <= u32::MAX as usize,
            "encoded image data cannot exceed 4 GiB"
        );
        Self(EncodedImageBytes::Static(bytes))
    }

    pub fn as_bytes(&self) -> &[u8] {
        match &self.0 {
            EncodedImageBytes::Shared(bytes) => bytes,
            EncodedImageBytes::Static(bytes) => bytes,
        }
    }
}

impl fmt::Debug for EncodedImage {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("EncodedImage")
            .field("len", &self.as_bytes().len())
            .finish()
    }
}

impl PartialEq for EncodedImage {
    fn eq(&self, other: &Self) -> bool {
        match (&self.0, &other.0) {
            (EncodedImageBytes::Shared(left), EncodedImageBytes::Shared(right))
                if Arc::ptr_eq(left, right) =>
            {
                true
            }
            (EncodedImageBytes::Static(left), EncodedImageBytes::Static(right))
                if std::ptr::eq(*left, *right) =>
            {
                true
            }
            _ => self.as_bytes() == other.as_bytes(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ImageSource(ImageSourceValue);

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum ImageSourceValue {
    Encoded(EncodedImage),
    Uri(Rc<str>),
}

impl ImageSource {
    pub fn uri(value: impl Into<Rc<str>>) -> windows_core::Result<Self> {
        let value = value.into();
        validate_uri(&value)?;
        Ok(Self(ImageSourceValue::Uri(value)))
    }

    pub fn file(path: impl AsRef<Path>) -> windows_core::Result<Self> {
        Ok(Self(ImageSourceValue::Uri(file_uri(path.as_ref())?.into())))
    }

    pub fn encoded(value: EncodedImage) -> Self {
        Self(ImageSourceValue::Encoded(value))
    }

    pub(crate) fn value(&self) -> &ImageSourceValue {
        &self.0
    }
}

/// Icon content that can be realized for either an icon element or an icon source property.
#[derive(Clone, Debug, PartialEq)]
pub struct Icon(IconValue);

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum IconValue {
    Symbol(Symbol),
    Font(Rc<str>),
    Bitmap {
        uri: Rc<str>,
        show_as_monochrome: bool,
    },
    Image(ImageSource),
    Path(Rc<str>),
}

impl Icon {
    pub fn symbol(symbol: Symbol) -> Self {
        Self(IconValue::Symbol(symbol))
    }

    pub fn font(glyph: impl Into<Rc<str>>) -> Self {
        Self(IconValue::Font(glyph.into()))
    }

    pub fn bitmap(uri: impl Into<Rc<str>>, show_as_monochrome: bool) -> windows_core::Result<Self> {
        let uri = uri.into();
        validate_uri(&uri)?;
        Ok(Self(IconValue::Bitmap {
            uri,
            show_as_monochrome,
        }))
    }

    pub fn image(source: ImageSource) -> Self {
        Self(IconValue::Image(source))
    }

    pub fn image_uri(uri: impl Into<Rc<str>>) -> windows_core::Result<Self> {
        Ok(Self::image(ImageSource::uri(uri)?))
    }

    pub fn image_file(path: impl AsRef<Path>) -> windows_core::Result<Self> {
        Ok(Self::image(ImageSource::file(path)?))
    }

    pub fn image_data(data: EncodedImage) -> Self {
        Self::image(ImageSource::encoded(data))
    }

    pub fn path(data: impl Into<Rc<str>>) -> Self {
        Self(IconValue::Path(data.into()))
    }

    pub(crate) fn value(&self) -> &IconValue {
        &self.0
    }
}

impl From<Symbol> for Icon {
    fn from(value: Symbol) -> Self {
        Self::symbol(value)
    }
}

fn file_uri(path: &Path) -> windows_core::Result<String> {
    if !path.is_absolute() {
        return Err(windows_core::Error::new(
            windows_core::HRESULT(0x80070057_u32 as i32),
            "image source file path must be absolute",
        ));
    }
    let path = path.to_string_lossy();
    let path = if let Some(path) = path.strip_prefix(r"\\?\UNC\") {
        format!(r"\\{path}")
    } else {
        path.strip_prefix(r"\\?\").unwrap_or(&path).to_string()
    };
    let path = path.replace('\\', "/");
    let mut encoded = String::with_capacity(path.len());
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    for byte in path.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~' | b'/' | b':') {
            encoded.push(char::from(byte));
        } else {
            encoded.push('%');
            encoded.push(char::from(HEX[usize::from(byte >> 4)]));
            encoded.push(char::from(HEX[usize::from(byte & 0x0f)]));
        }
    }
    if let Some(path) = encoded.strip_prefix("//") {
        Ok(format!("file://{path}"))
    } else if encoded.starts_with('/') {
        Ok(format!("file://{encoded}"))
    } else {
        Ok(format!("file:///{encoded}"))
    }
}

fn canonical_rich_edit_text(value: Rc<str>) -> Rc<str> {
    if value.contains('\r') {
        Rc::from(value.replace("\r\n", "\n").replace('\r', "\n"))
    } else {
        value
    }
}

/// A brush resolved from the active WinUI theme resources.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ThemeBrush {
    Accent,
    AccentText,
    PrimaryText,
    SolidBackground,
    CardBackground,
    CardStroke,
    SystemCritical,
    SystemCriticalBackground,
}

impl ThemeBrush {
    pub(crate) const fn resource_key(self) -> &'static str {
        match self {
            Self::Accent => "AccentFillColorDefaultBrush",
            Self::AccentText => "AccentTextFillColorPrimaryBrush",
            Self::PrimaryText => "TextFillColorPrimaryBrush",
            Self::SolidBackground => "SolidBackgroundFillColorBaseBrush",
            Self::CardBackground => "CardBackgroundFillColorDefaultBrush",
            Self::CardStroke => "CardStrokeColorDefaultBrush",
            Self::SystemCritical => "SystemFillColorCriticalBrush",
            Self::SystemCriticalBackground => "SystemFillColorCriticalBackgroundBrush",
        }
    }
}

/// A theme resource brush or a fixed solid color.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Brush {
    Theme(ThemeBrush),
    Solid(Color),
}

impl From<ThemeBrush> for Brush {
    fn from(value: ThemeBrush) -> Self {
        Self::Theme(value)
    }
}

impl From<Color> for Brush {
    fn from(value: Color) -> Self {
        Self::Solid(value)
    }
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ButtonStyle {
    Default,
    Accent,
    Subtle,
    TextLink,
}

/// A supported WinUI resource override value.
#[derive(Clone, Debug, PartialEq)]
pub enum ResourceValue {
    Color(Color),
    Thickness(Thickness),
    CornerRadius(CornerRadius),
}

impl From<Color> for ResourceValue {
    fn from(value: Color) -> Self {
        Self::Color(value)
    }
}

impl From<Thickness> for ResourceValue {
    fn from(value: Thickness) -> Self {
        Self::Thickness(value)
    }
}

impl From<CornerRadius> for ResourceValue {
    fn from(value: CornerRadius) -> Self {
        Self::CornerRadius(value)
    }
}

/// Theme resource values applied to a control subtree.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourceOverrides(Rc<BTreeMap<String, ResourceValue>>);

impl ResourceOverrides {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds or replaces a resource value.
    ///
    /// # Panics
    ///
    /// Panics if the key is empty, or if a thickness or radius is negative or non-finite.
    pub fn set(mut self, key: impl Into<String>, value: impl Into<ResourceValue>) -> Self {
        let key = key.into();
        assert!(!key.is_empty(), "resource override key must not be empty");
        let value = value.into();
        match &value {
            ResourceValue::Color(_) => {}
            ResourceValue::Thickness(value) => assert!(
                value.is_finite_non_negative(),
                "resource override thickness must be finite and non-negative"
            ),
            ResourceValue::CornerRadius(value) => assert!(
                value.is_finite_non_negative(),
                "resource override corner radius must be finite and non-negative"
            ),
        }
        Rc::make_mut(&mut self.0).insert(key, value);
        self
    }

    pub(crate) fn values(&self) -> impl Iterator<Item = (&str, &ResourceValue)> {
        self.0.iter().map(|(key, value)| (key.as_str(), value))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExitTransition {
    duration: Duration,
}

impl ExitTransition {
    pub fn fade(duration: Duration) -> Option<Self> {
        (!duration.is_zero()).then_some(Self { duration })
    }

    pub const fn duration(self) -> Duration {
        self.duration
    }
}

/// Converts a closure or existing callback into a payload callback.
///
/// This trait is sealed; applications use it through callback-taking APIs.
///
/// ```compile_fail,E0277
/// struct CustomCallback;
///
/// impl windows_reactor::IntoPayloadCallback<u32> for CustomCallback {
///     fn into_payload_callback(self) -> windows_reactor::Callback<u32> {
///         windows_reactor::Callback::new(|_| {})
///     }
/// }
/// ```
pub trait IntoPayloadCallback<T>: sealed::PayloadCallback<T> {
    fn into_payload_callback(self) -> Callback<T>;
}

impl<T, F> sealed::PayloadCallback<T> for F where F: Fn(T) + 'static {}

impl<T, F> IntoPayloadCallback<T> for F
where
    F: Fn(T) + 'static,
{
    fn into_payload_callback(self) -> Callback<T> {
        Callback::new(self)
    }
}

impl<T> sealed::PayloadCallback<T> for Callback<T> {}

impl<T> IntoPayloadCallback<T> for Callback<T> {
    fn into_payload_callback(self) -> Self {
        self
    }
}

/// Converts a closure or existing callback into a parameterless callback.
///
/// This trait is sealed; applications use it through callback-taking APIs.
///
/// ```compile_fail,E0277
/// struct CustomCallback;
///
/// impl windows_reactor::IntoUnitCallback for CustomCallback {
///     fn into_unit_callback(self) -> windows_reactor::Callback<()> {
///         windows_reactor::Callback::new(|_| {})
///     }
/// }
/// ```
pub trait IntoUnitCallback: sealed::UnitCallback {
    fn into_unit_callback(self) -> Callback<()>;
}

impl<F> sealed::UnitCallback for F where F: Fn() + 'static {}

impl<F> IntoUnitCallback for F
where
    F: Fn() + 'static,
{
    fn into_unit_callback(self) -> Callback<()> {
        Callback::new(move |()| self())
    }
}

impl sealed::UnitCallback for Callback<()> {}

impl IntoUnitCallback for Callback<()> {
    fn into_unit_callback(self) -> Self {
        self
    }
}

/// A key supported by Reactor keyboard accelerators.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AcceleratorKey {
    Left,
    Up,
    Right,
    Down,
    Space,
    N,
    P,
    R,
    NumberPad0,
    NumberPad1,
    NumberPad2,
    NumberPad3,
    NumberPad4,
    NumberPad5,
    NumberPad6,
    NumberPad7,
    NumberPad8,
    NumberPad9,
    Divide,
    Multiply,
    Subtract,
    Add,
    Decimal,
    Enter,
}

/// Modifier keys for a keyboard accelerator.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum AcceleratorModifiers {
    #[default]
    None,
    Control,
}

/// A keyboard accelerator and its callback.
#[derive(Clone, Debug, PartialEq)]
pub struct KeyAccelerator {
    pub(crate) key: AcceleratorKey,
    pub(crate) modifiers: AcceleratorModifiers,
    pub(crate) callback: Callback<()>,
}

impl KeyAccelerator {
    pub fn new(
        key: AcceleratorKey,
        modifiers: AcceleratorModifiers,
        callback: impl IntoUnitCallback,
    ) -> Self {
        Self {
            key,
            modifiers,
            callback: callback.into_unit_callback(),
        }
    }
}

/// A set of keyboard accelerators assigned to a control.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct KeyAccelerators(Rc<Vec<KeyAccelerator>>);

impl KeyAccelerators {
    pub fn new(values: impl IntoIterator<Item = KeyAccelerator>) -> Self {
        Self(Rc::new(values.into_iter().collect()))
    }

    pub(crate) fn values(&self) -> &[KeyAccelerator] {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Color {
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const fn argb(a: u8, r: u8, g: u8, b: u8) -> Self {
        Self { a, r, g, b }
    }

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::argb(255, r, g, b)
    }

    pub const fn transparent() -> Self {
        Self::argb(0, 0, 0, 0)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RichText {
    pub(crate) paragraphs: Rc<Vec<RichTextParagraph>>,
}

impl RichText {
    pub fn new(paragraphs: impl IntoIterator<Item = RichTextParagraph>) -> Self {
        Self {
            paragraphs: Rc::new(paragraphs.into_iter().collect()),
        }
    }

    pub fn single_paragraph(inlines: impl IntoIterator<Item = RichTextInline>) -> Self {
        Self::new([RichTextParagraph::new(inlines)])
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RichTextParagraph {
    pub(crate) inlines: Vec<RichTextInline>,
}

impl RichTextParagraph {
    pub fn new(inlines: impl IntoIterator<Item = RichTextInline>) -> Self {
        Self {
            inlines: inlines.into_iter().collect(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RichTextInline {
    Run(RichTextRun),
    Hyperlink(RichTextHyperlink),
    LineBreak,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RichTextRun {
    pub text: String,
    pub is_bold: bool,
    pub is_italic: bool,
}

impl RichTextRun {
    pub fn plain(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            ..Self::default()
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RichTextHyperlink {
    pub text: String,
    pub uri: String,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CornerRadius {
    pub top_left: f64,
    pub top_right: f64,
    pub bottom_right: f64,
    pub bottom_left: f64,
}

impl CornerRadius {
    pub const fn new(top_left: f64, top_right: f64, bottom_right: f64, bottom_left: f64) -> Self {
        Self {
            top_left,
            top_right,
            bottom_right,
            bottom_left,
        }
    }

    pub const fn uniform(value: f64) -> Self {
        Self::new(value, value, value, value)
    }

    pub fn is_finite_non_negative(self) -> bool {
        [
            self.top_left,
            self.top_right,
            self.bottom_right,
            self.bottom_left,
        ]
        .into_iter()
        .all(|value| value.is_finite() && value >= 0.0)
    }
}

impl From<f64> for CornerRadius {
    fn from(value: f64) -> Self {
        Self::uniform(value)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Thickness {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}

impl Thickness {
    pub const fn new(left: f64, top: f64, right: f64, bottom: f64) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }

    pub const fn uniform(value: f64) -> Self {
        Self::new(value, value, value, value)
    }

    pub const fn xy(horizontal: f64, vertical: f64) -> Self {
        Self::new(horizontal, vertical, horizontal, vertical)
    }

    pub fn is_finite(self) -> bool {
        [self.left, self.top, self.right, self.bottom]
            .into_iter()
            .all(f64::is_finite)
    }

    pub fn is_finite_non_negative(self) -> bool {
        [self.left, self.top, self.right, self.bottom]
            .into_iter()
            .all(|value| value.is_finite() && value >= 0.0)
    }
}

impl From<f64> for Thickness {
    fn from(value: f64) -> Self {
        Self::uniform(value)
    }
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ThemeTransition {
    Reposition,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum TooltipPlacement {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
    Mouse,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum FlyoutPlacement {
    #[default]
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
    Auto,
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ContentDialogResult {
    #[default]
    None,
    Primary,
    Secondary,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Tooltip {
    content: Box<View>,
    placement: TooltipPlacement,
}

impl Tooltip {
    pub fn text(value: impl AsRef<str>) -> Self {
        Self::rich(TextBlock::new().text(value))
    }

    pub fn rich(content: impl Into<View>) -> Self {
        Self {
            content: Box::new(content.into()),
            placement: TooltipPlacement::Top,
        }
    }

    pub fn placement(mut self, placement: TooltipPlacement) -> Self {
        self.placement = placement;
        self
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Flyout {
    content: Box<View>,
    placement: FlyoutPlacement,
}

impl Flyout {
    pub fn text(value: impl AsRef<str>) -> Self {
        Self::rich(TextBlock::new().text(value))
    }

    pub fn rich(content: impl Into<View>) -> Self {
        Self {
            content: Box::new(content.into()),
            placement: FlyoutPlacement::Top,
        }
    }

    pub fn placement(mut self, placement: FlyoutPlacement) -> Self {
        self.placement = placement;
        self
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MenuItem {
    Item {
        key: Key,
        label: Rc<str>,
        enabled: bool,
    },
    Separator {
        key: Key,
    },
    Submenu {
        key: Key,
        label: Rc<str>,
        items: Rc<Vec<Self>>,
    },
}

impl MenuItem {
    pub fn item(key: impl Into<Key>, label: impl Into<Rc<str>>) -> Self {
        Self::Item {
            key: key.into(),
            label: label.into(),
            enabled: true,
        }
    }

    pub fn disabled(key: impl Into<Key>, label: impl Into<Rc<str>>) -> Self {
        Self::Item {
            key: key.into(),
            label: label.into(),
            enabled: false,
        }
    }

    pub fn separator(key: impl Into<Key>) -> Self {
        Self::Separator { key: key.into() }
    }

    pub fn submenu(
        key: impl Into<Key>,
        label: impl Into<Rc<str>>,
        items: impl IntoIterator<Item = Self>,
    ) -> Self {
        Self::Submenu {
            key: key.into(),
            label: label.into(),
            items: Rc::new(items.into_iter().collect()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Menu {
    pub(crate) items: Rc<Vec<MenuItem>>,
    pub(crate) on_click: Callback<Key>,
}

impl Menu {
    pub fn new(
        items: impl IntoIterator<Item = MenuItem>,
        on_click: impl IntoPayloadCallback<Key>,
    ) -> Self {
        Self {
            items: Rc::new(items.into_iter().collect()),
            on_click: on_click.into_payload_callback(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CommandBarCommand {
    Button {
        key: Key,
        label: Rc<str>,
        icon: Option<Icon>,
        enabled: bool,
    },
    Separator {
        key: Key,
    },
}

impl CommandBarCommand {
    pub fn button(key: impl Into<Key>, label: impl Into<Rc<str>>) -> Self {
        Self::Button {
            key: key.into(),
            label: label.into(),
            icon: None,
            enabled: true,
        }
    }

    pub fn button_with_icon(
        key: impl Into<Key>,
        label: impl Into<Rc<str>>,
        icon: impl Into<Icon>,
    ) -> Self {
        Self::Button {
            key: key.into(),
            label: label.into(),
            icon: Some(icon.into()),
            enabled: true,
        }
    }

    pub fn disabled(key: impl Into<Key>, label: impl Into<Rc<str>>) -> Self {
        Self::Button {
            key: key.into(),
            label: label.into(),
            icon: None,
            enabled: false,
        }
    }

    pub fn separator(key: impl Into<Key>) -> Self {
        Self::Separator { key: key.into() }
    }

    fn into_keyed_visual(self, on_click: &Callback<Key>) -> KeyedView {
        match self {
            Self::Button {
                key,
                label,
                icon,
                enabled,
            } => {
                let callback = on_click.clone();
                let clicked = key.clone();
                let button = AppBarButton::new()
                    .label(label)
                    .is_enabled(enabled)
                    .on_click(move || callback.call(clicked.clone()));
                let button = match icon {
                    Some(icon) => button.icon(icon),
                    None => button,
                };
                keyed(key, button)
            }
            Self::Separator { key } => keyed(key, AppBarSeparator::new()),
        }
    }
}

impl CommandBar {
    pub fn owned_commands(
        self,
        primary: impl IntoIterator<Item = CommandBarCommand>,
        secondary: impl IntoIterator<Item = CommandBarCommand>,
        on_click: impl IntoPayloadCallback<Key>,
    ) -> Self {
        let on_click = on_click.into_payload_callback();
        self.keyed_primary_commands(
            primary
                .into_iter()
                .map(|command| command.into_keyed_visual(&on_click)),
        )
        .keyed_secondary_commands(
            secondary
                .into_iter()
                .map(|command| command.into_keyed_visual(&on_click)),
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CommandBarFlyout {
    pub(crate) primary: Rc<Vec<CommandBarCommand>>,
    pub(crate) secondary: Rc<Vec<CommandBarCommand>>,
    pub(crate) on_click: Callback<Key>,
}

impl CommandBarFlyout {
    pub fn new(
        primary: impl IntoIterator<Item = CommandBarCommand>,
        secondary: impl IntoIterator<Item = CommandBarCommand>,
        on_click: impl IntoPayloadCallback<Key>,
    ) -> Self {
        Self {
            primary: Rc::new(primary.into_iter().collect()),
            secondary: Rc::new(secondary.into_iter().collect()),
            on_click: on_click.into_payload_callback(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FontWeight(u16);

impl FontWeight {
    pub const BLACK: Self = Self(900);
    pub const BOLD: Self = Self(700);
    pub const EXTRA_BLACK: Self = Self(950);
    pub const EXTRA_BOLD: Self = Self(800);
    pub const EXTRA_LIGHT: Self = Self(200);
    pub const LIGHT: Self = Self(300);
    pub const MEDIUM: Self = Self(500);
    pub const NORMAL: Self = Self(400);
    pub const SEMI_BOLD: Self = Self(600);
    pub const SEMI_LIGHT: Self = Self(350);
    pub const THIN: Self = Self(100);

    pub const fn new(weight: u16) -> Option<Self> {
        if weight >= 1 && weight <= 999 {
            Some(Self(weight))
        } else {
            None
        }
    }

    pub const fn get(self) -> u16 {
        self.0
    }

    pub(crate) const fn value(self) -> u16 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GridLength {
    pub(crate) size: GridLengthSize,
    pub(crate) min: Option<f64>,
    pub(crate) max: Option<f64>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum GridLengthSize {
    Auto,
    Pixel(f64),
    Star(f64),
}

#[expect(non_upper_case_globals, non_snake_case)]
impl GridLength {
    pub const Auto: Self = Self::new(GridLengthSize::Auto);
    pub const STAR: Self = Self::new(GridLengthSize::Star(1.0));

    const fn new(size: GridLengthSize) -> Self {
        Self {
            size,
            min: None,
            max: None,
        }
    }

    pub const fn Pixel(value: f64) -> Self {
        assert_grid_length(value);
        Self::new(GridLengthSize::Pixel(value))
    }

    pub const fn Star(value: f64) -> Self {
        assert_grid_length(value);
        Self::new(GridLengthSize::Star(value))
    }

    pub fn min(mut self, value: f64) -> Self {
        assert_grid_length(value);
        assert!(self.max.is_none_or(|max| value <= max));
        self.min = Some(value);
        self
    }

    pub fn max(mut self, value: f64) -> Self {
        assert_grid_length(value);
        assert!(self.min.is_none_or(|min| min <= value));
        self.max = Some(value);
        self
    }
}

const fn assert_grid_length(value: f64) {
    assert!(value.is_finite() && value >= 0.0);
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Key(KeyKind);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum KeyKind {
    Integer(u64),
    Path(Rc<[ComponentPathSegment]>),
    String(Rc<str>),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) enum ComponentPathSegment {
    Relation(RelationId, Key),
    Component(TypeId),
    Tooltip,
    Flyout,
    ContentDialog,
    VirtualRoot,
}

impl Key {
    pub fn as_str(&self) -> Option<&str> {
        match &self.0 {
            KeyKind::String(value) => Some(value),
            KeyKind::Integer(_) | KeyKind::Path(_) => None,
        }
    }

    pub(crate) fn component_path(path: &[ComponentPathSegment]) -> Self {
        debug_assert!(!path.is_empty());
        Self(KeyKind::Path(path.into()))
    }
}

impl From<&str> for Key {
    fn from(value: &str) -> Self {
        Self(KeyKind::String(value.into()))
    }
}

impl From<String> for Key {
    fn from(value: String) -> Self {
        Self(KeyKind::String(value.into()))
    }
}

impl From<u64> for Key {
    fn from(value: u64) -> Self {
        Self(KeyKind::Integer(value))
    }
}

impl From<u32> for Key {
    fn from(value: u32) -> Self {
        Self(KeyKind::Integer(value.into()))
    }
}

impl From<usize> for Key {
    fn from(value: usize) -> Self {
        Self(KeyKind::Integer(value as u64))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PropertyValue {
    String(Rc<str>),
    Bool(bool),
    Brush(Brush),
    ButtonStyle(ButtonStyle),
    Color(Color),
    CornerRadius(CornerRadius),
    Duration(Duration),
    DragDropPolicy(Rc<DragDropPolicy>),
    F64(f64),
    FontWeight(FontWeight),
    GridLengths(Rc<[GridLength]>),
    I32(i32),
    Icon(Icon),
    ImageSource(ImageSource),
    KeyAccelerators(KeyAccelerators),
    OptionalF64(Option<f64>),
    OptionalBool(Option<bool>),
    ResourceOverrides(ResourceOverrides),
    RichText(RichText),
    SelectionIndex(Option<usize>),
    StringList(Rc<[Rc<str>]>),
    ThemeTransitions(Rc<[ThemeTransition]>),
    Thickness(Thickness),
    Enum {
        kind: &'static str,
        variant: &'static str,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct Property {
    pub id: PropertyId,
    pub value: PropertyValue,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) struct CallbackIdentity {
    pub(crate) queue: usize,
    pub(crate) component: ComponentId,
    pub(crate) mapping: TypeId,
}

pub struct Callback<T>(Rc<CallbackInner<T>>);

struct CallbackInner<T> {
    callback: Box<dyn Fn(T)>,
    identity: Option<CallbackIdentity>,
}

impl<T> Clone for Callback<T> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl<T> Callback<T> {
    pub fn new(callback: impl Fn(T) + 'static) -> Self {
        Self(Rc::new(CallbackInner {
            callback: Box::new(callback),
            identity: None,
        }))
    }

    pub(crate) fn new_identified(
        identity: CallbackIdentity,
        callback: impl Fn(T) + 'static,
    ) -> Self {
        Self(Rc::new(CallbackInner {
            callback: Box::new(callback),
            identity: Some(identity),
        }))
    }

    pub fn call(&self, value: T) {
        (self.0.callback)(value);
    }
}

impl<T> fmt::Debug for Callback<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_tuple("Callback").finish()
    }
}

impl<T> PartialEq for Callback<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self.0.identity, other.0.identity) {
            (Some(left), Some(right)) => left == right,
            (None, None) => Rc::ptr_eq(&self.0, &other.0),
            _ => false,
        }
    }
}

pub struct RoutedCallback<T>(Rc<dyn Fn(T) -> bool>);

impl<T> Clone for RoutedCallback<T> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl<T> RoutedCallback<T> {
    pub fn new(callback: impl Fn(T) -> bool + 'static) -> Self {
        Self(Rc::new(callback))
    }

    pub fn call(&self, value: T) -> bool {
        (self.0)(value)
    }
}

impl<T> fmt::Debug for RoutedCallback<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_tuple("RoutedCallback").finish()
    }
}

impl<T> PartialEq for RoutedCallback<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct VirtualKey(pub u32);

impl VirtualKey {
    pub const BACK: Self = Self(0x08);
    pub const TAB: Self = Self(0x09);
    pub const ENTER: Self = Self(0x0d);
    pub const SHIFT: Self = Self(0x10);
    pub const CONTROL: Self = Self(0x11);
    pub const MENU: Self = Self(0x12);
    pub const ESCAPE: Self = Self(0x1b);
    pub const SPACE: Self = Self(0x20);
    pub const PAGE_UP: Self = Self(0x21);
    pub const PAGE_DOWN: Self = Self(0x22);
    pub const END: Self = Self(0x23);
    pub const HOME: Self = Self(0x24);
    pub const LEFT: Self = Self(0x25);
    pub const UP: Self = Self(0x26);
    pub const RIGHT: Self = Self(0x27);
    pub const DOWN: Self = Self(0x28);
    pub const INSERT: Self = Self(0x2d);
    pub const DELETE: Self = Self(0x2e);
    pub const A: Self = Self(0x41);
    pub const B: Self = Self(0x42);
    pub const C: Self = Self(0x43);
    pub const D: Self = Self(0x44);
    pub const E: Self = Self(0x45);
    pub const F: Self = Self(0x46);
    pub const G: Self = Self(0x47);
    pub const H: Self = Self(0x48);
    pub const I: Self = Self(0x49);
    pub const J: Self = Self(0x4a);
    pub const K: Self = Self(0x4b);
    pub const L: Self = Self(0x4c);
    pub const M: Self = Self(0x4d);
    pub const N: Self = Self(0x4e);
    pub const O: Self = Self(0x4f);
    pub const P: Self = Self(0x50);
    pub const Q: Self = Self(0x51);
    pub const R: Self = Self(0x52);
    pub const S: Self = Self(0x53);
    pub const T: Self = Self(0x54);
    pub const U: Self = Self(0x55);
    pub const V: Self = Self(0x56);
    pub const W: Self = Self(0x57);
    pub const X: Self = Self(0x58);
    pub const Y: Self = Self(0x59);
    pub const Z: Self = Self(0x5a);
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct InputModifiers(u8);

impl InputModifiers {
    pub const NONE: Self = Self(0);
    pub const SHIFT: Self = Self(1 << 0);
    pub const CONTROL: Self = Self(1 << 1);
    pub const ALT: Self = Self(1 << 2);
    pub const WINDOWS: Self = Self(1 << 3);

    pub const fn contains(self, value: Self) -> bool {
        self.0 & value.0 == value.0
    }
}

impl std::ops::BitOrAssign for InputModifiers {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PhysicalKeyStatus {
    pub repeat_count: u32,
    pub scan_code: u32,
    pub is_extended: bool,
    pub is_menu_down: bool,
    pub was_down: bool,
    pub is_released: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KeyEventInfo {
    pub key: VirtualKey,
    pub original_key: VirtualKey,
    pub status: PhysicalKeyStatus,
    pub modifiers: InputModifiers,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CharacterEventInfo {
    pub character: u16,
    pub status: PhysicalKeyStatus,
    pub modifiers: InputModifiers,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ElementFocusState {
    Unfocused,
    Pointer,
    Keyboard,
    Programmatic,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FocusEventInfo {
    pub state: ElementFocusState,
    pub is_direct: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DragKind {
    StorageItems,
    Text,
    Unsupported,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DragDropOperation {
    Copy,
    Move,
    Link,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DragDropAction {
    pub operation: DragDropOperation,
    pub caption: Option<String>,
}

impl DragDropAction {
    pub fn new(operation: DragDropOperation) -> Self {
        Self {
            operation,
            caption: None,
        }
    }

    pub fn caption(mut self, caption: impl Into<String>) -> Self {
        self.caption = Some(caption.into());
        self
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DragDropPolicy {
    pub storage_items: Option<DragDropAction>,
    pub text: Option<DragDropAction>,
}

impl DragDropPolicy {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn storage_items(mut self, action: impl Into<Option<DragDropAction>>) -> Self {
        self.storage_items = action.into();
        self
    }

    pub fn text(mut self, action: impl Into<Option<DragDropAction>>) -> Self {
        self.text = action.into();
        self
    }

    pub(crate) fn action(&self, kind: DragKind) -> Option<&DragDropAction> {
        match kind {
            DragKind::StorageItems => self.storage_items.as_ref(),
            DragKind::Text => self.text.as_ref(),
            DragKind::Unsupported => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DroppedStorageItem {
    pub name: String,
    pub path: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DroppedData {
    StorageItems(Vec<DroppedStorageItem>),
    Text(String),
    Unsupported,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NavigationViewDisplayMode {
    Minimal,
    Compact,
    Expanded,
}

/// Pointer state in element-local and window-relative device-independent pixels.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PointerEventInfo {
    pub x: f64,
    pub y: f64,
    pub window_x: f64,
    pub window_y: f64,
    pub pointer_id: u32,
    pub modifiers: InputModifiers,
    pub capture_succeeded: Option<bool>,
    pub is_captured: bool,
    pub is_left_button_pressed: bool,
    pub is_right_button_pressed: bool,
    pub is_middle_button_pressed: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum EventValue {
    Bool(Callback<bool>),
    Color(Callback<Color>),
    ContentDialogResult(Callback<ContentDialogResult>),
    CharacterEventInfo(RoutedCallback<CharacterEventInfo>),
    String(Callback<Rc<str>>),
    F64(Callback<f64>),
    FocusEventInfo(Callback<FocusEventInfo>),
    DragKind(Callback<DragKind>),
    DroppedData(Callback<DroppedData>),
    OptionalBool(Callback<Option<bool>>),
    OptionalDateTime(Callback<Option<DateTime>>),
    OptionalF64(Callback<Option<f64>>),
    OptionalTimeSpan(Callback<Option<TimeSpan>>),
    NavigationViewDisplayMode(Callback<NavigationViewDisplayMode>),
    PointerEventInfo(Callback<PointerEventInfo>),
    KeyEventInfo(RoutedCallback<KeyEventInfo>),
    Key(Callback<Key>),
    Selection(Callback<Option<Rc<str>>>),
    SelectionIndex(Callback<Option<usize>>),
    StringList(Callback<Vec<String>>),
    Unit(Callback<()>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum EventPayload {
    Bool(bool),
    Color(Color),
    ContentDialogResult(ContentDialogResult),
    #[cfg(any(test, feature = "test"))]
    CharacterEventInfo(CharacterEventInfo),
    String(Rc<str>),
    F64(f64),
    FocusEventInfo(FocusEventInfo),
    DragKind(DragKind),
    DroppedData(DroppedData),
    OptionalBool(Option<bool>),
    OptionalDateTime(Option<DateTime>),
    OptionalF64(Option<f64>),
    OptionalTimeSpan(Option<TimeSpan>),
    NavigationViewDisplayMode(NavigationViewDisplayMode),
    PointerEventInfo(PointerEventInfo),
    #[cfg(any(test, feature = "test"))]
    KeyEventInfo(KeyEventInfo),
    Key(Key),
    Selection(SelectionChange),
    SelectionIndex(Option<usize>),
    StringList(Vec<String>),
    Unit,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SelectionChange {
    pub item: Option<ObjectId>,
    pub value: Option<Rc<str>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Event {
    pub id: EventId,
    pub value: EventValue,
}

/// A lazily materialized keyed source for a virtualizing declaration.
///
/// ```
/// use windows_reactor::{ItemsRepeater, Key, TextBlock, VirtualSource, View};
///
/// let source = VirtualSource::new(
///     7,
///     10_000,
///     Key::from,
///     |index| -> View { TextBlock::new().text(index.to_string()).into() },
/// );
/// let view: View = ItemsRepeater::new().virtual_source(source).into();
/// ```
#[derive(Clone)]
pub struct VirtualSource {
    pub(crate) key_revision: u64,
    pub(crate) len: usize,
    pub(crate) key: Rc<dyn Fn(usize) -> Key>,
    pub(crate) view: Rc<dyn Fn(usize) -> View>,
}

impl VirtualSource {
    pub fn new<K, V, KI, VI>(key_revision: u64, len: usize, key: K, view: V) -> Self
    where
        K: Fn(usize) -> KI + 'static,
        V: Fn(usize) -> VI + 'static,
        KI: Into<Key>,
        VI: Into<View>,
    {
        Self {
            key_revision,
            len,
            key: Rc::new(move |index| key(index).into()),
            view: Rc::new(move |index| view(index).into()),
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn key_revision(&self) -> u64 {
        self.key_revision
    }
}

impl fmt::Debug for VirtualSource {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("VirtualSource")
            .field("key_revision", &self.key_revision)
            .field("len", &self.len)
            .finish_non_exhaustive()
    }
}

impl PartialEq for VirtualSource {
    fn eq(&self, other: &Self) -> bool {
        self.key_revision == other.key_revision
            && self.len == other.len
            && Rc::ptr_eq(&self.key, &other.key)
            && Rc::ptr_eq(&self.view, &other.view)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum VirtualItems {
    Eager(Rc<Vec<(Key, View)>>),
    Lazy(VirtualSource),
}

impl VirtualItems {
    pub(crate) fn eager(items: impl IntoIterator<Item = KeyedView>) -> Self {
        Self::Eager(Rc::new(
            items
                .into_iter()
                .map(|item| {
                    let visual = item.0;
                    let key = visual.0.key_ref().clone();
                    (key, visual)
                })
                .collect(),
        ))
    }

    pub(crate) fn len(&self) -> usize {
        match self {
            Self::Eager(items) => items.len(),
            Self::Lazy(source) => source.len,
        }
    }

    pub(crate) fn key(&self, index: usize) -> Option<Key> {
        match self {
            Self::Eager(items) => items.get(index).map(|(key, _)| key.clone()),
            Self::Lazy(source) => (index < source.len).then(|| (source.key)(index)),
        }
    }

    pub(crate) fn view(&self, index: usize) -> Option<View> {
        match self {
            Self::Eager(items) => items.get(index).map(|(_, view)| view.clone()),
            Self::Lazy(source) => (index < source.len).then(|| (source.view)(index)),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct DeclaredVirtualItems {
    pub(crate) relation: RelationId,
    pub(crate) owner: Option<ComponentId>,
    pub(crate) items: VirtualItems,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum RelationValue {
    One(Option<Rc<DeclaredNode>>),
    Many(Rc<Vec<DeclaredNode>>),
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct DeclaredRelation {
    pub id: RelationId,
    pub value: RelationValue,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct DeclaredTooltip {
    pub content: Box<DeclaredNode>,
    pub placement: TooltipPlacement,
}

impl DeclaredTooltip {
    pub(crate) fn declaration(&self) -> Declaration {
        Declaration::new(ObjectType::ToolTip).relation(
            RelationId::Content,
            RelationValue::One(Some(Rc::new(self.content.as_ref().clone()))),
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct DeclaredFlyout {
    pub content: Box<DeclaredNode>,
    pub placement: FlyoutPlacement,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct DeclaredMenu {
    pub menu: Menu,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct DeclaredCommandBarFlyout {
    pub flyout: CommandBarFlyout,
}

impl DeclaredFlyout {
    pub(crate) fn declaration(&self) -> Declaration {
        self.content.as_object().unwrap().clone()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct DeclaredContentDialog {
    pub declaration: Declaration,
    pub open: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct DeclaredAttachments {
    pub tooltip: Option<DeclaredTooltip>,
    pub flyout: Option<DeclaredFlyout>,
    pub menu: Option<DeclaredMenu>,
    pub command_bar_flyout: Option<DeclaredCommandBarFlyout>,
    pub content_dialog: Option<DeclaredContentDialog>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Declaration {
    pub kind: ObjectType,
    pub key: Option<Key>,
    pub component: Option<ComponentId>,
    pub reference: Option<ElementRef>,
    pub exit_transition: Option<ExitTransition>,
    pub window_title_bar: Option<WindowTitleBarHeight>,
    pub attachments: Option<Box<DeclaredAttachments>>,
    pub properties: SharedList<Property>,
    pub events: SharedList<Event>,
    pub relations: SharedList<DeclaredRelation>,
    pub virtual_items: Option<Box<DeclaredVirtualItems>>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) enum SharedList<T> {
    #[default]
    Empty,
    One(T),
    Many(Rc<Vec<T>>),
}

impl<T> SharedList<T> {
    pub fn as_slice(&self) -> &[T] {
        match self {
            Self::Empty => &[],
            Self::One(value) => std::slice::from_ref(value),
            Self::Many(values) => values,
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.as_slice().iter()
    }
}

impl<T: Clone> SharedList<T> {
    pub(crate) fn upsert(&mut self, matches: impl Fn(&T) -> bool, value: T) {
        let current = std::mem::take(self);
        *self = match current {
            Self::Empty => Self::One(value),
            Self::One(current) if matches(&current) => Self::One(value),
            Self::One(current) => Self::Many(Rc::new(vec![current, value])),
            Self::Many(mut values) => {
                let entries = Rc::make_mut(&mut values);
                if let Some(current) = entries.iter_mut().find(|current| matches(current)) {
                    *current = value;
                } else {
                    entries.push(value);
                }
                Self::Many(values)
            }
        };
    }

    fn sort_by_key<K: Ord>(&mut self, key: impl FnMut(&T) -> K) {
        if let Self::Many(values) = self {
            Rc::make_mut(values).sort_by_key(key);
        }
    }
}

impl<T> FromIterator<T> for SharedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(values: I) -> Self {
        let mut values = values.into_iter();
        let Some(first) = values.next() else {
            return Self::Empty;
        };
        let Some(second) = values.next() else {
            return Self::One(first);
        };
        Self::Many(Rc::new(
            std::iter::once(first)
                .chain(std::iter::once(second))
                .chain(values)
                .collect(),
        ))
    }
}

impl Drop for Declaration {
    fn drop(&mut self) {
        let mut pending = Vec::new();
        Self::queue_declaration(self, &mut pending);
        while let Some(frame) = pending.pop() {
            match frame {
                DropFrame::One(Some(child)) => {
                    Self::queue_node(child, &mut pending);
                }
                DropFrame::One(None) => {}
                DropFrame::Declarations(mut children) => {
                    if let Some(child) = children.next() {
                        pending.push(DropFrame::Declarations(children));
                        Self::queue_node(child, &mut pending);
                    }
                }
                DropFrame::Relations(mut relations) => {
                    if let Some(relation) = relations.next() {
                        pending.push(DropFrame::Relations(relations));
                        Self::queue_relation(relation, &mut pending);
                    }
                }
            }
        }
    }
}

enum DropFrame {
    One(Option<DeclaredNode>),
    Declarations(std::vec::IntoIter<DeclaredNode>),
    Relations(std::vec::IntoIter<DeclaredRelation>),
}

#[derive(Clone)]
pub(crate) enum DeclaredNode {
    Object(Declaration),
    Component {
        node: ComponentNode,
        relation_key: Option<RelationKey>,
        attachments: Option<Box<DeclaredAttachments>>,
    },
    Provider(Box<DeclaredProvider>),
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum RelationKey {
    Explicit(Key),
    Positional(Key),
}

impl RelationKey {
    pub(crate) fn key(&self) -> &Key {
        match self {
            Self::Explicit(key) | Self::Positional(key) => key,
        }
    }
}

#[derive(Clone)]
pub(crate) struct DeclaredProvider {
    pub(crate) provision: ContextProvision,
    pub(crate) child: DeclaredNode,
}

impl fmt::Debug for DeclaredNode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Object(value) => value.fmt(formatter),
            Self::Component {
                node,
                relation_key,
                attachments,
            } => formatter
                .debug_struct("Component")
                .field("key", &node.key)
                .field("relation_key", relation_key)
                .field("attachments", attachments)
                .finish(),
            Self::Provider(provider) => formatter
                .debug_struct("Provider")
                .field("provision", &provider.provision)
                .field("child", &provider.child)
                .finish(),
        }
    }
}

impl PartialEq for DeclaredNode {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Object(left), Self::Object(right)) => left == right,
            (
                Self::Component {
                    node: left,
                    relation_key: left_key,
                    attachments: left_attachments,
                },
                Self::Component {
                    node: right,
                    relation_key: right_key,
                    attachments: right_attachments,
                },
            ) => left == right && left_key == right_key && left_attachments == right_attachments,
            (Self::Provider(left), Self::Provider(right)) => {
                left.provision == right.provision && left.child == right.child
            }
            _ => false,
        }
    }
}

impl DeclaredNode {
    fn attachments_mut(&mut self) -> &mut Option<Box<DeclaredAttachments>> {
        match self {
            Self::Object(declaration) => &mut declaration.attachments,
            Self::Component { attachments, .. } => attachments,
            Self::Provider(provider) => provider.child.attachments_mut(),
        }
    }

    fn key(self, key: impl Into<Key>) -> Self {
        let key = key.into();
        match self {
            Self::Object(value) => Self::Object(value.key(key)),
            Self::Component {
                node, attachments, ..
            } => Self::Component {
                node,
                relation_key: Some(RelationKey::Explicit(key)),
                attachments,
            },
            Self::Provider(provider) => Self::Provider(Box::new(DeclaredProvider {
                provision: provider.provision,
                child: provider.child.key(key),
            })),
        }
    }

    fn positional_key(self, key: impl Into<Key>) -> Self {
        let key = key.into();
        match self {
            Self::Object(value) => Self::Object(value.key(key)),
            Self::Component {
                node, attachments, ..
            } => Self::Component {
                node,
                relation_key: Some(RelationKey::Positional(key)),
                attachments,
            },
            Self::Provider(provider) => Self::Provider(Box::new(DeclaredProvider {
                provision: provider.provision,
                child: provider.child.positional_key(key),
            })),
        }
    }

    pub(crate) fn object(self) -> Result<Declaration, GraphError> {
        match self {
            Self::Object(value) => Ok(value),
            Self::Component { .. } | Self::Provider(_) => Err(GraphError::UnresolvedComponent),
        }
    }

    pub(crate) fn as_object(&self) -> Result<&Declaration, GraphError> {
        match self {
            Self::Object(value) => Ok(value),
            Self::Component { .. } | Self::Provider(_) => Err(GraphError::UnresolvedComponent),
        }
    }

    fn key_ref(&self) -> &Key {
        match self {
            Self::Object(value) => {
                let Some(key) = value.key.as_ref() else {
                    unreachable!("KeyedView object without a key");
                };
                key
            }
            Self::Component {
                node, relation_key, ..
            } => relation_key.as_ref().map_or_else(
                || {
                    node.key
                        .as_ref()
                        .expect("unkeyed components require owning relation identity")
                },
                RelationKey::key,
            ),
            Self::Provider(provider) => provider.child.key_ref(),
        }
    }

    pub(crate) fn relation_identity(&self, index: usize) -> Key {
        match self {
            Self::Object(value) => value.key.clone().unwrap_or_else(|| index.into()),
            Self::Component { relation_key, .. } => relation_key
                .as_ref()
                .map(RelationKey::key)
                .cloned()
                .unwrap_or_else(|| index.into()),
            Self::Provider(provider) => provider.child.relation_identity(index),
        }
    }
}

impl Declaration {
    pub(crate) fn new(kind: ObjectType) -> Self {
        Self {
            kind,
            key: None,
            component: None,
            reference: None,
            exit_transition: None,
            window_title_bar: None,
            attachments: None,
            properties: SharedList::Empty,
            events: SharedList::Empty,
            relations: SharedList::Empty,
            virtual_items: None,
        }
    }

    fn key(mut self, key: impl Into<Key>) -> Self {
        self.key = Some(key.into());
        self
    }

    fn property(mut self, id: PropertyId, value: PropertyValue) -> Self {
        self.properties
            .upsert(|property| property.id == id, Property { id, value });
        self.properties
            .sort_by_key(|property| property_order(self.kind, property.id));
        self
    }

    fn window_title_bar(mut self, height: WindowTitleBarHeight) -> Self {
        self.window_title_bar = Some(height);
        self
    }

    pub(crate) fn relation(mut self, id: RelationId, value: RelationValue) -> Self {
        self.relations
            .upsert(|relation| relation.id == id, DeclaredRelation { id, value });
        self
    }

    pub(crate) fn virtual_items(mut self, relation: RelationId, items: VirtualItems) -> Self {
        self.virtual_items = Some(Box::new(DeclaredVirtualItems {
            relation,
            owner: None,
            items,
        }));
        self
    }

    pub(crate) fn virtual_item(mut self, relation: RelationId, item: KeyedView) -> Self {
        let visual = item.0;
        let item = (visual.0.key_ref().clone(), visual);
        let items = match self.virtual_items.take() {
            Some(items)
                if matches!(
                    items.as_ref(),
                    DeclaredVirtualItems {
                        relation: current,
                        items: VirtualItems::Eager(_),
                        ..
                    } if *current == relation
                ) =>
            {
                let DeclaredVirtualItems {
                    owner,
                    items: VirtualItems::Eager(items),
                    ..
                } = *items
                else {
                    unreachable!()
                };
                let mut items = items.as_ref().clone();
                items.push(item);
                self.virtual_items = Some(Box::new(DeclaredVirtualItems {
                    relation,
                    owner,
                    items: VirtualItems::Eager(Rc::new(items)),
                }));
                return self;
            }
            _ => Rc::new(vec![item]),
        };
        self.virtual_items = Some(Box::new(DeclaredVirtualItems {
            relation,
            owner: None,
            items: VirtualItems::Eager(items),
        }));
        self
    }

    fn event(mut self, id: EventId, value: EventValue) -> Self {
        self.events
            .upsert(|event| event.id == id, Event { id, value });
        self
    }

    fn queue_relations(relations: SharedList<DeclaredRelation>, pending: &mut Vec<DropFrame>) {
        match relations {
            SharedList::Empty => {}
            SharedList::One(relation) => Self::queue_relation(relation, pending),
            SharedList::Many(relations) => {
                if let Ok(relations) = Rc::try_unwrap(relations) {
                    pending.push(DropFrame::Relations(relations.into_iter()));
                }
            }
        }
    }

    fn queue_node(mut node: DeclaredNode, pending: &mut Vec<DropFrame>) {
        loop {
            match node {
                DeclaredNode::Object(mut declaration) => {
                    Self::queue_declaration(&mut declaration, pending);
                    return;
                }
                DeclaredNode::Component { .. } => return,
                DeclaredNode::Provider(provider) => node = provider.child,
            }
        }
    }

    fn queue_declaration(declaration: &mut Self, pending: &mut Vec<DropFrame>) {
        if let Some(attachments) = declaration.attachments.take() {
            let DeclaredAttachments {
                tooltip,
                flyout,
                menu: _,
                command_bar_flyout: _,
                content_dialog,
            } = *attachments;
            if let Some(tooltip) = tooltip {
                pending.push(DropFrame::One(Some(*tooltip.content)));
            }
            if let Some(flyout) = flyout {
                pending.push(DropFrame::One(Some(*flyout.content)));
            }
            if let Some(mut dialog) = content_dialog {
                Self::queue_declaration(&mut dialog.declaration, pending);
            }
        }
        Self::queue_relations(std::mem::take(&mut declaration.relations), pending);
    }

    fn queue_relation(relation: DeclaredRelation, pending: &mut Vec<DropFrame>) {
        match relation.value {
            RelationValue::One(Some(child)) => {
                if let Ok(child) = Rc::try_unwrap(child) {
                    pending.push(DropFrame::One(Some(child)));
                }
            }
            RelationValue::Many(children) => {
                if let Ok(children) = Rc::try_unwrap(children) {
                    pending.push(DropFrame::Declarations(children.into_iter()));
                }
            }
            RelationValue::One(None) => {}
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct View(pub(crate) DeclaredNode);

pub trait TooltipExt: Into<View> + Sized {
    fn tooltip(self, value: impl AsRef<str>) -> View {
        self.tooltip_with(Tooltip::text(value))
    }

    fn tooltip_with(self, tooltip: Tooltip) -> View {
        let mut visual = self.into();
        let tooltip = DeclaredTooltip {
            content: Box::new(tooltip.content.0),
            placement: tooltip.placement,
        };
        visual
            .0
            .attachments_mut()
            .get_or_insert_with(Default::default)
            .tooltip = Some(tooltip);
        visual
    }
}

impl<T> TooltipExt for T where T: Into<View> {}

pub trait FlyoutExt: Into<View> + Sized {
    fn flyout(self, value: impl AsRef<str>) -> View {
        self.flyout_with(Flyout::text(value))
    }

    fn flyout_with(self, flyout: Flyout) -> View {
        let mut visual = self.into();
        let flyout = DeclaredFlyout {
            content: Box::new(flyout.content.0),
            placement: flyout.placement,
        };
        visual
            .0
            .attachments_mut()
            .get_or_insert_with(Default::default)
            .flyout = Some(flyout);
        visual
    }
}

impl<T> FlyoutExt for T where T: Into<View> {}

pub trait MenuExt: Into<View> + Sized {
    fn menu(self, menu: Menu) -> View {
        let mut visual = self.into();
        let menu = DeclaredMenu { menu };
        visual
            .0
            .attachments_mut()
            .get_or_insert_with(Default::default)
            .menu = Some(menu);
        visual
    }
}

impl<T> MenuExt for T where T: Into<View> {}

pub trait CommandBarFlyoutExt: Into<View> + Sized {
    fn command_bar_flyout(self, flyout: CommandBarFlyout) -> View {
        let mut visual = self.into();
        let flyout = DeclaredCommandBarFlyout { flyout };
        visual
            .0
            .attachments_mut()
            .get_or_insert_with(Default::default)
            .command_bar_flyout = Some(flyout);
        visual
    }
}

impl<T> CommandBarFlyoutExt for T where T: Into<View> {}

pub trait ContentDialogExt: Into<View> + Sized {
    fn content_dialog(self, dialog: ContentDialog) -> View {
        let mut visual = self.into();
        let dialog = DeclaredContentDialog {
            declaration: dialog.0,
            open: dialog.1,
        };
        visual
            .0
            .attachments_mut()
            .get_or_insert_with(Default::default)
            .content_dialog = Some(dialog);
        visual
    }
}

impl<T> ContentDialogExt for T where T: Into<View> {}

#[derive(Clone, Debug, PartialEq)]
pub struct KeyedView(View);

impl KeyedView {
    pub fn new(key: impl Into<Key>, view: impl Into<View>) -> Self {
        keyed(key, view)
    }
}

pub fn keyed(key: impl Into<Key>, visual: impl Into<View>) -> KeyedView {
    let mut visual = visual.into();
    visual.0 = visual.0.key(key);
    KeyedView(visual)
}

/// Converts Reactor's supported view collections into an ordered list.
///
/// Accepts `()`, `Vec<View>`, `[View; N]`, and tuples of one through sixteen values implementing
/// `Into<View>`. Tuples may mix control types.
///
/// This trait is sealed; applications use it through child-taking builders. Custom collections
/// must convert to one of these forms rather than implement this trait:
///
/// ```
/// use windows_reactor::{StackPanel, TextBlock, View};
///
/// let labels = ["First", "Second"];
/// let children: Vec<View> = labels
///     .into_iter()
///     .map(|label| TextBlock::new().text(label).into())
///     .collect();
/// let _ = StackPanel::new().children(children);
/// ```
///
/// ```compile_fail,E0277
/// struct CustomViews;
///
/// impl windows_reactor::IntoViews for CustomViews {
///     fn into_visuals(self) -> Vec<windows_reactor::View> {
///         Vec::new()
///     }
/// }
/// ```
pub trait IntoViews: Sealed {
    #[doc(hidden)]
    fn into_visuals(self) -> Vec<View>;
}

impl Sealed for () {}

impl IntoViews for () {
    fn into_visuals(self) -> Vec<View> {
        Vec::new()
    }
}

impl<const N: usize> Sealed for [View; N] {}

impl<const N: usize> IntoViews for [View; N] {
    fn into_visuals(self) -> Vec<View> {
        self.into()
    }
}

impl Sealed for Vec<View> {}

impl IntoViews for Vec<View> {
    fn into_visuals(self) -> Vec<View> {
        self
    }
}

macro_rules! impl_into_visuals_tuple {
    ($($type:ident $index:tt),+ $(,)?) => {
        impl<$($type),+> Sealed for ($($type,)+)
        where
            $($type: Into<View>,)+
        {}

        impl<$($type),+> IntoViews for ($($type,)+)
        where
            $($type: Into<View>,)+
        {
            fn into_visuals(self) -> Vec<View> {
                vec![$(self.$index.into()),+]
            }
        }
    };
}

impl_into_visuals_tuple!(A 0);
impl_into_visuals_tuple!(A 0, B 1);
impl_into_visuals_tuple!(A 0, B 1, C 2);
impl_into_visuals_tuple!(A 0, B 1, C 2, D 3);
impl_into_visuals_tuple!(A 0, B 1, C 2, D 3, E 4);
impl_into_visuals_tuple!(A 0, B 1, C 2, D 3, E 4, F 5);
impl_into_visuals_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6);
impl_into_visuals_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7);
impl_into_visuals_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8);
impl_into_visuals_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9);
impl_into_visuals_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10);
impl_into_visuals_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11);
impl_into_visuals_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12);
impl_into_visuals_tuple!(
    A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13
);
impl_into_visuals_tuple!(
    A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14
);
impl_into_visuals_tuple!(
    A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15
);

pub(crate) mod generated_declarations {
    use super::*;
    include!("generated_declarations.rs");
}

pub use generated_declarations::*;
