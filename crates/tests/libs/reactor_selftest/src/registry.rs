use crate::fixtures::reconciler::FixtureFuture;
use crate::fixtures::{
    all_layouts, backdrop, controls, controls_extended, dynamic, error_boundary, event_detachment,
    grid_attached, hooks, interactions, layout, prop_updates, reconciler, reconciler_stress,
    tooltip,
};
use crate::harness::Harness;

pub type FixtureFn = fn(Harness) -> FixtureFuture;

pub static FIXTURES: &[(&str, FixtureFn)] = &[
    ("Reconciler_MountText", reconciler::mount_text),
    ("Reconciler_UpdateText", reconciler::update_text),
    (
        "Reconciler_AddRemoveChildren",
        reconciler::add_remove_children,
    ),
    (
        "Reconciler_ComponentRerender",
        reconciler::component_rerender,
    ),
    ("Reconciler_KeyedList", reconciler::keyed_list),
    (
        "ConditionalRendering_Toggle",
        reconciler::conditional_toggle,
    ),
    (
        "Reconciler_GridDynamicChildCount",
        reconciler::grid_dynamic_children,
    ),
    (
        "Reconciler_AllLayoutsDynamicChildCount",
        all_layouts::all_layouts_dynamic_child_count,
    ),
    ("Window_Backdrop_AllMaterials", backdrop::all_materials),
    (
        "ErrorBoundary_CatchesRenderError",
        error_boundary::catches_render_error,
    ),
    ("ErrorBoundary_Recovery", error_boundary::recovery),
    ("DynamicList_GrowShrink", dynamic::list_grow_shrink),
    ("Grid_RowColumnLayout", layout::grid_row_column_layout),
    (
        "Grid_AttachedProps_AllWidgetKinds",
        grid_attached::grid_attached_props_all_widget_kinds,
    ),
    ("Reconciler_Mount_Stack", controls::mount_stack),
    ("Reconciler_Mount_Border", controls::mount_border),
    ("Reconciler_Mount_Grid", controls::mount_grid),
    ("Reconciler_Mount_ScrollView", controls::mount_scroll_view),
    ("Reconciler_Mount_Viewbox", controls::mount_viewbox),
    ("Reconciler_Mount_Button", controls::mount_button),
    ("Reconciler_Mount_CheckBox", controls::mount_check_box),
    ("Reconciler_Mount_TextField", controls::mount_text_field),
    (
        "Reconciler_Mount_ToggleSwitch",
        controls::mount_toggle_switch,
    ),
    ("Reconciler_Mount_Slider", controls::mount_slider),
    ("Reconciler_Mount_RadioButton", controls::mount_radio_button),
    ("Reconciler_Mount_NumberBox", controls::mount_number_box),
    ("Reconciler_Mount_ProgressBar", controls::mount_progress_bar),
    (
        "Reconciler_Mount_ProgressRing",
        controls::mount_progress_ring,
    ),
    ("Reconciler_Mount_Expander", controls::mount_expander),
    (
        "Reconciler_Mount_HyperlinkButton",
        controls::mount_hyperlink_button,
    ),
    ("Reconciler_Mount_InfoBar", controls::mount_info_bar),
    ("Reconciler_Mount_InfoBadge", controls::mount_info_badge),
    ("Reconciler_Mount_Image", controls::mount_image),
    (
        "Reconciler_Mount_PersonPicture",
        controls::mount_person_picture,
    ),
    ("Reconciler_Mount_TabView", controls::mount_tab_view),
    (
        "Reconciler_Mount_NavigationView",
        controls::mount_navigation_view,
    ),
    ("Reconciler_Mount_TitleBar", controls::mount_title_bar),
    ("Reconciler_Mount_Pivot", controls::mount_pivot),
    (
        "Reconciler_Mount_BreadcrumbBar",
        controls::mount_breadcrumb_bar,
    ),
    ("Reconciler_Mount_Rectangle", controls::mount_rectangle),
    ("Reconciler_Mount_Ellipse", controls::mount_ellipse),
    ("Reconciler_Mount_Line", controls::mount_line),
    ("Reconciler_Mount_RichText", controls::mount_rich_text),
    (
        "Reconciler_Mount_TemplatedListView",
        controls::mount_templated_list_view,
    ),
    (
        "Reconciler_Mount_TemplatedGridView",
        controls::mount_templated_grid_view,
    ),
    (
        "Reconciler_Mount_TemplatedFlipView",
        controls::mount_templated_flip_view,
    ),
    (
        "Reconciler_Mount_VirtualList",
        controls::mount_virtual_list_alias,
    ),
    ("Reconciler_Mount_PasswordBox", controls::mount_password_box),
    (
        "Reconciler_Mount_RadioButtons",
        controls::mount_radio_buttons,
    ),
    ("Reconciler_Mount_ComboBox", controls::mount_combo_box),
    ("Reconciler_Mount_Canvas", controls::mount_canvas),
    (
        "Reconciler_Mount_SwapChainPanel",
        controls::mount_swap_chain_panel,
    ),
    (
        "Interaction_CheckBox_TogglesState",
        interactions::checkbox_toggles_state,
    ),
    (
        "Interaction_ToggleSwitch_ChangesState",
        interactions::toggle_switch_changes_state,
    ),
    (
        "Interaction_Slider_ValueChangesState",
        interactions::slider_value_changes_state,
    ),
    (
        "Interaction_TextField_ChangesState",
        interactions::text_field_changes_state,
    ),
    (
        "Interaction_Button_DisabledToEnabled",
        interactions::button_disabled_to_enabled_prop_update,
    ),
    (
        "Interaction_PoolChurn_GrowShrinkGrow",
        interactions::pool_churn_grow_shrink_grow,
    ),
    (
        "Interaction_PasswordBox_ChangesState",
        interactions::password_box_changes_state,
    ),
    (
        "Interaction_RadioButtons_ChangesSelection",
        interactions::radio_buttons_change_selection,
    ),
    (
        "Interaction_ComboBox_ChangesSelection",
        interactions::combo_box_changes_selection,
    ),
    (
        "Interaction_ButtonIcon_LabelPreserved",
        interactions::button_icon_label_preserved,
    ),
    (
        "Interaction_ButtonIcon_GlyphChangePreservesText",
        interactions::button_icon_glyph_change_preserves_text,
    ),
    ("Tooltip_Text_AppliedToButton", tooltip::tooltip_text),
    (
        "Tooltip_Clear_AfterRemove",
        tooltip::tooltip_clear_on_update,
    ),
    ("Tooltip_Placement_Top", tooltip::tooltip_placement),
    // ── Extended controls (previously missing mount coverage) ───────────
    (
        "Reconciler_Mount_AutoSuggestBox",
        controls_extended::mount_auto_suggest_box,
    ),
    (
        "Reconciler_Mount_CalendarDatePicker",
        controls_extended::mount_calendar_date_picker,
    ),
    (
        "Reconciler_Mount_CalendarView",
        controls_extended::mount_calendar_view,
    ),
    (
        "Reconciler_Mount_ColorPicker",
        controls_extended::mount_color_picker,
    ),
    (
        "Reconciler_Mount_CommandBar",
        controls_extended::mount_command_bar,
    ),
    (
        "Reconciler_Mount_ContentDialog",
        controls_extended::mount_content_dialog,
    ),
    (
        "Reconciler_Mount_DatePicker",
        controls_extended::mount_date_picker,
    ),
    (
        "Reconciler_Mount_DropDownButton",
        controls_extended::mount_drop_down_button,
    ),
    (
        "Reconciler_Mount_ListBox",
        controls_extended::mount_list_box,
    ),
    (
        "Reconciler_Mount_MenuBar",
        controls_extended::mount_menu_bar,
    ),
    (
        "Reconciler_Mount_RatingControl",
        controls_extended::mount_rating_control,
    ),
    (
        "Reconciler_Mount_RelativePanel",
        controls_extended::mount_relative_panel,
    ),
    (
        "Reconciler_Mount_RepeatButton",
        controls_extended::mount_repeat_button,
    ),
    (
        "Reconciler_Mount_RichEditBox",
        controls_extended::mount_rich_edit_box,
    ),
    (
        "Reconciler_Mount_ScrollView2",
        controls_extended::mount_scroll_view,
    ),
    (
        "Reconciler_Mount_SelectorBar",
        controls_extended::mount_selector_bar,
    ),
    (
        "Reconciler_Mount_SplitButton",
        controls_extended::mount_split_button,
    ),
    (
        "Reconciler_Mount_SplitView",
        controls_extended::mount_split_view,
    ),
    (
        "Reconciler_Mount_TeachingTip",
        controls_extended::mount_teaching_tip,
    ),
    (
        "Reconciler_Mount_TimePicker",
        controls_extended::mount_time_picker,
    ),
    (
        "Reconciler_Mount_ToggleButton",
        controls_extended::mount_toggle_button,
    ),
    (
        "Reconciler_Mount_TreeView",
        controls_extended::mount_tree_view,
    ),
    // ── Hook fixtures ──────────────────────────────────────────────────
    ("Hook_UseReducer_Increment", hooks::use_reducer_increment),
    ("Hook_UseReducerFn_Actions", hooks::use_reducer_fn_actions),
    ("Hook_UseMemo_Caches", hooks::use_memo_caches),
    ("Hook_UseRef_Persists", hooks::use_ref_persists),
    ("Hook_UseEffect_Fires", hooks::use_effect_fires),
    ("Hook_UseCallback_Stable", hooks::use_callback_stable),
    (
        "Hook_UseContext_ReadsDefault",
        hooks::use_context_reads_default,
    ),
    (
        "Hook_UseContext_ReadsProvided",
        hooks::use_context_reads_provided,
    ),
    // ── Prop update fixtures (control diff without recreate) ────────────
    ("PropUpdate_TextBlock", prop_updates::text_block_update),
    ("PropUpdate_SliderValue", prop_updates::slider_value_update),
    (
        "PropUpdate_ProgressBarValue",
        prop_updates::progress_bar_value_update,
    ),
    ("PropUpdate_SliderRange", prop_updates::slider_range_update),
    (
        "PropUpdate_ButtonEnabled",
        prop_updates::button_enabled_update,
    ),
    // ── Event detachment fixtures ──────────────────────────────────────
    (
        "EventDetach_OnClick_Unmount",
        event_detachment::on_click_detach_on_unmount,
    ),
    (
        "EventDetach_OnChanged_Rerender",
        event_detachment::on_changed_detach_on_rerender,
    ),
    (
        "EventDetach_HandlerReplacement",
        event_detachment::on_changed_handler_replacement,
    ),
    // ── Reconciler stress fixtures ─────────────────────────────────────
    (
        "KeyedStress_MiddleRemoval",
        reconciler_stress::keyed_middle_removal,
    ),
    (
        "KeyedStress_MiddleInsert",
        reconciler_stress::keyed_middle_insert,
    ),
    (
        "KeyedStress_ComplexReorder",
        reconciler_stress::keyed_complex_reorder,
    ),
    (
        "KeyedStress_InsertRemoveReorder",
        reconciler_stress::keyed_insert_remove_reorder,
    ),
    ("KeyedStress_LargeList", reconciler_stress::keyed_large_list),
    (
        "KeyedStress_TypeMismatch",
        reconciler_stress::positional_type_mismatch,
    ),
];
