#[cfg(feature = "canvas")]
use super::canvas;
use super::collection::VirtualCollection;
use super::native_host;
#[cfg(feature = "webview")]
use super::webview;
use super::window::NativeWindow;
use super::*;
pub(super) enum Handle {
    Border(bindings::Border),
    Button {
        _revoker: windows_core::EventRevoker,
        value: bindings::Button,
    },
    CommandBar {
        value: bindings::CommandBar,
        state: Box<command::CommandBarState>,
    },
    CompositionHost(Box<native_host::CompositionHostState>),
    CommandBarFlyout {
        _revokers: [windows_core::EventRevoker; 2],
        value: bindings::CommandBarFlyout,
        state: Box<command::CommandBarState>,
    },
    AppBarButton(Box<command::AppBarButtonState>),
    AppBarToggleButton(Box<command::AppBarToggleButtonState>),
    AppBarSeparator {
        element: bindings::ICommandBarElement,
    },
    Image(Box<media::ImageState>),
    SymbolIcon(bindings::SymbolIcon),
    FontIcon(bindings::FontIcon),
    BitmapIcon(bindings::BitmapIcon),
    ImageIcon(bindings::ImageIcon),
    PathIcon(bindings::PathIcon),
    NavigationView(Box<navigation::NavigationViewState>),
    NavigationViewItem(Box<navigation::NavigationViewItemState>),
    Rectangle(bindings::Rectangle),
    Ellipse(bindings::Ellipse),
    Line(bindings::Line),
    #[cfg(feature = "canvas")]
    CanvasImage(Box<canvas::CanvasImageState>),
    #[cfg(feature = "canvas")]
    SwapChainCanvas(Box<canvas::SwapChainCanvasState>),
    #[cfg(feature = "canvas")]
    SwapChainHost(Box<canvas::SwapChainHostState>),
    #[cfg(feature = "webview")]
    WebViewHost(Box<webview::WebViewHostState>),
    DropDownButton(bindings::DropDownButton),
    SplitButton {
        _revoker: windows_core::EventRevoker,
        value: bindings::SplitButton,
    },
    Flyout {
        _revokers: [windows_core::EventRevoker; 2],
        value: bindings::Flyout,
    },
    MenuFlyout {
        _revokers: [windows_core::EventRevoker; 2],
        value: bindings::MenuFlyout,
        state: Box<menu::MenuState>,
    },
    MenuBar {
        value: bindings::MenuBar,
        state: Box<menu::MenuState>,
    },
    ContentDialog {
        value: bindings::ContentDialog,
        state: Box<overlay::ContentDialogState>,
    },
    HyperlinkButton {
        _revoker: windows_core::EventRevoker,
        value: bindings::HyperlinkButton,
    },
    RepeatButton {
        _revoker: windows_core::EventRevoker,
        value: bindings::RepeatButton,
    },
    Canvas(bindings::Canvas),
    CheckBox {
        _revokers: [windows_core::EventRevoker; 2],
        expected: Rc<Cell<bool>>,
        value: bindings::CheckBox,
    },
    RadioButton {
        _revokers: [windows_core::EventRevoker; 2],
        expected: Rc<Cell<bool>>,
        value: bindings::RadioButton,
    },
    ToggleButton {
        _revokers: Box<[windows_core::EventRevoker; 3]>,
        expected: Rc<Cell<bool>>,
        value: bindings::ToggleButton,
    },
    ToggleSwitch {
        _revoker: windows_core::EventRevoker,
        expected: Rc<Cell<bool>>,
        value: bindings::ToggleSwitch,
    },
    InfoBadge(bindings::InfoBadge),
    InfoBar {
        _revoker: windows_core::EventRevoker,
        expected_open: Rc<Cell<bool>>,
        value: bindings::InfoBar,
    },
    PersonPicture(bindings::PersonPicture),
    ProgressBar(bindings::ProgressBar),
    ProgressRing(bindings::ProgressRing),
    Slider {
        _revoker: windows_core::EventRevoker,
        state: Rc<controlled::ScalarCallbackState<f64>>,
        value: bindings::Slider,
    },
    NumberBox {
        _revoker: windows_core::EventRevoker,
        state: Rc<controlled::ScalarCallbackState<Option<f64>>>,
        value: bindings::NumberBox,
    },
    RatingControl {
        _revoker: windows_core::EventRevoker,
        state: Rc<controlled::ScalarCallbackState<Option<f64>>>,
        value: bindings::RatingControl,
    },
    ColorPicker {
        _revoker: windows_core::EventRevoker,
        expected: Rc<Cell<Color>>,
        value: bindings::ColorPicker,
    },
    DatePicker {
        _revoker: windows_core::EventRevoker,
        expected: Rc<Cell<Option<DateTime>>>,
        value: bindings::DatePicker,
    },
    CalendarDatePicker {
        _revoker: windows_core::EventRevoker,
        expected: Rc<Cell<Option<DateTime>>>,
        value: bindings::CalendarDatePicker,
    },
    TimePicker {
        _revoker: windows_core::EventRevoker,
        expected: Rc<Cell<Option<TimeSpan>>>,
        value: bindings::TimePicker,
    },
    CalendarView {
        _revoker: windows_core::EventRevoker,
        state: Rc<controlled::ListCallbackState<Vec<DateTime>>>,
        value: bindings::CalendarView,
    },
    Grid(bindings::Grid),
    RelativePanel(bindings::RelativePanel),
    Collection {
        value: bindings::IListViewBase,
        state: VirtualCollection,
    },
    ListBox {
        _revoker: windows_core::EventRevoker,
        value: bindings::IListBox,
        state: Box<collection::SelectorState>,
    },
    ComboBox {
        _revoker: windows_core::EventRevoker,
        value: bindings::ISelector,
        state: Box<collection::SingleSelectorState>,
    },
    RadioButtons {
        _revoker: windows_core::EventRevoker,
        value: bindings::IRadioButtons,
        state: Box<collection::SingleSelectorState>,
    },
    FlipView {
        _revoker: windows_core::EventRevoker,
        value: bindings::ISelector,
        state: Rc<selector::IndexSelectorState>,
    },
    TabView {
        _revokers: Box<[windows_core::EventRevoker; 4]>,
        value: bindings::ITabView,
        state: Rc<selector::TabViewState>,
    },
    TabViewItem(Box<selector::TabViewItemState>),
    SelectorBar {
        _revoker: windows_core::EventRevoker,
        value: bindings::ISelectorBar,
        state: Box<selector_bar::SelectorBarState>,
    },
    SelectorBarItem(Box<selector_bar::SelectorBarItemState>),
    BreadcrumbBar {
        _revoker: windows_core::EventRevoker,
        value: bindings::IBreadcrumbBar,
        state: Box<breadcrumb_bar::BreadcrumbBarState>,
    },
    AutoSuggestBox(Box<auto_suggest_box::AutoSuggestBoxState>),
    Pivot {
        _revoker: windows_core::EventRevoker,
        value: bindings::IPivot,
        state: Rc<selector::IndexSelectorState>,
    },
    PivotItem(bindings::PivotItem),
    StackPanel(bindings::StackPanel),
    ScrollViewer {
        value: bindings::ScrollViewer,
        view_changed: Option<windows_core::EventRevoker>,
    },
    ScrollView {
        value: bindings::ScrollView,
        view_changed: Option<windows_core::EventRevoker>,
    },
    SplitView {
        value: bindings::SplitView,
        state: Rc<container::SplitViewState>,
    },
    Expander(Box<container::ExpanderState>),
    TeachingTip {
        value: bindings::TeachingTip,
        state: Box<overlay::TeachingTipState>,
    },
    TitleBar {
        _revokers: [windows_core::EventRevoker; 2],
        value: bindings::TitleBar,
    },
    TextBlock(bindings::TextBlock),
    RichEditBox(Box<rich::RichEditBoxState>),
    RichTextBlock(bindings::RichTextBlock),
    TreeView(Box<rich::TreeViewState>),
    TextBox {
        _revoker: windows_core::EventRevoker,
        expected: Rc<RefCell<String>>,
        value: bindings::TextBox,
    },
    PasswordBox {
        _revoker: windows_core::EventRevoker,
        expected: Rc<RefCell<String>>,
        value: bindings::PasswordBox,
    },
    ToolTip(bindings::ToolTip),
    Viewbox(bindings::Viewbox),
}

