use super::*;

fn text_edges(runtime: &Runtime<WinUiAdapter>, object: ObjectId) -> (f32, f32) {
    let first = runtime
        .adapter()
        .text_box_character_rect(object, 0, false)
        .unwrap();
    let last = runtime
        .adapter()
        .text_box_character_rect(object, 9, true)
        .unwrap();
    assert!(first[3] > 0.0 && last[0] > first[0], "{first:?}, {last:?}");
    (first[0], last[0])
}

#[test]
#[ignore = "requires an interactive WinUI desktop"]
fn read_only_text_box_preserves_selection_and_resets_properties() {
    // WinUI supports one Application lifetime per process.
    if std::env::var_os("REACTOR_TEXT_BOX_TEST_CHILD").is_none() {
        let status = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "native::winui::tests::text_box::read_only_text_box_preserves_selection_and_resets_properties",
                "--ignored",
                "--nocapture",
            ])
            .env("REACTOR_TEXT_BOX_TEST_CHILD", "1")
            .status()
            .unwrap();
        assert!(status.success(), "{status}");
        return;
    }
    let completed = Rc::new(Cell::new(false));
    let finished = Rc::clone(&completed);
    App::run_with(move |app| {
        let calls = Rc::new(Cell::new(0));
        let changed = Rc::clone(&calls);
        let callback = Callback::new(move |_: Rc<str>| changed.set(changed.get() + 1));
        let text_box = move |text| TextBox::new(text).on_text_changed(callback.clone());
        let view = |control: TextBox, width| Border::new().width(width).content(control);
        let mut runtime = Runtime::new(WinUiAdapter::default());
        runtime
            .update(view(
                text_box("0123456789")
                    .is_read_only(true)
                    .text_alignment(TextAlignment::Center),
                400.0,
            ))
            .unwrap();
        let root = runtime.graph().root().unwrap();
        let object = runtime.graph().child(root, RelationId::Content).unwrap();
        let window = runtime
            .adapter_mut()
            .create_window_with_policy(root, &WindowPolicy::new().client_size(800.0, 400.0))
            .unwrap();
        window.activate().unwrap();
        let window = RefCell::new(Some(window));
        let runtime = RefCell::new(Some(runtime));
        let timer = native::DispatcherQueue::GetForCurrentThread()?.CreateTimer()?;
        timer.SetInterval(TimeSpan::try_from(Duration::from_millis(100)).unwrap())?;
        let app = app.clone();
        let tick = timer.Tick(move |sender, _| {
            let Some(mut runtime) = runtime.borrow_mut().take() else {
                sender.ok().unwrap().Stop().unwrap();
                window.borrow_mut().take().unwrap().close().unwrap();
                finished.set(true);
                app.exit().unwrap();
                return;
            };
            {
                let Some(Handle::TextBox(handle)) = runtime.adapter().handles.get(&object) else {
                    panic!()
                };
                let value = handle.value.clone();
                runtime.dispatch_native_events().unwrap();
                assert!(value.IsReadOnly().unwrap());
                assert_eq!(
                    value.TextAlignment().unwrap(),
                    native::TextAlignment::Center
                );
                let control = value.cast::<native::IControl>().unwrap();
                assert!(control.IsEnabled().unwrap());
                let element = value.cast::<native::UIElement>().unwrap();
                assert!(element.Focus(native::FocusState::Programmatic).unwrap());
                value.SetSelectionStart(2).unwrap();
                value.SetSelectionLength(4).unwrap();
                assert_eq!(
                    runtime.adapter().text_box_editing_state(object).unwrap(),
                    (true, TextAlignment::Center, "2345".to_string())
                );
                element.UpdateLayout().unwrap();
                let framework = value.cast::<native::IFrameworkElement>().unwrap();
                assert_eq!(framework.ActualWidth().unwrap(), 400.0);
                let initial = text_edges(&runtime, object);

                runtime
                    .update(view(
                        text_box("0123456789")
                            .is_read_only(true)
                            .text_alignment(TextAlignment::Center),
                        600.0,
                    ))
                    .unwrap();
                element.UpdateLayout().unwrap();
                assert_eq!(framework.ActualWidth().unwrap(), 600.0);
                assert_eq!(
                    value.TextAlignment().unwrap(),
                    native::TextAlignment::Center
                );
                let resized = text_edges(&runtime, object);
                // Character rectangles exclude control padding. Centered text moves by half
                // the width increase, independently of that padding.
                for movement in [resized.0 - initial.0, resized.1 - initial.1] {
                    assert!(
                        (movement - 100.0).abs() <= 2.0,
                        "{initial:?} -> {resized:?}"
                    );
                }

                runtime
                    .update(view(
                        text_box("0123456789")
                            .is_read_only(false)
                            .text_alignment(TextAlignment::Right),
                        600.0,
                    ))
                    .unwrap();
                assert!(!value.IsReadOnly().unwrap());
                assert_eq!(value.SelectedText().unwrap(), "2345");

                runtime
                    .update(view(
                        text_box("Updated from state")
                            .is_read_only(true)
                            .text_alignment(TextAlignment::Right),
                        600.0,
                    ))
                    .unwrap();
                assert!(value.IsReadOnly().unwrap());
                assert_eq!(value.Text().unwrap(), "Updated from state");
                element.UpdateLayout().unwrap();
                let first = runtime
                    .adapter()
                    .text_box_character_rect(object, 0, false)
                    .unwrap();
                assert!(first[0] > resized.1, "{first:?}");
                runtime.dispatch_native_events().unwrap();
                assert_eq!(calls.get(), 0);

                for (alignment, expected) in [
                    (TextAlignment::Left, native::TextAlignment::Left),
                    (TextAlignment::Start, native::TextAlignment::Start),
                    (TextAlignment::Center, native::TextAlignment::Center),
                    (TextAlignment::Right, native::TextAlignment::Right),
                    (TextAlignment::End, native::TextAlignment::End),
                    (TextAlignment::Justify, native::TextAlignment::Justify),
                    (
                        TextAlignment::DetectFromContent,
                        native::TextAlignment::DetectFromContent,
                    ),
                ] {
                    runtime
                        .update(view(
                            text_box("Updated from state")
                                .is_read_only(true)
                                .text_alignment(alignment),
                            600.0,
                        ))
                        .unwrap();
                    assert_eq!(value.TextAlignment().unwrap(), expected);
                }
                runtime
                    .update(view(
                        text_box("Updated from state").text_alignment(TextAlignment::Right),
                        600.0,
                    ))
                    .unwrap();
                assert!(!value.IsReadOnly().unwrap());
                assert_eq!(value.TextAlignment().unwrap(), native::TextAlignment::Right);

                // Native clearing can differ from a freshly constructed control's values.
                let dependency = value.cast::<native::IDependencyObject>().unwrap();
                dependency
                    .ClearValue(&native::TextBox::IsReadOnlyProperty().unwrap())
                    .unwrap();
                dependency
                    .ClearValue(&native::TextBox::TextAlignmentProperty().unwrap())
                    .unwrap();
                let cleared_read_only = value.IsReadOnly().unwrap();
                let cleared_alignment = value.TextAlignment().unwrap();
                assert!(!cleared_read_only);
                assert_ne!(cleared_alignment, native::TextAlignment::Right);
                value.SetIsReadOnly(true).unwrap();
                value
                    .SetTextAlignment(native::TextAlignment::Right)
                    .unwrap();
                runtime
                    .update(view(
                        text_box("Updated from state").is_read_only(true),
                        600.0,
                    ))
                    .unwrap();
                assert!(value.IsReadOnly().unwrap());
                assert_eq!(value.TextAlignment().unwrap(), cleared_alignment);
                let omitted = view(text_box("Updated from state"), 600.0);
                runtime.update(omitted.clone()).unwrap();
                assert_eq!(value.IsReadOnly().unwrap(), cleared_read_only);
                assert_eq!(value.TextAlignment().unwrap(), cleared_alignment);
                assert_eq!(
                    runtime.graph().child(root, RelationId::Content),
                    Some(object)
                );
                assert!(runtime.update(omitted).unwrap().is_empty());
                assert_eq!(value.Text().unwrap(), "Updated from state");
                runtime.dispatch_native_events().unwrap();
                assert_eq!(calls.get(), 0);
            }
            // Unload the focused text control before destroying the window's input site.
            runtime.update(Border::new()).unwrap();
        })?;
        timer.Start()?;
        Ok((timer, tick))
    })
    .unwrap();
    assert!(completed.get());
}
