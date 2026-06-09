use windows_reactor::core::element::BreadcrumbBar;
use windows_reactor::core::element::{
    Canvas, ComboBox, Expander, HyperlinkButton, Image, InfoBadge, InfoBar, NavViewItem,
    NavigationView, NumberBox, PasswordBox, PasswordRevealMode, PersonPicture, Pivot, PivotItem,
    ProgressBar, ProgressRing, RadioButton, RadioButtons, Shape, Slider, TabItem, TabView,
    TitleBar, ToggleSwitch, Viewbox,
};
use windows_reactor::core::element::{Color, GridLength};
use windows_reactor::core::rich_text::{RichText, RichTextInline, RichTextRun};
use windows_reactor::core::templated_list::{flip_view, grid_view, list_view, virtual_list};
use windows_reactor::dsl::{
    ElementExt, border, button, check_box, scroll_viewer, swap_chain_panel, text_block, text_box,
};

use crate::bindings;

use crate::fixtures::reconciler::{FixtureFuture, cc};
use crate::harness::Harness;
use windows_reactor::{grid, vstack};

macro_rules! assert_present {
    ($h:expr, $name:expr, $ty:ty) => {{
        let n = $h.count_controls::<$ty>();
        $h.check($name, n >= 1);
    }};
}

pub fn mount_stack(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| vstack((text_block("a"), text_block("b"))).into()));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_Stack", bindings::StackPanel);
    })
}

pub fn mount_border(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| border(text_block("inside")).into()));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_Border", bindings::Border);
        h.check(
            "Reconciler_Mount_Border_Child",
            h.find_text("inside").is_some(),
        );
    })
}

pub fn mount_grid(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| {
            grid((text_block("cell"),))
                .rows([GridLength::Star(1.0)])
                .columns([GridLength::Star(1.0)])
                .into()
        }));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_Grid", bindings::Grid);
    })
}

pub fn mount_scroll_view(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| scroll_viewer(text_block("scrollable")).into()));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_ScrollView", bindings::ScrollViewer);
    })
}

pub fn mount_viewbox(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| Viewbox::new(text_block("scaled")).into()));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_Viewbox", bindings::Viewbox);
        h.check(
            "Reconciler_Mount_Viewbox_Child",
            h.find_text("scaled").is_some(),
        );
    })
}

pub fn mount_check_box(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| check_box(true).into()));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_CheckBox", bindings::CheckBox);
    })
}

pub fn mount_text_field(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| text_box("typed").into()));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_TextField", bindings::TextBox);
    })
}

pub fn mount_toggle_switch(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| ToggleSwitch::new(true).into()));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_ToggleSwitch", bindings::ToggleSwitch);
    })
}

pub fn mount_slider(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| Slider::new(50.0).range(0.0, 100.0).into()));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_Slider", bindings::Slider);
    })
}

pub fn mount_radio_button(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| RadioButton::new("Option A").into()));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_RadioButton", bindings::RadioButton);
    })
}

pub fn mount_number_box(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| NumberBox::new(42.0).into()));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_NumberBox", bindings::NumberBox);
    })
}

pub fn mount_progress_bar(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| ProgressBar::new(60.0).into()));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_ProgressBar", bindings::ProgressBar);
    })
}

pub fn mount_progress_ring(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| ProgressRing::new(75.0).into()));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_ProgressRing", bindings::ProgressRing);
    })
}

pub fn mount_expander(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| {
            Expander::new(text_block("body")).header("Header").into()
        }));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_Expander", bindings::Expander);
    })
}

pub fn mount_hyperlink_button(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| HyperlinkButton::new("Hyperlink").into()));
        h.render().await;
        assert_present!(
            h,
            "Reconciler_Mount_HyperlinkButton",
            bindings::HyperlinkButton
        );
    })
}

pub fn mount_info_bar(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| InfoBar::new("Note").message("body").into()));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_InfoBar", bindings::InfoBar);
    })
}

pub fn mount_info_badge(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| InfoBadge::numeric(7).into()));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_InfoBadge", bindings::InfoBadge);
    })
}

pub fn mount_image(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| {
            Image::new_with_uri("ms-appx:///Assets/none.png").into()
        }));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_Image", bindings::Image);
    })
}

