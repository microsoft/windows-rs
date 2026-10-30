use super::bindings as native;
use super::*;

mod adapter;
mod attachments;
mod automation;
mod content_dialog;
mod dependency;
mod event_queue;
mod events;
mod images;
mod objects;
mod relations;
mod style;
mod virtualization;
mod window;

pub(crate) use adapter::SharedWinUiAdapter;
use attachments::*;
use automation::*;
use content_dialog::*;
use dependency::*;
use event_queue::*;
use images::*;
use objects::set_properties;
use relations::*;
use style::*;
use virtualization::*;
pub use window::*;

enum Handle {
    Generated(GeneratedHandle),
    TreeView(NativeTreeView),
    TreeNode(NativeTreeNode),
    Data(IObservableMap<HSTRING, IInspectable>),
}

struct NativeTreeView {
    value: native::TreeView,
    item_invoked: Rc<RefCell<NativeStringEvent>>,
    _item_invoked: windows_core::EventRevoker,
}

struct NativeTreeNode {
    value: native::TreeViewNode,
    text: HSTRING,
    content: Option<ObjectId>,
}

struct NativeValueEvent<T> {
    subscription: Option<NativeSubscription<T>>,
}

/// A subscribed native event. Replacing the callback keeps the subscription; removing it ends the
/// subscription and invalidates occurrences still queued or pending under it.
struct NativeSubscription<T> {
    callback: Callback<T>,
    live: Rc<Cell<bool>>,
}

impl<T> Clone for NativeSubscription<T> {
    fn clone(&self) -> Self {
        Self {
            callback: self.callback.clone(),
            live: Rc::clone(&self.live),
        }
    }
}

impl<T: EventArg> NativeSubscription<T> {
    /// The handler for an occurrence under this subscription.
    fn handler(self, value: T::Native) -> QueuedHandler {
        QueuedHandler::Callback {
            call: T::bind(self.callback, value),
            live: self.live,
        }
    }
}

impl<T> Default for NativeValueEvent<T> {
    fn default() -> Self {
        Self { subscription: None }
    }
}

impl<T> NativeValueEvent<T> {
    fn set(&mut self, callback: Option<Callback<T>>) {
        match (&mut self.subscription, callback) {
            (Some(subscription), Some(callback)) => subscription.callback = callback,
            (_, callback) => {
                if let Some(subscription) = self.subscription.take() {
                    subscription.live.set(false);
                }
                self.subscription = callback.map(|callback| NativeSubscription {
                    callback,
                    live: Rc::new(Cell::new(true)),
                });
            }
        }
    }

    /// The handler for an occurrence firing now under the current subscription.
    fn handler(&self, value: T::Native) -> Option<QueuedHandler>
    where
        T: EventArg,
    {
        self.subscription
            .clone()
            .map(|subscription| subscription.handler(value))
    }
}

struct NativeRoutedValueEvent<T> {
    callback: Option<RoutedCallback<T>>,
}

impl<T> Default for NativeRoutedValueEvent<T> {
    fn default() -> Self {
        Self { callback: None }
    }
}

impl<T> NativeRoutedValueEvent<T> {
    fn set(&mut self, callback: Option<RoutedCallback<T>>) {
        self.callback = callback;
    }
}

type NativeBoolEvent = NativeValueEvent<bool>;
type NativeColorEvent = NativeValueEvent<Color>;
type NativeCharacterEventInfoEvent = NativeRoutedValueEvent<CharacterEventInfo>;
type NativeContentDialogResultEvent = NativeValueEvent<ContentDialogResult>;
type NativeDragKindEvent = NativeValueEvent<DragKind>;
type NativeDroppedDataEvent = NativeValueEvent<DroppedData>;
type NativeF64Event = NativeValueEvent<f64>;
type NativeFocusEventInfoEvent = NativeValueEvent<FocusEventInfo>;
type NativeOptionalBoolEvent = NativeValueEvent<Option<bool>>;
type NativeOptionalDateTimeEvent = NativeValueEvent<Option<DateTime>>;
type NativeOptionalF64Event = NativeValueEvent<Option<f64>>;
type NativeOptionalTimeSpanEvent = NativeValueEvent<Option<TimeSpan>>;
type NativeNavigationViewDisplayModeEvent = NativeValueEvent<NavigationViewDisplayMode>;
type NativePointerEventInfoEvent = NativeValueEvent<PointerEventInfo>;
type NativeKeyEvent = NativeValueEvent<Key>;
type NativeKeyEventInfoEvent = NativeRoutedValueEvent<KeyEventInfo>;
type NativeKeysEvent = NativeValueEvent<Vec<Key>>;
type NativeSelectionEvent = NativeValueEvent<Option<Rc<str>>>;
type NativeSelectionIndexEvent = NativeValueEvent<Option<usize>>;
type NativeStringEvent = NativeValueEvent<Rc<str>>;
type NativeUnitEvent = NativeValueEvent<()>;