impl Handle {
    pub(super) fn ui_element(&self) -> WindowsResult<bindings::UIElement> {
        match self {
            Self::Border(value) => value.cast(),
            Self::Button { value, .. } => value.cast(),
            Self::CommandBar { value, .. } => value.cast(),
            Self::CompositionHost(state) => Ok(state.ui_element()),
            Self::Image(state) => Ok(state.ui_element()),
            Self::SymbolIcon(value) => value.cast(),
            Self::FontIcon(value) => value.cast(),
            Self::BitmapIcon(value) => value.cast(),
            Self::ImageIcon(value) => value.cast(),
            Self::PathIcon(value) => value.cast(),
            Self::NavigationView(state) => state.value.cast(),
            Self::NavigationViewItem(state) => state.value.cast(),
            Self::Rectangle(value) => value.cast(),
            Self::Ellipse(value) => value.cast(),
            Self::Line(value) => value.cast(),
            #[cfg(feature = "canvas")]
            Self::CanvasImage(state) => Ok(state.ui_element()),
            #[cfg(feature = "canvas")]
            Self::SwapChainCanvas(state) => Ok(state.ui_element()),
            #[cfg(feature = "canvas")]
            Self::SwapChainHost(state) => Ok(state.ui_element()),
            #[cfg(feature = "webview")]
            Self::WebViewHost(state) => Ok(state.ui_element()),
            Self::AppBarButton(_) | Self::AppBarToggleButton(_) | Self::AppBarSeparator { .. } => {
                Err(windows_core::Error::new(
                    windows_core::HRESULT(0x80004002_u32 as i32),
                    "command item does not project UIElement",
                ))
            }
            Self::DropDownButton(value) => value.cast(),
            Self::SplitButton { value, .. } => value.cast(),
            Self::MenuBar { value, .. } => value.cast(),
            Self::ContentDialog { state, .. } => Ok(state.ui_element()),
            Self::HyperlinkButton { value, .. } => value.cast(),
            Self::RepeatButton { value, .. } => value.cast(),
            Self::Canvas(value) => value.cast(),
            Self::CheckBox { value, .. } => value.cast(),
            Self::RadioButton { value, .. } => value.cast(),
            Self::ToggleButton { value, .. } => value.cast(),
            Self::ToggleSwitch { value, .. } => value.cast(),
            Self::InfoBadge(value) => value.cast(),
            Self::InfoBar { value, .. } => value.cast(),
            Self::PersonPicture(value) => value.cast(),
            Self::ProgressBar(value) => value.cast(),
            Self::ProgressRing(value) => value.cast(),
            Self::Slider { value, .. } => value.cast(),
            Self::NumberBox { value, .. } => value.cast(),
            Self::RatingControl { value, .. } => value.cast(),
            Self::ColorPicker { value, .. } => value.cast(),
            Self::DatePicker { value, .. } => value.cast(),
            Self::CalendarDatePicker { value, .. } => value.cast(),
            Self::TimePicker { value, .. } => value.cast(),
            Self::CalendarView { value, .. } => value.cast(),
            Self::Grid(value) => value.cast(),
            Self::RelativePanel(value) => value.cast(),
            Self::Collection { value, .. } => value.cast(),
            Self::ListBox { value, .. } => value.cast(),
            Self::ComboBox { value, .. } => value.cast(),
            Self::RadioButtons { value, .. } => value.cast(),
            Self::FlipView { value, .. } => value.cast(),
            Self::TabView { value, .. } => value.cast(),
            Self::TabViewItem(state) => state.value.cast(),
            Self::SelectorBar { value, .. } => value.cast(),
            Self::SelectorBarItem(state) => state.value.cast(),
            Self::BreadcrumbBar { value, .. } => value.cast(),
            Self::AutoSuggestBox(state) => state.value.cast(),
            Self::Pivot { value, .. } => value.cast(),
            Self::PivotItem(value) => value.cast(),
            Self::StackPanel(value) => value.cast(),
            Self::ScrollViewer { value, .. } => value.cast(),
            Self::ScrollView { value, .. } => value.cast(),
            Self::SplitView { value, .. } => value.cast(),
            Self::Expander(state) => state.value.cast(),
            Self::TeachingTip { value, .. } => value.cast(),
            Self::TitleBar { value, .. } => value.cast(),
            Self::TextBlock(value) => value.cast(),
            Self::RichEditBox(state) => state.value.cast(),
            Self::RichTextBlock(value) => value.cast(),
            Self::TreeView(state) => state.value.cast(),
            Self::TextBox { value, .. } => value.cast(),
            Self::PasswordBox { value, .. } => value.cast(),
            Self::ToolTip(value) => value.cast(),
            Self::Viewbox(value) => value.cast(),
            Self::Flyout { .. } | Self::MenuFlyout { .. } | Self::CommandBarFlyout { .. } => {
                Err(windows_core::Error::new(
                    windows_core::HRESULT(0x80004002_u32 as i32),
                    "native node does not implement UIElement",
                ))
            }
        }
    }