pub fn mount_tab_view(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| {
            TabView::new([
                TabItem::new("Home", text_block("home-content")),
                TabItem::new("Settings", text_block("settings-content")),
            ])
            .selected_index(0)
            .into()
        }));
        h.render().await;
        // TabView's items (TabViewItem) and selected content live inside a
        // templated ListView/ContentPresenter and are realized lazily, so
        // VisualTreeHelper-based checks for them are unreliable on the first
        // frame. The presence of the TabView control itself is sufficient to
        // confirm mount+attach succeeded.
        assert_present!(h, "Reconciler_Mount_TabView", bindings::TabView);
    })
}

pub fn mount_tab_view_add_button(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| {
            TabView::new([
                TabItem::new("Home", text_block("home-content")),
                TabItem::new("Settings", text_block("settings-content")),
            ])
            .selected_index(0)
            .is_add_tab_button_visible(true)
            .on_add_tab_button_click(|()| {})
            .into()
        }));
        h.render().await;
        // Confirms that the TabView mounts successfully with the add-tab
        // button visible and the Click event handler attached.
        assert_present!(h, "Reconciler_Mount_TabView_AddButton", bindings::TabView);
    })
}

pub fn mount_navigation_view(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| {
            NavigationView::new(
                [NavViewItem::new("Home"), NavViewItem::new("Settings")],
                text_block("page body"),
            )
            .into()
        }));
        h.render().await;
        assert_present!(
            h,
            "Reconciler_Mount_NavigationView",
            bindings::NavigationView
        );
    })
}

pub fn mount_pivot(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| {
            Pivot::new([
                PivotItem::new("A", text_block("page A")),
                PivotItem::new("B", text_block("page B")),
            ])
            .into()
        }));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_Pivot", bindings::Pivot);
        h.check(
            "Reconciler_Mount_Pivot_HasPivotItems",
            h.count_controls::<bindings::PivotItem>() >= 2,
        );
    })
}

pub fn mount_breadcrumb_bar(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| BreadcrumbBar::new(["root", "child"]).into()));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_BreadcrumbBar", bindings::BreadcrumbBar);
    })
}

pub fn mount_rectangle(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| {
            Shape::rectangle()
                .fill(Color::rgb(255, 0, 0))
                .width(40.0)
                .height(20.0)
                .into()
        }));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_Rectangle", bindings::Rectangle);
    })
}

pub fn mount_ellipse(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| {
            Shape::ellipse()
                .fill(Color::rgb(0, 200, 0))
                .width(40.0)
                .height(40.0)
                .into()
        }));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_Ellipse", bindings::Ellipse);
    })
}

pub fn mount_line(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| {
            Shape::line(0.0, 0.0, 100.0, 0.0)
                .stroke(Color::rgb(0, 0, 200))
                .stroke_thickness(2.0)
                .into()
        }));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_Line", bindings::Line);
    })
}

pub fn mount_rich_text(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| {
            RichText::single_paragraph(vec![RichTextInline::Run(RichTextRun::plain(
                "rich content",
            ))])
            .into()
        }));
        h.render().await;

        h.check(
            "Reconciler_Mount_RichText_TextBlockPresent",
            h.count_controls::<bindings::RichTextBlock>() >= 1,
        );
        // Verify the RichTextBlock has paragraph content (Blocks collection non-empty).
        let has_content = !h
            .find_all::<bindings::RichTextBlock>(&|rtb| {
                rtb.get_Blocks().is_ok_and(|b| b.Size().unwrap_or(0) > 0)
            })
            .is_empty();
        h.check("Reconciler_Mount_RichText_HasContent", has_content);
    })
}

pub fn mount_templated_list_view(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| {
            list_view(vec![1i32, 2, 3], |n, _| text_block(format!("#{n}")))
                .height(120.0)
                .build()
        }));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_TemplatedListView", bindings::ListView);
    })
}

pub fn mount_templated_grid_view(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| {
            grid_view(vec![1i32, 2, 3], |n, _| text_block(format!("#{n}")))
                .height(120.0)
                .build()
        }));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_TemplatedGridView", bindings::GridView);
    })
}

pub fn mount_templated_flip_view(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| {
            flip_view(vec![1i32, 2, 3], |n, _| text_block(format!("page #{n}")))
                .width(160.0)
                .height(120.0)
                .build()
        }));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_TemplatedFlipView", bindings::FlipView);
    })
}

// ── W1: virtualised lists (named alias) ────────────────────────────────

