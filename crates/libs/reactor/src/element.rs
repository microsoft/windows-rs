use std::any::Any;
use std::fmt;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;

use super::*;
use crate::core::{ComponentToken, ComponentView, ContextProvision};

pub(crate) fn validate_image_uri(value: &str) -> windows_core::Result<()> {
    native::validate_native_image_uri(value)
}

pub(crate) fn validate_uri(value: &str) -> windows_core::Result<()> {
    native::validate_native_uri(value)
}

/// Immutable encoded bitmap data for an [`Image`] or [`ImageIcon`] source.
#[derive(Clone)]
pub struct EncodedImage(EncodedImageBytes);

#[derive(Clone)]
enum EncodedImageBytes {
    Static(&'static [u8]),
    Shared(Arc<[u8]>),
}

impl EncodedImage {
    /// Owns encoded bitmap data that may be shared across views.
    ///
    /// # Panics
    ///
    /// Panics if the data exceeds the WinRT buffer limit of 4 GiB.
    pub fn new(bytes: impl Into<Arc<[u8]>>) -> Self {
        let bytes = bytes.into();
        assert!(
            bytes.len() <= u32::MAX as usize,
            "encoded image data cannot exceed 4 GiB"
        );
        Self(EncodedImageBytes::Shared(bytes))
    }

    /// Retains encoded bitmap data with static storage without copying it.
    ///
    /// # Panics
    ///
    /// Panics if the data exceeds the WinRT buffer limit of 4 GiB.
    pub fn from_static(bytes: &'static [u8]) -> Self {
        assert!(
            bytes.len() <= u32::MAX as usize,
            "encoded image data cannot exceed 4 GiB"
        );
        Self(EncodedImageBytes::Static(bytes))
    }

