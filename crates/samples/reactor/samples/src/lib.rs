use windows_reactor::{Application, Element, RenderCx, Window, component, run_reactor_winui_app};

extern crate self as reactor_samples;

macro_rules! gallery_example {
    ($name:ident, $path:literal) => {
        #[allow(dead_code, unused_attributes)]
        #[path = $path]
        pub mod $name;
    };
}

gallery_example!(auto_suggest_box, "../examples/auto_suggest_box.rs");
gallery_example!(border, "../examples/border.rs");
gallery_example!(breadcrumb_bar, "../examples/breadcrumb_bar.rs");
gallery_example!(button, "../examples/button.rs");
gallery_example!(calendar_date_picker, "../examples/calendar_date_picker.rs");
gallery_example!(calendar_view, "../examples/calendar_view.rs");
gallery_example!(canvas, "../examples/canvas.rs");
gallery_example!(check_box, "../examples/check_box.rs");
gallery_example!(color_picker, "../examples/color_picker.rs");
gallery_example!(combo_box, "../examples/combo_box.rs");
gallery_example!(command_bar, "../examples/command_bar.rs");
gallery_example!(command_bar_flyout, "../examples/command_bar_flyout.rs");
gallery_example!(composition, "../examples/composition.rs");
gallery_example!(content_dialog, "../examples/content_dialog.rs");
gallery_example!(date_picker, "../examples/date_picker.rs");
gallery_example!(direct2d_host, "../examples/direct2d_host.rs");
gallery_example!(drop_down_button, "../examples/drop_down_button.rs");
gallery_example!(expander, "../examples/expander.rs");
gallery_example!(flip_view, "../examples/flip_view.rs");
gallery_example!(flyout, "../examples/flyout.rs");
gallery_example!(grid, "../examples/grid.rs");
gallery_example!(grid_view, "../examples/grid_view.rs");
gallery_example!(hyperlink_button, "../examples/hyperlink_button.rs");
gallery_example!(image, "../examples/image.rs");
gallery_example!(info_badge, "../examples/info_badge.rs");
gallery_example!(info_bar, "../examples/info_bar.rs");
gallery_example!(list_box, "../examples/list_box.rs");
gallery_example!(list_view, "../examples/list_view.rs");
gallery_example!(menu_bar, "../examples/menu_bar.rs");
gallery_example!(menu_flyout, "../examples/menu_flyout.rs");
gallery_example!(navigation_view, "../examples/navigation_view.rs");
gallery_example!(number_box, "../examples/number_box.rs");
gallery_example!(password_box, "../examples/password_box.rs");
gallery_example!(person_picture, "../examples/person_picture.rs");
gallery_example!(pivot, "../examples/pivot.rs");
gallery_example!(progress_bar, "../examples/progress_bar.rs");
gallery_example!(progress_ring, "../examples/progress_ring.rs");
gallery_example!(radio_button, "../examples/radio_button.rs");
gallery_example!(rating_control, "../examples/rating_control.rs");
gallery_example!(relative_panel, "../examples/relative_panel.rs");
gallery_example!(repeat_button, "../examples/repeat_button.rs");
gallery_example!(rich_edit_box, "../examples/rich_edit_box.rs");
gallery_example!(rich_text, "../examples/rich_text.rs");
gallery_example!(scroll_view, "../examples/scroll_view.rs");
gallery_example!(selector_bar, "../examples/selector_bar.rs");
gallery_example!(slider, "../examples/slider.rs");
gallery_example!(split_button, "../examples/split_button.rs");
gallery_example!(split_view, "../examples/split_view.rs");
gallery_example!(stack, "../examples/stack.rs");
gallery_example!(tab_view, "../examples/tab_view.rs");
gallery_example!(teaching_tip, "../examples/teaching_tip.rs");
gallery_example!(text_box, "../examples/text_box.rs");
gallery_example!(time_picker, "../examples/time_picker.rs");
gallery_example!(title_bar, "../examples/title_bar.rs");
gallery_example!(toggle_button, "../examples/toggle_button.rs");
gallery_example!(toggle_switch, "../examples/toggle_switch.rs");
gallery_example!(tooltip, "../examples/tooltip.rs");
gallery_example!(tree_view, "../examples/tree_view.rs");
gallery_example!(type_ramp, "../examples/type_ramp.rs");
gallery_example!(viewbox, "../examples/viewbox.rs");
gallery_example!(webview_host, "../examples/webview_host.rs");

pub fn run(
    title: impl Into<String>,
    render: for<'a> fn(&mut RenderCx<'a>) -> Element,
) -> windows_core::Result<()> {
    run_with_window(title, |window| window, render)
}