/// Adapter state that generated native event handlers capture when they subscribe.
#[derive(Clone, Copy)]
struct EventContext<'a> {
    event_queue: &'a Rc<NativeEventQueue>,
    pending_focus_states: &'a Rc<RefCell<HashMap<ObjectId, ElementFocusState>>>,
}

enum GeneratedRevoker {
    Event(windows_core::EventRevoker),
    Property(PropertyChangedRevoker),
}

impl Drop for GeneratedRevoker {
    fn drop(&mut self) {
        match self {
            Self::Event(value) => _ = value,
            Self::Property(value) => _ = value,
        }
    }
}

struct PropertyChangedRevoker {
    object: native::DependencyObject,
    property: native::DependencyProperty,
    token: i64,
}

impl Drop for PropertyChangedRevoker {
    fn drop(&mut self) {
        let _ = self
            .object
            .UnregisterPropertyChangedCallback(&self.property, self.token);
    }
}

macro_rules! generated_collection {
    ($($item:ident),*) => {
        enum GeneratedCollection {
            Visual(native::UIElementCollection),
            Inspectable(IVector<IInspectable>),
            $($item(IVector<native::$item>),)*
        }

        impl GeneratedCollection {
            fn size(&self) -> Result<u32, WinUiError> {
                match self {
                    Self::Visual(value) => value.Size().map_err(Into::into),
                    Self::Inspectable(value) => value.Size().map_err(Into::into),
                    $(Self::$item(value) => value.Size().map_err(Into::into),)*
                }
            }

            fn get_at(&self, index: u32) -> Result<native::UIElement, WinUiError> {
                match self {
                    Self::Visual(value) => value.GetAt(index).map_err(Into::into),
                    Self::Inspectable(value) => Ok(value.GetAt(index)?.cast()?),
                    $(Self::$item(value) => Ok(value.GetAt(index)?.cast()?),)*
                }
            }

            fn insert_at(&self, index: u32, child: &native::UIElement) -> Result<(), WinUiError> {
                match self {
                    Self::Visual(value) => value.InsertAt(index, child).map_err(Into::into),
                    Self::Inspectable(value) => value
                        .InsertAt(index, &child.cast::<IInspectable>()?)
                        .map_err(Into::into),
                    $(Self::$item(value) => value
                        .InsertAt(index, &child.cast::<native::$item>()?)
                        .map_err(Into::into),)*
                }
            }

            fn index_of(&self, child: &native::UIElement) -> Result<Option<u32>, WinUiError> {
                let mut index = 0;
                let found = match self {
                    Self::Visual(value) => value.IndexOf(child, &mut index)?,
                    Self::Inspectable(value) => {
                        value.IndexOf(&child.cast::<IInspectable>()?, &mut index)?
                    }
                    $(Self::$item(value) => {
                        value.IndexOf(&child.cast::<native::$item>()?, &mut index)?
                    })*
                };
                Ok(found.then_some(index))
            }

            fn remove_at(&self, index: u32) -> Result<(), WinUiError> {
                match self {
                    Self::Visual(value) => value.RemoveAt(index).map_err(Into::into),
                    Self::Inspectable(value) => value.RemoveAt(index).map_err(Into::into),
                    $(Self::$item(value) => value.RemoveAt(index).map_err(Into::into),)*
                }
            }
        }
    };
}

include!("../generated.rs");