    pub(super) fn framework_element(&self) -> WindowsResult<bindings::FrameworkElement> {
        match self {
            Self::Border(value) => value.cast(),
            Self::Button { value, .. } => value.cast(),
            Self::CommandBar { value, .. } => value.cast(),
            Self::CompositionHost(state) => Ok(state.framework_element()),
            Self::Image(state) => Ok(state.framework_element()),
            Self::SymbolIcon(value) => value.cast(),
            Self::FontIcon(value) => value.cast(),
            Self::BitmapIcon(value) => value.cast(),
            Self::ImageIcon(value) => value.cast(),
            Self::PathIcon(value) => value.cast(),
            Self::NavigationView(state) => state.value.cast(),
            Self::NavigationViewItem(state) => state.value.cast(),
            Self::Rectangle(value) => value.cast(),
            Self::Ellipse(value) => value.cast(),
            Self::Line(value) => value.cast(),
            #[cfg(feature = "canvas")]
            Self::CanvasImage(state) => Ok(state.framework_element()),
            #[cfg(feature = "canvas")]
            Self::SwapChainCanvas(state) => Ok(state.framework_element()),
            #[cfg(feature = "canvas")]
            Self::SwapChainHost(state) => Ok(state.framework_element()),
            #[cfg(feature = "webview")]
            Self::WebViewHost(state) => Ok(state.framework_element()),
            Self::AppBarButton(_) | Self::AppBarToggleButton(_) | Self::AppBarSeparator { .. } => {
                Err(windows_core::Error::new(
                    windows_core::HRESULT(0x80004002_u32 as i32),
                    "command item does not project FrameworkElement",
                ))
            }
            Self::DropDownButton(value) => value.cast(),
            Self::SplitButton { value, .. } => value.cast(),
            Self::MenuBar { value, .. } => value.cast(),
            Self::ContentDialog { .. } => Err(windows_core::Error::new(
                windows_core::HRESULT(0x80004002_u32 as i32),
                "owned native node does not project FrameworkElement",
            )),
            Self::HyperlinkButton { value, .. } => value.cast(),
            Self::RepeatButton { value, .. } => value.cast(),
            Self::Canvas(value) => value.cast(),
            Self::CheckBox { value, .. } => value.cast(),
            Self::RadioButton { value, .. } => value.cast(),
            Self::ToggleButton { value, .. } => value.cast(),
            Self::ToggleSwitch { value, .. } => value.cast(),
            Self::InfoBadge(value) => value.cast(),
            Self::InfoBar { value, .. } => value.cast(),
            Self::PersonPicture(value) => value.cast(),
            Self::ProgressBar(value) => value.cast(),
            Self::ProgressRing(value) => value.cast(),
            Self::Slider { value, .. } => value.cast(),
            Self::NumberBox { value, .. } => value.cast(),
            Self::RatingControl { value, .. } => value.cast(),
            Self::ColorPicker { value, .. } => value.cast(),
            Self::DatePicker { value, .. } => value.cast(),
            Self::CalendarDatePicker { value, .. } => value.cast(),
            Self::TimePicker { value, .. } => value.cast(),
            Self::CalendarView { value, .. } => value.cast(),
            Self::Grid(value) => value.cast(),
            Self::RelativePanel(value) => value.cast(),
            Self::Collection { value, .. } => value.cast(),
            Self::ListBox { value, .. } => value.cast(),
            Self::ComboBox { value, .. } => value.cast(),
            Self::RadioButtons { value, .. } => value.cast(),
            Self::FlipView { value, .. } => value.cast(),
            Self::TabView { value, .. } => value.cast(),
            Self::TabViewItem(state) => state.value.cast(),
            Self::SelectorBar { value, .. } => value.cast(),
            Self::SelectorBarItem(state) => state.value.cast(),
            Self::BreadcrumbBar { value, .. } => value.cast(),
            Self::AutoSuggestBox(state) => state.value.cast(),
            Self::Pivot { value, .. } => value.cast(),
            Self::PivotItem(value) => value.cast(),
            Self::StackPanel(value) => value.cast(),
            Self::ScrollViewer { value, .. } => value.cast(),
            Self::ScrollView { value, .. } => value.cast(),
            Self::SplitView { value, .. } => value.cast(),
            Self::Expander(state) => state.value.cast(),
            Self::TeachingTip { value, .. } => value.cast(),
            Self::TitleBar { value, .. } => value.cast(),
            Self::TextBlock(value) => value.cast(),
            Self::RichEditBox(state) => state.value.cast(),
            Self::RichTextBlock(value) => value.cast(),
            Self::TreeView(state) => state.value.cast(),
            Self::TextBox { value, .. } => value.cast(),
            Self::PasswordBox { value, .. } => value.cast(),
            Self::ToolTip(value) => value.cast(),
            Self::Viewbox(value) => value.cast(),
            Self::Flyout { .. } | Self::MenuFlyout { .. } | Self::CommandBarFlyout { .. } => {
                Err(windows_core::Error::new(
                    windows_core::HRESULT(0x80004002_u32 as i32),
                    "native node does not implement FrameworkElement",
                ))
            }
        }
    }