    /// Returns the encoded bitmap data.
    pub fn as_bytes(&self) -> &[u8] {
        match &self.0 {
            EncodedImageBytes::Static(bytes) => bytes,
            EncodedImageBytes::Shared(bytes) => bytes,
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
            (EncodedImageBytes::Static(left), EncodedImageBytes::Static(right))
                if std::ptr::eq(*left, *right) =>
            {
                true
            }
            (EncodedImageBytes::Shared(left), EncodedImageBytes::Shared(right))
                if Arc::ptr_eq(left, right) =>
            {
                true
            }
            _ => self.as_bytes() == other.as_bytes(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum ImageValue {
    Uri(String),
    Encoded(EncodedImage),
}

pub(crate) fn file_uri(path: &std::path::Path) -> windows_core::Result<String> {
    if !path.is_absolute() {
        return Err(windows_core::Error::new(
            windows_core::HRESULT(0x80070057_u32 as _),
            "Image::source_file requires an absolute path",
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

#[cfg(test)]
mod file_uri_tests {
    use super::*;

    #[test]
    fn encodes_drive_paths_and_reserved_characters() {
        assert_eq!(
            file_uri(std::path::Path::new(r"\\?\C:\work dir\a#b%20.png")).unwrap(),
            "file:///C:/work%20dir/a%23b%2520.png"
        );
    }

    #[test]
    fn encodes_extended_unc_paths_with_an_authority() {
        assert_eq!(
            file_uri(std::path::Path::new(r"\\?\UNC\server\share dir\asset.png")).unwrap(),
            "file://server/share%20dir/asset.png"
        );
    }

    #[test]
    fn rejects_relative_paths() {
        assert!(file_uri(std::path::Path::new(r"images\asset.png")).is_err());
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

/// An OpenType font weight in the inclusive range 1 through 999.
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

    /// Creates a font weight, returning `None` outside the OpenType range 1 through 999.
    pub const fn new(weight: u16) -> Option<Self> {
        if weight >= 1 && weight <= 999 {
            Some(Self(weight))
        } else {
            None
        }
    }

    /// Returns the numeric OpenType weight.
    pub const fn get(self) -> u16 {
        self.0
    }
}

impl Default for FontWeight {
    fn default() -> Self {
        Self::NORMAL
    }
}

#[cfg(test)]
mod font_weight_tests {
    use super::*;

    #[test]
    fn validates_open_type_weight_range() {
        assert_eq!(FontWeight::new(0), None);
        assert_eq!(FontWeight::new(1).unwrap().get(), 1);
        assert_eq!(FontWeight::new(999).unwrap().get(), 999);
        assert_eq!(FontWeight::new(1000), None);
        assert_eq!(FontWeight::default(), FontWeight::NORMAL);
    }
}

impl ThemeBrush {
    pub(crate) fn resource_key(self) -> &'static str {
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

/// An 8-bit-per-channel ARGB color.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Color {
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    /// Creates a color from alpha, red, green, and blue channels.
    pub const fn argb(a: u8, r: u8, g: u8, b: u8) -> Self {
        Self { a, r, g, b }
    }

    /// Creates an opaque color from red, green, and blue channels.
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::argb(255, r, g, b)
    }

    /// Returns fully transparent black.
    pub const fn transparent() -> Self {
        Self::argb(0, 0, 0, 0)
    }
}

/// A theme resource brush or a fixed solid color.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Brush {
    Theme(ThemeBrush),
    Solid(Color),
}

/// Pointer state in element-local and window-relative device-independent pixels.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PointerEventInfo {
    pub x: f64,
    pub y: f64,
    pub window_x: f64,
    pub window_y: f64,
    pub capture_succeeded: bool,
    pub is_left_button_pressed: bool,
    pub is_right_button_pressed: bool,
    pub is_middle_button_pressed: bool,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum NavigationViewDisplayMode {
    Minimal,
    Compact,
    Expanded,
}

impl Brush {
    pub(crate) const fn theme(self) -> Option<ThemeBrush> {
        match self {
            Self::Theme(value) => Some(value),
            Self::Solid(_) => None,
        }
    }
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

pub(crate) mod sealed {
    pub trait Sealed {}

    pub(crate) trait NativeControl: Sealed + Sized {
        fn into_element(self) -> super::Element;
    }

    pub(crate) trait LayoutControl: NativeControl {
        fn element_state_mut(&mut self) -> &mut Option<std::rc::Rc<super::ElementState>>;
    }

    pub(crate) trait ContentControl: NativeControl {
        fn into_content_view(self, content: super::View) -> super::View {
            super::View(super::ViewKind::Content {
                control: self.into_element(),
                content: Box::new(content.into_kind()),
            })
        }
    }

    pub(crate) trait SlotIndex<S> {
        fn slot_index(slot: S) -> u8;
    }

    pub trait StaticViews {
        fn into_positioned(self) -> Vec<super::KeyedView>;
    }
}

/// Stable identity for a keyed child, menu item, tree node, or command.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Key(KeyKind);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum KeyKind {
    Integer(u64),
    String(Rc<str>),
    Position(usize),
}

impl Key {
    pub(crate) fn position(value: usize) -> Self {
        Self(KeyKind::Position(value))
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
        Self(KeyKind::Integer(u64::try_from(value).unwrap()))
    }
}

impl From<String> for Key {
    fn from(value: String) -> Self {
        Self(KeyKind::String(value.into()))
    }
}

impl From<&str> for Key {
    fn from(value: &str) -> Self {
        Self(KeyKind::String(value.into()))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct KeyedElement {
    key: Key,
    element: Element,
}

impl KeyedElement {
    #[cfg(test)]
    pub(crate) fn new(key: impl Into<Key>, element: impl Into<Element>) -> Self {
        Self {
            key: key.into(),
            element: element.into(),
        }
    }

    pub(crate) fn key(&self) -> &Key {
        &self.key
    }

    pub(crate) fn element(&self) -> &Element {
        &self.element
    }

    pub(crate) fn into_parts(self) -> (Key, Element) {
        (self.key, self.element)
    }
}

#[cfg(test)]
pub(crate) trait NativeContentTestExt: Sized {
    fn native_content(self, content: impl Into<Element>) -> Self;
}

#[cfg(test)]
pub(crate) trait NativeChildrenTestExt: Sized {
    fn native_child(self, key: impl Into<Key>, child: impl Into<Element>) -> Self;
    fn native_children(self, children: impl IntoIterator<Item = KeyedElement>) -> Self;
}

/// A declarative Reactor subtree.
///
/// Positional fragments preserve identity by position. Keyed fragments preserve child identity by
/// [`Key`] as items are inserted, removed, or reordered.
#[derive(Clone, Debug, PartialEq)]
pub struct View(ViewKind);

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum ViewKind {
    Native(Element),
    Component(ComponentView),
    Fragment(Rc<Vec<KeyedView>>),
    Provider {
        provision: ContextProvision,
        child: Box<Self>,
    },
    Content {
        control: Element,
        content: Box<Self>,
    },
    Children {
        control: Element,
        children: Rc<Vec<KeyedView>>,
    },
    Slots {
        control: Element,
        slots: Rc<Vec<SlottedView>>,
    },
    Tooltip {
        target: Box<Self>,
        tooltip: Tooltip,
    },
    Flyout {
        target: Box<Self>,
        flyout: Flyout,
    },
    Menu {
        target: Box<Self>,
        menu: Menu,
    },
    CommandBarFlyout {
        target: Box<Self>,
        flyout: CommandBarFlyout,
    },
    TreeNodes {
        tree: Box<Self>,
        nodes: Rc<Vec<TreeNode>>,
    },
    ContentDialog {
        dialog: Box<Self>,
        open: bool,
    },
}

/// Converts a statically shaped expression into positional views.
///
/// This trait is sealed. `()` represents no views, fixed-size arrays represent homogeneous
/// shapes, and tuples represent heterogeneous shapes. Dynamic collections require
/// [`ChildrenControl::keyed_children`] or [`View::keyed_fragment`].
///
/// A `Vec` cannot supply positional children:
///
/// ```compile_fail
/// use windows_reactor::*;
///
/// let dynamic: Vec<View> = vec![TextBlock::new().into()];
/// let _ = StackPanel::new().children(dynamic);
/// ```
///
/// Iterator adapters cannot supply positional children:
///
/// ```compile_fail
/// use windows_reactor::*;
///
/// let dynamic = (0..3).map(|index| TextBlock::new().text(index.to_string()));
/// let _ = StackPanel::new().children(dynamic);
/// ```
pub trait IntoViews: sealed::StaticViews {}

/// Placement of a tooltip relative to its target.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum TooltipPlacement {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
    Mouse,
}

/// Text or view content displayed as a tooltip.
#[derive(Clone, Debug, PartialEq)]
pub struct Tooltip {
    pub(crate) content: Box<View>,
    pub(crate) placement: TooltipPlacement,
}

impl Tooltip {
    /// Creates a text tooltip with top placement.
    pub fn text(value: impl Into<String>) -> Self {
        Self::rich(TextBlock::new().text(value))
    }

    /// Creates a rich-content tooltip with top placement.
    pub fn rich(content: impl Into<View>) -> Self {
        Self {
            content: Box::new(content.into()),
            placement: TooltipPlacement::Top,
        }
    }

    /// Sets the preferred placement.
    pub fn placement(mut self, placement: TooltipPlacement) -> Self {
        self.placement = placement;
        self
    }
}

/// Adds a tooltip to a view.
pub trait TooltipExt: Into<View> + Sized {
    fn tooltip(self, value: impl Into<String>) -> View {
        self.tooltip_with(Tooltip::text(value))
    }

    fn tooltip_with(self, tooltip: Tooltip) -> View {
        View(ViewKind::Tooltip {
            target: Box::new(self.into().into_kind()),
            tooltip,
        })
    }
}

impl<T> TooltipExt for T where T: Into<View> {}

/// Placement of a flyout relative to its target.
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

/// Text or view content displayed in a flyout.
#[derive(Clone, Debug, PartialEq)]
pub struct Flyout {
    pub(crate) content: Box<View>,
    pub(crate) placement: FlyoutPlacement,
}

impl Flyout {
    /// Creates a text flyout with top placement.
    pub fn text(value: impl Into<String>) -> Self {
        Self::rich(TextBlock::new().text(value))
    }

    /// Creates a rich-content flyout with top placement.
    pub fn rich(content: impl Into<View>) -> Self {
        Self {
            content: Box::new(content.into()),
            placement: FlyoutPlacement::Top,
        }
    }

    /// Sets the preferred placement.
    pub fn placement(mut self, placement: FlyoutPlacement) -> Self {
        self.placement = placement;
        self
    }
}

/// Adds a flyout to a view.
pub trait FlyoutExt: Into<View> + Sized {
    fn flyout(self, value: impl Into<String>) -> View {
        self.flyout_with(Flyout::text(value))
    }

    fn flyout_with(self, flyout: Flyout) -> View {
        View(ViewKind::Flyout {
            target: Box::new(self.into().into_kind()),
            flyout,
        })
    }
}

impl<T> FlyoutExt for T where T: Into<View> {}

/// A keyed item in a context menu.
#[derive(Clone, Debug, PartialEq)]
pub enum MenuItem {
    Item {
        key: Key,
        label: String,
        enabled: bool,
    },
    Separator {
        key: Key,
    },
    Submenu {
        key: Key,
        label: String,
        items: Vec<Self>,
    },
}

impl MenuItem {
    /// Creates an enabled command item.
    pub fn item(key: impl Into<Key>, label: impl Into<String>) -> Self {
        Self::Item {
            key: key.into(),
            label: label.into(),
            enabled: true,
        }
    }

    /// Creates a disabled command item.
    pub fn disabled(key: impl Into<Key>, label: impl Into<String>) -> Self {
        Self::Item {
            key: key.into(),
            label: label.into(),
            enabled: false,
        }
    }

    /// Creates a separator.
    pub fn separator(key: impl Into<Key>) -> Self {
        Self::Separator { key: key.into() }
    }

    /// Creates a nested submenu.
    pub fn submenu(
        key: impl Into<Key>,
        label: impl Into<String>,
        items: impl IntoIterator<Item = Self>,
    ) -> Self {
        Self::Submenu {
            key: key.into(),
            label: label.into(),
            items: items.into_iter().collect(),
        }
    }

    pub(crate) fn key(&self) -> &Key {
        match self {
            Self::Item { key, .. } | Self::Separator { key } | Self::Submenu { key, .. } => key,
        }
    }
}

/// A context menu whose click callback receives the selected item's label.
#[derive(Clone, Debug, PartialEq)]
pub struct Menu {
    pub(crate) items: Vec<MenuItem>,
    pub(crate) on_click: Callback<String>,
}

impl Menu {
    /// Creates a menu and a callback that receives the selected item's label.
    pub fn new(
        items: impl IntoIterator<Item = MenuItem>,
        on_click: impl IntoPayloadCallback<String>,
    ) -> Self {
        Self {
            items: items.into_iter().collect(),
            on_click: on_click.into_payload_callback(),
        }
    }
}

/// Adds a context menu to a view.
pub trait MenuExt: Into<View> + Sized {
    fn menu(self, menu: Menu) -> View {
        View(ViewKind::Menu {
            target: Box::new(self.into().into_kind()),
            menu,
        })
    }
}

impl<T> MenuExt for T where T: Into<View> {}

/// A keyed command owned by a command bar or command-bar flyout.
#[derive(Clone, Debug, PartialEq)]
pub enum CommandBarCommand {
    Button {
        key: Key,
        label: String,
        icon: Option<Symbol>,
        enabled: bool,
    },
    Separator {
        key: Key,
    },
}

impl CommandBarCommand {
    pub fn button(key: impl Into<Key>, label: impl Into<String>) -> Self {
        Self::Button {
            key: key.into(),
            label: label.into(),
            icon: None,
            enabled: true,
        }
    }

    pub fn button_with_icon(key: impl Into<Key>, label: impl Into<String>, icon: Symbol) -> Self {
        Self::Button {
            key: key.into(),
            label: label.into(),
            icon: Some(icon),
            enabled: true,
        }
    }

    pub fn separator(key: impl Into<Key>) -> Self {
        Self::Separator { key: key.into() }
    }

    pub(crate) fn key(&self) -> &Key {
        match self {
            Self::Button { key, .. } | Self::Separator { key } => key,
        }
    }

    fn into_keyed_view(self, on_click: &Callback<String>) -> KeyedView {
        match self {
            Self::Button {
                key,
                label,
                icon,
                enabled,
            } => {
                let callback = on_click.clone();
                let clicked = label.clone();
                let button = AppBarButton::new()
                    .label(label)
                    .is_enabled(enabled)
                    .on_click(move || {
                        let _ = callback.call(clicked.clone());
                    });
                let view = match icon {
                    Some(icon) => button.slots([SlotView::new(
                        AppBarButtonSlot::Icon,
                        SymbolIcon::new().symbol(icon),
                    )]),
                    None => button.into(),
                };
                KeyedView { key, view }
            }
            Self::Separator { key } => KeyedView {
                key,
                view: AppBarSeparator::new().into(),
            },
        }
    }
}

impl CommandBar {
    pub fn owned_commands(
        self,
        primary: impl IntoIterator<Item = CommandBarCommand>,
        secondary: impl IntoIterator<Item = CommandBarCommand>,
        on_click: impl IntoPayloadCallback<String>,
    ) -> View {
        let on_click = on_click.into_payload_callback();
        self.slots([
            SlotView::collection(
                CommandBarSlot::PrimaryCommands,
                primary
                    .into_iter()
                    .map(|command| command.into_keyed_view(&on_click)),
            ),
            SlotView::collection(
                CommandBarSlot::SecondaryCommands,
                secondary
                    .into_iter()
                    .map(|command| command.into_keyed_view(&on_click)),
            ),
        ])
    }
}

/// Primary and secondary commands displayed in a command-bar flyout.
#[derive(Clone, Debug, PartialEq)]
pub struct CommandBarFlyout {
    pub(crate) primary: Vec<CommandBarCommand>,
    pub(crate) secondary: Vec<CommandBarCommand>,
    pub(crate) on_click: Callback<String>,
}

impl CommandBarFlyout {
    pub fn new(
        primary: impl IntoIterator<Item = CommandBarCommand>,
        secondary: impl IntoIterator<Item = CommandBarCommand>,
        on_click: impl IntoPayloadCallback<String>,
    ) -> Self {
        Self {
            primary: primary.into_iter().collect(),
            secondary: secondary.into_iter().collect(),
            on_click: on_click.into_payload_callback(),
        }
    }
}

/// Adds a command-bar flyout to a view.
pub trait CommandBarFlyoutExt: Into<View> + Sized {
    fn command_bar_flyout(self, flyout: CommandBarFlyout) -> View {
        View(ViewKind::CommandBarFlyout {
            target: Box::new(self.into().into_kind()),
            flyout,
        })
    }
}

impl<T> CommandBarFlyoutExt for T where T: Into<View> {}

/// Paragraph content for a [`RichTextBlock`].
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RichText {
    pub(crate) paragraphs: Rc<Vec<RichTextParagraph>>,
}

impl RichText {
    /// Creates rich text from paragraphs.
    pub fn new(paragraphs: impl IntoIterator<Item = RichTextParagraph>) -> Self {
        Self {
            paragraphs: Rc::new(paragraphs.into_iter().collect()),
        }
    }

    /// Creates rich text containing one paragraph.
    pub fn single_paragraph(inlines: impl IntoIterator<Item = RichTextInline>) -> Self {
        Self::new([RichTextParagraph::new(inlines)])
    }
}

/// A paragraph of rich-text inline values.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RichTextParagraph {
    pub(crate) inlines: Vec<RichTextInline>,
}

impl RichTextParagraph {
    /// Creates a paragraph from inline values.
    pub fn new(inlines: impl IntoIterator<Item = RichTextInline>) -> Self {
        Self {
            inlines: inlines.into_iter().collect(),
        }
    }
}

/// An inline run, hyperlink, or line break in rich text.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RichTextInline {
    Run(RichTextRun),
    Hyperlink(RichTextHyperlink),
    LineBreak,
}

/// A text run with optional bold and italic styling.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RichTextRun {
    pub text: String,
    pub is_bold: bool,
    pub is_italic: bool,
}

impl RichTextRun {
    /// Creates an unstyled text run.
    pub fn plain(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            ..Self::default()
        }
    }
}

/// A hyperlink with display text and a target URI.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RichTextHyperlink {
    pub text: String,
    pub uri: String,
}

/// A keyed node in a tree view.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TreeNode {
    pub(crate) key: Key,
    pub(crate) text: String,
    pub(crate) expanded: bool,
    pub(crate) children: Vec<Self>,
}

impl TreeNode {
    /// Creates a collapsed leaf node.
    pub fn new(key: impl Into<Key>, text: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            text: text.into(),
            expanded: false,
            children: Vec::new(),
        }
    }

    /// Sets whether the node is expanded.
    pub fn expanded(mut self, value: bool) -> Self {
        self.expanded = value;
        self
    }

    /// Replaces the node's children.
    pub fn children(mut self, children: impl IntoIterator<Item = Self>) -> Self {
        self.children = children.into_iter().collect();
        self
    }

    /// Appends one child node.
    pub fn child(mut self, child: Self) -> Self {
        self.children.push(child);
        self
    }
}

/// Supplies a keyed node hierarchy to a tree view.
pub trait TreeViewExt: Into<View> + Sized {
    fn nodes(self, nodes: impl IntoIterator<Item = TreeNode>) -> View {
        View(ViewKind::TreeNodes {
            tree: Box::new(self.into().into_kind()),
            nodes: Rc::new(nodes.into_iter().collect()),
        })
    }
}

impl<T> TreeViewExt for T where T: Into<View> {}

impl View {
    /// Creates a fragment with no children.
    pub fn empty() -> Self {
        Self::fragment(())
    }

