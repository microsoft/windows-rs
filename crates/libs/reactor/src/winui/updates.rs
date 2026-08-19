use super::*;

impl WinUiRuntime {
    pub(super) fn apply_update(&mut self, id: NodeId, update: &NativeUpdate) -> WindowsResult<()> {
        match update {
            NativeUpdate::Resources(resources) => self.apply_element_resources(id, resources),
            NativeUpdate::Visual(update) => self.apply_visual_update(id, *update),
            NativeUpdate::Accessibility(update) => self.apply_accessibility_update(id, update),
            NativeUpdate::Attached(update) => self.apply_attached_update(id, *update),
            NativeUpdate::Input(update) => self.apply_input_update(id, update),
            NativeUpdate::TextStyle(update) => self.apply_text_style_update(id, update),
            NativeUpdate::Framework(update) => self.apply_framework_update(id, *update),
            NativeUpdate::Control(update) => self.apply_control_update(id, update),
        }
    }

    fn apply_control_update(&mut self, id: NodeId, update: &ControlUpdate) -> WindowsResult<()> {
        match update {
            ControlUpdate::Border(update) => self.apply_border_update(id, (**update).clone()),
            ControlUpdate::Shape(update) => self.apply_shape_update(id, update),
            ControlUpdate::ButtonEmphasis(value) => self.apply_button_emphasis(id, *value),
            ControlUpdate::FlyoutPlacement(value) => self.apply_flyout_placement(id, *value),
            ControlUpdate::MenuBar(items) => self.apply_menu_bar_update(id, items),
            ControlUpdate::MenuFlyout(items) => self.apply_menu_flyout_update(id, items),
            ControlUpdate::TextBlockText(text) => self.apply_text_block_text(id, text),
            ControlUpdate::RichEditBox(update) => self.apply_rich_edit_box(id, update),
            ControlUpdate::RichTextBlock(update) => self.apply_rich_text_block(id, update),
            ControlUpdate::TreeView(update) => self.apply_tree_view_update(id, update),
            ControlUpdate::TextBox(update) => self.apply_text_box_update(id, update),
            ControlUpdate::PasswordBox(update) => self.apply_password_box_update(id, update),
            ControlUpdate::HyperlinkButtonNavigateUri(value) => {
                self.apply_hyperlink_button_navigate_uri(id, value)
            }
            ControlUpdate::RepeatButton(update) => self.apply_repeat_button_update(id, *update),
            ControlUpdate::ToggleChecked(value) => self.apply_toggle_checked(id, *value),
            ControlUpdate::ToggleSwitch(update) => self.apply_toggle_switch_update(id, update),
            ControlUpdate::InfoBadgeValue(value) => self.apply_info_badge_value(id, *value),
            ControlUpdate::InfoBar(update) => self.apply_info_bar_update(id, update),
            ControlUpdate::PersonPicture(update) => self.apply_person_picture_update(id, update),
            ControlUpdate::ProgressBar(update) => self.apply_progress_bar_update(id, **update),
            ControlUpdate::ProgressRing(update) => self.apply_progress_ring_update(id, **update),
            ControlUpdate::Slider(update) => self.apply_slider_update(id, update),
            ControlUpdate::NumberBox(update) => self.apply_number_box_update(id, update),
            ControlUpdate::RatingControl(update) => self.apply_rating_control_update(id, update),
            ControlUpdate::ColorPicker(update) => self.apply_color_picker_update(id, *update),
            ControlUpdate::DatePicker(update) => self.apply_date_picker_update(id, update),
            ControlUpdate::CalendarDatePicker(update) => {
                self.apply_calendar_date_picker_update(id, update)
            }
            ControlUpdate::TimePicker(update) => self.apply_time_picker(id, update),
            ControlUpdate::CalendarView(update) => self.apply_calendar_view(id, update),
            ControlUpdate::NavigationView(update) => self.apply_navigation_update(id, update),
            ControlUpdate::NavigationViewItem(update) => {
                self.apply_navigation_view_item_update(id, update)
            }
            ControlUpdate::Grid(update) => self.apply_grid_update(id, update),
            ControlUpdate::StackPanel(update) => self.apply_stack_panel_update(id, *update),
            ControlUpdate::ListBox(ListBoxUpdate::Items(items)) => {
                self.apply_list_box_items(id, items)
            }
            ControlUpdate::ListBox(ListBoxUpdate::SelectionMode(value)) => {
                self.apply_list_box_selection_mode(id, *value)
            }
            ControlUpdate::ListBox(ListBoxUpdate::Selection(value)) => {
                self.apply_list_box_selection(id, value)
            }
            ControlUpdate::ComboBox(update) => self.apply_combo_box_update(id, update),
            ControlUpdate::RadioButtons(update) => self.apply_radio_buttons_update(id, update),
            ControlUpdate::RadioButtonGroupName(value) => match &self.node(id)?.handle {
                Handle::RadioButton { value: button, .. } => {
                    button.SetGroupName(value.as_deref().unwrap_or_default())
                }
                _ => unreachable!(),
            },
            ControlUpdate::IndexSelector(index) => self.apply_index_selector(id, *index),
            ControlUpdate::TabView(update) => self.apply_tab_view_update(id, update),
            ControlUpdate::TabViewItem(update) => self.apply_tab_view_item_update(id, update),
            ControlUpdate::SelectorBarSelection(key) => self.apply_selector_bar_selection(id, *key),
            ControlUpdate::SelectorBarItem(update) => {
                self.apply_selector_bar_item_update(id, update)
            }
            ControlUpdate::BreadcrumbBarItems(items) => self.apply_breadcrumb_bar_items(id, items),
            ControlUpdate::AutoSuggestBox(update) => self.apply_auto_suggest_box_update(id, update),
            ControlUpdate::Pivot(update) => self.apply_pivot_update(id, update),
            ControlUpdate::PivotItemHeader(value) => match &self.node(id)?.handle {
                Handle::PivotItem(item) => {
                    let item: bindings::IPivotItem = item.cast()?;
                    let header = controlled::inspectable_text(value);
                    item.SetHeader(&header)
                }
                _ => unreachable!(),
            },
            ControlUpdate::Collection(CollectionUpdate::ItemCount(count)) => {
                self.apply_collection_item_count(id, *count)
            }
            ControlUpdate::Collection(CollectionUpdate::ItemKeys(keys)) => {
                self.apply_collection_item_keys(id, keys)
            }
            ControlUpdate::Collection(CollectionUpdate::SelectionMode(value)) => {
                self.apply_collection_selection_mode(id, *value)
            }
            ControlUpdate::Collection(CollectionUpdate::Selection(value)) => {
                self.apply_collection_selection(id, value)
            }
            ControlUpdate::Collection(CollectionUpdate::SelectionDisplayOnly(value)) => {
                self.apply_collection_selection_display_only(id, *value)
            }
            ControlUpdate::Collection(CollectionUpdate::ItemClickEnabled(value)) => {
                self.apply_collection_item_click_enabled(id, *value)
            }
            ControlUpdate::Collection(CollectionUpdate::CanReorderItems(value)) => {
                self.apply_collection_can_reorder_items(id, *value)
            }
            ControlUpdate::ViewboxStretch(stretch) => self.apply_viewbox_stretch(id, *stretch),
            ControlUpdate::ScrollViewer(update) => self.apply_scroll_viewer_update(id, *update),
            ControlUpdate::ScrollView(update) => self.apply_scroll_view_update(id, *update),
            ControlUpdate::SplitView(update) => self.apply_split_view_update(id, *update),
            ControlUpdate::Expander(update) => self.apply_expander_update(id, *update),
            ControlUpdate::TeachingTip(update) => self.apply_teaching_tip_update(id, update),
            ControlUpdate::TitleBar(update) => self.apply_title_bar_update(id, update),
            ControlUpdate::ContentDialog(update) => self.apply_content_dialog_update(id, update),
            ControlUpdate::CommandBar(value) => {
                let Handle::CommandBar { value: bar, .. } = &self.node(id)?.handle else {
                    panic!("CommandBar update target is not a CommandBar");
                };
                bar.SetDefaultLabelPosition(match value {
                    CommandBarDefaultLabelPosition::Bottom => {
                        bindings::CommandBarDefaultLabelPosition::Bottom
                    }
                    CommandBarDefaultLabelPosition::Right => {
                        bindings::CommandBarDefaultLabelPosition::Right
                    }
                    CommandBarDefaultLabelPosition::Collapsed => {
                        bindings::CommandBarDefaultLabelPosition::Collapsed
                    }
                })
            }
            ControlUpdate::CompositionHost(update) => {
                self.apply_composition_host_update(id, update)
            }
            #[cfg(feature = "canvas")]
            ControlUpdate::CanvasImage(update) => self.apply_canvas_image_update(id, update),
            #[cfg(feature = "canvas")]
            ControlUpdate::SwapChainCanvas(update) => {
                self.apply_swap_chain_canvas_control_update(id, update)
            }
            #[cfg(feature = "canvas")]
            ControlUpdate::SwapChainHost(update) => self.apply_swap_chain_host_update(id, update),
            #[cfg(feature = "webview")]
            ControlUpdate::WebViewHost(update) => self.apply_webview_host_update(id, update),
            ControlUpdate::Image(update) => self.apply_image_update(id, update),
            ControlUpdate::Icon(icon) => self.apply_icon_update(id, icon),
            ControlUpdate::AppBarButton(update) => self.apply_app_bar_button_update(id, update),
            ControlUpdate::AppBarToggleButton(update) => {
                self.apply_app_bar_toggle_button_update(id, update)
            }
        }
    }
}