    pub(super) fn dependency_object(&self) -> WindowsResult<bindings::DependencyObject> {
        self.ui_element()?.cast()
    }

    pub(super) fn clear_text_property(
        &self,
        control_property: fn() -> WindowsResult<bindings::DependencyProperty>,
        text_block_property: fn() -> WindowsResult<bindings::DependencyProperty>,
    ) -> WindowsResult<()> {
        let property = match self {
            Self::TextBlock(_) => text_block_property()?,
            _ => control_property()?,
        };
        self.dependency_object()?.ClearValue(&property)
    }

    pub(super) fn control(&self) -> WindowsResult<bindings::IControl> {
        match self {
            Self::Button { value, .. } => value.cast(),
            Self::CommandBar { value, .. } => value.cast(),
            Self::AppBarButton(state) => Ok(state.control.clone()),
            Self::AppBarToggleButton(state) => Ok(state.control.clone()),
            Self::AppBarSeparator { .. } => Err(windows_core::Error::new(
                windows_core::HRESULT(0x80004002_u32 as i32),
                "command separator does not implement Control updates",
            )),
            Self::DropDownButton(value) => value.cast(),
            Self::SplitButton { value, .. } => value.cast(),
            Self::MenuBar { value, .. } => value.cast(),
            Self::ContentDialog { .. } => Err(windows_core::Error::new(
                windows_core::HRESULT(0x80004002_u32 as i32),
                "owned native node does not project Control",
            )),
            Self::HyperlinkButton { value, .. } => value.cast(),
            Self::RepeatButton { value, .. } => value.cast(),
            Self::CheckBox { value, .. } => value.cast(),
            Self::RadioButton { value, .. } => value.cast(),
            Self::ToggleButton { value, .. } => value.cast(),
            Self::ToggleSwitch { value, .. } => value.cast(),
            Self::InfoBadge(value) => value.cast(),
            Self::ProgressBar(value) => value.cast(),
            Self::ProgressRing(value) => value.cast(),
            Self::Slider { value, .. } => value.cast(),
            Self::NumberBox { value, .. } => value.cast(),
            Self::RatingControl { value, .. } => value.cast(),
            Self::ColorPicker { value, .. } => value.cast(),
            Self::DatePicker { value, .. } => value.cast(),
            Self::CalendarDatePicker { value, .. } => value.cast(),
            Self::TimePicker { value, .. } => value.cast(),
            Self::CalendarView { value, .. } => value.cast(),
            Self::NavigationView(state) => state.value.cast(),
            Self::Collection { value, .. } => value.cast(),
            Self::ListBox { value, .. } => value.cast(),
            Self::ComboBox { value, .. } => value.cast(),
            Self::RadioButtons { value, .. } => value.cast(),
            Self::FlipView { value, .. } => value.cast(),
            Self::TabView { value, .. } => value.cast(),
            Self::SelectorBar { value, .. } => value.cast(),
            Self::BreadcrumbBar { value, .. } => value.cast(),
            Self::AutoSuggestBox(state) => state.value.cast(),
            Self::Pivot { value, .. } => value.cast(),
            Self::ScrollViewer { value, .. } => value.cast(),
            Self::ScrollView { value, .. } => value.cast(),
            Self::SplitView { value, .. } => value.cast(),
            Self::Expander(state) => state.value.cast(),
            Self::TextBox { value, .. } => value.cast(),
            Self::RichEditBox(state) => state.value.cast(),
            Self::TreeView(state) => state.value.cast(),
            Self::PasswordBox { value, .. } => value.cast(),
            _ => Err(windows_core::Error::new(
                windows_core::HRESULT(0x80004002_u32 as i32),
                "native node does not implement IControl",
            )),
        }
    }