    pub(crate) fn native(control: impl Into<Element>) -> Self {
        Self(ViewKind::Native(control.into()))
    }

    /// Creates a component view from its input.
    pub fn component<C: Component>(input: C::Input) -> Self {
        Self(ViewKind::Component(ComponentView::new::<C>(input)))
    }

    /// Creates a statically shaped fragment whose children are identified by position.
    pub fn fragment(children: impl IntoViews) -> Self {
        Self(ViewKind::Fragment(positioned(children)))
    }

    /// Creates a dynamic fragment whose children are reconciled by key.
    pub fn keyed_fragment<T>(children: impl IntoIterator<Item = T>) -> Self
    where
        T: Into<KeyedView>,
    {
        Self(ViewKind::Fragment(Rc::new(
            children.into_iter().map(Into::into).collect(),
        )))
    }

    /// Provides a context value to `child` and its descendants.
    pub fn provide<T>(context: &Context<T>, value: T, child: impl Into<Self>) -> Self
    where
        T: Clone + PartialEq + 'static,
    {
        Self(ViewKind::Provider {
            provision: ContextProvision::new(context, value),
            child: Box::new(child.into().into_kind()),
        })
    }

    pub(crate) fn from_kind(kind: ViewKind) -> Self {
        Self(kind)
    }

    pub(crate) fn content_dialog(dialog: Element, content: Option<Self>, open: bool) -> Self {
        let dialog = match content {
            Some(content) => Self(ViewKind::Content {
                control: dialog,
                content: Box::new(content.into_kind()),
            }),
            None => Self::native(dialog),
        };
        Self(ViewKind::ContentDialog {
            dialog: Box::new(dialog.into_kind()),
            open,
        })
    }

    pub(crate) fn as_kind(&self) -> &ViewKind {
        &self.0
    }

    pub(crate) fn into_kind(self) -> ViewKind {
        self.0
    }
}

/// Content assigned to one typed control slot.
#[derive(Clone, Debug, PartialEq)]
pub struct SlotView<S> {
    slot: S,
    content: SlotContent,
}

impl<S> SlotView<S> {
    /// Creates a slot containing one view.
    pub fn new(slot: S, view: impl Into<View>) -> Self {
        Self {
            slot,
            content: SlotContent::Single(view.into()),
        }
    }

    /// Creates a collection slot whose children are reconciled by key.
    pub fn collection<T>(slot: S, children: impl IntoIterator<Item = T>) -> Self
    where
        T: Into<KeyedView>,
    {
        Self {
            slot,
            content: SlotContent::Collection(Rc::new(
                children.into_iter().map(Into::into).collect(),
            )),
        }
    }

