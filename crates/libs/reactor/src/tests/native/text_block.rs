use super::*;

fn text_edges(runtime: &Runtime<WinUiAdapter>, object: ObjectId) -> (f32, f32) {
    let first = runtime
        .adapter()
        .text_block_edge_rect(object, false)
        .unwrap();
    let last = runtime
        .adapter()
        .text_block_edge_rect(object, true)
        .unwrap();
    assert!(first[3] > 0.0 && last[0] > first[0], "{first:?}, {last:?}");
    (first[0], last[0] + last[2])
}

fn alignment_updates(runtime: &mut Runtime<WinUiAdapter>) {
    let view = |alignment: Option<TextAlignment>, width| {
        let mut text = TextBlock::new().text("0123456789");
        if let Some(alignment) = alignment {
            text = text.text_alignment(alignment);
        }
        Border::new().width(width).content(text)
    };
    runtime.update(view(None, 400.0)).unwrap();
    let root = runtime.graph().root().unwrap();
    let object = runtime.graph().child(root, RelationId::Content).unwrap();
    let Some(Handle::Generated(GeneratedHandle::TextBlock(value))) =
        runtime.adapter().handles.get(&object)
    else {
        panic!()
    };
    let value = value.clone();
    let element = value.cast::<native::UIElement>().unwrap();
    // TextBlock.ActualWidth reports text extent, not the available layout width.
    let framework = runtime
        .adapter()
        .ui_element(root)
        .unwrap()
        .cast::<native::IFrameworkElement>()
        .unwrap();
    element.UpdateLayout().unwrap();
    let default = runtime.adapter().text_block_alignment(object).unwrap();
    let initial = text_edges(runtime, object);

    value
        .SetTextAlignment(native::TextAlignment::Right)
        .unwrap();
    assert!(runtime.update(view(None, 400.0)).unwrap().is_empty());
    assert_eq!(
        runtime.adapter().text_block_alignment(object).unwrap(),
        TextAlignment::Right
    );

    runtime
        .update(view(Some(TextAlignment::Center), 400.0))
        .unwrap();
    element.UpdateLayout().unwrap();
    assert_eq!(framework.ActualWidth().unwrap(), 400.0);
    let centered = text_edges(runtime, object);
    assert!(
        ((centered.0 + centered.1) / 2.0 - 200.0).abs() <= 2.0,
        "{centered:?}"
    );
    assert!(
        runtime
            .update(view(Some(TextAlignment::Center), 400.0))
            .unwrap()
            .is_empty()
    );

    runtime
        .update(view(Some(TextAlignment::Center), 600.0))
        .unwrap();
    element.UpdateLayout().unwrap();
    assert_eq!(framework.ActualWidth().unwrap(), 600.0);
    let resized = text_edges(runtime, object);
    for movement in [resized.0 - centered.0, resized.1 - centered.1] {
        assert!(
            (movement - 100.0).abs() <= 2.0,
            "{centered:?} -> {resized:?}"
        );
    }

    for (alignment, expected) in [
        (TextAlignment::Left, TextAlignment::Left),
        (TextAlignment::Start, TextAlignment::Left),
        (TextAlignment::Center, TextAlignment::Center),
        (TextAlignment::Right, TextAlignment::Right),
        (TextAlignment::End, TextAlignment::Right),
        (TextAlignment::Justify, TextAlignment::Justify),
        (
            TextAlignment::DetectFromContent,
            TextAlignment::DetectFromContent,
        ),
    ] {
        runtime.update(view(Some(alignment), 600.0)).unwrap();
        assert_eq!(
            runtime.adapter().text_block_alignment(object).unwrap(),
            expected
        );
        let Some(Handle::Generated(GeneratedHandle::TextBlock(retained))) =
            runtime.adapter().handles.get(&object)
        else {
            panic!()
        };
        assert_eq!(retained, &value);
    }
    runtime
        .update(view(Some(TextAlignment::Right), 600.0))
        .unwrap();
    element.UpdateLayout().unwrap();
    let right = text_edges(runtime, object);
    assert!((right.1 - 600.0).abs() <= 2.0, "{right:?}");
    assert!(right.0 > resized.0, "{resized:?} -> {right:?}");

    runtime.update(view(None, 400.0)).unwrap();
    element.UpdateLayout().unwrap();
    assert_eq!(
        runtime.adapter().text_block_alignment(object).unwrap(),
        default
    );
    let cleared = text_edges(runtime, object);
    assert!(
        (cleared.0 - initial.0).abs() <= 2.0 && (cleared.1 - initial.1).abs() <= 2.0,
        "{initial:?} -> {cleared:?}"
    );
    value
        .SetTextAlignment(native::TextAlignment::Center)
        .unwrap();
    assert!(runtime.update(view(None, 400.0)).unwrap().is_empty());
    assert_eq!(
        runtime.adapter().text_block_alignment(object).unwrap(),
        TextAlignment::Center
    );
    assert_eq!(
        runtime.graph().child(root, RelationId::Content),
        Some(object)
    );
}

#[test]
#[ignore = "requires an interactive WinUI desktop"]
fn text_block_alignment_updates_rendering_and_resets() {
    // WinUI supports one Application lifetime per process.
    if std::env::var_os("REACTOR_TEXT_BLOCK_TEST_CHILD").is_none() {
        let status = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "native::winui::tests::text_block::text_block_alignment_updates_rendering_and_resets",
                "--ignored",
                "--nocapture",
            ])
            .env("REACTOR_TEXT_BLOCK_TEST_CHILD", "1")
            .status()
            .unwrap();
        assert!(status.success(), "{status}");
        return;
    }
    let completed = Rc::new(Cell::new(false));
    let finished = Rc::clone(&completed);
    App::run_with(move |app| {
        let mut runtime = Runtime::new(WinUiAdapter::default());
        runtime.update(Border::new()).unwrap();
        let root = runtime.graph().root().unwrap();
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
            alignment_updates(&mut runtime);
            runtime.update(Border::new()).unwrap();
        })?;
        timer.Start()?;
        Ok((timer, tick))
    })
    .unwrap();
    assert!(completed.get());
}
