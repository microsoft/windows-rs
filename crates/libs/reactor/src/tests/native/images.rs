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

fn image(index: usize, directory: &Path) -> Image {
    let image = Image::new().width(48.0).height(48.0);
    match index % 6 {
        0 => image.source_data(EncodedImage::new(PNG.to_vec())),
        1 => image.source_file(directory.join("image.png")).unwrap(),
        2 => image.source_file(directory.join("image.svg")).unwrap(),
        3 => image.source_data(EncodedImage::new(vec![0, 1, 2])),
        4 => image.source_file(directory.join("invalid.png")).unwrap(),
        _ => image.source_file(directory.join("invalid.svg")).unwrap(),
    }
}

fn page(generation: u32, directory: &Path, events: &Rc<RefCell<Vec<(usize, bool)>>>) -> View {
    StackPanel::new()
        .keyed_children([keyed(
            generation,
            StackPanel::new().children(
                (0..12)
                    .map(|index| {
                        let opened = Rc::clone(events);
                        let failed = Rc::clone(events);
                        image(index, directory)
                            .on_opened(move || opened.borrow_mut().push((index, true)))
                            .on_failed(move || failed.borrow_mut().push((index, false)))
                            .into()
                    })
                    .collect::<Vec<View>>(),
            ),
        )])
        .into()
}

fn live_images(adapter: &WinUiAdapter) -> Vec<native::ImageSource> {
    adapter
        .images
        .values()
        .filter_map(Weak::upgrade)
        .map(|image| image.value.clone())
        .collect()
}

#[test]
#[ignore = "requires an interactive WinUI desktop"]
fn equal_image_sources_share_native_images_across_page_replacement() {
    // WinUI supports one Application lifetime per process.
    if std::env::var_os("REACTOR_IMAGE_TEST_CHILD").is_none() {
        let status = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "native::winui::tests::images::equal_image_sources_share_native_images_across_page_replacement",
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
    let path = directory.clone();
    let result = App::run_with(move |app| {
        let directory = path;
        let events = Rc::new(RefCell::new(Vec::new()));
        let notifications = Rc::new(RefCell::new(Vec::new()));
        let mut runtime = Runtime::new(WinUiAdapter::default());
        runtime.update(StackPanel::new()).unwrap();
        let root = runtime.graph().root().unwrap();
        let window = runtime.adapter_mut().create_window(root).unwrap();
        window.activate().unwrap();
        let weak = RefCell::new(Vec::new());
        let timer = native::DispatcherQueue::GetForCurrentThread()?.CreateTimer()?;
        timer.SetInterval(TimeSpan::try_from(Duration::from_millis(100)).unwrap())?;
        let runtime = RefCell::new(runtime);
        let window = RefCell::new(Some(window));
        let mounted = Cell::new(false);
        let phase = Cell::new(0);
        let ticks = Cell::new(0);
        let object = Cell::new(None);
        let app = app.clone();
        let single = {
            let notifications = Rc::clone(&notifications);
            move |data: Option<EncodedImage>| {
                let opened = Rc::clone(&notifications);
                let failed = Rc::clone(&notifications);
                let image = Image::new()
                    .on_opened(move || opened.borrow_mut().push(true))
                    .on_failed(move || failed.borrow_mut().push(false));
                let image = match data {
                    Some(data) => image.source_data(data),
                    None => image,
                };
                StackPanel::new().children((image,))
            }
        };
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
                runtime.update(page(0, &directory, &events)).unwrap();
                let images = live_images(runtime.adapter());
                assert_eq!(images.len(), 6);
                *weak.borrow_mut() = images
                    .iter()
                    .map(|image| image.downgrade().unwrap())
                    .collect();
                return;
            }
            runtime.dispatch_native_events().unwrap();
            if phase.get() < 3 {
                if events.borrow().len() < 12 {
                    return;
                }
                let mut actual = events.borrow().clone();
                actual.sort_unstable();
                let expected = (0..12)
                    .map(|index| (index, index % 6 < 3))
                    .collect::<Vec<_>>();
                assert_eq!(actual, expected);
                let images = live_images(runtime.adapter());
                assert_eq!(images.len(), 6);
                for original in weak.borrow().iter() {
                    assert!(images.contains(&original.upgrade().unwrap()));
                }
                events.borrow_mut().clear();
                phase.set(phase.get() + 1);
                if phase.get() < 3 {
                    runtime
                        .update(page(phase.get(), &directory, &events))
                        .unwrap();
                } else {
                    runtime
                        .update(single(Some(EncodedImage::new(vec![0, 1, 2]))))
                        .unwrap();
                    object.set(Some(
                        runtime
                            .graph()
                            .children(root, RelationId::Children)
                            .unwrap()[0],
                    ));
                }
            } else if phase.get() == 3 {
                if notifications.borrow().is_empty() {
                    return;
                }
                assert_eq!(*notifications.borrow(), [false]);
                notifications.borrow_mut().clear();
                let object = object.get().unwrap();
                let png = || Some(EncodedImage::new(PNG.to_vec()));
                runtime.update(single(png())).unwrap();
                runtime.update(single(None)).unwrap();
                runtime.update(single(png())).unwrap();
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
                assert!(runtime.adapter().images.is_empty());
                runtime
                    .update(single(Some(EncodedImage::new([PNG, &[0]].concat()))))
                    .unwrap();
                let temporary =
                    Rc::downgrade(&runtime.adapter().image_subscriptions[&object].image);
                runtime.update(single(None)).unwrap();
                assert!(temporary.upgrade().is_none());
                runtime
                    .update(
                        StackPanel::new().children((
                            ImageIcon::new().source_data(EncodedImage::new(PNG.to_vec())),
                            AppBarButton::new()
                                .icon(Icon::image_data(EncodedImage::new(PNG.to_vec()))),
                            TitleBar::new()
                                .icon(Icon::image_file(directory.join("image.svg")).unwrap()),
                        )),
                    )
                    .unwrap();
                runtime.update(StackPanel::new()).unwrap();
                assert!(runtime.adapter().images.is_empty());
                window.borrow_mut().take().unwrap().close().unwrap();
                phase.set(4);
            } else if weak.borrow().iter().all(|image| image.upgrade().is_none()) {
                assert!(events.borrow().is_empty());
                assert!(notifications.borrow().is_empty());
                finished.set(true);
                app.exit().unwrap();
            }
        })?;
        timer.Start()?;
        Ok((timer, tick))
    });
    std::fs::remove_dir_all(directory).unwrap();
    result.unwrap();
    assert!(completed.get());
}