    fn into_parts(self) -> (S, SlotContent) {
        (self.slot, self.content)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum SlotContent {
    Single(View),
    Collection(Rc<Vec<KeyedView>>),
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SlottedView {
    pub(crate) slot: SlotId,
    pub(crate) content: SlotContent,
}

impl From<Element> for View {
    fn from(value: Element) -> Self {
        Self(ViewKind::Native(value))
    }
}

impl From<String> for View {
    fn from(value: String) -> Self {
        TextBlock::new().text(value).into()
    }
}

impl From<&str> for View {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

/// A view paired with stable reconciliation identity.
#[derive(Clone, Debug, PartialEq)]
pub struct KeyedView {
    key: Key,
    view: View,
}

impl KeyedView {
    /// Associates `view` with `key`.
    pub fn new(key: impl Into<Key>, view: impl Into<View>) -> Self {
        Self {
            key: key.into(),
            view: view.into(),
        }
    }

    pub fn key(&self) -> &Key {
        &self.key
    }

    pub fn view(&self) -> &View {
        &self.view
    }

    pub(crate) fn into_parts(self) -> (Key, View) {
        (self.key, self.view)
    }

    fn position(position: usize, view: View) -> Self {
        Self {
            key: Key::position(position),
            view,
        }
    }
}

impl<K, V> From<(K, V)> for KeyedView
where
    K: Into<Key>,
    V: Into<View>,
{
    fn from((key, view): (K, V)) -> Self {
        Self::new(key, view)
    }
}

/// A lazily materialized, keyed source for a virtualizing control.
///
/// Item functions are called only for indices that the control needs to realize.
#[derive(Clone)]
pub struct VirtualSource {
    key_revision: u64,
    len: usize,
    key: Rc<dyn Fn(usize) -> Key>,
    view: Rc<dyn Fn(usize) -> View>,
}

impl VirtualSource {
    /// Creates an indexed source whose item views are built only when needed.
    ///
    /// `key_revision` must change whenever the length, order, or value of any key changes. It may
    /// remain unchanged when only item view data changes. The key and view functions are called
    /// only with indices less than `len`.
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

    fn key(&self, index: usize) -> Key {
        (self.key)(index)
    }

    fn view(&self, index: usize) -> View {
        (self.view)(index)
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
    Eager(Rc<Vec<KeyedView>>),
    Lazy(VirtualSource),
}

impl Default for VirtualItems {
    fn default() -> Self {
        Self::Eager(Rc::default())
    }
}

impl VirtualItems {
    pub(crate) fn len(&self) -> usize {
        match self {
            Self::Eager(items) => items.len(),
            Self::Lazy(source) => source.len(),
        }
    }

    pub(crate) fn key(&self, index: usize) -> Option<Key> {
        match self {
            Self::Eager(items) => items.get(index).map(|item| item.key().clone()),
            Self::Lazy(source) => (index < source.len()).then(|| source.key(index)),
        }
    }

    pub(crate) fn view(&self, index: usize) -> Option<View> {
        match self {
            Self::Eager(items) => items.get(index).map(|item| item.view().clone()),
            Self::Lazy(source) => (index < source.len()).then(|| source.view(index)),
        }
    }

    pub(crate) fn changed_keys(&self, previous: &Self, keys: &[Key]) -> Option<Vec<Key>> {
        if let (Self::Lazy(current), Self::Lazy(previous)) = (self, previous)
            && current.key_revision() == previous.key_revision()
            && current.len() == previous.len()
            && current.len() == keys.len()
        {
            return None;
        }
        if let Self::Eager(items) = self
            && items.len() == keys.len()
            && keys
                .iter()
                .zip(items.iter())
                .all(|(key, item)| key == item.key())
        {
            return None;
        }
        let next = (0..self.len())
            .map(|index| self.key(index).unwrap())
            .collect::<Vec<_>>();
        (next != keys).then_some(next)
    }
}

fn positioned(children: impl IntoViews) -> Rc<Vec<KeyedView>> {
    Rc::new(sealed::StaticViews::into_positioned(children))
}

impl sealed::StaticViews for () {
    fn into_positioned(self) -> Vec<KeyedView> {
        Vec::new()
    }
}

impl IntoViews for () {}

impl<T, const N: usize> sealed::StaticViews for [T; N]
where
    T: Into<View>,
{
    fn into_positioned(self) -> Vec<KeyedView> {
        self.into_iter()
            .enumerate()
            .map(|(position, view)| KeyedView::position(position, view.into()))
            .collect()
    }
}

impl<T, const N: usize> IntoViews for [T; N] where T: Into<View> {}

macro_rules! impl_into_views_tuple {
    ($($type:ident $index:tt),+ $(,)?) => {
        impl<$($type),+> sealed::StaticViews for ($($type,)+)
        where
            $($type: Into<View>,)+
        {
            fn into_positioned(self) -> Vec<KeyedView> {
                vec![$(KeyedView::position($index, self.$index.into())),+]
            }
        }

        impl<$($type),+> IntoViews for ($($type,)+)
        where
            $($type: Into<View>,)+
        {
        }
    };
}

impl_into_views_tuple!(A 0);
impl_into_views_tuple!(A 0, B 1);
impl_into_views_tuple!(A 0, B 1, C 2);
impl_into_views_tuple!(A 0, B 1, C 2, D 3);
impl_into_views_tuple!(A 0, B 1, C 2, D 3, E 4);
impl_into_views_tuple!(A 0, B 1, C 2, D 3, E 4, F 5);
impl_into_views_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6);
impl_into_views_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7);
impl_into_views_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8);
impl_into_views_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9);
impl_into_views_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10);
impl_into_views_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11);
impl_into_views_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12);
impl_into_views_tuple!(
    A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13
);
impl_into_views_tuple!(
    A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14
);
impl_into_views_tuple!(
    A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15
);

#[derive(Clone, Debug)]
enum FourValues {
    Uniform(f64),
    Values(Rc<[f64; 4]>),
}

impl FourValues {
    fn new(values: [f64; 4]) -> Self {
        if values.iter().all(|value| *value == values[0]) {
            Self::Uniform(values[0])
        } else {
            Self::Values(Rc::new(values))
        }
    }

    fn values(&self) -> [f64; 4] {
        match self {
            Self::Uniform(value) => [*value; 4],
            Self::Values(values) => **values,
        }
    }
}

/// Four edge values measured in device-independent pixels (DIPs).
#[derive(Clone, Debug)]
pub struct Thickness(FourValues);

impl Thickness {
    /// Uses the same DIP value for all four edges.
    pub fn uniform(value: f64) -> Self {
        Self(FourValues::Uniform(value))
    }

    /// Uses one DIP value for horizontal edges and another for vertical edges.
    pub fn xy(horizontal: f64, vertical: f64) -> Self {
        Self::new(horizontal, vertical, horizontal, vertical)
    }

    /// Creates edge values in left, top, right, bottom order, measured in DIPs.
    pub fn new(left: f64, top: f64, right: f64, bottom: f64) -> Self {
        Self(FourValues::new([left, top, right, bottom]))
    }

    pub fn left(&self) -> f64 {
        self.values()[0]
    }

    pub fn top(&self) -> f64 {
        self.values()[1]
    }

    pub fn right(&self) -> f64 {
        self.values()[2]
    }

    pub fn bottom(&self) -> f64 {
        self.values()[3]
    }

    pub(crate) fn values(&self) -> [f64; 4] {
        self.0.values()
    }

    pub(crate) fn is_finite_non_negative(&self) -> bool {
        self.values()
            .into_iter()
            .all(|value| value.is_finite() && value >= 0.0)
    }

    pub(crate) fn is_finite(&self) -> bool {
        self.values().into_iter().all(f64::is_finite)
    }
}

impl Default for Thickness {
    fn default() -> Self {
        Self::uniform(0.0)
    }
}

impl From<f64> for Thickness {
    fn from(value: f64) -> Self {
        Self::uniform(value)
    }
}

impl PartialEq for Thickness {
    fn eq(&self, other: &Self) -> bool {
        self.values() == other.values()
    }
}