enum ObservationSubscription {
    SwapChainPanel {
        _rendering: windows_core::EventRevoker,
        _scale: windows_core::EventRevoker,
        _size: windows_core::EventRevoker,
    },
    ImageScale {
        _changed: Rc<RefCell<Option<windows_core::EventRevoker>>>,
        _loaded: windows_core::EventRevoker,
    },
    CompositionHost {
        _changed: Rc<RefCell<Option<windows_core::EventRevoker>>>,
        _loaded: windows_core::EventRevoker,
        _size: windows_core::EventRevoker,
    },
}

struct WebViewInitialization {
    _action: Option<windows_future::IAsyncAction>,
    _loaded: Option<windows_core::EventRevoker>,
    _initialized: windows_core::EventRevoker,
    completions: Vec<Callback<Result<windows_core::IUnknown, IntegrationError>>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WinUiError {
    DuplicateObject(ObjectId),
    MissingObject(ObjectId),
    InvalidObject(ObjectId),
    InvalidRelation(ObjectId, RelationId),
    InvalidMutation(RelationId),
    InvalidEvent(ObjectId, EventId),
    MissingContract(ObjectType, RelationId),
    IndexOverflow(usize),
    ChildNotFound(ObjectId),
    StillOwned(ObjectId),
    InvalidReplacement(ObjectId),
    InvalidDuration,
    DuplicateWindowRoot(ObjectId),
    Native(windows_core::Error),
}

impl From<windows_core::Error> for WinUiError {
    fn from(value: windows_core::Error) -> Self {
        Self::Native(value)
    }
}

impl From<WinUiError> for windows_core::Error {
    fn from(value: WinUiError) -> Self {
        match value {
            WinUiError::Native(error) => error,
            error => Self::new(HRESULT(0x80004005_u32 as i32), format!("{error:?}")),
        }
    }
}

fn solid_color_brush(value: Color) -> Result<native::SolidColorBrush, WinUiError> {
    let brush = native::SolidColorBrush::new()?;
    brush.SetColor(native::Color {
        a: value.a,
        r: value.r,
        g: value.g,
        b: value.b,
    })?;
    Ok(brush)
}

fn to_native_color(value: Color) -> native::Color {
    native::Color {
        a: value.a,
        r: value.r,
        g: value.g,
        b: value.b,
    }
}

fn from_native_color(value: native::Color) -> Color {
    Color::argb(value.a, value.r, value.g, value.b)
}

fn navigation_view_display_mode(
    value: native::NavigationViewDisplayMode,
) -> NavigationViewDisplayMode {
    match value {
        native::NavigationViewDisplayMode::Minimal => NavigationViewDisplayMode::Minimal,
        native::NavigationViewDisplayMode::Compact => NavigationViewDisplayMode::Compact,
        native::NavigationViewDisplayMode::Expanded => NavigationViewDisplayMode::Expanded,
        _ => unreachable!(),
    }
}

pub(crate) fn validate_native_uri(value: &str) -> windows_core::Result<()> {
    native::Uri::CreateUri(value).map(drop)
}

fn set_rich_text_blocks(
    target: &native::RichTextBlock,
    value: Option<&RichText>,
) -> Result<(), WinUiError> {
    let blocks = target.Blocks()?;
    blocks.Clear()?;
    let Some(value) = value else {
        return Ok(());
    };
    let append_run =
        |inlines: &IVector<native::Inline>, value: &RichTextRun| -> Result<(), WinUiError> {
            let run = native::Run::new()?;
            run.SetText(&value.text)?;
            if value.is_bold {
                run.cast::<native::ITextElement>()?
                    .SetFontWeight(native::FontWeight { weight: 700 })?;
            }
            if value.is_italic {
                run.cast::<native::ITextElement>()?
                    .SetFontStyle(native::FontStyle::Italic)?;
            }
            let run: native::Inline = run.cast()?;
            inlines.Append(&run)?;
            Ok(())
        };
    for paragraph in value.paragraphs.iter() {
        let native_paragraph = native::Paragraph::new()?;
        let inlines = native_paragraph.Inlines()?;
        for inline in &paragraph.inlines {
            match inline {
                RichTextInline::Run(value) => append_run(&inlines, value)?,
                RichTextInline::Hyperlink(value) => {
                    let hyperlink = native::Hyperlink::new()?;
                    hyperlink.SetNavigateUri(&native::Uri::CreateUri(&value.uri)?)?;
                    let hyperlink_inlines = hyperlink.cast::<native::ISpan>()?.Inlines()?;
                    append_run(&hyperlink_inlines, &RichTextRun::plain(&value.text))?;
                    let hyperlink: native::Inline = hyperlink.cast()?;
                    inlines.Append(&hyperlink)?;
                }
                RichTextInline::LineBreak => {
                    let line_break: native::Inline = native::LineBreak::new()?.cast()?;
                    inlines.Append(&line_break)?;
                }
            }
        }
        let native_paragraph: native::Block = native_paragraph.cast()?;
        blocks.Append(&native_paragraph)?;
    }
    Ok(())
}

fn parse_path_data(value: &str) -> Result<native::Geometry, WinUiError> {
    let type_name = native::TypeName {
        name: "Microsoft.UI.Xaml.Media.Geometry".into(),
        kind: native::TypeKind::Metadata,
    };
    let value = windows_reference::IReference::<HSTRING>::from(value);
    Ok(native::XamlBindingHelper::ConvertValue(&type_name, &value)?.cast()?)
}

fn set_opacity_transition(
    element: &native::UIElement,
    duration: Option<Duration>,
) -> Result<(), WinUiError> {
    let Some(duration) = duration else {
        return element
            .SetOpacityTransition(None::<&native::ScalarTransition>)
            .map_err(Into::into);
    };
    let duration = TimeSpan::try_from(duration).map_err(|_| WinUiError::InvalidDuration)?;
    let transition = native::ScalarTransition::new()?;
    transition.SetDuration(duration)?;
    element.SetOpacityTransition(&transition)?;
    Ok(())
}

fn set_implicit_scale(element: &native::UIElement, value: f64) -> Result<(), WinUiError> {
    let framework = element.cast::<native::IFrameworkElement>()?;
    element.SetCenterPoint(native::Vector3 {
        x: framework.ActualWidth()? as f32 / 2.0,
        y: framework.ActualHeight()? as f32 / 2.0,
        z: 0.0,
    })?;
    let value = value as f32;
    element.SetScale(native::Vector3 {
        x: value,
        y: value,
        z: 1.0,
    })?;
    Ok(())
}

fn set_translation_transition(
    element: &native::UIElement,
    duration: Option<Duration>,
) -> Result<(), WinUiError> {
    let transition = vector_transition(duration)?;
    element.SetTranslationTransition(transition.as_ref())?;
    Ok(())
}

fn vector_transition(
    duration: Option<Duration>,
) -> Result<Option<native::Vector3Transition>, WinUiError> {
    let Some(duration) = duration else {
        return Ok(None);
    };
    let duration = TimeSpan::try_from(duration).map_err(|_| WinUiError::InvalidDuration)?;
    let transition = native::Vector3Transition::new()?;
    transition.SetDuration(duration)?;
    Ok(Some(transition))
}

fn set_scale_transition(
    element: &native::UIElement,
    duration: Option<Duration>,
) -> Result<(), WinUiError> {
    let transition = vector_transition(duration)?;
    element.SetScaleTransition(transition.as_ref())?;
    Ok(())
}

fn integration_error(error: windows_core::Error) -> IntegrationError {
    IntegrationError::Native(error.code().0)
}

fn complete_webview_initialization(
    initializations: &Rc<RefCell<HashMap<ObjectId, WebViewInitialization>>>,
    object: ObjectId,
    result: Result<windows_core::IUnknown, IntegrationError>,
) {
    if let Some(initialization) = initializations.borrow_mut().remove(&object) {
        for completion in initialization.completions {
            completion.call(result.clone());
        }
    }
}

fn xaml_scale(element: &native::UIElement) -> windows_core::Result<f64> {
    match element.XamlRoot() {
        Ok(root) => root.RasterizationScale(),
        Err(error) if error.code().is_ok() => Ok(1.0),
        Err(error) => Err(error),
    }
}

fn observe_xaml_scale(
    element: &native::UIElement,
    changed: &Rc<RefCell<Option<windows_core::EventRevoker>>>,
    callback: Callback<f64>,
) -> windows_core::Result<()> {
    let root = match element.XamlRoot() {
        Ok(root) => root,
        Err(error) if error.code().is_ok() => return Ok(()),
        Err(error) => return Err(error),
    };
    callback.call(root.RasterizationScale()?);
    changed.replace(Some(root.Changed(move |sender, _| {
        callback.call(sender.as_ref().unwrap().RasterizationScale().unwrap());
    })?));
    Ok(())
}

pub struct WinUiAdapter {
    handles: HashMap<ObjectId, Handle>,
    image_subscriptions: HashMap<ObjectId, ImageSubscription>,
    images: ImageCache,
    released_images: Vec<Rc<NativeImageSource>>,
    owners: HashMap<ObjectId, (ObjectId, RelationId)>,
    /// Index properties written by the batch being applied. Reactor writes an index after the
    /// items it indexes change, so selection moves caused by those changes are not reported.
    index_writes: HashSet<(ObjectId, PropertyId)>,
    /// Nesting depth of the batches being applied.
    apply_depth: usize,
    live_regions: HashSet<ObjectId>,
    tree_template: Option<native::DataTemplate>,
    list_template: Option<native::DataTemplate>,
    data_content_key: HSTRING,
    data_text_key: HSTRING,
    data_text_visibility_key: HSTRING,
    event_queue: Rc<NativeEventQueue>,
    retirements: HashMap<ObjectId, NativeRetirement>,
    virtual_items: HashMap<ObjectId, NativeVirtualItems>,
    observations: HashMap<(ObjectId, u64), ObservationSubscription>,
    webview_initializations: Rc<RefCell<HashMap<ObjectId, WebViewInitialization>>>,
    windows: Rc<RefCell<Vec<Weak<NativeWindowState>>>>,
    window_title_bar: Option<(ObjectId, WindowTitleBarHeight)>,
    tooltips: HashMap<ObjectId, (ObjectId, TooltipPlacement)>,
    tooltip_owners: HashMap<ObjectId, ObjectId>,
    flyouts: HashMap<ObjectId, (native::Flyout, ObjectId, FlyoutPlacement)>,
    flyout_owners: HashMap<ObjectId, ObjectId>,
    menus: HashMap<ObjectId, NativeMenu>,
    command_bar_flyouts: HashMap<ObjectId, NativeCommandBarFlyout>,
    content_dialogs: HashMap<ObjectId, ObjectId>,
    content_dialog_owners: HashMap<ObjectId, ObjectId>,
    resource_override_keys: HashMap<ObjectId, HashSet<String>>,
    style_states: HashMap<ObjectId, NativeStyleState>,
    style_cache: HashMap<NativeStyleKey, native::Style>,
    pending_focus_states: Rc<RefCell<HashMap<ObjectId, ElementFocusState>>>,
    tree_node_texts: Rc<RefCell<HashMap<usize, Rc<str>>>>,
}

struct NativeRetirement {
    nodes: Vec<ObjectId>,
    parent: ObjectId,
    relation: RelationId,
    timer: native::DispatcherQueueTimer,
    _tick: windows_core::EventRevoker,
}

impl Default for WinUiAdapter {
    fn default() -> Self {
        Self {
            handles: HashMap::new(),
            image_subscriptions: HashMap::new(),
            images: ImageCache::default(),
            released_images: Vec::new(),
            owners: HashMap::new(),
            index_writes: HashSet::new(),
            apply_depth: 0,
            live_regions: HashSet::new(),
            tree_template: None,
            list_template: None,
            data_content_key: HSTRING::from("Content"),
            data_text_key: HSTRING::from("Text"),
            data_text_visibility_key: HSTRING::from("TextVisibility"),
            event_queue: Rc::new(NativeEventQueue::default()),
            retirements: HashMap::new(),
            virtual_items: HashMap::new(),
            observations: HashMap::new(),
            webview_initializations: Rc::new(RefCell::new(HashMap::new())),
            windows: Rc::new(RefCell::new(Vec::new())),
            window_title_bar: None,
            tooltips: HashMap::new(),
            tooltip_owners: HashMap::new(),
            flyouts: HashMap::new(),
            flyout_owners: HashMap::new(),
            menus: HashMap::new(),
            command_bar_flyouts: HashMap::new(),
            content_dialogs: HashMap::new(),
            content_dialog_owners: HashMap::new(),
            resource_override_keys: HashMap::new(),
            style_states: HashMap::new(),
            style_cache: HashMap::new(),
            pending_focus_states: Rc::new(RefCell::new(HashMap::new())),
            tree_node_texts: Rc::new(RefCell::new(HashMap::new())),
        }
    }
}

impl Drop for WinUiAdapter {
    fn drop(&mut self) {
        for retirement in self.retirements.values() {
            _ = retirement.timer.Stop();
        }
        self.retirements.clear();
        let hide = self.event_queue.content_dialogs.borrow_mut().reset();
        for dialog in hide {
            _ = dialog.Hide();
        }
        self.event_queue.events.borrow_mut().clear();
        self.observations.clear();
        for target in self.menus.keys().copied().collect::<Vec<_>>() {
            _ = self.set_menu(target, None, 0);
        }
        for target in self.command_bar_flyouts.keys().copied().collect::<Vec<_>>() {
            _ = self.set_command_bar_flyout(target, None, 0);
        }
        for (target, (flyout, _, _)) in self.flyouts.drain() {
            _ = flyout.SetContent(None::<&native::UIElement>);
            match self.handles.get(&target) {
                Some(Handle::Generated(GeneratedHandle::Button(control))) => {
                    _ = control.value.SetFlyout(None::<&native::FlyoutBase>);
                }
                Some(Handle::Generated(GeneratedHandle::SplitButton(control))) => {
                    _ = control.value.SetFlyout(None::<&native::FlyoutBase>);
                }
                _ => {}
            }
        }
        self.flyout_owners.clear();
        for initialization in
            std::mem::take(&mut *self.webview_initializations.borrow_mut()).into_values()
        {
            for completion in initialization.completions {
                completion.call(Err(IntegrationError::Unavailable));
            }
        }
    }
}

fn relation_contract(
    kind: ObjectType,
    relation: RelationId,
) -> Result<&'static RelationContract, WinUiError> {
    relation_contracts(kind)
        .iter()
        .find(|contract| contract.id == relation)
        .ok_or(WinUiError::MissingContract(kind, relation))
}

