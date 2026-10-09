use super::*;

const PNG: &[u8] = &[
    0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48, 0x44, 0x52,
    0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x06, 0x00, 0x00, 0x00, 0x1f, 0x15, 0xc4,
    0x89, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x44, 0x41, 0x54, 0x78, 0x9c, 0x63, 0xb8, 0xa3, 0xa1, 0xf1,
    0x1f, 0x00, 0x05, 0x3c, 0x02, 0x2c, 0x0e, 0xc4, 0x2f, 0xc5, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45,
    0x4e, 0x44, 0xae, 0x42, 0x60, 0x82,
];

#[test]
fn image_load_subscriptions_complete_once_and_do_not_retain_controls() {
    let state = ImageLoadState::default();
    let values = Rc::new(RefCell::new(Vec::new()));
    let result = Rc::clone(&values);
    let callback: Rc<dyn Fn(bool)> = Rc::new(move |value| result.borrow_mut().push(value));
    let removed: Rc<dyn Fn(bool)> = Rc::new(|_| panic!("removed subscription"));
    assert_eq!(state.subscribe(&removed), None);
    drop(removed);
    assert_eq!(state.subscribe(&callback), None);
    state.complete(true);
    state.complete(false);
    assert_eq!(*values.borrow(), [true]);
    assert_eq!(state.subscribe(&callback), Some(true));
    assert_eq!(*values.borrow(), [true]);
    assert!(state.subscribers.borrow().is_empty());
}

fn page(
    generation: u32,
    sources: &[ImageSource],
    events: &Rc<RefCell<Vec<(usize, bool)>>>,
) -> View {
    StackPanel::new()
        .keyed_children([keyed(
            generation,
            StackPanel::new().children(
                sources
                    .iter()
                    .chain(sources.iter())
                    .enumerate()
                    .map(|(index, source)| {
                        let opened = Rc::clone(events);
                        let failed = Rc::clone(events);
                        Image::new()
                            .width(48.0)
                            .height(48.0)
                            .source(source.clone())
                            .unwrap()
                            .on_opened(move || opened.borrow_mut().push((index, true)))
                            .on_failed(move || failed.borrow_mut().push((index, false)))
                            .into()
                    })
                    .collect::<Vec<View>>(),
            ),
        )])
        .into()
}

fn source_property(source: &ImageSource) -> Property {
    Property {
        id: PropertyId::Source,
        value: PropertyValue::ImageSource(source.clone()),
    }
}