    pub(super) fn content_control(&self) -> Option<WindowsResult<bindings::IContentControl>> {
        Some(match self {
            Self::Button { value, .. } => value.cast(),
            Self::DropDownButton(value) => value.cast(),
            Self::SplitButton { value, .. } => value.cast(),
            Self::ContentDialog { state, .. } => Ok(state.content_control()),
            Self::HyperlinkButton { value, .. } => value.cast(),
            Self::RepeatButton { value, .. } => value.cast(),
            Self::CheckBox { value, .. } => value.cast(),
            Self::RadioButton { value, .. } => value.cast(),
            Self::ToggleButton { value, .. } => value.cast(),
            Self::ScrollViewer { value, .. } => value.cast(),
            Self::Expander(state) => state.value.cast(),
            Self::ToolTip(value) => value.cast(),
            Self::TabViewItem(state) => state.value.cast(),
            Self::PivotItem(value) => value.cast(),
            Self::NavigationView(state) => state.value.cast(),
            _ => return None,
        })
    }

    pub(super) fn item_collection(
        &self,
    ) -> Option<WindowsResult<windows_collections::IVector<windows_core::IInspectable>>> {
        Some(match self {
            Self::FlipView { value, .. } => value
                .cast::<bindings::IItemsControl>()
                .and_then(|items| items.Items())
                .and_then(|items| items.cast()),
            Self::Pivot { value, .. } => value
                .cast::<bindings::IItemsControl>()
                .and_then(|items| items.Items())
                .and_then(|items| items.cast()),
            Self::TabView { value, .. } => value.TabItems(),
            _ => return None,
        })
    }