pub fn mount_virtual_list_alias(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| {
            virtual_list(vec![1i32, 2, 3, 4, 5], |n, _| {
                text_block(format!("row #{n}"))
            })
            .height(120.0)
            .build()
        }));
        h.render().await;
        // `virtual_list` is an alias for `list_view` — the WinUI control
        // backing it is still ListView. Item containers are realized
        // lazily via ContainerContentChanging (same as mount_tab_view
        // above), so we don't assert on row TextBlocks here — the
        // presence of the ListView is sufficient to confirm mount.
        assert_present!(h, "Reconciler_Mount_VirtualList", bindings::ListView);
    })
}

// ── W2: PasswordBox ────────────────────────────────────────────────────

pub fn mount_password_box(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| {
            PasswordBox::new()
                .header("Password")
                .placeholder_text("type something")
                .password_reveal_mode(PasswordRevealMode::Peek)
                .into()
        }));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_PasswordBox", bindings::PasswordBox);
    })
}

// ── W3: RadioButtons grouped container ─────────────────────────────────

pub fn mount_radio_buttons(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| {
            RadioButtons::new(["Email", "SMS", "None"])
                .header("Notifications")
                .selected_index(0)
                .into()
        }));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_RadioButtons", bindings::RadioButtons);
    })
}

// ── W4: ComboBox ───────────────────────────────────────────────────────

pub fn mount_combo_box(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| {
            ComboBox::new(["Red", "Green", "Blue"])
                .header("Color")
                .placeholder_text("pick a color")
                .into()
        }));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_ComboBox", bindings::ComboBox);
    })
}

// ── W5: Canvas + canvas_left / canvas_top attached props ───────────────

pub fn mount_canvas(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| {
            Canvas::new([
                text_block("top-left"),
                text_block("offset").canvas_left(40.0).canvas_top(80.0),
            ])
            .into()
        }));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_Canvas", bindings::Canvas);
        h.check(
            "Reconciler_Mount_Canvas_HasChildren",
            h.find_text("top-left").is_some() && h.find_text("offset").is_some(),
        );

        // Verify the attached Canvas.Left/Top reached the WinUI runtime.
        use windows_core::Interface;
        if let Some(tb) = h.find_text("offset") {
            let ui: bindings::UIElement = tb.cast().unwrap();
            let left = bindings::Canvas::GetLeft(&ui).unwrap_or(0.0);
            let top = bindings::Canvas::GetTop(&ui).unwrap_or(0.0);
            h.check(
                "Reconciler_Mount_Canvas_AttachedLeft",
                (left - 40.0).abs() < 0.5,
            );
            h.check(
                "Reconciler_Mount_Canvas_AttachedTop",
                (top - 80.0).abs() < 0.5,
            );
        } else {
            h.check("Reconciler_Mount_Canvas_AttachedLeft", false);
            h.check("Reconciler_Mount_Canvas_AttachedTop", false);
        }
    })
}

pub fn mount_button(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| button("Press").into()));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_Button", bindings::Button);
        h.check(
            "Reconciler_Mount_Button_HasLabel",
            h.find_button("Press").is_some(),
        );
    })
}

pub fn mount_button_accent(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| button("Accent").accent().into()));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_ButtonAccent", bindings::Button);
        h.check(
            "Reconciler_Mount_ButtonAccent_HasLabel",
            h.find_button("Accent").is_some(),
        );
    })
}

pub fn mount_button_subtle(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| button("Subtle").subtle().into()));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_ButtonSubtle", bindings::Button);
        h.check(
            "Reconciler_Mount_ButtonSubtle_HasLabel",
            h.find_button("Subtle").is_some(),
        );
    })
}

pub fn mount_button_text_link(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| button("Link").text_link().into()));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_ButtonTextLink", bindings::Button);
        h.check(
            "Reconciler_Mount_ButtonTextLink_HasLabel",
            h.find_button("Link").is_some(),
        );
    })
}

pub fn mount_person_picture(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| PersonPicture::new().initials("AB").into()));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_PersonPicture", bindings::PersonPicture);
    })
}

pub fn mount_title_bar(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| {
            TitleBar::new("MyApp")
                .subtitle("selftest")
                .back_button_visible(true)
                .into()
        }));
        h.render().await;
        assert_present!(h, "Reconciler_Mount_TitleBar", bindings::TitleBar);
    })
}

pub fn mount_swap_chain_panel(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| swap_chain_panel().width(200.0).height(100.0).into()));
        h.render().await;
        assert_present!(
            h,
            "Reconciler_Mount_SwapChainPanel",
            bindings::SwapChainPanel
        );
    })
}