fn com_identity(value: &impl Interface) -> Result<usize, WinUiError> {
    Ok(value.cast::<windows_core::IUnknown>()?.as_raw() as usize)
}

fn index32(index: usize) -> Result<u32, WinUiError> {
    index
        .try_into()
        .map_err(|_| WinUiError::IndexOverflow(index))
}

fn native_selection_index(value: Option<usize>) -> Result<i32, WinUiError> {
    value
        .map(|value| i32::try_from(value).map_err(|_| WinUiError::IndexOverflow(value)))
        .transpose()
        .map(|value| value.unwrap_or(-1))
}

fn native_number_box_value(value: Option<f64>) -> f64 {
    value.unwrap_or(f64::NAN)
}

fn number_box_value(value: f64) -> Option<f64> {
    (!value.is_nan()).then_some(value)
}

fn native_rating_value(value: Option<f64>) -> f64 {
    value.unwrap_or(-1.0)
}

fn rating_value(value: f64) -> Option<f64> {
    (value != -1.0).then_some(value)
}

/// A text input whose app-controlled text can be rewritten while the user is typing.
trait EditText {
    fn edit_text(&self) -> windows_core::Result<String>;
    fn set_edit_text(&self, text: &str) -> windows_core::Result<()>;
    /// The TextBox that owns the caret, if the template has been applied.
    fn edit_box(&self) -> Option<native::TextBox>;
}

