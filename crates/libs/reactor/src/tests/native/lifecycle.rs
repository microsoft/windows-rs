use super::*;

fn page(mounted: bool, grid: &ElementRef<Grid>, web: &ElementRef<WebView2>) -> View {
    let page = StackPanel::new()
        .children((
            ItemsRepeater::new().virtual_source(VirtualSource::new(
                0,
                50,
                |index| index as u64,
                |index| TextBlock::new().text(index.to_string()),
            )),
            TreeView::new().nodes([TreeNode::new("a", "a"), TreeNode::new("b", "b")]),
            Grid::new().width(10.0).height(10.0).element_ref(grid),
            WebView2::new().width(10.0).height(10.0).element_ref(web),
        ))
        .exit_fade(Duration::from_millis(50));
    StackPanel::new()
        .keyed_children(mounted.then(|| keyed("page", page)))
        .into()
}

fn assert_released(adapter: &WinUiAdapter) {
    assert_eq!(adapter.handles.len(), 1);
    assert!(adapter.retirements.is_empty());
    assert!(adapter.virtual_items.is_empty());
    assert!(adapter.tree_node_texts.borrow().is_empty());
    assert!(adapter.observations.is_empty());
    assert!(adapter.webview_initializations.borrow().is_empty());
}

#[test]
#[ignore = "requires an interactive WinUI desktop"]
fn retirement_and_replacement_release_adapter_state() {
    // WinUI supports one Application lifetime per process.
    if std::env::var_os("REACTOR_LIFECYCLE_TEST_CHILD").is_none() {
        let status = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "native::winui::tests::lifecycle::retirement_and_replacement_release_adapter_state",
                "--ignored",
                "--nocapture",
            ])
            .env("REACTOR_LIFECYCLE_TEST_CHILD", "1")
            .status()
            .unwrap();
        assert!(status.success());
        return;
    }
    let completed = Rc::new(Cell::new(false));
    let finished = Rc::clone(&completed);
    let result = App::run_with(move |app| {
        let grid = ElementRef::<Grid>::new();
        let web = ElementRef::<WebView2>::new();
        let mut runtime = Runtime::new(WinUiAdapter::default());
        runtime.update(page(false, &grid, &web)).unwrap();
        let root = runtime.graph().root().unwrap();
        let window = runtime.adapter_mut().create_window(root).unwrap();
        window.activate().unwrap();
        let timer = native::DispatcherQueue::GetForCurrentThread()?.CreateTimer()?;
        timer.SetInterval(TimeSpan::try_from(Duration::from_millis(100)).unwrap())?;
        let runtime = RefCell::new(runtime);
        let window = RefCell::new(Some(window));
        let observation = RefCell::new(None);
        let completions = Rc::new(Cell::new(0));
        let phase = Cell::new(0);
        let ticks = Cell::new(0);
        let app = app.clone();
        let tick = timer.Tick(move |_, _| {
            ticks.set(ticks.get() + 1);
            assert!(
                ticks.get() < 100,
                "lifecycle test timed out at phase {}",
                phase.get()
            );
            let mut runtime = runtime.borrow_mut();
            runtime.dispatch_native_events().unwrap();
            let request = || {
                let completions = Rc::clone(&completions);
                assert!(
                    web.request_core_web_view2(move |_| completions.set(completions.get() + 1))
                );
            };
            match phase.get() {
                0 => {
                    runtime.update(page(true, &grid, &web)).unwrap();
                    *observation.borrow_mut() = Some(grid.observe_composition_host(|_| {}));
                    request();
                    runtime.dispatch_native_events().unwrap();
                    let adapter = runtime.adapter();
                    assert_eq!(adapter.virtual_items.len(), 1);
                    assert_eq!(adapter.tree_node_texts.borrow().len(), 2);
                    assert_eq!(adapter.observations.len(), 1);
                    assert_eq!(adapter.webview_initializations.borrow().len(), 1);
                    runtime.update(page(false, &grid, &web)).unwrap();
                    runtime.dispatch_native_events().unwrap();
                    let adapter = runtime.adapter();
                    assert_eq!(adapter.retirements.len(), 1);
                    assert!(adapter.observations.is_empty());
                    phase.set(1);
                }
                1 => {
                    runtime.update(page(false, &grid, &web)).unwrap();
                    if !runtime.adapter().retirements.is_empty() {
                        return;
                    }
                    assert_released(runtime.adapter());
                    assert_eq!(completions.get(), 1);
                    runtime
                        .update(StackPanel::new().children((WebView2::new().element_ref(&web),)))
                        .unwrap();
                    request();
                    runtime.dispatch_native_events().unwrap();
                    assert_eq!(runtime.adapter().webview_initializations.borrow().len(), 1);
                    let object = runtime
                        .graph()
                        .children(root, RelationId::Children)
                        .unwrap()[0];
                    runtime
                        .update_subtree_before_apply(object, Grid::new(), || {})
                        .unwrap();
                    assert!(
                        runtime
                            .adapter()
                            .webview_initializations
                            .borrow()
                            .is_empty()
                    );
                    assert_eq!(completions.get(), 2);
                    runtime.update(page(false, &grid, &web)).unwrap();
                    assert_released(runtime.adapter());
                    window.borrow_mut().take().unwrap().close().unwrap();
                    finished.set(true);
                    app.exit().unwrap();
                    phase.set(2);
                }
                _ => {}
            }
        })?;
        timer.Start()?;
        Ok((timer, tick))
    });
    result.unwrap();
    assert!(completed.get());
}