    pub(super) fn command_bar_element(&self) -> Option<&bindings::ICommandBarElement> {
        match self {
            Self::AppBarButton(state) => Some(&state.element),
            Self::AppBarToggleButton(state) => Some(&state.element),
            Self::AppBarSeparator { element } => Some(element),
            _ => None,
        }
    }

    pub(super) fn panel(&self) -> WindowsResult<bindings::IPanel> {
        match self {
            Self::Canvas(value) => value.cast(),
            Self::Grid(value) => value.cast(),
            Self::RelativePanel(value) => value.cast(),
            Self::StackPanel(value) => value.cast(),
            _ => Err(windows_core::Error::new(
                windows_core::HRESULT(0x80004002_u32 as i32),
                "native node does not implement IPanel",
            )),
        }
    }

    pub(super) fn text_block(&self) -> WindowsResult<&bindings::TextBlock> {
        match self {
            Self::TextBlock(value) => Ok(value),
            _ => Err(windows_core::Error::new(
                windows_core::HRESULT(0x80004002_u32 as i32),
                "native node is not a TextBlock",
            )),
        }
    }
}

pub(super) struct NativeNode {
    pub(super) handle: Handle,
    pub(super) parent: Option<NodeId>,
    pub(super) attachment: Option<Attachment>,
    pub(super) children: Vec<NodeId>,
    pub(super) input: Option<Box<input::NativeInputState>>,
}