impl EditText for native::TextBox {
    fn edit_text(&self) -> windows_core::Result<String> {
        self.Text()
    }

    fn set_edit_text(&self, text: &str) -> windows_core::Result<()> {
        self.SetText(text)
    }

    fn edit_box(&self) -> Option<native::TextBox> {
        Some(self.clone())
    }
}

impl EditText for native::AutoSuggestBox {
    fn edit_text(&self) -> windows_core::Result<String> {
        self.Text()
    }

    fn set_edit_text(&self, text: &str) -> windows_core::Result<()> {
        self.SetText(text)
    }

    fn edit_box(&self) -> Option<native::TextBox> {
        find_text_box(&self.cast().ok()?)
    }
}

/// Finds the template part named `TextBox`. Matching on the part name rather than the first
/// TextBox descendant avoids picking an editor supplied through the header.
fn find_text_box(object: &native::DependencyObject) -> Option<native::TextBox> {
    let count = native::VisualTreeHelper::GetChildrenCount(object).ok()?;
    (0..count).find_map(|index| {
        let child = native::VisualTreeHelper::GetChild(object, index).ok()?;
        child
            .cast::<native::TextBox>()
            .ok()
            .filter(|text_box| {
                text_box
                    .cast::<native::IFrameworkElement>()
                    .and_then(|element| element.Name())
                    .is_ok_and(|name| name == "TextBox")
            })
            .or_else(|| find_text_box(&child))
    })
}

