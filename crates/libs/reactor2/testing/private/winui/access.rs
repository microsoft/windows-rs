use windows_core::Interface;

use super::*;

impl Handle {
    fn kind(&self) -> NativeKind {
        match self {
            Self::Border(_) => NativeKind::Border,
            Self::Button { .. } => NativeKind::Button,
            Self::CommandBar { .. } => NativeKind::CommandBar,
            Self::CompositionHost(_) => NativeKind::CompositionHost,
            Self::CommandBarFlyout { .. } => NativeKind::CommandBarFlyout,
            Self::Image(_) => NativeKind::Image,
            Self::SymbolIcon(_) => NativeKind::SymbolIcon,
            Self::FontIcon(_) => NativeKind::FontIcon,
            Self::BitmapIcon(_) => NativeKind::BitmapIcon,
            Self::ImageIcon(_) => NativeKind::ImageIcon,
            Self::PathIcon(_) => NativeKind::PathIcon,
            Self::NavigationView(_) => NativeKind::NavigationView,
            Self::NavigationViewItem(_) => NativeKind::NavigationViewItem,
            Self::Rectangle(_) => NativeKind::Rectangle,
            Self::Ellipse(_) => NativeKind::Ellipse,
            Self::Line(_) => NativeKind::Line,
            #[cfg(feature = "canvas")]
            Self::CanvasImage(_) => NativeKind::CanvasImage,
            #[cfg(feature = "canvas")]
            Self::SwapChainCanvas(_) => NativeKind::SwapChainCanvas,
            #[cfg(feature = "canvas")]
            Self::SwapChainHost(_) => NativeKind::SwapChainHost,
            #[cfg(feature = "webview")]
            Self::WebViewHost(_) => NativeKind::WebViewHost,
            Self::AppBarButton(_) => NativeKind::AppBarButton,
            Self::AppBarToggleButton(_) => NativeKind::AppBarToggleButton,
            Self::AppBarSeparator { .. } => NativeKind::AppBarSeparator,
            Self::DropDownButton(_) => NativeKind::DropDownButton,
            Self::SplitButton { .. } => NativeKind::SplitButton,
            Self::Flyout { .. } => NativeKind::Flyout,
            Self::MenuFlyout { .. } => NativeKind::MenuFlyout,
            Self::MenuBar { .. } => NativeKind::MenuBar,
            Self::ContentDialog { .. } => NativeKind::ContentDialog,
            Self::HyperlinkButton { .. } => NativeKind::HyperlinkButton,
            Self::RepeatButton { .. } => NativeKind::RepeatButton,
            Self::Canvas(_) => NativeKind::Canvas,
            Self::CheckBox { .. } => NativeKind::CheckBox,
            Self::RadioButton { .. } => NativeKind::RadioButton,
            Self::ToggleButton { .. } => NativeKind::ToggleButton,
            Self::ToggleSwitch { .. } => NativeKind::ToggleSwitch,
            Self::InfoBadge(_) => NativeKind::InfoBadge,
            Self::InfoBar { .. } => NativeKind::InfoBar,
            Self::PersonPicture(_) => NativeKind::PersonPicture,
            Self::ProgressBar(_) => NativeKind::ProgressBar,
            Self::ProgressRing(_) => NativeKind::ProgressRing,
            Self::Slider { .. } => NativeKind::Slider,
            Self::NumberBox { .. } => NativeKind::NumberBox,
            Self::RatingControl { .. } => NativeKind::RatingControl,
            Self::ColorPicker { .. } => NativeKind::ColorPicker,
            Self::DatePicker { .. } => NativeKind::DatePicker,
            Self::CalendarDatePicker { .. } => NativeKind::CalendarDatePicker,
            Self::TimePicker { .. } => NativeKind::TimePicker,
            Self::CalendarView { .. } => NativeKind::CalendarView,
            Self::Grid(_) => NativeKind::Grid,
            Self::RelativePanel(_) => NativeKind::RelativePanel,
            Self::Collection { value, .. } => {
                if value.cast::<bindings::GridView>().is_ok() {
                    NativeKind::GridView
                } else {
                    NativeKind::ListView
                }
            }
            Self::ListBox { .. } => NativeKind::ListBox,
            Self::ComboBox { .. } => NativeKind::ComboBox,
            Self::RadioButtons { .. } => NativeKind::RadioButtons,
            Self::FlipView { .. } => NativeKind::FlipView,
            Self::TabView { .. } => NativeKind::TabView,
            Self::TabViewItem(_) => NativeKind::TabViewItem,
            Self::SelectorBar { .. } => NativeKind::SelectorBar,
            Self::SelectorBarItem(_) => NativeKind::SelectorBarItem,
            Self::BreadcrumbBar { .. } => NativeKind::BreadcrumbBar,
            Self::AutoSuggestBox(_) => NativeKind::AutoSuggestBox,
            Self::Pivot { .. } => NativeKind::Pivot,
            Self::PivotItem(_) => NativeKind::PivotItem,
            Self::StackPanel(_) => NativeKind::StackPanel,
            Self::ScrollViewer { .. } => NativeKind::ScrollViewer,
            Self::ScrollView { .. } => NativeKind::ScrollView,
            Self::SplitView { .. } => NativeKind::SplitView,
            Self::Expander { .. } => NativeKind::Expander,
            Self::TeachingTip { .. } => NativeKind::TeachingTip,
            Self::TitleBar { .. } => NativeKind::TitleBar,
            Self::TextBlock(_) => NativeKind::TextBlock,
            Self::RichEditBox(_) => NativeKind::RichEditBox,
            Self::RichTextBlock(_) => NativeKind::RichTextBlock,
            Self::TreeView(_) => NativeKind::TreeView,
            Self::TextBox { .. } => NativeKind::TextBox,
            Self::PasswordBox { .. } => NativeKind::PasswordBox,
            Self::ToolTip(_) => NativeKind::ToolTip,
            Self::Viewbox(_) => NativeKind::Viewbox,
        }
    }
}

pub(super) struct RuntimeProbe<'a> {
    runtime: &'a WinUiRuntime,
}

impl<'a> RuntimeProbe<'a> {
    pub(super) fn new(runtime: &'a WinUiRuntime) -> Self {
        Self { runtime }
    }

    pub(super) fn windows(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.runtime.windows.keys().copied()
    }

    pub(super) fn application_resources(&self) -> &ApplicationResources {
        &self.runtime.application_resources
    }

    pub(super) fn nodes(&self, kind: NativeKind) -> Vec<NodeId> {
        self.runtime
            .nodes
            .iter()
            .filter_map(|(id, node)| (node.handle.kind() == kind).then_some(*id))
            .collect()
    }

    pub(super) fn children(&self, id: NodeId) -> Vec<NodeId> {
        self.runtime.node(id).unwrap().children.clone()
    }

    pub(super) fn contains(&self, id: NodeId) -> bool {
        self.runtime.nodes.contains_key(&id)
    }
}