/// Four corner radii measured in device-independent pixels (DIPs).
#[derive(Clone, Debug)]
pub struct CornerRadius(FourValues);

impl CornerRadius {
    /// Uses the same DIP radius for all four corners.
    pub fn uniform(value: f64) -> Self {
        Self(FourValues::Uniform(value))
    }

    /// Creates radii in top-left, top-right, bottom-right, bottom-left order.
    pub fn new(top_left: f64, top_right: f64, bottom_right: f64, bottom_left: f64) -> Self {
        Self(FourValues::new([
            top_left,
            top_right,
            bottom_right,
            bottom_left,
        ]))
    }

    pub fn top_left(&self) -> f64 {
        self.values()[0]
    }

    pub fn top_right(&self) -> f64 {
        self.values()[1]
    }

    pub fn bottom_right(&self) -> f64 {
        self.values()[2]
    }

    pub fn bottom_left(&self) -> f64 {
        self.values()[3]
    }

    pub(crate) fn values(&self) -> [f64; 4] {
        self.0.values()
    }

    pub(crate) fn is_finite_non_negative(&self) -> bool {
        self.values()
            .into_iter()
            .all(|value| value.is_finite() && value >= 0.0)
    }
}

impl Default for CornerRadius {
    fn default() -> Self {
        Self::uniform(0.0)
    }
}

impl From<f64> for CornerRadius {
    fn from(value: f64) -> Self {
        Self::uniform(value)
    }
}

impl PartialEq for CornerRadius {
    fn eq(&self, other: &Self) -> bool {
        self.values() == other.values()
    }
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
pub struct ResourceOverrides {
    values: std::collections::BTreeMap<String, ResourceValue>,
}

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
            ResourceValue::Thickness(value) => {
                assert!(
                    value.is_finite_non_negative(),
                    "resource override thickness must be finite and non-negative"
                );
            }
            ResourceValue::CornerRadius(value) => {
                assert!(
                    value.is_finite_non_negative(),
                    "resource override corner radius must be finite and non-negative"
                );
            }
        }
        self.values.insert(key, value);
        self
    }

    pub(crate) fn values(&self) -> impl Iterator<Item = (&str, &ResourceValue)> {
        self.values.iter().map(|(key, value)| (key.as_str(), value))
    }
}

/// The theme requested for a window.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum WindowTheme {
    #[default]
    System,
    Light,
    Dark,
}

/// Window client dimensions in device-independent pixels (DIPs).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WindowSize {
    pub width: f64,
    pub height: f64,
}

/// Whether the active application color scheme is light or dark.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ColorScheme {
    #[default]
    Light,
    Dark,
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

    pub(crate) fn accepts(&self, kind: DragKind) -> bool {
        match kind {
            DragKind::StorageItems => self.storage_items.is_some(),
            DragKind::Text => self.text.is_some(),
            DragKind::Unsupported => false,
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

/// Material used behind a window's content.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum WindowBackdrop {
    #[default]
    None,
    Mica,
    MicaAlt,
    Acrylic,
}

/// Height preset for an extended window title bar.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum WindowTitleBarHeight {
    #[default]
    Standard,
    Tall,
}

/// Optional window client-size limits in device-independent pixels (DIPs).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WindowConstraints {
    pub min_width: Option<f64>,
    pub min_height: Option<f64>,
    pub max_width: Option<f64>,
    pub max_height: Option<f64>,
}

impl WindowConstraints {
    fn validate(self) {
        for value in [
            self.min_width,
            self.min_height,
            self.max_width,
            self.max_height,
        ]
        .into_iter()
        .flatten()
        {
            assert!(
                value.is_finite() && value > 0.0,
                "window constraints must be finite and positive"
            );
        }
        assert!(
            self.min_width
                .zip(self.max_width)
                .is_none_or(|(min, max)| min <= max)
                && self
                    .min_height
                    .zip(self.max_height)
                    .is_none_or(|(min, max)| min <= max),
            "window minimum constraints must not exceed maximum constraints"
        );
    }
}

/// Window appearance and client sizing requested by a component publication.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WindowVisuals {
    pub(crate) backdrop: WindowBackdrop,
    pub(crate) client_size: Option<(f64, f64)>,
    pub(crate) constraints: Option<WindowConstraints>,
    pub(crate) icon: Option<&'static str>,
    pub(crate) theme: WindowTheme,
}

impl WindowVisuals {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn backdrop(mut self, backdrop: WindowBackdrop) -> Self {
        self.backdrop = backdrop;
        self
    }

    /// Sets the initial client size in DIPs.
    ///
    /// # Panics
    ///
    /// Panics unless both dimensions are finite and positive.
    pub fn client_size(mut self, width: f64, height: f64) -> Self {
        assert!(
            width.is_finite() && width > 0.0 && height.is_finite() && height > 0.0,
            "window client size must be finite and positive"
        );
        self.client_size = Some((width, height));
        self
    }

    /// Sets the path to the window icon.
    ///
    /// # Panics
    ///
    /// Panics if `path` is empty.
    pub fn icon(mut self, path: &'static str) -> Self {
        assert!(!path.is_empty(), "window icon path must not be empty");
        self.icon = Some(path);
        self
    }

    /// Sets client-size constraints in DIPs.
    ///
    /// # Panics
    ///
    /// Panics if a bound is non-positive or non-finite, or a minimum exceeds its maximum.
    pub fn constraints(mut self, constraints: WindowConstraints) -> Self {
        constraints.validate();
        self.constraints = Some(constraints);
        self
    }

    pub fn theme(mut self, theme: WindowTheme) -> Self {
        self.theme = theme;
        self
    }
}

#[cfg(test)]
mod visual_value_tests {
    use super::*;
    use crate::core::ThemeStyle;
    use std::mem::size_of;

    #[test]
    fn four_value_types_keep_compact_layout_and_semantic_equality() {
        assert_eq!(size_of::<Thickness>(), 16);
        assert_eq!(size_of::<CornerRadius>(), 16);
        assert_eq!(size_of::<ThemeStyle>(), 4);

        assert_eq!(Thickness::uniform(3.0), Thickness::new(3.0, 3.0, 3.0, 3.0));
        assert_eq!(
            CornerRadius::uniform(4.0),
            CornerRadius::new(4.0, 4.0, 4.0, 4.0)
        );
        assert_eq!(Thickness::xy(2.0, 5.0).values(), [2.0, 5.0, 2.0, 5.0]);
    }

    #[test]
    fn window_client_size_rejects_invalid_values() {
        for (width, height) in [
            (0.0, 1.0),
            (1.0, -1.0),
            (f64::NAN, 1.0),
            (1.0, f64::INFINITY),
        ] {
            assert!(
                std::panic::catch_unwind(|| {
                    WindowVisuals::new().client_size(width, height);
                })
                .is_err()
            );
        }
    }
}

/// A Grid row or column size.
#[derive(Clone, Copy, Debug)]
pub enum GridLength {
    /// Sizes to the content.
    Auto,
    /// Uses a fixed number of device-independent pixels (DIPs).
    Pixel(f64),
    /// Uses a weighted share of the remaining space.
    Star(f64),
}

impl GridLength {
    /// One weighted share of the remaining space.
    pub const STAR: Self = Self::Star(1.0);

    pub(crate) fn is_valid(self) -> bool {
        match self {
            Self::Auto => true,
            Self::Pixel(value) | Self::Star(value) => value.is_finite() && value >= 0.0,
        }
    }
}

impl PartialEq for GridLength {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Auto, Self::Auto) => true,
            (Self::Pixel(left), Self::Pixel(right)) | (Self::Star(left), Self::Star(right)) => {
                f64_eq(*left, *right)
            }
            _ => false,
        }
    }
}

/// Horizontal placement within the space assigned by a parent.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HorizontalAlignment {
    Left,
    Center,
    Right,
    Stretch,
}