#[test]
#[ignore = "requires an interactive WinUI desktop"]
fn retained_images_survive_remounts_and_release_with_their_owner() {
    // WinUI supports one Application lifetime per process.
    if std::env::var_os("REACTOR_IMAGE_TEST_CHILD").is_none() {
        let status = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "native::winui::tests::images::retained_images_survive_remounts_and_release_with_their_owner",
                "--ignored",
                "--nocapture",
            ])
            .env("REACTOR_IMAGE_TEST_CHILD", "1")
            .status()
            .unwrap();
        assert!(status.success());
        return;
    }
    let directory = std::env::temp_dir().join(format!("reactor-images-{}", std::process::id()));
    std::fs::create_dir(&directory).unwrap();
    std::fs::write(directory.join("image.png"), PNG).unwrap();
    std::fs::write(
        directory.join("image.svg"),
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="48" height="48"><rect width="48" height="48" fill="red"/></svg>"#,
    )
    .unwrap();
    std::fs::write(directory.join("invalid.png"), b"invalid").unwrap();
    std::fs::write(directory.join("invalid.svg"), b"invalid").unwrap();
    let completed = Rc::new(Cell::new(false));
    let finished = Rc::clone(&completed);
    let png = directory.join("image.png");
    let svg = directory.join("image.svg");
    let invalid_png = directory.join("invalid.png");
    let invalid_svg = directory.join("invalid.svg");
    let result = App::run_with(move |app| {
        let sources = Rc::new(RefCell::new(vec![
            ImageSource::encoded(EncodedImage::from_static(PNG)),
            ImageSource::file(png)?,
            ImageSource::file(svg)?,
            ImageSource::encoded(EncodedImage::from_static(&[0, 1, 2])),
            ImageSource::file(invalid_png)?,
            ImageSource::file(invalid_svg)?,
        ]));
        let events = Rc::new(RefCell::new(Vec::new()));
        let mut runtime = Runtime::new(WinUiAdapter::default());
        runtime.update(StackPanel::new()).unwrap();
        let root = runtime.graph().root().unwrap();
        let window = runtime.adapter_mut().create_window(root).unwrap();
        window.activate().unwrap();
        let weak = RefCell::new(Vec::new());
        let mounted = Cell::new(false);
        let timer = native::DispatcherQueue::GetForCurrentThread()?.CreateTimer()?;
        timer.SetInterval(TimeSpan::try_from(Duration::from_millis(100)).unwrap())?;
        let runtime = RefCell::new(runtime);
        let window = RefCell::new(Some(window));
        let phase = Cell::new(0);
        let ticks = Cell::new(0);
        let notifications = Rc::new(RefCell::new(Vec::new()));
        let image = Cell::new(None);
        let app = app.clone();
        let tick = timer.Tick(move |_, _| {
            ticks.set(ticks.get() + 1);
            assert!(
                ticks.get() < 100,
                "image lifecycle test timed out at phase {}: {:?}",
                phase.get(),
                events.borrow()
            );
            let mut runtime = runtime.borrow_mut();
            if !mounted.replace(true) {
                runtime.update(page(0, &sources.borrow(), &events)).unwrap();
                *weak.borrow_mut() = sources
                    .borrow()
                    .iter()
                    .map(|source| image_source(source).unwrap().value.downgrade().unwrap())
                    .collect();
                return;
            }
            runtime.dispatch_native_events().unwrap();
            if phase.get() < 3 {
                if events.borrow().len() < sources.borrow().len() * 2 {
                    return;
                }
                let mut actual = events.borrow().clone();
                actual.sort_unstable();
                let expected = (0..12)
                    .map(|index| (index, index % 6 < 3))
                    .collect::<Vec<_>>();
                assert_eq!(actual, expected);
                for (source, original) in sources.borrow().iter().zip(weak.borrow().iter()) {
                    assert_eq!(
                        image_source(source).unwrap().value,
                        original.upgrade().unwrap()
                    );
                }
                events.borrow_mut().clear();
                phase.set(phase.get() + 1);
                runtime.update(StackPanel::new()).unwrap();
                if phase.get() < 3 {
                    runtime
                        .update(page(phase.get(), &sources.borrow(), &events))
                        .unwrap();
                } else {
                    let opened = Rc::clone(&notifications);
                    let failed = Rc::clone(&notifications);
                    runtime
                        .update(
                            StackPanel::new().children((Image::new()
                                .source(sources.borrow()[0].clone())
                                .unwrap()
                                .on_opened(move || opened.borrow_mut().push(true))
                                .on_failed(move || failed.borrow_mut().push(false)),)),
                        )
                        .unwrap();
                    let object = runtime
                        .graph()
                        .children(root, RelationId::Children)
                        .unwrap()[0];
                    image.set(Some(object));
                    runtime
                        .adapter_mut()
                        .set_properties(object, &[source_property(&sources.borrow()[3])], &[])
                        .unwrap();
                }
            } else if phase.get() == 3 {
                if notifications.borrow().is_empty() {
                    return;
                }
                assert_eq!(*notifications.borrow(), [false]);
                notifications.borrow_mut().clear();
                let object = image.get().unwrap();
                let property = |index: usize| source_property(&sources.borrow()[index]);
                runtime
                    .adapter_mut()
                    .set_properties(object, &[property(0)], &[])
                    .unwrap();
                runtime
                    .adapter_mut()
                    .set_properties(object, &[], &[PropertyId::Source])
                    .unwrap();
                runtime
                    .adapter_mut()
                    .set_properties(object, &[property(0)], &[])
                    .unwrap();
                runtime
                    .adapter_mut()
                    .imperative(ImperativeRequest::SetNativeImageSource {
                        object,
                        source: None,
                        completion: Callback::new(|result: Result<(), IntegrationError>| {
                            result.unwrap();
                        }),
                    })
                    .unwrap();
                assert!(!runtime.adapter().image_subscriptions.contains_key(&object));
                let temporary = ImageSource::encoded(EncodedImage::from_static(PNG));
                let retained = Rc::downgrade(&temporary.native);
                runtime
                    .adapter_mut()
                    .set_properties(object, &[source_property(&temporary)], &[])
                    .unwrap();
                drop(temporary);
                assert!(retained.upgrade().is_some());
                runtime
                    .adapter_mut()
                    .set_properties(object, &[], &[PropertyId::Source])
                    .unwrap();
                assert!(retained.upgrade().is_none());
                runtime
                    .update(
                        StackPanel::new().children((
                            ImageIcon::new()
                                .source(sources.borrow()[0].clone())
                                .unwrap(),
                            AppBarButton::new().icon(Icon::image(sources.borrow()[1].clone())),
                            TitleBar::new().icon(Icon::image(sources.borrow()[2].clone())),
                        )),
                    )
                    .unwrap();
                runtime.update(StackPanel::new()).unwrap();
                window.borrow_mut().take().unwrap().close().unwrap();
                sources.borrow_mut().clear();
                phase.set(4);
            } else if weak
                .borrow()
                .iter()
                .all(|source| source.upgrade().is_none())
            {
                assert!(events.borrow().is_empty());
                assert!(notifications.borrow().is_empty());
                finished.set(true);
                app.exit().unwrap();
            }
        })?;
        timer.Start()?;
        Ok((timer, tick))
    });
    std::fs::remove_file(directory.join("image.png")).unwrap();
    std::fs::remove_file(directory.join("image.svg")).unwrap();
    std::fs::remove_file(directory.join("invalid.png")).unwrap();
    std::fs::remove_file(directory.join("invalid.svg")).unwrap();
    std::fs::remove_dir(directory).unwrap();
    result.unwrap();
    assert!(completed.get());
}