/// Sets the text and keeps the caret and selection where the user left them, clamped to the new
/// text. Without this, an app that rewrites each edit (for example to uppercase it) moves the
/// caret to the end of the text after every keystroke.
fn set_edit_text(value: &impl EditText, text: &str) -> Result<(), WinUiError> {
    if value.edit_text()? == text {
        return Ok(());
    }
    let selection = value
        .edit_box()
        .map(|edit| -> windows_core::Result<_> {
            let start = edit.SelectionStart()?;
            let length = edit.SelectionLength()?;
            Ok((edit, start, length))
        })
        .transpose()?;
    value.set_edit_text(text)?;
    if let Some((edit, start, length)) = selection {
        let text_length = i32::try_from(text.encode_utf16().count()).unwrap_or(i32::MAX);
        let start = start.clamp(0, text_length);
        edit.SetSelectionStart(start)?;
        edit.SetSelectionLength(length.clamp(0, text_length - start))?;
    }
    Ok(())
}

fn read_rich_edit_text(value: &native::RichEditBox) -> windows_core::Result<Rc<str>> {
    let document = value.Document()?;
    let mut text = HSTRING::new();
    document.GetText(native::TextGetOptions::UseLf, &mut text)?;
    Ok(Rc::from(text.to_string_lossy()))
}