/// Vertical placement within the space assigned by a parent.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VerticalAlignment {
    Top,
    Center,
    Bottom,
    Stretch,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct ElementState {
    width: Property<f64>,
    height: Property<f64>,
    min_width: Property<f64>,
    max_width: Property<f64>,
    min_height: Property<f64>,
    max_height: Property<f64>,
    opacity: Property<f64>,
    horizontal_alignment: Property<HorizontalAlignment>,
    vertical_alignment: Property<VerticalAlignment>,
    margin: Property<Thickness>,
    row: Option<i32>,
    column: Option<i32>,
    row_span: Option<i32>,
    column_span: Option<i32>,
    relative_align_left: bool,
    relative_align_top: bool,
    relative_align_right: bool,
    relative_align_bottom: bool,
    relative_align_horizontal_center: bool,
    relative_align_vertical_center: bool,
    canvas_left: Option<f64>,
    canvas_top: Option<f64>,
    automation_name: Option<String>,
    automation_id: Option<String>,
    automation_heading_level: Option<AutomationHeadingLevel>,
    exit_transition: Option<ExitTransition>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) enum Property<T> {
    #[default]
    Inherited,
    Set(T),
}

impl<T> Property<T> {
    pub(crate) fn as_set(&self) -> Option<&T> {
        match self {
            Self::Inherited => None,
            Self::Set(value) => Some(value),
        }
    }
}

impl<T> From<Option<T>> for Property<T> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(value) => Self::Set(value),
            None => Self::Inherited,
        }
    }
}

pub(crate) fn f64_eq(left: f64, right: f64) -> bool {
    left == right || left.is_nan() && right.is_nan()
}

pub(crate) fn f64_property_eq(left: &Property<f64>, right: &Property<f64>) -> bool {
    match (left, right) {
        (Property::Inherited, Property::Inherited) => true,
        (Property::Set(left), Property::Set(right)) => f64_eq(*left, *right),
        _ => false,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct CallbackSource {
    queue: usize,
    token: ComponentToken,
}

impl CallbackSource {
    pub(crate) fn new(queue: usize, token: ComponentToken) -> Self {
        Self { queue, token }
    }
}

trait ErasedCallbackIdentity {
    fn as_any(&self) -> &dyn Any;
    fn equals(&self, other: &dyn ErasedCallbackIdentity) -> bool;
}

struct TypedCallbackIdentity<K> {
    key: K,
    source: CallbackSource,
}

impl<K: PartialEq + 'static> ErasedCallbackIdentity for TypedCallbackIdentity<K> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn equals(&self, other: &dyn ErasedCallbackIdentity) -> bool {
        other
            .as_any()
            .downcast_ref::<Self>()
            .is_some_and(|other| self.source == other.source && self.key == other.key)
    }
}

/// A clonable callback used by Reactor events.
///
/// Clones compare equal and retain the same function. Callbacks made by a [`LocalSender`] from a
/// captureless mapper can also compare equal across publications.
pub struct Callback<T> {
    callback: Rc<dyn Fn(T) -> bool>,
    identity: Option<Rc<dyn ErasedCallbackIdentity>>,
}

impl<T> Callback<T> {
    /// Wraps a callback that always reports accepted delivery.
    pub fn new(callback: impl Fn(T) + 'static) -> Self {
        Self::new_with_acceptance(move |value| {
            callback(value);
            true
        })
    }

    pub(crate) fn new_with_acceptance(callback: impl Fn(T) -> bool + 'static) -> Self {
        Self {
            callback: Rc::new(callback),
            identity: None,
        }
    }

    pub(crate) fn new_identified<K>(
        source: CallbackSource,
        key: K,
        callback: impl Fn(T) -> bool + 'static,
    ) -> Self
    where
        K: PartialEq + 'static,
    {
        Self {
            callback: Rc::new(callback),
            identity: Some(Rc::new(TypedCallbackIdentity { key, source })),
        }
    }

    #[must_use = "false means the adapted message was rejected"]
    /// Calls the handler and returns whether it accepted the value.
    ///
    /// Sender-backed callbacks return `false` when their component message cannot be queued.
    pub fn call(&self, value: T) -> bool {
        (self.callback)(value)
    }
}

impl<T> Clone for Callback<T> {
    fn clone(&self) -> Self {
        Self {
            callback: Rc::clone(&self.callback),
            identity: self.identity.clone(),
        }
    }
}

impl<T> fmt::Debug for Callback<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("Callback")
            .field(&Rc::as_ptr(&self.callback))
            .finish()
    }
}

impl<T> PartialEq for Callback<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.callback, &other.callback)
            || self
                .identity
                .as_deref()
                .zip(other.identity.as_deref())
                .is_some_and(|(left, right)| left.equals(right))
    }
}

/// Converts a payload handler or typed message callback into an event callback.
pub trait IntoPayloadCallback<T> {
    fn into_payload_callback(self) -> Callback<T>;
}

impl<T, F> IntoPayloadCallback<T> for F
where
    F: Fn(T) + 'static,
{
    fn into_payload_callback(self) -> Callback<T> {
        Callback::new(self)
    }
}

impl<T> IntoPayloadCallback<T> for Callback<T> {
    fn into_payload_callback(self) -> Self {
        self
    }
}

/// Converts a zero-argument handler or typed message callback into an event callback.
pub trait IntoUnitCallback {
    fn into_unit_callback(self) -> Callback<()>;
}

impl<F> IntoUnitCallback for F
where
    F: Fn() + 'static,
{
    fn into_unit_callback(self) -> Callback<()> {
        Callback::new(move |()| self())
    }
}

impl IntoUnitCallback for Callback<()> {
    fn into_unit_callback(self) -> Self {
        self
    }
}

/// A key supported by Reactor keyboard accelerators.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AcceleratorKey {
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AutomationHeadingLevel {
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
    Level6,
    Level7,
    Level8,
    Level9,
}

/// A fade applied while an element is removed from the native tree.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExitTransition {
    duration: Duration,
}

impl ExitTransition {
    /// Creates a fade-out transition.
    ///
    /// # Panics
    ///
    /// Panics if `duration` is zero.
    pub fn fade(duration: Duration) -> Self {
        assert!(
            !duration.is_zero(),
            "Exit transition duration must be positive"
        );
        Self { duration }
    }

    pub fn duration(self) -> Duration {
        self.duration
    }
}

/// A keyboard accelerator and its callback.
#[derive(Clone, Debug, PartialEq)]
pub struct KeyAccelerator {
    pub(crate) key: AcceleratorKey,
    pub(crate) modifiers: AcceleratorModifiers,
    pub(crate) callback: Callback<()>,
}

impl KeyAccelerator {
    /// Creates an accelerator for `key` and `modifiers`.
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
pub struct KeyAccelerators {
    pub(crate) values: Vec<KeyAccelerator>,
}

impl KeyAccelerators {
    pub fn new(values: impl IntoIterator<Item = KeyAccelerator>) -> Self {
        Self {
            values: values.into_iter().collect(),
        }
    }
}

/// Applies layout, opacity, margin, and exit-transition properties to native controls.
///
/// Dimensions, margins, and Canvas positions use device-independent pixels (DIPs). Passing
/// `None` to an optional property leaves it inherited or unset.
#[allow(private_bounds)]
pub trait LayoutControl: sealed::LayoutControl {
    fn width(mut self, value: impl Into<Option<f64>>) -> Self
    where
        Self: Sized,
    {
        let value = value.into();
        assert!(
            value.is_none_or(|value| value.is_finite() && value >= 0.0),
            "Width must be finite and non-negative",
        );
        Rc::make_mut(
            sealed::LayoutControl::element_state_mut(&mut self)
                .get_or_insert_with(|| Rc::new(ElementState::default())),
        )
        .width = Property::from(value);
        self
    }

    fn height(mut self, value: impl Into<Option<f64>>) -> Self
    where
        Self: Sized,
    {
        let value = value.into();
        assert!(
            value.is_none_or(|value| value.is_finite() && value >= 0.0),
            "Height must be finite and non-negative",
        );
        Rc::make_mut(
            sealed::LayoutControl::element_state_mut(&mut self)
                .get_or_insert_with(|| Rc::new(ElementState::default())),
        )
        .height = Property::from(value);
        self
    }