pub(super) struct NativeTimer {
    pub(super) revision: u64,
    pub(super) timer: bindings::DispatcherQueueTimer,
    pub(super) _revoker: windows_core::EventRevoker,
}

impl Drop for NativeTimer {
    fn drop(&mut self) {
        _ = self.timer.Stop();
    }
}

pub struct WinUiRuntime {
    pub(super) dispatcher: bindings::DispatcherQueue,
    pub(super) windows: BTreeMap<NodeId, NativeWindow>,
    pub(super) nodes: BTreeMap<NodeId, NativeNode>,
    pub(super) timers: BTreeMap<(NodeId, u32), NativeTimer>,
    pub(super) events: Rc<RefCell<VecDeque<NativeEvent>>>,
    pub(super) waker: Rc<RefCell<Option<Rc<dyn Fn()>>>>,
    pub(super) next_lease: Rc<Cell<u64>>,
    pub(super) active_content_dialogs: Rc<RefCell<BTreeMap<NodeId, NodeId>>>,
    pub(super) shutting_down: Rc<Cell<bool>>,
    pub(super) shutdown_complete: bool,
    pub(super) on_windows_empty: Rc<dyn Fn()>,
    pub(super) exit_revision: Rc<Cell<u64>>,
    pub(super) template: Option<bindings::DataTemplate>,
    pub(super) application_resources: ApplicationResources,
    pub(super) element_resources: BTreeMap<NodeId, ElementResources>,
    #[cfg(test)]
    pub(super) timer_ticks: Rc<Cell<usize>>,
    #[cfg(test)]
    pub(super) window_activations: Vec<NodeId>,
    #[cfg(all(test, feature = "canvas"))]
    pub(super) canvas_test_present_loss: bool,
    #[cfg(all(test, feature = "canvas"))]
    pub(super) canvas_test_scale: Option<(f32, f32)>,
}

impl WinUiRuntime {
    pub(super) fn new(on_windows_empty: Rc<dyn Fn()>) -> WindowsResult<Self> {
        let dispatcher = bindings::DispatcherQueue::GetForCurrentThread()?;
        let waker = Rc::new(RefCell::new(None));
        Ok(Self {
            dispatcher,
            windows: BTreeMap::new(),
            nodes: BTreeMap::new(),
            timers: BTreeMap::new(),
            events: Rc::new(RefCell::new(VecDeque::new())),
            waker,
            next_lease: Rc::new(Cell::new(1)),
            active_content_dialogs: Rc::new(RefCell::new(BTreeMap::new())),
            shutting_down: Rc::new(Cell::new(false)),
            shutdown_complete: false,
            on_windows_empty,
            exit_revision: Rc::new(Cell::new(0)),
            template: None,
            application_resources: ApplicationResources::default(),
            element_resources: BTreeMap::new(),
            #[cfg(test)]
            timer_ticks: Rc::new(Cell::new(0)),
            #[cfg(test)]
            window_activations: Vec::new(),
            #[cfg(all(test, feature = "canvas"))]
            canvas_test_present_loss: false,
            #[cfg(all(test, feature = "canvas"))]
            canvas_test_scale: None,
        })
    }

    pub(super) fn node(&self, id: NodeId) -> WindowsResult<&NativeNode> {
        self.nodes
            .get(&id)
            .ok_or_else(|| panic!("native node is unknown"))
    }

    pub(super) fn node_mut(&mut self, id: NodeId) -> WindowsResult<&mut NativeNode> {
        self.nodes
            .get_mut(&id)
            .ok_or_else(|| panic!("native node is unknown"))
    }

    pub(super) fn collection(&self, id: NodeId) -> WindowsResult<&VirtualCollection> {
        match &self.node(id)?.handle {
            Handle::Collection { state, .. } => Ok(state),
            _ => panic!("native node is not a virtual collection"),
        }
    }

    pub(super) fn collection_mut(&mut self, id: NodeId) -> WindowsResult<&mut VirtualCollection> {
        match &mut self.node_mut(id)?.handle {
            Handle::Collection { state, .. } => Ok(state),
            _ => panic!("native node is not a virtual collection"),
        }
    }
}