fn set_rich_edit_text(value: &native::RichEditBox, text: &str) -> Result<(), WinUiError> {
    if read_rich_edit_text(value)
        .map_err(WinUiError::from)?
        .as_ref()
        == text
    {
        return Ok(());
    }
    let document = value.Document()?;
    // Like `set_edit_text`, keep the caret and selection where the user left them. SetText moves
    // them to the start, and the document clamps the restored range to the new text.
    let selection = document.Selection()?.cast::<native::ITextRange>()?;
    let start = selection.StartPosition()?;
    let end = selection.EndPosition()?;
    let read_only = value.IsReadOnly()?;
    if read_only {
        value.SetIsReadOnly(false)?;
    }
    let write = document
        .SetText(native::TextSetOptions::None, text)
        .and_then(|()| selection.SetRange(start, end))
        .map_err(WinUiError::from);
    let restore = if read_only {
        value.SetIsReadOnly(true).map_err(WinUiError::from)
    } else {
        Ok(())
    };
    write.and(restore)
}

fn native_grid_length(length: GridLength) -> native::GridLength {
    let (value, grid_unit_type) = match length.size {
        GridLengthSize::Auto => (0.0, native::GridUnitType::Auto),
        GridLengthSize::Pixel(value) => (value, native::GridUnitType::Pixel),
        GridLengthSize::Star(value) => (value, native::GridUnitType::Star),
    };
    native::GridLength {
        value,
        grid_unit_type,
    }
}

fn set_grid_definitions(
    grid: &native::Grid,
    values: &[GridLength],
    rows: bool,
) -> Result<(), WinUiError> {
    if rows {
        let definitions = grid.RowDefinitions()?;
        definitions.Clear()?;
        for length in values {
            let definition = native::RowDefinition::new()?;
            definition.SetHeight(native_grid_length(*length))?;
            if let Some(value) = length.min {
                definition.SetMinHeight(value)?;
            }
            if let Some(value) = length.max {
                definition.SetMaxHeight(value)?;
            }
            definitions.Append(&definition)?;
        }
    } else {
        let definitions = grid.ColumnDefinitions()?;
        definitions.Clear()?;
        for length in values {
            let definition = native::ColumnDefinition::new()?;
            definition.SetWidth(native_grid_length(*length))?;
            if let Some(value) = length.min {
                definition.SetMinWidth(value)?;
            }
            if let Some(value) = length.max {
                definition.SetMaxWidth(value)?;
            }
            definitions.Append(&definition)?;
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "../../tests/native/winui.rs"]
mod tests;