    fn min_width(mut self, value: impl Into<Option<f64>>) -> Self
    where
        Self: Sized,
    {
        let value = value.into();
        assert!(
            value.is_none_or(|value| value.is_finite() && value >= 0.0),
            "Minimum width must be finite and non-negative",
        );
        Rc::make_mut(
            sealed::LayoutControl::element_state_mut(&mut self)
                .get_or_insert_with(|| Rc::new(ElementState::default())),
        )
        .min_width = Property::from(value);
        self
    }

    fn max_width(mut self, value: impl Into<Option<f64>>) -> Self
    where
        Self: Sized,
    {
        let value = value.into();
        assert!(
            value.is_none_or(|value| value.is_finite() && value >= 0.0),
            "Maximum width must be finite and non-negative",
        );
        Rc::make_mut(
            sealed::LayoutControl::element_state_mut(&mut self)
                .get_or_insert_with(|| Rc::new(ElementState::default())),
        )
        .max_width = Property::from(value);
        self
    }

    fn min_height(mut self, value: impl Into<Option<f64>>) -> Self
    where
        Self: Sized,
    {
        let value = value.into();
        assert!(
            value.is_none_or(|value| value.is_finite() && value >= 0.0),
            "Minimum height must be finite and non-negative",
        );
        Rc::make_mut(
            sealed::LayoutControl::element_state_mut(&mut self)
                .get_or_insert_with(|| Rc::new(ElementState::default())),
        )
        .min_height = Property::from(value);
        self
    }

    fn max_height(mut self, value: impl Into<Option<f64>>) -> Self
    where
        Self: Sized,
    {
        let value = value.into();
        assert!(
            value.is_none_or(|value| value.is_finite() && value >= 0.0),
            "Maximum height must be finite and non-negative",
        );
        Rc::make_mut(
            sealed::LayoutControl::element_state_mut(&mut self)
                .get_or_insert_with(|| Rc::new(ElementState::default())),
        )
        .max_height = Property::from(value);
        self
    }

    fn opacity(mut self, value: impl Into<Option<f64>>) -> Self
    where
        Self: Sized,
    {
        let value = value.into();
        assert!(
            value.is_none_or(|value| value.is_finite() && value >= 0.0),
            "Opacity must be finite and non-negative",
        );
        Rc::make_mut(
            sealed::LayoutControl::element_state_mut(&mut self)
                .get_or_insert_with(|| Rc::new(ElementState::default())),
        )
        .opacity = Property::from(value);
        self
    }

    fn horizontal_alignment(mut self, value: impl Into<Option<HorizontalAlignment>>) -> Self
    where
        Self: Sized,
    {
        Rc::make_mut(
            sealed::LayoutControl::element_state_mut(&mut self)
                .get_or_insert_with(|| Rc::new(ElementState::default())),
        )
        .horizontal_alignment = Property::from(value.into());
        self
    }

    fn vertical_alignment(mut self, value: impl Into<Option<VerticalAlignment>>) -> Self
    where
        Self: Sized,
    {
        Rc::make_mut(
            sealed::LayoutControl::element_state_mut(&mut self)
                .get_or_insert_with(|| Rc::new(ElementState::default())),
        )
        .vertical_alignment = Property::from(value.into());
        self
    }

    fn margin(mut self, value: impl Into<Thickness>) -> Self
    where
        Self: Sized,
    {
        let value = value.into();
        assert!(value.is_finite(), "Margin must be finite");
        Rc::make_mut(
            sealed::LayoutControl::element_state_mut(&mut self)
                .get_or_insert_with(|| Rc::new(ElementState::default())),
        )
        .margin = Property::Set(value);
        self
    }

    fn margin_optional<T>(mut self, value: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Thickness>,
    {
        let value = value.map(Into::into);
        assert!(
            value.as_ref().is_none_or(Thickness::is_finite),
            "Margin must be finite",
        );
        Rc::make_mut(
            sealed::LayoutControl::element_state_mut(&mut self)
                .get_or_insert_with(|| Rc::new(ElementState::default())),
        )
        .margin = Property::from(value);
        self
    }

    fn exit_transition(mut self, transition: ExitTransition) -> Self
    where
        Self: Sized,
    {
        Rc::make_mut(
            sealed::LayoutControl::element_state_mut(&mut self)
                .get_or_insert_with(|| Rc::new(ElementState::default())),
        )
        .exit_transition = Some(transition);
        self
    }
}

impl ElementState {
    pub(crate) fn exit_transition(&self) -> Option<ExitTransition> {
        self.exit_transition
    }
}

/// Assigns one content view to a native content control.
#[allow(private_bounds)]
pub trait ContentControl: sealed::ContentControl + Sized {
    fn content(self, content: impl Into<View>) -> View {
        sealed::ContentControl::into_content_view(self, content.into())
    }
}

/// Assigns children to a native container.
///
/// [`children`](Self::children) uses positional identity for static shapes.
/// [`keyed_children`](Self::keyed_children) preserves identity by key for dynamic collections.
#[allow(private_bounds)]
pub trait ChildrenControl: sealed::NativeControl + Sized {
    fn children(self, children: impl IntoViews) -> View {
        View(ViewKind::Children {
            control: sealed::NativeControl::into_element(self),
            children: positioned(children),
        })
    }

    fn keyed_children<T>(self, children: impl IntoIterator<Item = T>) -> View
    where
        T: Into<KeyedView>,
    {
        View(ViewKind::Children {
            control: sealed::NativeControl::into_element(self),
            children: Rc::new(children.into_iter().map(Into::into).collect()),
        })
    }
}

/// Assigns single views or keyed collections to a control's typed slots.
#[allow(private_bounds)]
pub trait SlotsControl: sealed::NativeControl + sealed::SlotIndex<Self::Slot> + Sized {
    type Slot: Copy;

    fn slot(self, slot: Self::Slot, view: impl Into<View>) -> View {
        self.slots([SlotView::new(slot, view)])
    }

    fn collection_slot<T>(self, slot: Self::Slot, children: impl IntoIterator<Item = T>) -> View
    where
        T: Into<KeyedView>,
    {
        self.slots([SlotView::collection(slot, children)])
    }

    fn slots(self, slots: impl IntoIterator<Item = SlotView<Self::Slot>>) -> View {
        let control = sealed::NativeControl::into_element(self);
        let kind = control.kind();
        let slots = slots
            .into_iter()
            .map(|slot| {
                let (slot, content) = slot.into_parts();
                SlottedView {
                    slot: slot_id(
                        kind,
                        <Self as sealed::SlotIndex<Self::Slot>>::slot_index(slot),
                    )
                    .unwrap(),
                    content,
                }
            })
            .collect();
        View(ViewKind::Slots {
            control,
            slots: Rc::new(slots),
        })
    }
}

/// Places a concrete native control in its parent Grid.
///
/// Components and fragments can produce more than one native root, so place a native wrapper when
/// a composed view needs Grid placement.
///
/// ```compile_fail
/// use windows_reactor::*;
///
/// struct Child;
/// # impl Component for Child {
/// #     type Message = ();
/// #     type Input = ();
/// #     fn create(_: &(), _: &ComponentContext<Self>) -> Self { Self }
/// #     fn update(&mut self, _: (), _: &ComponentContext<Self>) {}
/// #     fn view(&self, _: &(), _: &mut ViewContext<Self>) -> View { View::empty() }
/// # }
/// let _ = View::component::<Child>(()).grid_row(0);
/// ```
///
/// ```compile_fail
/// use windows_reactor::*;
///
/// let _ = View::fragment((TextBlock::new(), TextBlock::new())).grid_column(0);
/// ```
pub trait GridChildExt: LayoutControl + Sized {
    fn grid_row(mut self, row: i32) -> Self {
        assert!(row >= 0, "Grid row must be non-negative");
        Rc::make_mut(
            self.element_state_mut()
                .get_or_insert_with(|| Rc::new(ElementState::default())),
        )
        .row = Some(row);
        self
    }

    fn grid_column(mut self, column: i32) -> Self {
        assert!(column >= 0, "Grid column must be non-negative");
        Rc::make_mut(
            self.element_state_mut()
                .get_or_insert_with(|| Rc::new(ElementState::default())),
        )
        .column = Some(column);
        self
    }