pub fn run_with_window(
    title: impl Into<String>,
    configure: impl Fn(Window) -> Window + 'static,
    render: for<'a> fn(&mut RenderCx<'a>) -> Element,
) -> windows_core::Result<()> {
    windows_reactor::bootstrap()?;
    let title = title.into();
    let root = component(move |cx| {
        let open = cx.use_state(|| true);
        let windows = if open.value() {
            let content = component(render);
            vec![
                configure(Window::new(title.clone(), content, move || {
                    open.set(false);
                }))
                .build()
                .key(0),
            ]
        } else {
            Vec::new()
        };
        Application::new(windows).build()
    });

    run_reactor_winui_app(root)
}

pub fn run_application(
    render: for<'a> fn(&mut RenderCx<'a>) -> Element,
) -> windows_core::Result<()> {
    windows_reactor::bootstrap()?;
    run_reactor_winui_app(component(render))
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::rc::Rc;

    use windows_reactor::{AutomationHeadingLevel, Button, RadioButton, TextBlock, hstack, vstack};

    fn examples_dir() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("examples")
    }

    fn example_sources() -> Vec<(String, String)> {
        let mut sources = fs::read_dir(examples_dir())
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| path.extension().is_some_and(|extension| extension == "rs"))
            .map(|path| {
                let name = path.file_name().unwrap().to_string_lossy().into_owned();
                let source = fs::read_to_string(path).unwrap();
                (name, source)
            })
            .collect::<Vec<_>>();
        sources.sort_by(|left, right| left.0.cmp(&right.0));
        sources
    }

    #[test]
    fn representative_sample_chains_construct() {
        let selected = true;
        let save_count = Rc::new(Cell::new(0));
        let record_save = Rc::clone(&save_count);
        let radio_value = Rc::new(Cell::new(selected));
        let update_radio = Rc::clone(&radio_value);
        let tree = vstack(
            12.0,
            [
                TextBlock::new("Heading")
                    .automation_id("heading")
                    .heading_level(AutomationHeadingLevel::Level1)
                    .build(),
                hstack(
                    8.0,
                    [
                        Button::new("Save")
                            .on_click(move || record_save.set(record_save.get() + 1))
                            .build(),
                        RadioButton::new("Choice", selected, move |value| {
                            update_radio.set(value);
                        })
                        .group_name("choices")
                        .build(),
                    ],
                ),
            ],
        );
        drop(tree);
        assert_eq!(save_count.get(), 0);
        assert!(radio_value.get());
    }

    #[test]
    fn examples_use_the_shared_host_and_current_state_api() {
        let sources = example_sources();
        assert!(sources.len() >= 30, "sample inventory unexpectedly shrank");
        for (name, source) in sources {
            let compact = source.split_whitespace().collect::<String>();
            assert!(
                source.contains("reactor_samples::run(")
                    || source.contains("reactor_samples::run_with_window(")
                    || source.contains("reactor_samples::run_application("),
                "{name} bypasses the shared sample host"
            );
            if source.contains(".get().unwrap()") {
                assert!(
                    source.contains("cx.use_ref("),
                    "{name} teaches the retired state-read pattern"
                );
            }
            assert!(
                !compact.contains(".on_click(||{})"),
                "{name} contains a placeholder action callback"
            );
            assert!(
                !compact.contains("|_|{}"),
                "{name} contains an ignored controlled-value callback"
            );
        }
    }

    #[test]
    fn resource_sample_observes_cancellation() {
        let source = fs::read_to_string(examples_dir().join("use_resource.rs")).unwrap();
        assert!(source.contains("cancel.is_cancelled()"));
        assert!(!source.contains("|_cancel"));
    }

    #[test]
    fn counter_retains_accessibility_targets() {
        let source = fs::read_to_string(examples_dir().join("counter.rs")).unwrap();
        for value in [
            "count-label",
            "decrement-button",
            "increment-button",
            "reset-button",
            "AutomationHeadingLevel::Level1",
        ] {
            assert!(source.contains(value), "counter is missing {value}");
        }
    }

    #[test]
    fn framework_input_samples_keep_typed_contracts() {
        let resize = fs::read_to_string(examples_dir().join("pointer_resize.rs")).unwrap();
        assert!(resize.contains(".capture_pointer_on_press()"));
        assert!(resize.contains("event.window_x"));

        let drop = fs::read_to_string(examples_dir().join("drag_drop.rs")).unwrap();
        assert!(drop.contains("DropTarget::new("));
        assert!(drop.contains("DropFormats::TEXT | DropFormats::STORAGE_ITEMS"));
        assert!(drop.contains("windows_core::Result"));
    }

    #[test]
    fn info_bar_sample_keeps_controlled_close_contract() {
        let source = fs::read_to_string(examples_dir().join("info_bar.rs")).unwrap();
        for value in [
            ".open(current_open)",
            ".on_close_requested(",
            "close.set(false)",
            "show.set(true)",
        ] {
            assert!(source.contains(value), "InfoBar sample is missing {value}");
        }
    }

    #[test]
    fn person_picture_sample_keeps_typed_display_contract() {
        let source = fs::read_to_string(examples_dir().join("person_picture.rs")).unwrap();
        assert!(source.contains(".display_name(\"Ada Lovelace\")"));
        assert!(source.contains(".initials(\"WR\")"));
    }

    #[test]
    fn hook_samples_keep_current_lifecycle_contracts() {
        let callback = fs::read_to_string(examples_dir().join("use_callback.rs")).unwrap();
        assert!(callback.contains("cx.use_callback((),"));

        let memo = fs::read_to_string(examples_dir().join("use_memo.rs")).unwrap();
        assert!(memo.contains("cx.use_memo(current_number,"));

        let reducer = fs::read_to_string(examples_dir().join("use_reducer.rs")).unwrap();
        assert!(reducer.contains("cx.use_reducer(CounterState::default, reducer)"));

        let resource = fs::read_to_string(examples_dir().join("use_resource_retry.rs")).unwrap();
        assert!(resource.contains("cancel.is_cancelled()"));
        assert!(resource.contains("Resource::Failed(error)"));
    }

    #[test]
    fn presentation_samples_keep_typed_style_values() {
        let card = fs::read_to_string(examples_dir().join("card.rs")).unwrap();
        for value in [
            ".border_brush(",
            ".border_thickness(",
            ".corner_radius(",
            "CornerRadius::uniform(",
        ] {
            assert!(card.contains(value), "card is missing {value}");
        }

        let trimming = fs::read_to_string(examples_dir().join("text_trimming.rs")).unwrap();
        assert!(trimming.contains("TextTrimming::CharacterEllipsis"));
        assert!(trimming.contains("TextTrimming::WordEllipsis"));
    }

    #[test]
    fn collection_samples_keep_virtual_identity_contracts() {
        let list = fs::read_to_string(examples_dir().join("virtual_list.rs")).unwrap();
        assert!(list.contains("VirtualList::new(5_000,"));

        let reorder = fs::read_to_string(examples_dir().join("keyed_list_reorder.rs")).unwrap();
        for value in [
            "component",
            "VirtualItemKeys::new(",
            ".item_keys(current)",
            "keys.update(",
        ] {
            assert!(reorder.contains(value), "keyed reorder is missing {value}");
        }

        let list_view = fs::read_to_string(examples_dir().join("list_view.rs")).unwrap();
        for value in [
            "SelectionMode::None",
            "SelectionMode::Extended",
            ".reorderable(",
            "CollectionSelection",
        ] {
            assert!(
                list_view.contains(value),
                "ListView sample is missing {value}"
            );
        }

        let grid = fs::read_to_string(examples_dir().join("grid_view.rs")).unwrap();
        assert!(grid.contains("VirtualGrid::new("));
        assert!(grid.contains(".selection(current,"));
    }

    #[test]
    fn status_samples_keep_typed_display_contracts() {
        let badge = fs::read_to_string(examples_dir().join("info_badge.rs")).unwrap();
        assert!(badge.contains("InfoBadge::dot()"));
        assert!(badge.contains("InfoBadge::numeric(42)"));
        assert!(!badge.contains("InfoBadge::numeric(-"));
    }

    #[test]
    fn final_application_samples_keep_current_composition_contracts() {
        let calculator = fs::read_to_string(examples_dir().join("calculator.rs")).unwrap();
        assert!(calculator.contains("cx.use_reducer(Calculator::default, reducer)"));
        assert!(calculator.contains("ButtonEmphasis::Accent"));
        assert!(calculator.contains("AutomationHeadingLevel::Level1"));
        assert!(calculator.contains(".backdrop(WindowBackdrop::Mica)"));
        assert!(calculator.contains(".client_size(350.0, 500.0)"));
        assert!(calculator.contains("min_width: Some(350.0)"));
        assert!(calculator.contains("VirtualKey::NUMBER_PAD_0"));
        assert!(!calculator.contains(".width(360.0)"));
        assert!(!calculator.contains(".height(520.0)"));

        let composition = fs::read_to_string(examples_dir().join("composition.rs")).unwrap();
        assert!(composition.contains("cx.use_composition_host_ref::<SpriteVisual>()"));
        assert!(composition.contains("CompositionHost::new("));
        assert!(composition.contains("CompositionContent::new("));
        assert!(composition.contains("toggle.update("));

        let memo = fs::read_to_string(examples_dir().join("memo_widget_descendant.rs")).unwrap();
        assert!(memo.contains("memo_component"));
        assert!(memo.contains("component"));
    }
}
