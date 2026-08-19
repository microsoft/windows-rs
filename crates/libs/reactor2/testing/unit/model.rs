//! Low-level ownership, protocol, and virtualization model tests.

use std::collections::{BTreeMap, BTreeSet};
use std::mem::size_of;
use std::rc::Rc;

use super::*;
use crate::element::tree::StructuralSlot;

macro_rules! define_update_capability_cases {
    (
        direct {
            $(
                $(#[$direct_attr:meta])*
                $direct_pattern:pat => [$direct_case:expr],
            )*
        }
        families {
            $(
                $(#[$family_attr:meta])*
                $outer_pattern:pat => $binding:ident, $subject:expr => {
                    $(
                        $(#[$leaf_attr:meta])*
                        $leaf_pattern:pat => [$family_case:expr],
                    )*
                }
            )*
        }
    ) => {
        fn update_capability_key(update: &NativeUpdate) -> &'static str {
            match update {
                $($(#[$direct_attr])* $direct_pattern => stringify!($direct_pattern),)*
                $(
                    $(#[$family_attr])*
                    $outer_pattern => match $subject {
                        $($(#[$leaf_attr])* $leaf_pattern => concat!(
                            stringify!($outer_pattern),
                            "::",
                            stringify!($leaf_pattern)
                        ),)*
                    },
                )*
            }
        }

        fn update_capability_cases() -> Vec<NativeUpdate> {
            vec![
                $($(#[$direct_attr])* $direct_case,)*
                $($($(#[$leaf_attr])* $family_case,)*)*
            ]
        }
    };
}

define_update_capability_cases! {
    direct {
        NativeUpdate::Resources(_) => [
            NativeUpdate::Resources(Box::default())
        ],
        NativeUpdate::Visual(VisualUpdate::ImplicitTransitions(_)) => [
            NativeUpdate::Visual(VisualUpdate::ImplicitTransitions(
                ImplicitTransitions::default(),
            ))
        ],
        NativeUpdate::Visual(VisualUpdate::Scale(_)) => [
            NativeUpdate::Visual(VisualUpdate::Scale(Some(1.0)))
        ],
        NativeUpdate::Visual(VisualUpdate::FadeTo { .. }) => [
            NativeUpdate::Visual(VisualUpdate::FadeTo {
                opacity: 0.0,
                duration: std::time::Duration::from_millis(100),
            })
        ],
        NativeUpdate::Accessibility(AccessibilityUpdate::AutomationName(_)) => [
            NativeUpdate::Accessibility(AccessibilityUpdate::AutomationName(
                "name".to_string(),
            ))
        ],
        NativeUpdate::Accessibility(AccessibilityUpdate::AutomationId(_)) => [
            NativeUpdate::Accessibility(AccessibilityUpdate::AutomationId(
                "identifier".to_string(),
            ))
        ],
        NativeUpdate::Accessibility(AccessibilityUpdate::HeadingLevel(_)) => [
            NativeUpdate::Accessibility(AccessibilityUpdate::HeadingLevel(Some(
                AutomationHeadingLevel::Level1,
            )))
        ],
        NativeUpdate::Accessibility(AccessibilityUpdate::HelpText(_)) => [
            NativeUpdate::Accessibility(AccessibilityUpdate::HelpText("help".to_string()))
        ],
        NativeUpdate::Attached(AttachedUpdate::Row(_)) => [
            NativeUpdate::Attached(AttachedUpdate::Row(Some(1)))
        ],
        NativeUpdate::Attached(AttachedUpdate::Column(_)) => [
            NativeUpdate::Attached(AttachedUpdate::Column(Some(1)))
        ],
        NativeUpdate::Attached(AttachedUpdate::RowSpan(_)) => [
            NativeUpdate::Attached(AttachedUpdate::RowSpan(Some(2)))
        ],
        NativeUpdate::Attached(AttachedUpdate::ColumnSpan(_)) => [
            NativeUpdate::Attached(AttachedUpdate::ColumnSpan(Some(2)))
        ],
        NativeUpdate::Attached(AttachedUpdate::CanvasLeft(_)) => [
            NativeUpdate::Attached(AttachedUpdate::CanvasLeft(Some(1.0)))
        ],
        NativeUpdate::Attached(AttachedUpdate::CanvasTop(_)) => [
            NativeUpdate::Attached(AttachedUpdate::CanvasTop(Some(1.0)))
        ],
        NativeUpdate::Attached(AttachedUpdate::CanvasZIndex(_)) => [
            NativeUpdate::Attached(AttachedUpdate::CanvasZIndex(Some(1)))
        ],
        NativeUpdate::Attached(AttachedUpdate::RelativeAlignLeft(_)) => [
            NativeUpdate::Attached(AttachedUpdate::RelativeAlignLeft(Some(true)))
        ],
        NativeUpdate::Attached(AttachedUpdate::RelativeAlignRight(_)) => [
            NativeUpdate::Attached(AttachedUpdate::RelativeAlignRight(Some(false)))
        ],
        NativeUpdate::Attached(AttachedUpdate::RelativeAlignTop(_)) => [
            NativeUpdate::Attached(AttachedUpdate::RelativeAlignTop(Some(true)))
        ],
        NativeUpdate::Attached(AttachedUpdate::RelativeAlignBottom(_)) => [
            NativeUpdate::Attached(AttachedUpdate::RelativeAlignBottom(Some(false)))
        ],
        NativeUpdate::Attached(AttachedUpdate::RelativeAlignHorizontalCenter(_)) => [
            NativeUpdate::Attached(AttachedUpdate::RelativeAlignHorizontalCenter(Some(true)))
        ],
        NativeUpdate::Attached(AttachedUpdate::RelativeAlignVerticalCenter(_)) => [
            NativeUpdate::Attached(AttachedUpdate::RelativeAlignVerticalCenter(Some(false)))
        ],
        NativeUpdate::Attached(AttachedUpdate::TooltipPlacement(_)) => [
            NativeUpdate::Attached(AttachedUpdate::TooltipPlacement(Some(
                TooltipPlacement::Bottom,
            )))
        ],
        NativeUpdate::Input(InputUpdate::KeyboardAccelerators(_)) => [
            NativeUpdate::Input(InputUpdate::KeyboardAccelerators(vec![
                KeyboardAcceleratorSpec {
                    key: VirtualKey::S,
                    modifiers: VirtualKeyModifiers::CONTROL,
                },
            ]))
        ],
        NativeUpdate::Input(InputUpdate::Pointer(_)) => [
            NativeUpdate::Input(InputUpdate::Pointer(PointerSubscription {
                events: PointerEvents::PRESSED | PointerEvents::RELEASED,
                capture_on_press: true,
            }))
        ],
        NativeUpdate::Input(InputUpdate::Drop(_)) => [
            NativeUpdate::Input(InputUpdate::Drop(Some(DropTarget::new(
                DropOperation::Copy,
                DropFormats::TEXT,
            ))))
        ],
        NativeUpdate::Framework(FrameworkUpdate::Width(_)) => [
            NativeUpdate::Framework(FrameworkUpdate::Width(Dimension::Pixels(24.0)))
        ],
        NativeUpdate::Framework(FrameworkUpdate::Height(_)) => [
            NativeUpdate::Framework(FrameworkUpdate::Height(Dimension::Pixels(24.0)))
        ],
        NativeUpdate::Framework(FrameworkUpdate::MinWidth(_)) => [
            NativeUpdate::Framework(FrameworkUpdate::MinWidth(Dimension::Pixels(24.0)))
        ],
        NativeUpdate::Framework(FrameworkUpdate::MaxWidth(_)) => [
            NativeUpdate::Framework(FrameworkUpdate::MaxWidth(Dimension::Pixels(24.0)))
        ],
        NativeUpdate::Framework(FrameworkUpdate::MinHeight(_)) => [
            NativeUpdate::Framework(FrameworkUpdate::MinHeight(Dimension::Pixels(24.0)))
        ],
        NativeUpdate::Framework(FrameworkUpdate::MaxHeight(_)) => [
            NativeUpdate::Framework(FrameworkUpdate::MaxHeight(Dimension::Pixels(24.0)))
        ],
        NativeUpdate::Framework(FrameworkUpdate::Margin(_)) => [
            NativeUpdate::Framework(FrameworkUpdate::Margin(Some(Thickness::uniform(4.0))))
        ],
        NativeUpdate::Framework(FrameworkUpdate::Padding(_)) => [
            NativeUpdate::Framework(FrameworkUpdate::Padding(Thickness::uniform(4.0)))
        ],
        NativeUpdate::Framework(FrameworkUpdate::HorizontalAlignment(_)) => [
            NativeUpdate::Framework(FrameworkUpdate::HorizontalAlignment(Some(
                HorizontalAlignment::Center,
            )))
        ],
        NativeUpdate::Framework(FrameworkUpdate::VerticalAlignment(_)) => [
            NativeUpdate::Framework(FrameworkUpdate::VerticalAlignment(Some(
                VerticalAlignment::Center,
            )))
        ],
        NativeUpdate::Control(ControlUpdate::ButtonEmphasis(_)) => [
            NativeUpdate::Control(ControlUpdate::ButtonEmphasis(ButtonEmphasis::Accent))
        ],
        NativeUpdate::Control(ControlUpdate::FlyoutPlacement(_)) => [
            NativeUpdate::Control(ControlUpdate::FlyoutPlacement(FlyoutPlacement::Bottom))
        ],
        NativeUpdate::Control(ControlUpdate::MenuBar(_)) => [
            NativeUpdate::Control(ControlUpdate::MenuBar(Vec::new()))
        ],
        NativeUpdate::Control(ControlUpdate::MenuFlyout(_)) => [
            NativeUpdate::Control(ControlUpdate::MenuFlyout(Vec::new()))
        ],
        NativeUpdate::Framework(FrameworkUpdate::Opacity(_)) => [
            NativeUpdate::Framework(FrameworkUpdate::Opacity(Some(0.5)))
        ],
        NativeUpdate::Framework(FrameworkUpdate::FontSize(_)) => [
            NativeUpdate::Framework(FrameworkUpdate::FontSize(Some(18.0)))
        ],
        NativeUpdate::TextStyle(TextStyleUpdate::FontFamily(_)) => [
            NativeUpdate::TextStyle(TextStyleUpdate::FontFamily(Some("Segoe UI".into())))
        ],
        NativeUpdate::TextStyle(TextStyleUpdate::Foreground(_)) => [
            NativeUpdate::TextStyle(TextStyleUpdate::Foreground(Some(
                Color::rgb(10, 20, 30).into(),
            )))
        ],
        NativeUpdate::Framework(FrameworkUpdate::CharacterSpacing(_)) => [
            NativeUpdate::Framework(FrameworkUpdate::CharacterSpacing(Some(100)))
        ],
        NativeUpdate::Framework(FrameworkUpdate::FontWeight(_)) => [
            NativeUpdate::Framework(FrameworkUpdate::FontWeight(Some(FontWeight::BOLD)))
        ],
        NativeUpdate::Framework(FrameworkUpdate::FontStyle(_)) => [
            NativeUpdate::Framework(FrameworkUpdate::FontStyle(Some(FontStyle::Italic)))
        ],
        NativeUpdate::Framework(FrameworkUpdate::FontStretch(_)) => [
            NativeUpdate::Framework(FrameworkUpdate::FontStretch(Some(FontStretch::Expanded)))
        ],
        NativeUpdate::Framework(FrameworkUpdate::TextWrapping(_)) => [
            NativeUpdate::Framework(FrameworkUpdate::TextWrapping(Some(TextWrapping::Wrap)))
        ],
        NativeUpdate::Framework(FrameworkUpdate::TextSelectionEnabled(_)) => [
            NativeUpdate::Framework(FrameworkUpdate::TextSelectionEnabled(Some(true)))
        ],
        NativeUpdate::Framework(FrameworkUpdate::TextTrimming(_)) => [
            NativeUpdate::Framework(FrameworkUpdate::TextTrimming(Some(
                TextTrimming::WordEllipsis,
            )))
        ],
        NativeUpdate::Framework(FrameworkUpdate::Visibility(_)) => [
            NativeUpdate::Framework(FrameworkUpdate::Visibility(Some(Visibility::Collapsed)))
        ],
        NativeUpdate::Framework(FrameworkUpdate::Enabled(_)) => [
            NativeUpdate::Framework(FrameworkUpdate::Enabled(true))
        ],
        NativeUpdate::Control(ControlUpdate::TextBlockText(_)) => [
            NativeUpdate::Control(ControlUpdate::TextBlockText(String::new()))
        ],
        NativeUpdate::Control(ControlUpdate::HyperlinkButtonNavigateUri(_)) => [
            NativeUpdate::Control(ControlUpdate::HyperlinkButtonNavigateUri(Some(
                "https://example.com".to_string(),
            )))
        ],
        NativeUpdate::Control(ControlUpdate::RepeatButton(RepeatButtonUpdate::Delay(_))) => [
            NativeUpdate::Control(ControlUpdate::RepeatButton(RepeatButtonUpdate::Delay(500)))
        ],
        NativeUpdate::Control(ControlUpdate::RepeatButton(RepeatButtonUpdate::Interval(_))) => [
            NativeUpdate::Control(ControlUpdate::RepeatButton(RepeatButtonUpdate::Interval(
                33,
            )))
        ],
        NativeUpdate::Control(ControlUpdate::ToggleChecked(_)) => [
            NativeUpdate::Control(ControlUpdate::ToggleChecked(false))
        ],
        NativeUpdate::Control(ControlUpdate::ToggleSwitch(ToggleSwitchUpdate::On(_))) => [
            NativeUpdate::Control(ControlUpdate::ToggleSwitch(ToggleSwitchUpdate::On(false)))
        ],
        NativeUpdate::Control(ControlUpdate::ToggleSwitch(ToggleSwitchUpdate::Content(_))) => [
            NativeUpdate::Control(ControlUpdate::ToggleSwitch(ToggleSwitchUpdate::Content(
                Box::new(ToggleSwitchContentUpdate {
                    header: Some("Header".into()),
                    on_content: Some("Yes".into()),
                    off_content: Some("No".into()),
                }),
            )))
        ],
        NativeUpdate::Control(ControlUpdate::InfoBadgeValue(_)) => [
            NativeUpdate::Control(ControlUpdate::InfoBadgeValue(Some(42)))
        ],
        NativeUpdate::Control(ControlUpdate::InfoBar(_)) => [
            NativeUpdate::Control(ControlUpdate::InfoBar(Box::new(InfoBarUpdate {
                title: "Title".into(),
                message: "Message".into(),
                severity: InfoBarSeverity::Warning,
                open: true,
                closable: true,
            })))
        ],
        NativeUpdate::Control(ControlUpdate::PersonPicture(_)) => [
            NativeUpdate::Control(ControlUpdate::PersonPicture(Box::new(
                PersonPictureUpdate {
                    display_name: Some("Ada Lovelace".into()),
                    initials: Some("AL".into()),
                },
            )))
        ],
        NativeUpdate::Control(ControlUpdate::ColorPicker(ColorPickerUpdate::Color(_))) => [
            NativeUpdate::Control(ControlUpdate::ColorPicker(ColorPickerUpdate::Color(
                Color::rgb(10, 20, 30),
            )))
        ],
        NativeUpdate::Control(ControlUpdate::ColorPicker(ColorPickerUpdate::AlphaEnabled(_))) => [
            NativeUpdate::Control(ControlUpdate::ColorPicker(ColorPickerUpdate::AlphaEnabled(
                true,
            )))
        ],
        NativeUpdate::Control(ControlUpdate::ColorPicker(
            ColorPickerUpdate::HexInputVisible(_)
        )) => [
            NativeUpdate::Control(ControlUpdate::ColorPicker(
                ColorPickerUpdate::HexInputVisible(true),
            ))
        ],
        NativeUpdate::Control(ControlUpdate::ColorPicker(ColorPickerUpdate::SliderVisible(_))) => [
            NativeUpdate::Control(ControlUpdate::ColorPicker(
                ColorPickerUpdate::SliderVisible(true),
            ))
        ],
        NativeUpdate::Control(ControlUpdate::ColorPicker(
            ColorPickerUpdate::ChannelInputVisible(_)
        )) => [
            NativeUpdate::Control(ControlUpdate::ColorPicker(
                ColorPickerUpdate::ChannelInputVisible(true),
            ))
        ],
        NativeUpdate::Control(ControlUpdate::TimePicker(_)) => [
            NativeUpdate::Control(ControlUpdate::TimePicker(Box::new(TimePickerUpdate {
                time: Some(TimeSpan::from_minutes(30)),
                header: Some("Time".into()),
                minute_increment: 15,
            })))
        ],
        NativeUpdate::Control(ControlUpdate::CalendarView(_)) => [
            NativeUpdate::Control(ControlUpdate::CalendarView(Box::new(CalendarViewUpdate {
                selected_dates: Rc::from([DateTime::UNIX_EPOCH]),
                selection_mode: CalendarSelectionMode::Single,
                today_highlighted: true,
                group_label_visible: false,
            })))
        ],
        NativeUpdate::Control(ControlUpdate::RichEditBox(_)) => [
            NativeUpdate::Control(ControlUpdate::RichEditBox(Box::new(RichEditBoxUpdate {
                text: "text".into(),
                header: Some("header".into()),
                placeholder: Some("placeholder".into()),
                read_only: false,
            })))
        ],
        NativeUpdate::Control(ControlUpdate::RichTextBlock(_)) => [
            NativeUpdate::Control(ControlUpdate::RichTextBlock(Box::new(
                RichTextBlockUpdate {
                    paragraphs: Rc::from([RichTextParagraph::new([RichTextInline::Run(
                        RichTextRun::plain("text"),
                    )])]),
                    font_size: Some(14.0),
                    selectable: true,
                    wrap: true,
                },
            )))
        ],
        NativeUpdate::Control(ControlUpdate::TreeView(_)) => [
            NativeUpdate::Control(ControlUpdate::TreeView(Box::new(TreeViewUpdate::Nodes(
                Rc::from([TreeNode::new(1, "node")]),
            ))))
        ],
        NativeUpdate::Control(ControlUpdate::NavigationView(NavigationUpdate::Properties(_))) => [
            NativeUpdate::Control(ControlUpdate::NavigationView(NavigationUpdate::Properties(
                Box::new(NavigationViewUpdate {
                    header: Some("Header".into()),
                    pane_title: Some("Pane".into()),
                    settings_visible: false,
                    pane_toggle_visible: true,
                    pane_open: true,
                    open_pane_length: 240.0,
                    pane_display_mode: NavigationPaneDisplayMode::Left,
                    selection_feedback: true,
                    pane_feedback: true,
                }),
            )))
        ],
        NativeUpdate::Control(ControlUpdate::NavigationView(NavigationUpdate::Selection(_))) => [
            NativeUpdate::Control(ControlUpdate::NavigationView(NavigationUpdate::Selection(
                Some(1),
            )))
        ],
        NativeUpdate::Control(ControlUpdate::NavigationViewItem(_)) => [
            NativeUpdate::Control(ControlUpdate::NavigationViewItem(Box::new(
                NavigationViewItemUpdate {
                    item_key: 1,
                    label: "Home".into(),
                    icon: Some(Icon::symbol(IconSymbol::HOME)),
                },
            )))
        ],
        NativeUpdate::Control(ControlUpdate::Collection(CollectionUpdate::ItemCount(_))) => [
            NativeUpdate::Control(ControlUpdate::Collection(CollectionUpdate::ItemCount(0)))
        ],
        NativeUpdate::Control(ControlUpdate::Collection(CollectionUpdate::ItemKeys(_))) => [
            NativeUpdate::Control(ControlUpdate::Collection(CollectionUpdate::ItemKeys(
                Rc::from([]),
            )))
        ],
        NativeUpdate::Control(ControlUpdate::Collection(CollectionUpdate::SelectionMode(_))) => [
            NativeUpdate::Control(ControlUpdate::Collection(CollectionUpdate::SelectionMode(
                SelectionMode::Multiple,
            )))
        ],
        NativeUpdate::Control(ControlUpdate::Collection(CollectionUpdate::Selection(_))) => [
            NativeUpdate::Control(ControlUpdate::Collection(CollectionUpdate::Selection(
                CollectionSelection::new([1, 2]),
            )))
        ],
        NativeUpdate::Control(ControlUpdate::Collection(
            CollectionUpdate::SelectionDisplayOnly(_)
        )) => [
            NativeUpdate::Control(ControlUpdate::Collection(
                CollectionUpdate::SelectionDisplayOnly(true),
            ))
        ],
        NativeUpdate::Control(ControlUpdate::Collection(CollectionUpdate::ItemClickEnabled(_))) => [
            NativeUpdate::Control(ControlUpdate::Collection(
                CollectionUpdate::ItemClickEnabled(true),
            ))
        ],
        NativeUpdate::Control(ControlUpdate::Collection(CollectionUpdate::CanReorderItems(_))) => [
            NativeUpdate::Control(ControlUpdate::Collection(
                CollectionUpdate::CanReorderItems(true),
            ))
        ],
        NativeUpdate::Control(ControlUpdate::ViewboxStretch(_)) => [
            NativeUpdate::Control(ControlUpdate::ViewboxStretch(Stretch::UniformToFill))
        ],
        NativeUpdate::Control(ControlUpdate::ScrollViewer(
            ScrollViewerUpdate::HorizontalScrollBarVisibility(_)
        )) => [
            NativeUpdate::Control(ControlUpdate::ScrollViewer(
                ScrollViewerUpdate::HorizontalScrollBarVisibility(ScrollBarVisibility::Auto),
            ))
        ],
        NativeUpdate::Control(ControlUpdate::ScrollViewer(
            ScrollViewerUpdate::VerticalScrollBarVisibility(_)
        )) => [
            NativeUpdate::Control(ControlUpdate::ScrollViewer(
                ScrollViewerUpdate::VerticalScrollBarVisibility(ScrollBarVisibility::Visible),
            ))
        ],
        NativeUpdate::Control(ControlUpdate::ScrollViewer(ScrollViewerUpdate::ViewChanged(_))) => [
            NativeUpdate::Control(ControlUpdate::ScrollViewer(
                ScrollViewerUpdate::ViewChanged(true),
            ))
        ],
        NativeUpdate::Control(ControlUpdate::ScrollView(
            ScrollViewUpdate::HorizontalScrollBarVisibility(_)
        )) => [
            NativeUpdate::Control(ControlUpdate::ScrollView(
                ScrollViewUpdate::HorizontalScrollBarVisibility(
                    ScrollViewBarVisibility::Visible,
                ),
            ))
        ],
        NativeUpdate::Control(ControlUpdate::ScrollView(
            ScrollViewUpdate::VerticalScrollBarVisibility(_)
        )) => [
            NativeUpdate::Control(ControlUpdate::ScrollView(
                ScrollViewUpdate::VerticalScrollBarVisibility(ScrollViewBarVisibility::Hidden),
            ))
        ],
        NativeUpdate::Control(ControlUpdate::ScrollView(
            ScrollViewUpdate::ContentOrientation(_)
        )) => [
            NativeUpdate::Control(ControlUpdate::ScrollView(
                ScrollViewUpdate::ContentOrientation(ScrollOrientation::Both),
            ))
        ],
        NativeUpdate::Control(ControlUpdate::ScrollView(ScrollViewUpdate::ViewChanged(_))) => [
            NativeUpdate::Control(ControlUpdate::ScrollView(ScrollViewUpdate::ViewChanged(
                true,
            )))
        ],
        NativeUpdate::Control(ControlUpdate::SplitView(SplitViewUpdate::DisplayMode(_))) => [
            NativeUpdate::Control(ControlUpdate::SplitView(SplitViewUpdate::DisplayMode(
                SplitViewDisplayMode::CompactOverlay,
            )))
        ],
        NativeUpdate::Control(ControlUpdate::SplitView(SplitViewUpdate::IsPaneOpen(_))) => [
            NativeUpdate::Control(ControlUpdate::SplitView(SplitViewUpdate::IsPaneOpen(false)))
        ],
        NativeUpdate::Control(ControlUpdate::SplitView(SplitViewUpdate::OpenPaneLength(_))) => [
            NativeUpdate::Control(ControlUpdate::SplitView(SplitViewUpdate::OpenPaneLength(
                280.0,
            )))
        ],
        NativeUpdate::Control(ControlUpdate::SplitView(SplitViewUpdate::CompactPaneLength(_))) => [
            NativeUpdate::Control(ControlUpdate::SplitView(
                SplitViewUpdate::CompactPaneLength(40.0),
            ))
        ],
        NativeUpdate::Control(ControlUpdate::SplitView(SplitViewUpdate::PaneClosed(_))) => [
            NativeUpdate::Control(ControlUpdate::SplitView(SplitViewUpdate::PaneClosed(true)))
        ],
        NativeUpdate::Control(ControlUpdate::Expander(ExpanderUpdate::Expanded(_))) => [
            NativeUpdate::Control(ControlUpdate::Expander(ExpanderUpdate::Expanded(true)))
        ],
        NativeUpdate::Control(ControlUpdate::Expander(ExpanderUpdate::ExpandedChanged(_))) => [
            NativeUpdate::Control(ControlUpdate::Expander(ExpanderUpdate::ExpandedChanged(
                true,
            )))
        ],
        NativeUpdate::Control(ControlUpdate::Grid(GridUpdate::Columns(_))) => [
            NativeUpdate::Control(ControlUpdate::Grid(GridUpdate::Columns(
                vec![GridLength::Auto, GridLength::Pixel(64.0), GridLength::STAR]
                    .into_boxed_slice(),
            )))
        ],
        NativeUpdate::Control(ControlUpdate::Grid(GridUpdate::Rows(_))) => [
            NativeUpdate::Control(ControlUpdate::Grid(GridUpdate::Rows(
                vec![GridLength::Pixel(18.0)].into_boxed_slice(),
            )))
        ],
        NativeUpdate::Control(ControlUpdate::Grid(GridUpdate::ColumnSpacing(_))) => [
            NativeUpdate::Control(ControlUpdate::Grid(GridUpdate::ColumnSpacing(6.0)))
        ],
        NativeUpdate::Control(ControlUpdate::Grid(GridUpdate::RowSpacing(_))) => [
            NativeUpdate::Control(ControlUpdate::Grid(GridUpdate::RowSpacing(8.0)))
        ],
        NativeUpdate::Control(ControlUpdate::StackPanel(StackPanelUpdate::Orientation(_))) => [
            NativeUpdate::Control(ControlUpdate::StackPanel(StackPanelUpdate::Orientation(
                Orientation::Horizontal,
            )))
        ],
        NativeUpdate::Control(ControlUpdate::StackPanel(StackPanelUpdate::Spacing(_))) => [
            NativeUpdate::Control(ControlUpdate::StackPanel(StackPanelUpdate::Spacing(12.0)))
        ],
        NativeUpdate::Control(ControlUpdate::ListBox(ListBoxUpdate::Items(_))) => [
            NativeUpdate::Control(ControlUpdate::ListBox(ListBoxUpdate::Items(Rc::from([
                ListBoxItem::new(7, "seven"),
            ]))))
        ],
        NativeUpdate::Control(ControlUpdate::ListBox(ListBoxUpdate::SelectionMode(_))) => [
            NativeUpdate::Control(ControlUpdate::ListBox(ListBoxUpdate::SelectionMode(
                SelectionMode::Multiple,
            )))
        ],
        NativeUpdate::Control(ControlUpdate::ListBox(ListBoxUpdate::Selection(_))) => [
            NativeUpdate::Control(ControlUpdate::ListBox(ListBoxUpdate::Selection(
                CollectionSelection::new([7]),
            )))
        ],
        NativeUpdate::Control(ControlUpdate::RadioButtonGroupName(_)) => [
            NativeUpdate::Control(ControlUpdate::RadioButtonGroupName(Some("group".into())))
        ],
        NativeUpdate::Control(ControlUpdate::Pivot(PivotUpdate::Title(_))) => [
            NativeUpdate::Control(ControlUpdate::Pivot(PivotUpdate::Title(Some(
                "Title".into(),
            ))))
        ],
        NativeUpdate::Control(ControlUpdate::IndexSelector(_)) => [
            NativeUpdate::Control(ControlUpdate::IndexSelector(1))
        ],
        NativeUpdate::Control(ControlUpdate::TabView(TabViewUpdate::CanReorderTabs(_))) => [
            NativeUpdate::Control(ControlUpdate::TabView(TabViewUpdate::CanReorderTabs(true)))
        ],
        NativeUpdate::Control(ControlUpdate::TabView(TabViewUpdate::IsAddTabButtonVisible(_))) => [
            NativeUpdate::Control(ControlUpdate::TabView(
                TabViewUpdate::IsAddTabButtonVisible(true),
            ))
        ],
        NativeUpdate::Control(ControlUpdate::TabViewItem(TabViewItemUpdate::Key(_))) => [
            NativeUpdate::Control(ControlUpdate::TabViewItem(TabViewItemUpdate::Key(1)))
        ],
        NativeUpdate::Control(ControlUpdate::TabViewItem(TabViewItemUpdate::Header(_))) => [
            NativeUpdate::Control(ControlUpdate::TabViewItem(TabViewItemUpdate::Header(
                "Header".into(),
            )))
        ],
        NativeUpdate::Control(ControlUpdate::TabViewItem(TabViewItemUpdate::Closable(_))) => [
            NativeUpdate::Control(ControlUpdate::TabViewItem(TabViewItemUpdate::Closable(
                true,
            )))
        ],
        NativeUpdate::Control(ControlUpdate::SelectorBarSelection(_)) => [
            NativeUpdate::Control(ControlUpdate::SelectorBarSelection(Some(1)))
        ],
        NativeUpdate::Control(ControlUpdate::SelectorBarItem(SelectorBarItemUpdate::Key(_))) => [
            NativeUpdate::Control(ControlUpdate::SelectorBarItem(SelectorBarItemUpdate::Key(
                1,
            )))
        ],
        NativeUpdate::Control(ControlUpdate::SelectorBarItem(SelectorBarItemUpdate::Text(_))) => [
            NativeUpdate::Control(ControlUpdate::SelectorBarItem(SelectorBarItemUpdate::Text(
                "Item".into(),
            )))
        ],
        NativeUpdate::Control(ControlUpdate::SelectorBarItem(SelectorBarItemUpdate::Icon(_))) => [
            NativeUpdate::Control(ControlUpdate::SelectorBarItem(SelectorBarItemUpdate::Icon(
                None,
            )))
        ],
        NativeUpdate::Control(ControlUpdate::BreadcrumbBarItems(_)) => [
            NativeUpdate::Control(ControlUpdate::BreadcrumbBarItems(Rc::from([])))
        ],
        NativeUpdate::Control(ControlUpdate::AutoSuggestBox(_)) => [
            NativeUpdate::Control(ControlUpdate::AutoSuggestBox(Box::new(
                AutoSuggestUpdate::Text("Text".into()),
            )))
        ],
        NativeUpdate::Control(ControlUpdate::PivotItemHeader(_)) => [
            NativeUpdate::Control(ControlUpdate::PivotItemHeader("Header".into()))
        ],
        NativeUpdate::Control(ControlUpdate::TeachingTip(TeachingTipUpdate::Title(_))) => [
            NativeUpdate::Control(ControlUpdate::TeachingTip(TeachingTipUpdate::Title(
                "Title".into(),
            )))
        ],
        NativeUpdate::Control(ControlUpdate::TeachingTip(
            TeachingTipUpdate::Subtitle(_)
        )) => [
            NativeUpdate::Control(ControlUpdate::TeachingTip(TeachingTipUpdate::Subtitle(
                "Subtitle".into(),
            )))
        ],
        NativeUpdate::Control(ControlUpdate::TeachingTip(TeachingTipUpdate::Open(_))) => [
            NativeUpdate::Control(ControlUpdate::TeachingTip(TeachingTipUpdate::Open(true)))
        ],
        NativeUpdate::Control(ControlUpdate::TeachingTip(
            TeachingTipUpdate::LightDismiss(_)
        )) => [
            NativeUpdate::Control(ControlUpdate::TeachingTip(TeachingTipUpdate::LightDismiss(
                true,
            )))
        ],
        NativeUpdate::Control(ControlUpdate::TeachingTip(
            TeachingTipUpdate::ActionButton(_)
        )) => [
            NativeUpdate::Control(ControlUpdate::TeachingTip(TeachingTipUpdate::ActionButton(
                Some("Action".into()),
            )))
        ],
        NativeUpdate::Control(ControlUpdate::TeachingTip(
            TeachingTipUpdate::CloseButton(_)
        )) => [
            NativeUpdate::Control(ControlUpdate::TeachingTip(TeachingTipUpdate::CloseButton(
                Some("Close".into()),
            )))
        ],
        NativeUpdate::Control(ControlUpdate::TeachingTip(
            TeachingTipUpdate::ActionButtonClick(_)
        )) => [
            NativeUpdate::Control(ControlUpdate::TeachingTip(
                TeachingTipUpdate::ActionButtonClick(true),
            ))
        ],
        NativeUpdate::Control(ControlUpdate::ContentDialog(_)) => [
            NativeUpdate::Control(ControlUpdate::ContentDialog(Box::new(
                ContentDialogUpdate {
                    primary_button_text: "Primary".into(),
                    secondary_button_text: "Secondary".into(),
                    close_button_text: "Close".into(),
                    primary_button_enabled: true,
                    secondary_button_enabled: true,
                    open: true,
                },
            )))
        ],
        NativeUpdate::Control(ControlUpdate::CommandBar(_)) => [
            NativeUpdate::Control(ControlUpdate::CommandBar(
                CommandBarDefaultLabelPosition::Right,
            ))
        ],
        NativeUpdate::Control(ControlUpdate::Image(_)) => [
            NativeUpdate::Control(ControlUpdate::Image(Box::new(ImageUpdate {
                source: ImageSource::bitmap("ms-appx:///image.png"),
                source_revision: 1,
                source_changed: true,
                stretch: Stretch::Uniform,
            })))
        ],
        NativeUpdate::Control(ControlUpdate::AppBarButton(_)) => [
            NativeUpdate::Control(ControlUpdate::AppBarButton(Box::new(AppBarButtonUpdate {
                label: "Open".into(),
                enabled: true,
                icon: None,
            })))
        ],
        NativeUpdate::Control(ControlUpdate::AppBarToggleButton(_)) => [
            NativeUpdate::Control(ControlUpdate::AppBarToggleButton(Box::new(
                AppBarToggleButtonUpdate {
                    label: "Pin".into(),
                    enabled: true,
                    checked: false,
                    icon: None,
                },
            )))
        ],
    }
    families {
        NativeUpdate::Control(ControlUpdate::Border(update)) => update, update.as_ref() => {
            BorderUpdate::Background(_) => [
                NativeUpdate::Control(ControlUpdate::Border(Box::new(BorderUpdate::Background(
                    Some(Color::rgb(1, 2, 3).into()),
                ))))
            ],
            BorderUpdate::Padding(_) => [
                NativeUpdate::Control(ControlUpdate::Border(Box::new(BorderUpdate::Padding(
                    Some(Thickness::uniform(4.0)),
                ))))
            ],
            BorderUpdate::BorderBrush(_) => [
                NativeUpdate::Control(ControlUpdate::Border(Box::new(BorderUpdate::BorderBrush(
                    Some(Color::rgb(4, 5, 6).into()),
                ))))
            ],
            BorderUpdate::BorderThickness(_) => [
                NativeUpdate::Control(ControlUpdate::Border(Box::new(
                    BorderUpdate::BorderThickness(Some(Thickness::uniform(2.0))),
                )))
            ],
            BorderUpdate::CornerRadius(_) => [
                NativeUpdate::Control(ControlUpdate::Border(Box::new(BorderUpdate::CornerRadius(
                    Some(CornerRadius::uniform(8.0)),
                ))))
            ],
        }
        NativeUpdate::Control(ControlUpdate::TitleBar(update)) => update, update.as_ref() => {
            TitleBarUpdate::Title(_) => [
                NativeUpdate::Control(ControlUpdate::TitleBar(Box::new(TitleBarUpdate::Title(
                    Some("Title".into()),
                ))))
            ],
            TitleBarUpdate::Subtitle(_) => [
                NativeUpdate::Control(ControlUpdate::TitleBar(Box::new(TitleBarUpdate::Subtitle(
                    Some("Subtitle".into()),
                ))))
            ],
            TitleBarUpdate::BackButtonVisible(_) => [
                NativeUpdate::Control(ControlUpdate::TitleBar(Box::new(
                    TitleBarUpdate::BackButtonVisible(true),
                )))
            ],
            TitleBarUpdate::BackButtonEnabled(_) => [
                NativeUpdate::Control(ControlUpdate::TitleBar(Box::new(
                    TitleBarUpdate::BackButtonEnabled(true),
                )))
            ],
            TitleBarUpdate::PaneToggleButtonVisible(_) => [
                NativeUpdate::Control(ControlUpdate::TitleBar(Box::new(
                    TitleBarUpdate::PaneToggleButtonVisible(true),
                )))
            ],
        }
        NativeUpdate::Control(ControlUpdate::Icon(icon)) => icon, icon.kind() => {
            IconKind::Symbol(_) => [
                NativeUpdate::Control(ControlUpdate::Icon(Box::new(Icon::symbol(
                    IconSymbol::SAVE,
                ))))
            ],
            IconKind::Font { .. } => [
                NativeUpdate::Control(ControlUpdate::Icon(Box::new(Icon::font(
                    "\u{E10F}",
                    "Segoe Fluent Icons",
                ))))
            ],
            IconKind::Bitmap { .. } => [
                NativeUpdate::Control(ControlUpdate::Icon(Box::new(Icon::bitmap(
                    "ms-appx:///image.png",
                    true,
                ))))
            ],
            IconKind::Image(_) => [
                NativeUpdate::Control(ControlUpdate::Icon(Box::new(Icon::image(
                    ImageSource::svg("ms-appx:///image.svg"),
                ))))
            ],
            IconKind::Path(_) => [
                NativeUpdate::Control(ControlUpdate::Icon(Box::new(Icon::path("M 0,0 L 1,1"))))
            ],
        }
        NativeUpdate::Control(ControlUpdate::Shape(update)) => update, update.kind => {
            ShapeKind::Rectangle => [
                NativeUpdate::Control(ControlUpdate::Shape(Box::new(ShapeUpdate {
                    kind: ShapeKind::Rectangle,
                    fill: None,
                    stroke: None,
                    stroke_thickness: None,
                    corner_radius: Some(4.0),
                    line: [0.0; 4],
                })))
            ],
            ShapeKind::Ellipse => [
                NativeUpdate::Control(ControlUpdate::Shape(Box::new(ShapeUpdate {
                    kind: ShapeKind::Ellipse,
                    fill: None,
                    stroke: None,
                    stroke_thickness: None,
                    corner_radius: None,
                    line: [0.0; 4],
                })))
            ],
            ShapeKind::Line => [
                NativeUpdate::Control(ControlUpdate::Shape(Box::new(ShapeUpdate {
                    kind: ShapeKind::Line,
                    fill: None,
                    stroke: None,
                    stroke_thickness: Some(2.0),
                    corner_radius: None,
                    line: [0.0, 1.0, 2.0, 3.0],
                })))
            ],
        }
        NativeUpdate::Control(ControlUpdate::TextBox(update)) => update, update.as_ref() => {
            TextBoxUpdate::Text(_) => [
                NativeUpdate::Control(ControlUpdate::TextBox(Box::new(TextBoxUpdate::Text(
                    String::new(),
                ))))
            ],
            TextBoxUpdate::Header(_) => [
                NativeUpdate::Control(ControlUpdate::TextBox(Box::new(TextBoxUpdate::Header(
                    Some("Header".into()),
                ))))
            ],
            TextBoxUpdate::Placeholder(_) => [
                NativeUpdate::Control(ControlUpdate::TextBox(Box::new(
                    TextBoxUpdate::Placeholder(Some("Placeholder".into())),
                )))
            ],
            TextBoxUpdate::AcceptsReturn(_) => [
                NativeUpdate::Control(ControlUpdate::TextBox(Box::new(
                    TextBoxUpdate::AcceptsReturn(true),
                )))
            ],
            TextBoxUpdate::Chrome(_) => [
                NativeUpdate::Control(ControlUpdate::TextBox(Box::new(TextBoxUpdate::Chrome(
                    Box::new(ControlChromeUpdate {
                        background: Some(Color::rgb(1, 2, 3).into()),
                        border_brush: Some(Color::rgb(4, 5, 6).into()),
                        border_thickness: Some(Thickness::uniform(2.0)),
                    }),
                ))))
            ],
        }
        NativeUpdate::Control(ControlUpdate::PasswordBox(update)) => update, update.as_ref() => {
            PasswordBoxUpdate::Password(_) => [
                NativeUpdate::Control(ControlUpdate::PasswordBox(Box::new(
                    PasswordBoxUpdate::Password(String::new()),
                )))
            ],
            PasswordBoxUpdate::Header(_) => [
                NativeUpdate::Control(ControlUpdate::PasswordBox(Box::new(
                    PasswordBoxUpdate::Header(Some("Header".into())),
                )))
            ],
            PasswordBoxUpdate::Placeholder(_) => [
                NativeUpdate::Control(ControlUpdate::PasswordBox(Box::new(
                    PasswordBoxUpdate::Placeholder(Some("Placeholder".into())),
                )))
            ],
            PasswordBoxUpdate::RevealMode(_) => [
                NativeUpdate::Control(ControlUpdate::PasswordBox(Box::new(
                    PasswordBoxUpdate::RevealMode(PasswordRevealMode::Hidden),
                )))
            ],
        }
        NativeUpdate::Control(ControlUpdate::ProgressBar(update)) => update, update.as_ref() => {
            ProgressBarUpdate::Range(_) => [
                NativeUpdate::Control(ControlUpdate::ProgressBar(Box::new(
                    ProgressBarUpdate::Range(RangeState {
                        value: 25.0,
                        minimum: 0.0,
                        maximum: 100.0,
                    }),
                )))
            ],
            ProgressBarUpdate::Indeterminate(_) => [
                NativeUpdate::Control(ControlUpdate::ProgressBar(Box::new(
                    ProgressBarUpdate::Indeterminate(true),
                )))
            ],
        }
        NativeUpdate::Control(ControlUpdate::ProgressRing(update)) => update, update.as_ref() => {
            ProgressRingUpdate::Range(_) => [
                NativeUpdate::Control(ControlUpdate::ProgressRing(Box::new(
                    ProgressRingUpdate::Range(RangeState {
                        value: 25.0,
                        minimum: 0.0,
                        maximum: 100.0,
                    }),
                )))
            ],
            ProgressRingUpdate::Active(_) => [
                NativeUpdate::Control(ControlUpdate::ProgressRing(Box::new(
                    ProgressRingUpdate::Active(true),
                )))
            ],
            ProgressRingUpdate::Indeterminate(_) => [
                NativeUpdate::Control(ControlUpdate::ProgressRing(Box::new(
                    ProgressRingUpdate::Indeterminate(true),
                )))
            ],
        }
        NativeUpdate::Control(ControlUpdate::Slider(update)) => update, update.as_ref() => {
            SliderUpdate::Range(_) => [
                NativeUpdate::Control(ControlUpdate::Slider(Box::new(SliderUpdate::Range(
                    RangeState {
                        value: 25.0,
                        minimum: 0.0,
                        maximum: 100.0,
                    },
                ))))
            ],
            SliderUpdate::Orientation(_) => [
                NativeUpdate::Control(ControlUpdate::Slider(Box::new(SliderUpdate::Orientation(
                    Orientation::Vertical,
                ))))
            ],
            SliderUpdate::Step(_) => [
                NativeUpdate::Control(ControlUpdate::Slider(Box::new(SliderUpdate::Step(2.0))))
            ],
            SliderUpdate::Header(_) => [
                NativeUpdate::Control(ControlUpdate::Slider(Box::new(SliderUpdate::Header(Some(
                    "Volume".into(),
                )))))
            ],
        }
        NativeUpdate::Control(ControlUpdate::NumberBox(update)) => update, update => {
            NumberBoxUpdate::Bounds { .. } => [
                NativeUpdate::Control(ControlUpdate::NumberBox(NumberBoxUpdate::Bounds {
                    minimum: 0.0,
                    maximum: 100.0,
                }))
            ],
            NumberBoxUpdate::Value(_) => [
                NativeUpdate::Control(ControlUpdate::NumberBox(NumberBoxUpdate::Value(None)))
            ],
            NumberBoxUpdate::Header(_) => [
                NativeUpdate::Control(ControlUpdate::NumberBox(NumberBoxUpdate::Header(Some(
                    "Quantity".into(),
                ))))
            ],
        }
        NativeUpdate::Control(ControlUpdate::RatingControl(update)) => update, update => {
            RatingControlUpdate::Max(_) => [
                NativeUpdate::Control(ControlUpdate::RatingControl(RatingControlUpdate::Max(10)))
            ],
            RatingControlUpdate::Placeholder(_) => [
                NativeUpdate::Control(ControlUpdate::RatingControl(
                    RatingControlUpdate::Placeholder(Some(7.5)),
                ))
            ],
            RatingControlUpdate::Caption(_) => [
                NativeUpdate::Control(ControlUpdate::RatingControl(RatingControlUpdate::Caption(
                    "Average".to_string(),
                )))
            ],
            RatingControlUpdate::ReadOnly(_) => [
                NativeUpdate::Control(ControlUpdate::RatingControl(RatingControlUpdate::ReadOnly(
                    true,
                )))
            ],
            RatingControlUpdate::Value(_) => [
                NativeUpdate::Control(ControlUpdate::RatingControl(RatingControlUpdate::Value(
                    None,
                )))
            ],
        }
        NativeUpdate::Control(ControlUpdate::DatePicker(update)) => update, update => {
            DatePickerUpdate::Date(_) => [
                NativeUpdate::Control(ControlUpdate::DatePicker(DatePickerUpdate::Date(None)))
            ],
            DatePickerUpdate::Header(_) => [
                NativeUpdate::Control(ControlUpdate::DatePicker(DatePickerUpdate::Header(Some(
                    "Date".into(),
                ))))
            ],
            DatePickerUpdate::DayVisible(_) => [
                NativeUpdate::Control(ControlUpdate::DatePicker(DatePickerUpdate::DayVisible(
                    true,
                )))
            ],
            DatePickerUpdate::MonthVisible(_) => [
                NativeUpdate::Control(ControlUpdate::DatePicker(DatePickerUpdate::MonthVisible(
                    true,
                )))
            ],
            DatePickerUpdate::YearVisible(_) => [
                NativeUpdate::Control(ControlUpdate::DatePicker(DatePickerUpdate::YearVisible(
                    true,
                )))
            ],
        }
        NativeUpdate::Control(ControlUpdate::CalendarDatePicker(update)) =>
            update, update.as_ref() => {
            CalendarDatePickerUpdate::Date(_) => [
                NativeUpdate::Control(ControlUpdate::CalendarDatePicker(Box::new(
                    CalendarDatePickerUpdate::Date(None),
                )))
            ],
            CalendarDatePickerUpdate::Header(_) => [
                NativeUpdate::Control(ControlUpdate::CalendarDatePicker(Box::new(
                    CalendarDatePickerUpdate::Header(Some("Date".into())),
                )))
            ],
            CalendarDatePickerUpdate::Placeholder(_) => [
                NativeUpdate::Control(ControlUpdate::CalendarDatePicker(Box::new(
                    CalendarDatePickerUpdate::Placeholder(Some("Choose".into())),
                )))
            ],
            CalendarDatePickerUpdate::TodayHighlighted(_) => [
                NativeUpdate::Control(ControlUpdate::CalendarDatePicker(Box::new(
                    CalendarDatePickerUpdate::TodayHighlighted(false),
                )))
            ],
        }
        NativeUpdate::Control(ControlUpdate::ComboBox(update)) => update, update.as_ref() => {
            ComboBoxUpdate::Items(_) => [
                NativeUpdate::Control(ControlUpdate::ComboBox(Box::new(ComboBoxUpdate::Items(
                    Rc::from([SelectorItem::new(7, "seven")]),
                ))))
            ],
            ComboBoxUpdate::Selection(_) => [
                NativeUpdate::Control(ControlUpdate::ComboBox(Box::new(
                    ComboBoxUpdate::Selection(Some(7)),
                )))
            ],
            ComboBoxUpdate::Header(_) => [
                NativeUpdate::Control(ControlUpdate::ComboBox(Box::new(ComboBoxUpdate::Header(
                    Some("Header".into()),
                ))))
            ],
            ComboBoxUpdate::Placeholder(_) => [
                NativeUpdate::Control(ControlUpdate::ComboBox(Box::new(
                    ComboBoxUpdate::Placeholder(Some("Placeholder".into())),
                )))
            ],
            ComboBoxUpdate::Editable(_) => [
                NativeUpdate::Control(ControlUpdate::ComboBox(Box::new(ComboBoxUpdate::Editable(
                    true,
                ))))
            ],
        }
        NativeUpdate::Control(ControlUpdate::RadioButtons(update)) => update, update => {
            RadioButtonsUpdate::Items(_) => [
                NativeUpdate::Control(ControlUpdate::RadioButtons(RadioButtonsUpdate::Items(
                    Rc::from([SelectorItem::new(7, "seven")]),
                )))
            ],
            RadioButtonsUpdate::Selection(_) => [
                NativeUpdate::Control(ControlUpdate::RadioButtons(RadioButtonsUpdate::Selection(
                    Some(7),
                )))
            ],
            RadioButtonsUpdate::Header(_) => [
                NativeUpdate::Control(ControlUpdate::RadioButtons(RadioButtonsUpdate::Header(
                    Some("Header".into()),
                )))
            ],
            RadioButtonsUpdate::MaxColumns(_) => [
                NativeUpdate::Control(ControlUpdate::RadioButtons(RadioButtonsUpdate::MaxColumns(
                    2,
                )))
            ],
        }
        #[cfg(feature = "canvas")]
        NativeUpdate::Control(ControlUpdate::CanvasImage(update)) => update, update => {
            #[cfg(feature = "canvas")]
            CanvasUpdate::Props { .. } => [
                NativeUpdate::Control(ControlUpdate::CanvasImage(CanvasUpdate::Props {
                    draw: canvas::CanvasDrawCallback::new(|_| Ok(())),
                    invalidation_revision: 1,
                }))
            ],
            #[cfg(feature = "canvas")]
            CanvasUpdate::Rebind { .. } => [
                NativeUpdate::Control(ControlUpdate::CanvasImage(CanvasUpdate::Rebind {
                    draw: canvas::CanvasDrawCallback::new(|_| Ok(())),
                    invalidation_revision: 2,
                }))
            ],
            #[cfg(feature = "canvas")]
            CanvasUpdate::Invalidate(_) => [
                NativeUpdate::Control(ControlUpdate::CanvasImage(CanvasUpdate::Invalidate(3)))
            ],
        }
        #[cfg(feature = "canvas")]
        NativeUpdate::Control(ControlUpdate::SwapChainCanvas(update)) =>
            update, update.as_ref() => {
            #[cfg(feature = "canvas")]
            SwapChainCanvasUpdate::Canvas(CanvasUpdate::Props { .. }) => [
                NativeUpdate::Control(ControlUpdate::SwapChainCanvas(Box::new(
                    SwapChainCanvasUpdate::Canvas(CanvasUpdate::Props {
                        draw: canvas::CanvasDrawCallback::new(|_| Ok(())),
                        invalidation_revision: 1,
                    }),
                )))
            ],
            #[cfg(feature = "canvas")]
            SwapChainCanvasUpdate::Canvas(CanvasUpdate::Rebind { .. }) => [
                NativeUpdate::Control(ControlUpdate::SwapChainCanvas(Box::new(
                    SwapChainCanvasUpdate::Canvas(CanvasUpdate::Rebind {
                        draw: canvas::CanvasDrawCallback::new(|_| Ok(())),
                        invalidation_revision: 2,
                    }),
                )))
            ],
            #[cfg(feature = "canvas")]
            SwapChainCanvasUpdate::Canvas(CanvasUpdate::Invalidate(_)) => [
                NativeUpdate::Control(ControlUpdate::SwapChainCanvas(Box::new(
                    SwapChainCanvasUpdate::Canvas(CanvasUpdate::Invalidate(3)),
                )))
            ],
            #[cfg(feature = "canvas")]
            SwapChainCanvasUpdate::Continuous(_) => [
                NativeUpdate::Control(ControlUpdate::SwapChainCanvas(Box::new(
                    SwapChainCanvasUpdate::Continuous(true),
                )))
            ],
        }
        NativeUpdate::Control(ControlUpdate::CompositionHost(update)) =>
            update, update.as_ref() => {
            CompositionHostUpdate::Initialize { .. } => [
                NativeUpdate::Control(ControlUpdate::CompositionHost(Box::new(
                    CompositionHostUpdate::Initialize {
                        factory: composition::CompositionFactory::new::<()>(|_| {
                            panic!("not invoked")
                        }),
                        layout: composition::CompositionLayoutCallback::new(
                            |_: &mut (), _| Ok(()),
                        ),
                    },
                )))
            ],
            CompositionHostUpdate::LayoutCallback(_) => [
                NativeUpdate::Control(ControlUpdate::CompositionHost(Box::new(
                    CompositionHostUpdate::LayoutCallback(
                        composition::CompositionLayoutCallback::new(
                            |_: &mut (), _| Ok(()),
                        ),
                    ),
                )))
            ],
            CompositionHostUpdate::Action(_) => [
                NativeUpdate::Control(ControlUpdate::CompositionHost(Box::new(
                    CompositionHostUpdate::Action(
                        composition::CompositionAction::new(
                            |_: &mut ()| Ok(()),
                        ),
                    ),
                )))
            ],
        }
        #[cfg(feature = "canvas")]
        NativeUpdate::Control(ControlUpdate::SwapChainHost(update)) =>
            update, update.as_ref() => {
            #[cfg(feature = "canvas")]
            SwapChainHostUpdate::Initialize { .. } => [
                NativeUpdate::Control(ControlUpdate::SwapChainHost(Box::new(
                    SwapChainHostUpdate::Initialize {
                        factory: canvas::SwapChainHostFactory::new::<()>(|_| {
                            panic!("not invoked")
                        }),
                        layout: canvas::SwapChainHostLayoutCallback::new(
                            |_: &mut (), _, _| Ok(()),
                        ),
                        frame: canvas::SwapChainHostFrameCallback::new(
                            |_: &mut (), _, _| Ok(()),
                        ),
                        continuous: false,
                    },
                )))
            ],
            #[cfg(feature = "canvas")]
            SwapChainHostUpdate::Props { .. } => [
                NativeUpdate::Control(ControlUpdate::SwapChainHost(Box::new(
                    SwapChainHostUpdate::Props {
                        layout: canvas::SwapChainHostLayoutCallback::new(
                            |_: &mut (), _, _| Ok(()),
                        ),
                        frame: canvas::SwapChainHostFrameCallback::new(
                            |_: &mut (), _, _| Ok(()),
                        ),
                        continuous: true,
                    },
                )))
            ],
            #[cfg(feature = "canvas")]
            SwapChainHostUpdate::Action(_) => [
                NativeUpdate::Control(ControlUpdate::SwapChainHost(Box::new(
                    SwapChainHostUpdate::Action(
                        canvas::SwapChainHostAction::Invalidate,
                    ),
                )))
            ],
        }
        #[cfg(feature = "webview")]
        NativeUpdate::Control(ControlUpdate::WebViewHost(update)) => update, update => {
            #[cfg(feature = "webview")]
            WebViewHostUpdate::Source(_) => [
                NativeUpdate::Control(ControlUpdate::WebViewHost(
                    WebViewHostUpdate::Source(Some("https://example.com".to_string())),
                ))
            ],
            #[cfg(feature = "webview")]
            WebViewHostUpdate::Action(_) => [
                NativeUpdate::Control(ControlUpdate::WebViewHost(
                    WebViewHostUpdate::Action(webview::WebViewAction::Reload),
                ))
            ],
        }
    }
}

fn queue_capability_case(
    engine: &mut Engine<RecordingRuntime>,
    id: NodeId,
    update: NativeUpdate,
) -> Result<(), EngineError> {
    match update {
        NativeUpdate::Resources(resources) => engine.queue_resources_update(id, *resources),
        NativeUpdate::Visual(update) => engine.queue_visual_update(id, update),
        NativeUpdate::Accessibility(update) => engine.queue_accessibility_update(id, update),
        NativeUpdate::Attached(update) => engine.queue_attached_update(id, update),
        NativeUpdate::Input(update) => engine.queue_input_update(id, update),
        NativeUpdate::TextStyle(update) => engine.queue_text_style_update(id, update),
        NativeUpdate::Framework(update) => engine.queue_framework_update(id, update),
        NativeUpdate::Control(update) => engine.queue_control_update(id, update),
    }
}
mod protocol {
    use super::*;

    macro_rules! native_event_cases {
        (
            $(
                $(#[$attr:meta])*
                $variant:ident { $($field:ident $(: $value:expr)?),* $(,)? }
            ),*
            $(,)?
        ) => {{
            let cases = vec![
                $($(#[$attr])* NativeEvent::$variant { $($field $(: $value)?),* },)*
            ];
            let variants = cases
                .iter()
                .map(|event| match event {
                    $($(#[$attr])* NativeEvent::$variant { .. } => stringify!($variant),)*
                })
                .collect::<BTreeSet<_>>();
            assert_eq!(variants.len(), cases.len());
            cases
        }};
    }

    fn event_cases(target: NodeId) -> Vec<NativeEvent> {
        native_event_cases! {
            TimerFired {
                owner: target,
                slot: 0,
                revision: 1,
            },
            WindowCloseRequested { target },
            WindowSizeChanged {
                target,
                size: WindowSize {
                    width: 1.0,
                    height: 1.0,
                },
            },
            WindowColorSchemeChanged {
                target,
                scheme: ColorScheme::Light,
            },
            Click { target },
            MenuItemClick { target, key: 1 },
            TextChanged {
                target,
                value: String::new(),
            },
            PasswordChanged {
                target,
                value: String::new(),
            },
            Toggled {
                target,
                value: false,
            },
            ValueChanged { target, value: 0.0 },
            OptionalValueChanged {
                target,
                value: None,
            },
            ColorChanged {
                target,
                value: Color::rgb(0, 0, 0),
            },
            DateChanged {
                target,
                value: None,
            },
            TimeChanged {
                target,
                value: None,
            },
            DatesChanged {
                target,
                value: Vec::new(),
            },
            KeyboardAcceleratorInvoked {
                target,
                accelerator: KeyboardAcceleratorSpec {
                    key: VirtualKey::S,
                    modifiers: VirtualKeyModifiers::NONE,
                },
            },
            Pointer {
                target,
                kind: PointerEventKind::Pressed,
                event: PointerEvent::default(),
            },
            Tapped { target },
            RightTapped { target },
            Drop {
                target,
                result: Box::new(Err(windows_core::Error::new(
                    windows_core::HRESULT(0x80004005_u32 as i32),
                    "drop",
                ))),
            },
            Scroll {
                target,
                event: ScrollEvent::default(),
            },
            PaneClosed { target },
            NavigationPaneOpenChanged {
                target,
                open: false,
            },
            NavigationDisplayModeChanged {
                target,
                mode: NavigationDisplayMode::Compact,
            },
            ExpandedChanged {
                target,
                expanded: false,
            },
            TreeNodeExpandedChanged {
                target,
                key: 1,
                expanded: false,
            },
            TeachingTipClosed { target },
            TeachingTipAction { target },
            InfoBarCloseRequested { target },
            TitleBarBackRequested { target },
            TitleBarPaneRequested { target },
            FlyoutOpened { target },
            FlyoutClosed { target },
            ContentDialogClosed {
                target,
                result: ContentDialogResult::None,
            },
            ImageLoad {
                target,
                source_revision: 1,
                result: Err(windows_core::Error::new(
                    windows_core::HRESULT(0x80004005_u32 as i32),
                    "image",
                )),
            },
            CompositionLayout {
                target,
                width: 1.0,
                height: 1.0,
                rasterization_scale: 1.0,
            },
            DeferredReady {
                target,
                revision: 1,
                action: DeferredAction::ContentDialogOpen,
            },
            ItemInvoked { target, key: 1 },
            QuerySubmitted {
                target,
                value: String::new(),
            },
            SelectionChanged {
                target,
                selection: CollectionSelection::default(),
            },
            IndexChanged {
                target,
                index: Some(0),
            },
            TabCloseRequested { target, key: 1 },
            AddTabButtonClick { target },
            TabsReordered {
                target,
                keys: Vec::new(),
            },
            ItemsReordered {
                target,
                keys: Vec::new(),
            },
            SelectedKeyChanged { target, key: None },
            Realize {
                host: target,
                index: 0,
                lease: 1,
            },
            Recycle {
                host: target,
                index: 0,
                lease: 1,
            },
            #[cfg(feature = "canvas")]
            CanvasImageLayout {
                target,
                width: 1.0,
                height: 1.0,
                scale: 1.0,
            },
            #[cfg(feature = "canvas")]
            CanvasImageFrame { target },
            #[cfg(feature = "canvas")]
            CanvasLayout {
                target,
                width: 1.0,
                height: 1.0,
                scale_x: 1.0,
                scale_y: 1.0,
            },
            #[cfg(feature = "canvas")]
            CanvasFrame { target },
            #[cfg(feature = "canvas")]
            SwapChainHostLayout {
                target,
                layout: Box::new(SwapChainHostLayout {
                    width: 1.0,
                    height: 1.0,
                    scale_x: 1.0,
                    scale_y: 1.0,
                    pixel_width: 1,
                    pixel_height: 1,
                    revision: 1,
                }),
            },
            #[cfg(feature = "canvas")]
            SwapChainHostFrame { target },
            #[cfg(feature = "webview")]
            WebViewInitializationReady {
                target,
                revision: 1,
            },
            #[cfg(feature = "webview")]
            WebViewCreated {
                target,
                result: Ok(()),
            },
            #[cfg(feature = "webview")]
            WebViewNavigationCompleted {
                target,
                navigation_id: 1,
                is_success: true,
                source: "https://example.com".into(),
            },
        }
    }

    #[test]
    fn accessibility_keeps_the_protocol_size_floor() {
        assert_eq!(size_of::<AccessibilityUpdate>(), 32);
        assert_eq!(size_of::<FrameworkUpdate>(), 40);
        assert_eq!(size_of::<TextStyleUpdate>(), 32);
        #[cfg(not(any(feature = "canvas", feature = "webview")))]
        assert_eq!(size_of::<ControlUpdate>(), 32);
        #[cfg(any(feature = "canvas", feature = "webview"))]
        assert_eq!(size_of::<ControlUpdate>(), 40);
        #[cfg(not(any(feature = "canvas", feature = "webview")))]
        assert_eq!(size_of::<NativeUpdate>(), 40);
        #[cfg(any(feature = "canvas", feature = "webview"))]
        assert_eq!(size_of::<NativeUpdate>(), 48);
        #[cfg(not(any(feature = "canvas", feature = "webview")))]
        assert_eq!(size_of::<Command>(), 48);
        #[cfg(any(feature = "canvas", feature = "webview"))]
        assert_eq!(size_of::<Command>(), 56);
        assert_eq!(size_of::<NativeEvent>(), 40);
    }

    #[test]
    fn every_native_event_has_one_production_owned_class_and_compatibility_family() {
        let target = NodeId::new(7, 11);
        let cases = event_cases(target);
        let names = cases.iter().map(NativeEvent::name).collect::<BTreeSet<_>>();

        assert_eq!(names.len(), cases.len());
        assert!(cases.iter().all(|event| event.target() == target));
        for event in &cases {
            let _ = (event.class(), event.compatibility());
        }
    }

    #[test]
    fn native_event_class_and_compatibility_boundaries_are_semantic() {
        let target = NodeId::new(7, 11);
        for (event, class, compatibility) in [
            (
                NativeEvent::TextChanged {
                    target,
                    value: String::new(),
                },
                NativeEventClass::ControlledFeedback,
                NativeEventCompatibility::Text,
            ),
            (
                NativeEvent::Click { target },
                NativeEventClass::NotificationOnly,
                NativeEventCompatibility::Click,
            ),
            (
                NativeEvent::WindowCloseRequested { target },
                NativeEventClass::CancelableRequest,
                NativeEventCompatibility::Window,
            ),
            (
                NativeEvent::TeachingTipClosed { target },
                NativeEventClass::ClosureFailureSynchronization,
                NativeEventCompatibility::TeachingTip,
            ),
            (
                NativeEvent::TimerFired {
                    owner: target,
                    slot: 0,
                    revision: 1,
                },
                NativeEventClass::InternalRuntime,
                NativeEventCompatibility::TimerOwner,
            ),
        ] {
            assert_eq!(event.class(), class);
            assert_eq!(event.compatibility(), compatibility);
        }

        assert_eq!(
            NativeEvent::NavigationPaneOpenChanged {
                target,
                open: false,
            }
            .compatibility(),
            NativeEvent::NavigationDisplayModeChanged {
                target,
                mode: NavigationDisplayMode::Compact,
            }
            .compatibility()
        );
        assert_eq!(
            NativeEvent::NavigationPaneOpenChanged {
                target,
                open: false,
            }
            .class(),
            NativeEventClass::ControlledFeedback
        );
        assert_eq!(
            NativeEvent::NavigationDisplayModeChanged {
                target,
                mode: NavigationDisplayMode::Compact,
            }
            .class(),
            NativeEventClass::NotificationOnly
        );
    }

    #[test]
    fn deferred_event_compatibility_keeps_action_specific_targets() {
        let target = NodeId::new(3, 5);
        for (action, compatibility) in [
            (
                DeferredAction::ContentDialogOpen,
                NativeEventCompatibility::DeferredContentDialog,
            ),
            (
                DeferredAction::TeachingTipOpen,
                NativeEventCompatibility::DeferredTeachingTip,
            ),
            (
                DeferredAction::RadioButtonsSelection,
                NativeEventCompatibility::DeferredRadioButtons,
            ),
        ] {
            let event = NativeEvent::DeferredReady {
                target,
                revision: 1,
                action,
            };
            assert_eq!(event.class(), NativeEventClass::InternalRuntime);
            assert_eq!(event.compatibility(), compatibility);
        }
    }

    #[test]
    fn font_weight_reserves_zero_for_the_unset_state() {
        assert_eq!(FontWeight::from_weight(0), None);
        assert_eq!(FontWeight::from_weight(1).unwrap().weight(), 1);
        assert_eq!(FontWeight::from_weight(999).unwrap().weight(), 999);
        assert_eq!(FontWeight::from_weight(1000), None);
    }

    #[test]
    fn grid_definitions_reject_the_wrong_native_kind() {
        let mut engine = Engine::new(RecordingRuntime::default());
        let text = engine.create_native(NativeKind::TextBlock).unwrap();
        assert!(matches!(
            engine.set_grid_columns(text, vec![GridLength::Auto]),
            Err(EngineError::UnsupportedCommand { .. })
        ));
        assert!(matches!(
            engine.queue_control_update(
                text,
                ControlUpdate::StackPanel(StackPanelUpdate::Spacing(12.0))
            ),
            Err(EngineError::UnsupportedCommand { .. })
        ));
        let button = engine.create_native(NativeKind::Button).unwrap();
        assert!(matches!(
            engine.set_padding(button, Some(Thickness::uniform(4.0))),
            Err(EngineError::UnsupportedCommand { .. })
        ));
    }

    #[test]
    fn incompatible_property_commands_fail_before_native_apply() {
        let mut engine = Engine::new(RecordingRuntime::default());
        let text = engine.create_native(NativeKind::TextBlock).unwrap();

        assert!(matches!(
            engine.queue_control_update(text, ControlUpdate::ToggleChecked(true)),
            Err(EngineError::UnsupportedCommand {
                id,
                kind: NativeKind::TextBlock,
                command: "set checked",
            }) if id == text
        ));
        let logical = engine.create_logical().unwrap();
        assert!(matches!(
            engine.set_width(logical, Some(24.0)),
            Err(EngineError::InvalidNode(id)) if id == logical
        ));
        let ordinary_list = engine.create_native(NativeKind::ListView).unwrap();
        assert!(matches!(
            engine.set_virtual_item_count(ordinary_list, 1),
            Err(EngineError::InvalidNode(id)) if id == ordinary_list
        ));
        assert!(engine.is_valid());
        engine.commit().unwrap();
        engine.remove_subtree(logical).unwrap();
        assert!(!engine.contains(logical));
        assert!(matches!(
            engine.remove_subtree(logical),
            Err(EngineError::InvalidNode(id)) if id == logical
        ));
    }

    #[test]
    fn grouped_control_updates_reject_neighboring_native_kinds() {
        let mut engine = Engine::new(RecordingRuntime::default());
        let password = engine.create_native(NativeKind::PasswordBox).unwrap();
        let progress_bar = engine.create_native(NativeKind::ProgressBar).unwrap();
        let radio_buttons = engine.create_native(NativeKind::RadioButtons).unwrap();
        let navigation_item = engine
            .create_native(NativeKind::NavigationViewItem)
            .unwrap();

        for (id, update, kind, command) in [
            (
                password,
                ControlUpdate::TextBox(Box::new(TextBoxUpdate::Header(None))),
                NativeKind::PasswordBox,
                "set text box header",
            ),
            (
                progress_bar,
                ControlUpdate::ProgressRing(Box::new(ProgressRingUpdate::Active(true))),
                NativeKind::ProgressBar,
                "set ProgressRing active state",
            ),
            (
                radio_buttons,
                ControlUpdate::ComboBox(Box::new(ComboBoxUpdate::Editable(true))),
                NativeKind::RadioButtons,
                "set ComboBox editable",
            ),
            (
                navigation_item,
                ControlUpdate::NavigationView(NavigationUpdate::Selection(Some(1))),
                NativeKind::NavigationViewItem,
                "set NavigationView selection",
            ),
        ] {
            assert!(matches!(
                engine.queue_control_update(id, update),
                Err(EngineError::UnsupportedCommand {
                    id: target,
                    kind: actual_kind,
                    command: actual_command,
                }) if target == id && actual_kind == kind && actual_command == command
            ));
        }
    }

    #[test]
    fn engine_errors_have_specific_diagnostics() {
        let mut engine = Engine::new(RecordingRuntime::default());
        let id = engine.create_logical().unwrap();
        let errors = [
            EngineError::InvalidNode(id),
            EngineError::ParentConflict {
                child: id,
                parent: id,
            },
            EngineError::RowRootAlreadyParented(id),
            EngineError::VirtualRowNativeRootCount {
                host: id,
                index: 3,
                count: 2,
            },
            EngineError::AttachedChildNativeRootCount { edge: id, count: 2 },
            EngineError::VirtualHostManaged(id),
            EngineError::VirtualRowMissing(id),
            EngineError::NativeParentRejectsChildren(id),
            EngineError::UnsupportedCommand {
                id,
                kind: NativeKind::TextBlock,
                command: "set checked",
            },
        ];

        for error in errors {
            assert!(!error.to_string().is_empty());
        }
    }

    #[test]
    fn shared_enabled_update_supports_control_kinds() {
        let mut engine = Engine::new(RecordingRuntime::default());
        let button = engine.create_native(NativeKind::Button).unwrap();
        let check_box = engine.create_native(NativeKind::CheckBox).unwrap();
        let text_box = engine.create_native(NativeKind::TextBox).unwrap();
        let border = engine.create_native(NativeKind::Border).unwrap();

        engine
            .queue_framework_update(button, FrameworkUpdate::Enabled(false))
            .unwrap();
        engine
            .queue_framework_update(check_box, FrameworkUpdate::Enabled(false))
            .unwrap();
        engine
            .queue_framework_update(text_box, FrameworkUpdate::Enabled(false))
            .unwrap();
        assert!(matches!(
            engine.queue_framework_update(border, FrameworkUpdate::Enabled(false)),
            Err(EngineError::UnsupportedCommand {
                id,
                kind: NativeKind::Border,
                command: "set enabled",
            }) if id == border
        ));
        engine.commit().unwrap();

        let updates = engine
            .runtime()
            .batches()
            .iter()
            .flatten()
            .filter(|command| {
                matches!(
                    command,
                    Command::Update {
                        update: NativeUpdate::Framework(FrameworkUpdate::Enabled(false)),
                        ..
                    }
                )
            })
            .count();
        assert_eq!(updates, 3);
    }

    #[test]
    fn category_queue_helpers_preserve_command_order() {
        let mut engine = Engine::new(RecordingRuntime::default());
        let button = engine.create_native(NativeKind::Button).unwrap();
        engine.commit().unwrap();

        engine
            .queue_visual_update(button, VisualUpdate::Scale(Some(0.5)))
            .unwrap();
        engine
            .queue_framework_update(button, FrameworkUpdate::Enabled(false))
            .unwrap();
        engine
            .queue_accessibility_update(
                button,
                AccessibilityUpdate::AutomationName("action".to_string()),
            )
            .unwrap();
        engine
            .queue_control_update(
                button,
                ControlUpdate::ButtonEmphasis(ButtonEmphasis::Accent),
            )
            .unwrap();
        engine.commit().unwrap();

        assert_eq!(
            engine.runtime().batches().last().unwrap(),
            &[
                Command::Update {
                    id: button,
                    update: NativeUpdate::Visual(VisualUpdate::Scale(Some(0.5))),
                },
                Command::Update {
                    id: button,
                    update: NativeUpdate::Framework(FrameworkUpdate::Enabled(false)),
                },
                Command::Update {
                    id: button,
                    update: NativeUpdate::Accessibility(AccessibilityUpdate::AutomationName(
                        "action".to_string(),
                    )),
                },
                Command::Update {
                    id: button,
                    update: NativeUpdate::Control(ControlUpdate::ButtonEmphasis(
                        ButtonEmphasis::Accent,
                    )),
                },
            ]
        );
    }

    #[test]
    fn typed_update_capability_matrix_is_exhaustive() {
        let all_kinds = NativeKind::ALL;
        let updates = update_capability_cases();

        let mut covered = BTreeSet::new();

        for update in &updates {
            let capability = update_capability_key(update);
            assert!(
                covered.insert(capability),
                "duplicate update capability case: {capability}"
            );
            assert!(!update.name().is_empty());

            let accepted = all_kinds
                .iter()
                .copied()
                .filter(|kind| update.supports(*kind))
                .count();
            assert!(accepted > 0, "{capability} has no compatible native kind");
            assert!(
                accepted < all_kinds.len(),
                "{capability} has no incompatible native kind"
            );
        }

        assert_eq!(covered.len(), updates.len());

        for &kind in all_kinds {
            let mut engine = Engine::new(RecordingRuntime::default());
            let id = engine.create_native(kind).unwrap();
            for update in &updates {
                let expected = update.supports(kind);
                let result = queue_capability_case(&mut engine, id, update.clone());
                assert_eq!(
                    result.is_ok(),
                    expected,
                    "{} command acceptance mismatch for {kind:?}",
                    update.name()
                );
            }
        }
    }

    #[test]
    fn shared_update_capabilities_follow_the_native_catalog() {
        let cases: [(NativeUpdate, fn(NativeKind) -> bool); 4] = [
            (
                NativeUpdate::Resources(Box::default()),
                NativeKind::supports_ui_element,
            ),
            (
                NativeUpdate::TextStyle(TextStyleUpdate::FontFamily(None)),
                NativeKind::supports_text,
            ),
            (
                NativeUpdate::Framework(FrameworkUpdate::Enabled(true)),
                NativeKind::supports_enabled,
            ),
            (
                NativeUpdate::Control(ControlUpdate::ToggleChecked(false)),
                NativeKind::supports_toggle,
            ),
        ];

        for (update, capability) in cases {
            for &kind in NativeKind::ALL {
                assert_eq!(update.supports(kind), capability(kind), "{kind:?}");
            }
        }
    }

    #[test]
    fn native_attachment_shapes_are_explicit_and_exhaustive() {
        let children = [
            NativeKind::Canvas,
            NativeKind::Grid,
            NativeKind::RelativePanel,
            NativeKind::StackPanel,
        ];
        let content = [
            NativeKind::Border,
            NativeKind::Button,
            NativeKind::DropDownButton,
            NativeKind::SplitButton,
            NativeKind::Flyout,
            NativeKind::HyperlinkButton,
            NativeKind::RepeatButton,
            NativeKind::ToggleButton,
            NativeKind::CheckBox,
            NativeKind::RadioButton,
            NativeKind::TabViewItem,
            NativeKind::PivotItem,
            NativeKind::ScrollView,
            NativeKind::ScrollViewer,
            NativeKind::ToolTip,
            NativeKind::Viewbox,
        ];
        let items = [
            NativeKind::FlipView,
            NativeKind::TabView,
            NativeKind::SelectorBar,
            NativeKind::Pivot,
        ];

        for &kind in NativeKind::ALL {
            let expected = if children.contains(&kind) {
                AttachmentShape::Children
            } else if items.contains(&kind) {
                AttachmentShape::Items
            } else if content.contains(&kind) {
                AttachmentShape::Content
            } else if matches!(kind, NativeKind::SplitView | NativeKind::TitleBar) {
                AttachmentShape::ContentPane
            } else if matches!(kind, NativeKind::Expander | NativeKind::ContentDialog) {
                AttachmentShape::HeaderContent
            } else {
                AttachmentShape::None
            };
            assert_eq!(kind.attachment_shape(), expected, "{kind:?}");
        }
    }

    #[test]
    fn compact_input_protocol_values_cover_empty_and_combined_states() {
        let events = PointerEvents::PRESSED | PointerEvents::MOVED;
        assert!(events.contains(PointerEvents::PRESSED));
        assert!(events.contains(PointerEvents::MOVED));
        assert!(!events.contains(PointerEvents::RELEASED));
        assert!(!events.contains(PointerEvents::ENTERED));
        assert!(!events.contains(PointerEvents::EXITED));
        assert!(!events.contains(PointerEvents::TAPPED));
        assert!(!events.contains(PointerEvents::RIGHT_TAPPED));

        let mut assigned = PointerEvents::PRESSED;
        assigned |= PointerEvents::RELEASED;
        assert_eq!(assigned, PointerEvents::PRESSED | PointerEvents::RELEASED);

        assert!(PointerSubscription::default().is_empty());
        assert!(
            !PointerSubscription {
                events: PointerEvents::PRESSED,
                capture_on_press: false,
            }
            .is_empty()
        );
        assert!(
            !PointerSubscription {
                events: PointerEvents::default(),
                capture_on_press: true,
            }
            .is_empty()
        );
    }
}

#[derive(Default)]
struct PartialRuntime {
    live: Vec<NodeId>,
}

impl NativeRuntime for PartialRuntime {
    fn apply(&mut self, commands: &[Command]) {
        for command in commands {
            if let Command::Create { id, .. } = command {
                self.live.push(*id);
                panic!("failure after partial mutation");
            }
        }
    }

    fn drain_events(&mut self) -> Vec<NativeEvent> {
        Vec::new()
    }
}

mod virtualization {
    use super::*;

    #[test]
    fn named_slots_select_attachments_without_projected_position() {
        let mut engine = Engine::new(RecordingRuntime::default());

        let split = engine.create_native(NativeKind::SplitView).unwrap();
        let pane_slot = engine.create_structural_slot(StructuralSlot::Pane).unwrap();
        let pane = engine.create_native(NativeKind::TextBlock).unwrap();
        engine.attach(pane_slot, pane).unwrap();
        engine.attach(split, pane_slot).unwrap();
        let content_slot = engine
            .create_structural_slot(StructuralSlot::Content)
            .unwrap();
        let content = engine.create_native(NativeKind::Border).unwrap();
        engine.attach(content_slot, content).unwrap();
        engine.attach(split, content_slot).unwrap();

        let expander = engine.create_native(NativeKind::Expander).unwrap();
        let expander_content_slot = engine
            .create_structural_slot(StructuralSlot::Content)
            .unwrap();
        let expander_content = engine.create_native(NativeKind::Grid).unwrap();
        engine
            .attach(expander_content_slot, expander_content)
            .unwrap();
        engine.attach(expander, expander_content_slot).unwrap();
        let header_slot = engine
            .create_structural_slot(StructuralSlot::Header)
            .unwrap();
        let header = engine.create_native(NativeKind::TextBlock).unwrap();
        engine.attach(header_slot, header).unwrap();
        engine.attach(expander, header_slot).unwrap();

        engine.commit().unwrap();

        assert_eq!(
            engine.node_kind(pane_slot),
            Some(&NodeKind::StructuralSlot(StructuralSlot::Pane))
        );
        assert_eq!(
            engine.node_kind(content_slot),
            Some(&NodeKind::StructuralSlot(StructuralSlot::Content))
        );
        assert_eq!(
            engine.runtime().attachment(content),
            Some(Attachment::Content)
        );
        assert_eq!(engine.runtime().attachment(pane), Some(Attachment::Pane));
        assert_eq!(
            engine.runtime().attachment(header),
            Some(Attachment::Header)
        );
        assert_eq!(
            engine.runtime().attachment(expander_content),
            Some(Attachment::Content)
        );
    }

    #[test]
    fn empty_named_slot_updates_and_teardown_preserve_peer_identity_and_order() {
        let mut engine = Engine::new(RecordingRuntime::default());
        let split = engine.create_native(NativeKind::SplitView).unwrap();
        let content_slot = engine
            .create_structural_slot(StructuralSlot::Content)
            .unwrap();
        let pane_slot = engine.create_structural_slot(StructuralSlot::Pane).unwrap();
        let pane = engine.create_native(NativeKind::TextBlock).unwrap();
        engine.attach(pane_slot, pane).unwrap();
        engine.attach(split, content_slot).unwrap();
        engine.attach(split, pane_slot).unwrap();
        engine.commit().unwrap();

        assert_eq!(engine.runtime().children(split), [pane]);
        assert_eq!(engine.runtime().attachment(pane), Some(Attachment::Pane));

        let content = engine.create_native(NativeKind::TextBlock).unwrap();
        engine.attach(content_slot, content).unwrap();
        engine.commit().unwrap();
        assert_eq!(engine.runtime().children(split), [content, pane]);
        assert_eq!(
            engine.runtime().attachment(content),
            Some(Attachment::Content)
        );
        assert_eq!(engine.runtime().attachment(pane), Some(Attachment::Pane));

        engine.remove_subtree(content).unwrap();
        engine.commit().unwrap();
        let removal = engine.runtime().batches().last().unwrap();
        let detach = removal
            .iter()
            .position(|command| {
                matches!(
                    command,
                    Command::Detach { parent, child }
                        if *parent == split && *child == content
                )
            })
            .unwrap();
        let destroy = removal
            .iter()
            .position(|command| matches!(command, Command::Destroy { id } if *id == content))
            .unwrap();
        assert!(detach < destroy);
        assert_eq!(engine.runtime().children(split), [pane]);
        assert_eq!(engine.runtime().attachment(pane), Some(Attachment::Pane));

        let replacement = engine.create_native(NativeKind::Button).unwrap();
        engine.attach(content_slot, replacement).unwrap();
        engine.commit().unwrap();
        assert_eq!(engine.runtime().children(split), [replacement, pane]);

        let empty = engine.create_logical().unwrap();
        engine.attach(content_slot, empty).unwrap();
        engine.commit().unwrap();
        let batch_count = engine.runtime().batches().len();
        engine
            .reorder_children(content_slot, &[empty, replacement])
            .unwrap();
        engine.commit().unwrap();
        assert_eq!(engine.runtime().batches().len(), batch_count);
        assert_eq!(engine.runtime().children(split), [replacement, pane]);
        assert_eq!(engine.runtime().attachment(pane), Some(Attachment::Pane));

        engine.remove_subtree(split).unwrap();
        engine.commit().unwrap();
        let teardown = engine.runtime().batches().last().unwrap();
        let replacement_destroy = teardown
            .iter()
            .position(|command| matches!(command, Command::Destroy { id } if *id == replacement))
            .unwrap();
        let pane_destroy = teardown
            .iter()
            .position(|command| matches!(command, Command::Destroy { id } if *id == pane))
            .unwrap();
        let split_destroy = teardown
            .iter()
            .position(|command| matches!(command, Command::Destroy { id } if *id == split))
            .unwrap();
        assert!(replacement_destroy < split_destroy);
        assert!(pane_destroy < split_destroy);
    }

    #[test]
    fn multiple_roots_in_one_named_slot_do_not_relabel_the_peer_slot() {
        let mut engine = Engine::new(RecordingRuntime::default());
        let split = engine.create_native(NativeKind::SplitView).unwrap();
        let pane_slot = engine.create_structural_slot(StructuralSlot::Pane).unwrap();
        let pane = engine.create_native(NativeKind::TextBlock).unwrap();
        engine.attach(pane_slot, pane).unwrap();
        engine.attach(split, pane_slot).unwrap();
        let content_slot = engine
            .create_structural_slot(StructuralSlot::Content)
            .unwrap();
        engine.attach(split, content_slot).unwrap();
        engine.commit().unwrap();

        let fragment = engine.create_logical().unwrap();
        let first = engine.create_native(NativeKind::TextBlock).unwrap();
        let second = engine.create_native(NativeKind::Button).unwrap();
        engine.attach(fragment, first).unwrap();
        engine.attach(fragment, second).unwrap();
        assert!(matches!(
            engine.attach(content_slot, fragment),
            Err(EngineError::NativeParentRejectsChildren(id)) if id == split
        ));
        assert_eq!(engine.parent(fragment), None);

        engine.commit().unwrap();
        assert_eq!(engine.runtime().children(split), [pane]);
        assert_eq!(engine.runtime().attachment(pane), Some(Attachment::Pane));
        assert_eq!(engine.runtime().parent(first), None);
        assert_eq!(engine.runtime().parent(second), None);
    }

    #[test]
    fn one_arena_owns_native_and_logical_nodes() {
        let mut engine = Engine::new(RecordingRuntime::default());
        let root = engine.create_native(NativeKind::StackPanel).unwrap();
        let component = engine.create_logical().unwrap();
        let text = engine.create_native(NativeKind::TextBlock).unwrap();

        engine.attach(root, component).unwrap();
        engine.attach(component, text).unwrap();
        engine.commit().unwrap();

        assert_eq!(engine.parent(component), Some(root));
        assert_eq!(engine.parent(text), Some(component));
        assert!(engine.runtime().contains(root));
        assert!(engine.runtime().contains(text));
        assert_eq!(engine.runtime().parent(text), Some(root));
    }

    #[test]
    fn a_node_cannot_have_two_owners() {
        let mut engine = Engine::new(RecordingRuntime::default());
        let first = engine.create_logical().unwrap();
        let second = engine.create_logical().unwrap();
        let child = engine.create_logical().unwrap();
        engine.attach(first, child).unwrap();

        assert!(matches!(
            engine.attach(second, child),
            Err(EngineError::ParentConflict {
                child: actual_child,
                parent: actual_parent,
            }) if actual_child == child && actual_parent == first
        ));
    }

    #[test]
    fn content_parent_accepts_exactly_one_projected_root() {
        let mut engine = Engine::new(RecordingRuntime::default());
        let empty_button = engine.create_native(NativeKind::Button).unwrap();
        let empty_logical = engine.create_logical().unwrap();
        assert!(matches!(
            engine.attach(empty_button, empty_logical),
            Err(EngineError::NativeParentRejectsChildren(id)) if id == empty_button
        ));

        let border = engine.create_native(NativeKind::Border).unwrap();
        let logical = engine.create_logical().unwrap();
        let text = engine.create_native(NativeKind::TextBlock).unwrap();
        engine.attach(logical, text).unwrap();
        engine.attach(border, logical).unwrap();
        engine.commit().unwrap();

        assert_eq!(engine.runtime().parent(text), Some(border));
        let second = engine.create_native(NativeKind::TextBlock).unwrap();
        assert!(matches!(
            engine.attach(border, second),
            Err(EngineError::NativeParentRejectsChildren(id)) if id == border
        ));
        assert_eq!(engine.parent(second), None);

        let repeat_button = engine.create_native(NativeKind::RepeatButton).unwrap();
        let first = engine.create_native(NativeKind::TextBlock).unwrap();
        engine.attach(repeat_button, first).unwrap();
        let second = engine.create_native(NativeKind::TextBlock).unwrap();
        assert!(matches!(
            engine.attach(repeat_button, second),
            Err(EngineError::NativeParentRejectsChildren(id)) if id == repeat_button
        ));
        assert_eq!(engine.parent(second), None);
    }

    #[test]
    fn native_realization_mounts_ordinary_arena_owned_rows() {
        let mut engine = Engine::new(RecordingRuntime::default());
        let host = engine.create_virtual_host(NativeKind::ListView).unwrap();
        engine.commit().unwrap();
        engine.runtime().queue_event(NativeEvent::Realize {
            host,
            index: 7,
            lease: 1,
        });

        let mut rows = |engine: &mut Engine<RecordingRuntime>, _, index| {
            let row = engine.create_native(NativeKind::Border)?;
            let text = engine.create_native(NativeKind::TextBlock)?;
            engine.attach(row, text)?;
            assert_eq!(index, 7);
            Ok(row)
        };
        engine.process_events(&mut rows).unwrap();

        let row = match engine.node_kind(host).unwrap() {
            NodeKind::VirtualHost { realized } => realized[&7].root,
            _ => unreachable!(),
        };
        assert_eq!(engine.parent(row), Some(host));
        assert_eq!(engine.runtime().parent(row), Some(host));
        assert_eq!(engine.node_count(), 3);
    }

    #[test]
    fn recycle_destroys_the_realized_subtree_child_first() {
        let mut engine = Engine::new(RecordingRuntime::default());
        let host = engine.create_virtual_host(NativeKind::ListView).unwrap();
        engine.commit().unwrap();
        engine.runtime().queue_event(NativeEvent::Realize {
            host,
            index: 0,
            lease: 1,
        });
        let mut rows = |engine: &mut Engine<RecordingRuntime>, _, _| {
            let row = engine.create_native(NativeKind::Border)?;
            let text = engine.create_native(NativeKind::TextBlock)?;
            engine.attach(row, text)?;
            Ok(row)
        };
        engine.process_events(&mut rows).unwrap();
        let row = match engine.node_kind(host).unwrap() {
            NodeKind::VirtualHost { realized } => realized[&0].root,
            _ => unreachable!(),
        };
        engine.runtime().queue_event(NativeEvent::Recycle {
            host,
            index: 0,
            lease: 1,
        });
        engine.process_events(&mut rows).unwrap();

        assert!(!engine.contains(row));
        assert_eq!(engine.node_count(), 1);
        let last = engine.runtime().batches().last().unwrap();
        assert_eq!(last.len(), 4);
        assert!(matches!(last[0], Command::Detach { parent, .. } if parent == row));
        assert!(matches!(last[1], Command::Destroy { .. }));
        assert_eq!(
            last[2],
            Command::Detach {
                parent: host,
                child: row
            }
        );
        assert_eq!(last[3], Command::Destroy { id: row });
    }

    #[test]
    fn stale_realization_events_are_ignored_after_slot_reuse() {
        let mut engine = Engine::new(RecordingRuntime::default());
        let old = engine.create_virtual_host(NativeKind::ListView).unwrap();
        engine.commit().unwrap();
        engine.remove_subtree(old).unwrap();
        engine.commit().unwrap();
        let replacement = engine.create_virtual_host(NativeKind::ListView).unwrap();
        engine.commit().unwrap();
        assert_eq!(old.index(), replacement.index());
        assert_ne!(old.generation(), replacement.generation());

        engine.runtime().queue_event(NativeEvent::Realize {
            host: old,
            index: 3,
            lease: 1,
        });
        let mut calls = 0;
        let mut rows = |_: &mut Engine<RecordingRuntime>, _, _| {
            calls += 1;
            unreachable!()
        };
        engine.process_events(&mut rows).unwrap();
        assert_eq!(calls, 0);
    }

    #[test]
    #[should_panic(expected = "injected failure")]
    fn failed_native_batch_panics() {
        let mut engine = Engine::new(RecordingRuntime::default());
        engine.runtime().fail_next("injected failure");
        let root = engine.create_native(NativeKind::Border).unwrap();

        _ = root;
        engine.commit().unwrap();
    }

    #[test]
    #[should_panic(expected = "failure after partial mutation")]
    fn partial_native_failure_panics() {
        let mut engine = Engine::new(PartialRuntime::default());
        engine.create_native(NativeKind::StackPanel).unwrap();
        engine.create_native(NativeKind::TextBlock).unwrap();

        engine.commit().unwrap();
    }

    #[test]
    fn queued_events_do_not_reenter_a_native_batch() {
        let mut engine = Engine::new(RecordingRuntime::default());
        let host = engine.create_virtual_host(NativeKind::ListView).unwrap();
        engine.runtime().queue_event(NativeEvent::Realize {
            host,
            index: 1,
            lease: 1,
        });
        engine.commit().unwrap();

        assert_eq!(engine.node_count(), 1);
        let mut rows = |engine: &mut Engine<RecordingRuntime>, _, _| {
            engine.create_native(NativeKind::TextBlock)
        };
        engine.process_events(&mut rows).unwrap();
        assert_eq!(engine.node_count(), 2);
    }

    #[test]
    fn realization_order_follows_item_indices() {
        let mut engine = Engine::new(RecordingRuntime::default());
        let host = engine.create_virtual_host(NativeKind::ListView).unwrap();
        engine.commit().unwrap();
        for (index, lease) in [(9, 1), (2, 2), (5, 3)] {
            engine
                .runtime()
                .queue_event(NativeEvent::Realize { host, index, lease });
        }
        let mut rows = |engine: &mut Engine<RecordingRuntime>, _, _| {
            engine.create_native(NativeKind::TextBlock)
        };

        engine.process_events(&mut rows).unwrap();

        let NodeKind::VirtualHost { realized } = engine.node_kind(host).unwrap() else {
            unreachable!()
        };
        let expected: Vec<_> = realized.values().map(|row| row.root).collect();
        assert_eq!(engine.runtime().children(host), expected);
    }

    #[test]
    fn logical_row_roots_project_native_descendants() {
        let mut engine = Engine::new(RecordingRuntime::default());
        let host = engine.create_virtual_host(NativeKind::ListView).unwrap();
        engine.commit().unwrap();
        engine.runtime().queue_event(NativeEvent::Realize {
            host,
            index: 0,
            lease: 1,
        });
        let mut rows = |engine: &mut Engine<RecordingRuntime>, _, _| {
            let logical = engine.create_logical()?;
            let native = engine.create_native(NativeKind::TextBlock)?;
            engine.attach(logical, native)?;
            Ok(logical)
        };

        engine.process_events(&mut rows).unwrap();

        let row = match engine.node_kind(host).unwrap() {
            NodeKind::VirtualHost { realized } => realized[&0].root,
            _ => unreachable!(),
        };
        assert_eq!(engine.runtime().children(host).len(), 1);
        assert_ne!(engine.runtime().children(host)[0], row);
    }

    #[test]
    fn stale_recycle_does_not_remove_a_new_lease() {
        let mut engine = Engine::new(RecordingRuntime::default());
        let host = engine.create_virtual_host(NativeKind::ListView).unwrap();
        engine.commit().unwrap();
        engine.runtime().queue_event(NativeEvent::Realize {
            host,
            index: 0,
            lease: 1,
        });
        engine.runtime().queue_event(NativeEvent::Realize {
            host,
            index: 0,
            lease: 2,
        });
        engine.runtime().queue_event(NativeEvent::Recycle {
            host,
            index: 0,
            lease: 1,
        });
        let mut rows = |engine: &mut Engine<RecordingRuntime>, _, _| {
            engine.create_native(NativeKind::TextBlock)
        };

        engine.process_events(&mut rows).unwrap();

        let NodeKind::VirtualHost { realized } = engine.node_kind(host).unwrap() else {
            unreachable!()
        };
        assert_eq!(realized[&0].lease, 2);
        assert!(engine.contains(realized[&0].root));
    }

    #[test]
    fn duplicate_events_are_idempotent() {
        let mut engine = Engine::new(RecordingRuntime::default());
        let host = engine.create_virtual_host(NativeKind::ListView).unwrap();
        engine.commit().unwrap();
        for _ in 0..2 {
            engine.runtime().queue_event(NativeEvent::Realize {
                host,
                index: 0,
                lease: 1,
            });
        }
        let mut mounts = 0;
        {
            let mut rows = |engine: &mut Engine<RecordingRuntime>, _, _| {
                mounts += 1;
                engine.create_native(NativeKind::TextBlock)
            };
            engine.process_events(&mut rows).unwrap();
        }
        assert_eq!(mounts, 1);

        for _ in 0..2 {
            engine.runtime().queue_event(NativeEvent::Recycle {
                host,
                index: 0,
                lease: 1,
            });
        }
        let mut rows = |engine: &mut Engine<RecordingRuntime>, _, _| {
            engine.create_native(NativeKind::TextBlock)
        };
        engine.process_events(&mut rows).unwrap();
        assert_eq!(engine.node_count(), 1);
    }

    #[test]
    #[should_panic(expected = "second realization failed")]
    fn a_later_event_failure_panics() {
        let mut engine = Engine::new(RecordingRuntime::default());
        let host = engine.create_virtual_host(NativeKind::ListView).unwrap();
        engine.commit().unwrap();
        for index in 0..2 {
            engine.runtime().queue_event(NativeEvent::Realize {
                host,
                index,
                lease: index as u64,
            });
        }
        let mut rows = |engine: &mut Engine<RecordingRuntime>, _, index| {
            if index == 1 {
                engine.runtime().fail_next("second realization failed");
            }
            engine.create_native(NativeKind::TextBlock)
        };

        engine.process_events(&mut rows).unwrap();
    }

    #[test]
    fn deferred_ready_events_run_only_for_live_compatible_targets() {
        for (kind, action) in [
            (NativeKind::ContentDialog, DeferredAction::ContentDialogOpen),
            (NativeKind::TeachingTip, DeferredAction::TeachingTipOpen),
            (
                NativeKind::RadioButtons,
                DeferredAction::RadioButtonsSelection,
            ),
        ] {
            let mut engine = Engine::new(RecordingRuntime::default());
            let target = engine.create_native(kind).unwrap();
            engine.commit().unwrap();
            engine.runtime().queue_event(NativeEvent::DeferredReady {
                target,
                revision: 17,
                action,
            });
            let mut rows = |engine: &mut Engine<RecordingRuntime>, _, _| {
                engine.create_native(NativeKind::TextBlock)
            };

            engine.process_events(&mut rows).unwrap();

            assert!(engine.runtime().batches().iter().flatten().any(|command| {
                matches!(
                    command,
                    Command::RunDeferred {
                        target: command_target,
                        revision: 17,
                        action: command_action,
                        ..
                    } if *command_target == target && *command_action == action
                )
            }));
        }
    }

    #[test]
    fn stale_deferred_ready_events_are_ignored_after_removal() {
        let mut engine = Engine::new(RecordingRuntime::default());
        let target = engine.create_native(NativeKind::TeachingTip).unwrap();
        engine.commit().unwrap();
        engine.runtime().queue_event(NativeEvent::DeferredReady {
            target,
            revision: 3,
            action: DeferredAction::TeachingTipOpen,
        });
        engine.remove_subtree(target).unwrap();
        engine.commit().unwrap();
        let batches = engine.runtime().batches().len();
        let mut rows = |engine: &mut Engine<RecordingRuntime>, _, _| {
            engine.create_native(NativeKind::TextBlock)
        };

        engine.process_events(&mut rows).unwrap();

        assert_eq!(engine.runtime().batches().len(), batches);
        assert!(engine.is_valid());
    }

    #[test]
    #[should_panic(expected = "deferred open failed")]
    fn deferred_command_failures_panic() {
        let mut engine = Engine::new(RecordingRuntime::default());
        let target = engine.create_native(NativeKind::ContentDialog).unwrap();
        engine.commit().unwrap();
        engine.runtime().queue_event(NativeEvent::DeferredReady {
            target,
            revision: 9,
            action: DeferredAction::ContentDialogOpen,
        });
        engine.runtime().fail_next("deferred open failed");
        let mut rows = |engine: &mut Engine<RecordingRuntime>, _, _| {
            engine.create_native(NativeKind::TextBlock)
        };

        engine.process_events(&mut rows).unwrap();
    }

    #[test]
    fn removing_a_virtual_host_removes_all_realized_rows() {
        let mut engine = Engine::new(RecordingRuntime::default());
        let host = engine.create_virtual_host(NativeKind::ListView).unwrap();
        engine.commit().unwrap();
        for index in 0..3 {
            engine.runtime().queue_event(NativeEvent::Realize {
                host,
                index,
                lease: index as u64,
            });
        }
        let mut rows = |engine: &mut Engine<RecordingRuntime>, _, _| {
            engine.create_native(NativeKind::TextBlock)
        };
        engine.process_events(&mut rows).unwrap();
        let realized: Vec<_> = match engine.node_kind(host).unwrap() {
            NodeKind::VirtualHost { realized } => realized.values().map(|row| row.root).collect(),
            _ => unreachable!(),
        };

        engine.remove_subtree(host).unwrap();
        engine.commit().unwrap();

        assert_eq!(engine.node_count(), 0);
        assert!(!engine.runtime().contains(host));
        assert!(
            realized
                .into_iter()
                .all(|row| !engine.runtime().contains(row))
        );
    }

    #[test]
    fn nested_virtual_hosts_use_the_same_realization_protocol() {
        let mut engine = Engine::new(RecordingRuntime::default());
        let outer = engine.create_virtual_host(NativeKind::ListView).unwrap();
        engine.commit().unwrap();
        engine.runtime().queue_event(NativeEvent::Realize {
            host: outer,
            index: 0,
            lease: 1,
        });
        let mut outer_rows = |engine: &mut Engine<RecordingRuntime>, _, _| {
            engine.create_virtual_host(NativeKind::ListView)
        };
        engine.process_events(&mut outer_rows).unwrap();
        let inner = match engine.node_kind(outer).unwrap() {
            NodeKind::VirtualHost { realized } => realized[&0].root,
            _ => unreachable!(),
        };
        engine.runtime().queue_event(NativeEvent::Realize {
            host: inner,
            index: 4,
            lease: 2,
        });
        let mut inner_rows = |engine: &mut Engine<RecordingRuntime>, _, _| {
            engine.create_native(NativeKind::TextBlock)
        };

        engine.process_events(&mut inner_rows).unwrap();

        assert_eq!(engine.runtime().parent(inner), Some(outer));
        assert_eq!(engine.runtime().children(inner).len(), 1);
    }

    #[test]
    fn realization_recycled_before_the_pump_is_cancelled() {
        let mut engine = Engine::new(RecordingRuntime::default());
        let host = engine.create_virtual_host(NativeKind::ListView).unwrap();
        engine.commit().unwrap();
        engine.runtime().queue_event(NativeEvent::Realize {
            host,
            index: 7,
            lease: 19,
        });
        engine.runtime().queue_event(NativeEvent::Recycle {
            host,
            index: 7,
            lease: 19,
        });
        let mut mounts = 0;
        let mut rows = |engine: &mut Engine<RecordingRuntime>, _, _| {
            mounts += 1;
            engine.create_native(NativeKind::TextBlock)
        };

        engine.process_events(&mut rows).unwrap();

        assert_eq!(mounts, 0);
        assert!(engine.is_valid());
        assert_eq!(engine.node_count(), 1);
        let NodeKind::VirtualHost { realized } = engine.node_kind(host).unwrap() else {
            unreachable!()
        };
        assert!(realized.is_empty());
    }

    #[test]
    fn realization_lifetimes_are_stable_across_every_batch_boundary() {
        let events = [(true, 3, 1), (false, 3, 1), (true, 3, 2), (false, 3, 2)];

        for boundaries in 0..8 {
            let mut engine = Engine::new(RecordingRuntime::default());
            let host = engine.create_virtual_host(NativeKind::ListView).unwrap();
            engine.commit().unwrap();
            let events = events.map(|(realize, index, lease)| {
                if realize {
                    NativeEvent::Realize { host, index, lease }
                } else {
                    NativeEvent::Recycle { host, index, lease }
                }
            });
            let mut rows = |engine: &mut Engine<RecordingRuntime>, _, _| {
                engine.create_native(NativeKind::TextBlock)
            };

            for (index, event) in events.into_iter().enumerate() {
                engine.runtime().queue_event(event);
                if index == 3 || boundaries & (1 << index) != 0 {
                    engine.process_events(&mut rows).unwrap();
                }
            }

            assert!(engine.is_valid(), "batch boundaries {boundaries:03b}");
            assert_eq!(engine.node_count(), 1, "batch boundaries {boundaries:03b}");
            assert!(
                engine.runtime().children(host).is_empty(),
                "batch boundaries {boundaries:03b}"
            );
            let NodeKind::VirtualHost { realized } = engine.node_kind(host).unwrap() else {
                unreachable!()
            };
            assert!(realized.is_empty(), "batch boundaries {boundaries:03b}");
        }
    }

    #[test]
    fn randomized_virtualization_traces_preserve_all_ownership_invariants() {
        let mut engine = Engine::new(RecordingRuntime::default());
        let hosts = [
            engine.create_virtual_host(NativeKind::ListView).unwrap(),
            engine.create_virtual_host(NativeKind::ListView).unwrap(),
        ];
        engine.commit().unwrap();
        let mut expected: [BTreeMap<usize, u64>; 2] = std::array::from_fn(|_| BTreeMap::new());
        let mut pending = Vec::new();
        let mut seed = 0x9e37_79b9_7f4a_7c15_u64;
        let mut next_lease = 1_u64;
        let mut rows = |engine: &mut Engine<RecordingRuntime>, _, _| {
            engine.create_native(NativeKind::TextBlock)
        };

        for _ in 0..5_000 {
            seed ^= seed << 7;
            seed ^= seed >> 9;
            seed ^= seed << 8;
            let host_index = seed as usize & 1;
            let host = hosts[host_index];
            let index = (seed >> 8) as usize % 8;
            let event = match (seed >> 16) % 5 {
                0 | 1 => {
                    let lease = next_lease;
                    next_lease += 1;
                    NativeEvent::Realize { host, index, lease }
                }
                2 => {
                    if let Some((&index, &lease)) = expected[host_index]
                        .iter()
                        .nth((seed >> 24) as usize % expected[host_index].len().max(1))
                    {
                        NativeEvent::Realize { host, index, lease }
                    } else {
                        let lease = next_lease;
                        next_lease += 1;
                        NativeEvent::Realize { host, index, lease }
                    }
                }
                3 => {
                    if let Some((&index, &lease)) = expected[host_index]
                        .iter()
                        .nth((seed >> 24) as usize % expected[host_index].len().max(1))
                    {
                        NativeEvent::Recycle { host, index, lease }
                    } else {
                        NativeEvent::Recycle {
                            host,
                            index,
                            lease: next_lease + 100,
                        }
                    }
                }
                _ => NativeEvent::Recycle {
                    host,
                    index,
                    lease: next_lease + ((seed >> 32) % 100),
                },
            };
            pending.push(event.clone());
            engine.runtime().queue_event(event);

            if pending.len() == 16 || seed & 7 == 0 {
                apply_expected_events(&pending, &hosts, &mut expected);
                engine.process_events(&mut rows).unwrap();
                pending.clear();
                assert_virtualization_state(&engine, &hosts, &expected);
            }
        }

        if !pending.is_empty() {
            apply_expected_events(&pending, &hosts, &mut expected);
            engine.process_events(&mut rows).unwrap();
            assert_virtualization_state(&engine, &hosts, &expected);
        }

        engine.runtime().queue_event(NativeEvent::Realize {
            host: hosts[0],
            index: 2,
            lease: next_lease,
        });
        engine.remove_subtree(hosts[0]).unwrap();
        engine.commit().unwrap();
        engine.process_events(&mut rows).unwrap();

        assert!(!engine.contains(hosts[0]));
        assert_virtualization_state(&engine, &hosts[1..], &expected[1..]);
    }

    fn apply_expected_events(
        events: &[NativeEvent],
        hosts: &[NodeId],
        expected: &mut [BTreeMap<usize, u64>],
    ) {
        let recycled: BTreeSet<_> = events
            .iter()
            .filter_map(|event| match event {
                NativeEvent::Recycle { host, index, lease } => Some((*host, *index, *lease)),
                #[cfg(feature = "canvas")]
                NativeEvent::CanvasImageLayout { .. }
                | NativeEvent::CanvasImageFrame { .. }
                | NativeEvent::CanvasLayout { .. }
                | NativeEvent::CanvasFrame { .. }
                | NativeEvent::SwapChainHostLayout { .. }
                | NativeEvent::SwapChainHostFrame { .. } => None,
                NativeEvent::TimerFired { .. }
                | NativeEvent::WindowCloseRequested { .. }
                | NativeEvent::WindowSizeChanged { .. }
                | NativeEvent::WindowColorSchemeChanged { .. }
                | NativeEvent::Click { .. }
                | NativeEvent::MenuItemClick { .. }
                | NativeEvent::TextChanged { .. }
                | NativeEvent::PasswordChanged { .. }
                | NativeEvent::Toggled { .. }
                | NativeEvent::ValueChanged { .. }
                | NativeEvent::OptionalValueChanged { .. }
                | NativeEvent::ColorChanged { .. }
                | NativeEvent::DateChanged { .. }
                | NativeEvent::TimeChanged { .. }
                | NativeEvent::DatesChanged { .. }
                | NativeEvent::KeyboardAcceleratorInvoked { .. }
                | NativeEvent::Pointer { .. }
                | NativeEvent::Tapped { .. }
                | NativeEvent::RightTapped { .. }
                | NativeEvent::Drop { .. }
                | NativeEvent::Scroll { .. }
                | NativeEvent::PaneClosed { .. }
                | NativeEvent::NavigationPaneOpenChanged { .. }
                | NativeEvent::NavigationDisplayModeChanged { .. }
                | NativeEvent::ExpandedChanged { .. }
                | NativeEvent::TreeNodeExpandedChanged { .. }
                | NativeEvent::TeachingTipClosed { .. }
                | NativeEvent::TeachingTipAction { .. }
                | NativeEvent::InfoBarCloseRequested { .. }
                | NativeEvent::TitleBarBackRequested { .. }
                | NativeEvent::TitleBarPaneRequested { .. }
                | NativeEvent::FlyoutOpened { .. }
                | NativeEvent::FlyoutClosed { .. }
                | NativeEvent::ContentDialogClosed { .. }
                | NativeEvent::ImageLoad { .. }
                | NativeEvent::CompositionLayout { .. }
                | NativeEvent::DeferredReady { .. }
                | NativeEvent::ItemInvoked { .. }
                | NativeEvent::QuerySubmitted { .. }
                | NativeEvent::SelectionChanged { .. }
                | NativeEvent::IndexChanged { .. }
                | NativeEvent::TabCloseRequested { .. }
                | NativeEvent::AddTabButtonClick { .. }
                | NativeEvent::TabsReordered { .. }
                | NativeEvent::ItemsReordered { .. }
                | NativeEvent::SelectedKeyChanged { .. }
                | NativeEvent::Realize { .. } => None,
                #[cfg(feature = "webview")]
                NativeEvent::WebViewInitializationReady { .. }
                | NativeEvent::WebViewCreated { .. }
                | NativeEvent::WebViewNavigationCompleted { .. } => None,
            })
            .collect();
        for event in events {
            let (host, index, lease, realize) = match *event {
                NativeEvent::Realize { host, index, lease } => (host, index, lease, true),
                NativeEvent::Recycle { host, index, lease } => (host, index, lease, false),
                #[cfg(feature = "canvas")]
                NativeEvent::CanvasImageLayout { .. }
                | NativeEvent::CanvasImageFrame { .. }
                | NativeEvent::CanvasLayout { .. }
                | NativeEvent::CanvasFrame { .. }
                | NativeEvent::SwapChainHostLayout { .. }
                | NativeEvent::SwapChainHostFrame { .. } => continue,
                NativeEvent::TimerFired { .. }
                | NativeEvent::WindowCloseRequested { .. }
                | NativeEvent::WindowSizeChanged { .. }
                | NativeEvent::WindowColorSchemeChanged { .. }
                | NativeEvent::Click { .. }
                | NativeEvent::MenuItemClick { .. }
                | NativeEvent::TextChanged { .. }
                | NativeEvent::PasswordChanged { .. }
                | NativeEvent::Toggled { .. }
                | NativeEvent::ValueChanged { .. }
                | NativeEvent::OptionalValueChanged { .. }
                | NativeEvent::ColorChanged { .. }
                | NativeEvent::DateChanged { .. }
                | NativeEvent::TimeChanged { .. }
                | NativeEvent::DatesChanged { .. }
                | NativeEvent::KeyboardAcceleratorInvoked { .. }
                | NativeEvent::Pointer { .. }
                | NativeEvent::Tapped { .. }
                | NativeEvent::RightTapped { .. }
                | NativeEvent::Drop { .. }
                | NativeEvent::Scroll { .. }
                | NativeEvent::PaneClosed { .. }
                | NativeEvent::NavigationPaneOpenChanged { .. }
                | NativeEvent::NavigationDisplayModeChanged { .. }
                | NativeEvent::ExpandedChanged { .. }
                | NativeEvent::TreeNodeExpandedChanged { .. }
                | NativeEvent::TeachingTipClosed { .. }
                | NativeEvent::TeachingTipAction { .. }
                | NativeEvent::InfoBarCloseRequested { .. }
                | NativeEvent::TitleBarBackRequested { .. }
                | NativeEvent::TitleBarPaneRequested { .. }
                | NativeEvent::FlyoutOpened { .. }
                | NativeEvent::FlyoutClosed { .. }
                | NativeEvent::ContentDialogClosed { .. }
                | NativeEvent::ImageLoad { .. }
                | NativeEvent::CompositionLayout { .. }
                | NativeEvent::DeferredReady { .. }
                | NativeEvent::ItemInvoked { .. }
                | NativeEvent::QuerySubmitted { .. }
                | NativeEvent::SelectionChanged { .. }
                | NativeEvent::IndexChanged { .. }
                | NativeEvent::TabCloseRequested { .. }
                | NativeEvent::AddTabButtonClick { .. }
                | NativeEvent::TabsReordered { .. }
                | NativeEvent::ItemsReordered { .. }
                | NativeEvent::SelectedKeyChanged { .. } => continue,
                #[cfg(feature = "webview")]
                NativeEvent::WebViewInitializationReady { .. }
                | NativeEvent::WebViewCreated { .. }
                | NativeEvent::WebViewNavigationCompleted { .. } => continue,
            };
            let host_index = hosts
                .iter()
                .position(|candidate| *candidate == host)
                .unwrap();
            if realize {
                if !recycled.contains(&(host, index, lease)) {
                    expected[host_index].insert(index, lease);
                }
            } else if expected[host_index].get(&index) == Some(&lease) {
                expected[host_index].remove(&index);
            }
        }
    }

    fn assert_virtualization_state(
        engine: &Engine<RecordingRuntime>,
        hosts: &[NodeId],
        expected: &[BTreeMap<usize, u64>],
    ) {
        let mut node_count = hosts.len();
        for (host, expected) in hosts.iter().zip(expected) {
            let NodeKind::VirtualHost { realized } = engine.node_kind(*host).unwrap() else {
                unreachable!()
            };
            let actual = realized
                .iter()
                .map(|(index, row)| (*index, row.lease))
                .collect::<BTreeMap<_, _>>();
            assert_eq!(&actual, expected);
            let roots = realized.values().map(|row| row.root).collect::<Vec<_>>();
            assert_eq!(engine.runtime().children(*host), roots);
            for root in roots {
                assert_eq!(engine.parent(root), Some(*host));
                assert_eq!(engine.runtime().parent(root), Some(*host));
            }
            node_count += expected.len();
        }
        assert_eq!(engine.node_count(), node_count);
        assert!(engine.is_valid());
    }
}