    fn grid_row_span(mut self, span: i32) -> Self {
        assert!(span > 0, "Grid row span must be positive");
        Rc::make_mut(
            self.element_state_mut()
                .get_or_insert_with(|| Rc::new(ElementState::default())),
        )
        .row_span = Some(span);
        self
    }

    fn grid_column_span(mut self, span: i32) -> Self {
        assert!(span > 0, "Grid column span must be positive");
        Rc::make_mut(
            self.element_state_mut()
                .get_or_insert_with(|| Rc::new(ElementState::default())),
        )
        .column_span = Some(span);
        self
    }
}

impl<T: LayoutControl> GridChildExt for T {}

/// Places a concrete native control in its parent RelativePanel.
pub trait RelativePanelChildExt: LayoutControl + Sized {
    fn relative_align_left(mut self) -> Self {
        Rc::make_mut(
            self.element_state_mut()
                .get_or_insert_with(|| Rc::new(ElementState::default())),
        )
        .relative_align_left = true;
        self
    }

    fn relative_align_top(mut self) -> Self {
        Rc::make_mut(
            self.element_state_mut()
                .get_or_insert_with(|| Rc::new(ElementState::default())),
        )
        .relative_align_top = true;
        self
    }

    fn relative_align_right(mut self) -> Self {
        Rc::make_mut(
            self.element_state_mut()
                .get_or_insert_with(|| Rc::new(ElementState::default())),
        )
        .relative_align_right = true;
        self
    }

    fn relative_align_bottom(mut self) -> Self {
        Rc::make_mut(
            self.element_state_mut()
                .get_or_insert_with(|| Rc::new(ElementState::default())),
        )
        .relative_align_bottom = true;
        self
    }

    fn relative_align_horizontal_center(mut self) -> Self {
        Rc::make_mut(
            self.element_state_mut()
                .get_or_insert_with(|| Rc::new(ElementState::default())),
        )
        .relative_align_horizontal_center = true;
        self
    }

    fn relative_align_vertical_center(mut self) -> Self {
        Rc::make_mut(
            self.element_state_mut()
                .get_or_insert_with(|| Rc::new(ElementState::default())),
        )
        .relative_align_vertical_center = true;
        self
    }
}

impl<T: LayoutControl> RelativePanelChildExt for T {}

/// Places a concrete native control in its parent Canvas.
pub trait CanvasChildExt: LayoutControl + Sized {
    fn canvas_left(mut self, value: f64) -> Self {
        assert!(value.is_finite(), "Canvas left must be finite");
        Rc::make_mut(
            self.element_state_mut()
                .get_or_insert_with(|| Rc::new(ElementState::default())),
        )
        .canvas_left = Some(value);
        self
    }

    fn canvas_top(mut self, value: f64) -> Self {
        assert!(value.is_finite(), "Canvas top must be finite");
        Rc::make_mut(
            self.element_state_mut()
                .get_or_insert_with(|| Rc::new(ElementState::default())),
        )
        .canvas_top = Some(value);
        self
    }
}

impl<T: LayoutControl> CanvasChildExt for T {}

/// Adds UI Automation metadata to a native control.
pub trait AutomationExt: LayoutControl + Sized {
    fn automation_name(mut self, value: impl Into<String>) -> Self {
        Rc::make_mut(
            self.element_state_mut()
                .get_or_insert_with(|| Rc::new(ElementState::default())),
        )
        .automation_name = Some(value.into());
        self
    }

    fn automation_id(mut self, value: impl Into<String>) -> Self {
        Rc::make_mut(
            self.element_state_mut()
                .get_or_insert_with(|| Rc::new(ElementState::default())),
        )
        .automation_id = Some(value.into());
        self
    }

    fn automation_heading_level(mut self, value: AutomationHeadingLevel) -> Self {
        Rc::make_mut(
            self.element_state_mut()
                .get_or_insert_with(|| Rc::new(ElementState::default())),
        )
        .automation_heading_level = Some(value);
        self
    }
}

impl<T: LayoutControl> AutomationExt for T {}

pub(crate) fn visit_element_state(
    placement: Option<&ElementState>,
    visit: &mut dyn FnMut(PropertyId, Option<PropertyValueRef<'_>>),
) {
    visit(
        PropertyId::Width,
        placement
            .and_then(|value| value.width.as_set())
            .copied()
            .map(PropertyValueRef::F64),
    );
    visit(
        PropertyId::Height,
        placement
            .and_then(|value| value.height.as_set())
            .copied()
            .map(PropertyValueRef::F64),
    );
    visit(
        PropertyId::MinWidth,
        placement
            .and_then(|value| value.min_width.as_set())
            .copied()
            .map(PropertyValueRef::F64),
    );
    visit(
        PropertyId::MaxWidth,
        placement
            .and_then(|value| value.max_width.as_set())
            .copied()
            .map(PropertyValueRef::F64),
    );
    visit(
        PropertyId::MinHeight,
        placement
            .and_then(|value| value.min_height.as_set())
            .copied()
            .map(PropertyValueRef::F64),
    );
    visit(
        PropertyId::MaxHeight,
        placement
            .and_then(|value| value.max_height.as_set())
            .copied()
            .map(PropertyValueRef::F64),
    );
    visit(
        PropertyId::Opacity,
        placement
            .and_then(|value| value.opacity.as_set())
            .copied()
            .map(PropertyValueRef::F64),
    );
    visit(
        PropertyId::HorizontalAlignment,
        placement
            .and_then(|value| value.horizontal_alignment.as_set())
            .copied()
            .map(PropertyValueRef::HorizontalAlignment),
    );
    visit(
        PropertyId::VerticalAlignment,
        placement
            .and_then(|value| value.vertical_alignment.as_set())
            .copied()
            .map(PropertyValueRef::VerticalAlignment),
    );
    visit(
        PropertyId::Margin,
        placement
            .and_then(|value| value.margin.as_set())
            .map(PropertyValueRef::Thickness),
    );
    let value = |value: Option<i32>| value.map(PropertyValueRef::I32);
    visit(
        PropertyId::GridRow,
        value(placement.and_then(|value| value.row)),
    );
    visit(
        PropertyId::GridColumn,
        value(placement.and_then(|value| value.column)),
    );
    visit(
        PropertyId::GridRowSpan,
        value(placement.and_then(|value| value.row_span)),
    );
    visit(
        PropertyId::GridColumnSpan,
        value(placement.and_then(|value| value.column_span)),
    );
    let relative = |value: bool| value.then_some(PropertyValueRef::Bool(true));
    visit(
        PropertyId::RelativeAlignLeft,
        relative(placement.is_some_and(|value| value.relative_align_left)),
    );
    visit(
        PropertyId::RelativeAlignTop,
        relative(placement.is_some_and(|value| value.relative_align_top)),
    );
    visit(
        PropertyId::RelativeAlignRight,
        relative(placement.is_some_and(|value| value.relative_align_right)),
    );
    visit(
        PropertyId::RelativeAlignBottom,
        relative(placement.is_some_and(|value| value.relative_align_bottom)),
    );
    visit(
        PropertyId::RelativeAlignHorizontalCenter,
        relative(placement.is_some_and(|value| value.relative_align_horizontal_center)),
    );
    visit(
        PropertyId::RelativeAlignVerticalCenter,
        relative(placement.is_some_and(|value| value.relative_align_vertical_center)),
    );
    visit(
        PropertyId::CanvasLeft,
        placement
            .and_then(|value| value.canvas_left)
            .map(PropertyValueRef::F64),
    );
    visit(
        PropertyId::CanvasTop,
        placement
            .and_then(|value| value.canvas_top)
            .map(PropertyValueRef::F64),
    );
    visit(
        PropertyId::AutomationName,
        placement
            .and_then(|value| value.automation_name.as_deref())
            .map(PropertyValueRef::Str),
    );
    visit(
        PropertyId::AutomationId,
        placement
            .and_then(|value| value.automation_id.as_deref())
            .map(PropertyValueRef::Str),
    );
    visit(
        PropertyId::AutomationHeadingLevel,
        placement
            .and_then(|value| value.automation_heading_level)
            .map(|value| PropertyValueRef::I32(value as i32 + 1)),
    );
}
