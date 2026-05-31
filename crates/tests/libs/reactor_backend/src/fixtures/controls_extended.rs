//! Mount fixtures for widgets not covered in `controls.rs`.

use windows_reactor::core::element::{
    AutoSuggestBoxWidget, CalendarDatePickerWidget, CalendarViewWidget, ColorArgb,
    ColorPickerWidget, CommandBarWidget, ContentDialog, DatePickerWidget, DropDownButtonWidget,
    ListBoxWidget, MenuBarItemDef, MenuBarWidget, RatingControl as RatingControlWidget,
    RelativePanelWidget, RepeatButton as RepeatButtonWidget, RichEditBoxWidget, ScrollViewWidget,
    SelectorBarItemDef, SelectorBarWidget, SplitButtonWidget, SplitViewWidget, TeachingTipWidget,
    TimePickerWidget, ToggleButtonWidget, TreeNodeDef, TreeViewWidget,
};
use windows_reactor::dsl::text_block;

// Free-function constructors for enum variants
use windows_reactor::core::element::{app_bar_button, menu_item};

use crate::bindings;

use crate::fixtures::reconciler::{FixtureFuture, cc};
use crate::harness::Harness;

macro_rules! assert_present {
    ($h:expr, $name:expr, $ty:ty) => {{
        let n = $h.count_controls::<$ty>();
        $h.check($name, n >= 1);
    }};
}

pub fn mount_auto_suggest_box(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| {
            AutoSuggestBoxWidget::new("").placeholder("Search").into()
        }));
        h.render().await;
        assert_present!(
            h,
            "Reconciler_Mount_AutoSuggestBox",
            bindings::AutoSuggestBox
        );
    })
}

pub fn mount_calendar_date_picker(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| {
            CalendarDatePickerWidget::new().header("Pick a date").into()
        }));
        h.render().await;
        assert_present!(
            h,
            "Reconciler_Mount_CalendarDatePicker",
            bindings::CalendarDatePicker
        );
    })
}

pub fn mount_calendar_view(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| CalendarViewWidget::new().into()));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_CalendarView", bindings::CalendarView);
    })
}

pub fn mount_color_picker(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| {
            ColorPickerWidget::new(ColorArgb::new(255, 0, 0)).into()
        }));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_ColorPicker", bindings::ColorPicker);
    })
}

pub fn mount_command_bar(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| {
            CommandBarWidget::new(vec![app_bar_button("Save"), app_bar_button("Delete")]).into()
        }));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_CommandBar", bindings::CommandBar);
    })
}

pub fn mount_content_dialog(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| {
            ContentDialog::new("Confirm")
                .content("Are you sure?")
                .primary_button_text("Yes")
                .close_button_text("No")
                .into()
        }));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_ContentDialog", bindings::ContentDialog);
    })
}

pub fn mount_date_picker(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| DatePickerWidget::new().header("Birthday").into()));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_DatePicker", bindings::DatePicker);
    })
}

pub fn mount_drop_down_button(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| DropDownButtonWidget::new("Options").into()));
        h.render().await;
        assert_present!(
            h,
            "Reconciler_Mount_DropDownButton",
            bindings::DropDownButton
        );
    })
}

pub fn mount_list_box(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| {
            ListBoxWidget::new()
                .items(["Alpha", "Beta", "Gamma"])
                .selected_index(0)
                .into()
        }));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_ListBox", bindings::ListBox);
    })
}

pub fn mount_menu_bar(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| {
            MenuBarWidget::new(vec![MenuBarItemDef::new(
                "File",
                vec![menu_item("Open"), menu_item("Save")],
            )])
            .into()
        }));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_MenuBar", bindings::MenuBar);
    })
}

pub fn mount_rating_control(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| RatingControlWidget::new(3.5).max_rating(5).into()));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_RatingControl", bindings::RatingControl);
    })
}

pub fn mount_relative_panel(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| {
            RelativePanelWidget::new([text_block("centered")]).into()
        }));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_RelativePanel", bindings::RelativePanel);
    })
}

pub fn mount_repeat_button(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| RepeatButtonWidget::new("Hold me").into()));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_RepeatButton", bindings::RepeatButton);
    })
}

pub fn mount_rich_edit_box(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| RichEditBoxWidget::new("rich text").into()));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_RichEditBox", bindings::RichEditBox);
    })
}

pub fn mount_scroll_view(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| {
            ScrollViewWidget::new(text_block("scrollable content")).into()
        }));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_ScrollView2", bindings::ScrollView);
    })
}

pub fn mount_selector_bar(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| {
            SelectorBarWidget::new(vec![
                SelectorBarItemDef::new("Recent"),
                SelectorBarItemDef::new("Shared"),
            ])
            .into()
        }));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_SelectorBar", bindings::SelectorBar);
    })
}

pub fn mount_split_button(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| SplitButtonWidget::new("Reply").into()));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_SplitButton", bindings::SplitButton);
    })
}

pub fn mount_split_view(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| {
            SplitViewWidget::new(text_block("main content"))
                .pane(text_block("pane"))
                .is_pane_open(true)
                .into()
        }));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_SplitView", bindings::SplitView);
    })
}

pub fn mount_teaching_tip(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| {
            TeachingTipWidget::new("Did you know?")
                .subtitle("A helpful tip")
                .is_open(true)
                .into()
        }));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_TeachingTip", bindings::TeachingTip);
    })
}

pub fn mount_time_picker(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| TimePickerWidget::new().header("Alarm").into()));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_TimePicker", bindings::TimePicker);
    })
}

pub fn mount_toggle_button(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| ToggleButtonWidget::new("Bold", false).into()));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_ToggleButton", bindings::ToggleButton);
    })
}

pub fn mount_tree_view(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| {
            TreeViewWidget::new(vec![
                TreeNodeDef::new("Root")
                    .child(TreeNodeDef::new("Child A"))
                    .child(TreeNodeDef::new("Child B")),
            ])
            .into()
        }));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_TreeView", bindings::TreeView);
    })
}
