use super::*;

const RED: Color = Color::rgb(220, 20, 40);
const BLUE: Color = Color::rgb(20, 70, 220);
const PARENT: Color = Color::rgb(20, 140, 60);
const CHANGED_PARENT: Color = Color::rgb(150, 40, 180);

fn view(size: Option<f64>, foreground: Option<Brush>) -> Border {
    let mut icon = FontIcon::new().glyph("\u{E80F}");
    if let Some(size) = size {
        icon = icon.font_size(size);
    }
    if let Some(foreground) = foreground {
        icon = icon.foreground(foreground);
    }
    Border::new()
        .width(240.0)
        .height(180.0)
        .content(Button::new().content(icon))
}

fn assert_state(runtime: &Runtime<WinUiAdapter>, object: ObjectId, size: f64, color: Color) {
    assert_eq!(
        runtime.adapter().font_icon_state(object).unwrap(),
        (size, color)
    );
}

#[test]
#[ignore = "requires an interactive WinUI desktop"]
fn font_icon_size_and_foreground_preserve_native_behavior() {
    // WinUI supports one Application lifetime per process.
    if std::env::var_os("REACTOR_FONT_ICON_TEST_CHILD").is_none() {
        let status = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "native::winui::tests::font_icon::font_icon_size_and_foreground_preserve_native_behavior",
                "--ignored",
                "--nocapture",
            ])
            .env("REACTOR_FONT_ICON_TEST_CHILD", "1")
            .status().unwrap();
        assert!(status.success(), "{status}");
        return;
    }
    let completed = Rc::new(Cell::new(false));
    let finished = Rc::clone(&completed);
    App::run_with(move |app| {
        let mut runtime = Runtime::new(WinUiAdapter::default());
        runtime.update(view(None, None)).unwrap();
        let root = runtime.graph().root().unwrap();
        let parent = runtime.graph().child(root, RelationId::Content).unwrap();
        let object = runtime.graph().child(parent, RelationId::Content).unwrap();
        let root_element = runtime
            .adapter()
            .ui_element(root)
            .unwrap()
            .cast::<native::IFrameworkElement>()?;
        root_element.SetRequestedTheme(native::ElementTheme::Light)?;
        runtime
            .adapter()
            .set_control_foreground(parent, PARENT)
            .unwrap();
        let window = runtime
            .adapter_mut()
            .create_window_with_policy(root, &WindowPolicy::new().client_size(400.0, 300.0))
            .unwrap();
        window.activate().unwrap();
        let window = RefCell::new(Some(window));
        let state = RefCell::new(Some(runtime));
        let phase = Cell::new(0);
        let default_size = Cell::new(0.0);
        let light_color = Cell::new(RED);
        let timer = native::DispatcherQueue::GetForCurrentThread()?.CreateTimer()?;
        timer.SetInterval(TimeSpan::try_from(Duration::from_millis(150)).unwrap())?;
        let app = app.clone();
        let tick = timer.Tick(move |sender, _| {
            let Some(mut runtime) = state.borrow_mut().take() else {
                sender.ok().unwrap().Stop().unwrap();
                window.borrow_mut().take().unwrap().close().unwrap();
                finished.set(true);
                app.exit().unwrap();
                return;
            };
            {
                let Some(Handle::Generated(GeneratedHandle::FontIcon(value))) =
                    runtime.adapter().handles.get(&object)
                else {
                    panic!()
                };
                let value = value.clone();
                let element = value.cast::<native::UIElement>().unwrap();
                let framework = value.cast::<native::IFrameworkElement>().unwrap();
                match phase.get() {
                    0 => {
                        let (size, color) = runtime.adapter().font_icon_state(object).unwrap();
                        default_size.set(size);
                        assert_eq!(color, PARENT);
                        value.SetFontSize(24.0).unwrap();
                        assert!(runtime.update(view(None, None)).unwrap().is_empty());
                        assert_state(&runtime, object, 24.0, PARENT);
                        runtime.update(view(Some(16.0), Some(RED.into()))).unwrap();
                        element.UpdateLayout().unwrap();
                        assert_state(&runtime, object, 16.0, RED);
                        let small = framework.ActualHeight().unwrap();
                        assert!(small > 0.0);
                        runtime.update(view(Some(48.0), Some(BLUE.into()))).unwrap();
                        element.UpdateLayout().unwrap();
                        assert_state(&runtime, object, 48.0, BLUE);
                        let large = framework.ActualHeight().unwrap();
                        assert!(large > small * 2.0, "{small} -> {large}");
                        assert!(
                            runtime
                                .update(view(Some(48.0), Some(BLUE.into())))
                                .unwrap()
                                .is_empty()
                        );

                        runtime.update(view(None, Some(BLUE.into()))).unwrap();
                        assert_state(&runtime, object, default_size.get(), BLUE);
                        runtime.update(view(Some(32.0), None)).unwrap();
                        assert_state(&runtime, object, 32.0, PARENT);
                        runtime
                            .adapter()
                            .set_control_foreground(parent, CHANGED_PARENT)
                            .unwrap();
                        assert!(runtime.update(view(Some(32.0), None)).unwrap().is_empty());
                        assert_state(&runtime, object, 32.0, CHANGED_PARENT);
                        runtime.update(view(Some(32.0), Some(RED.into()))).unwrap();
                        runtime
                            .update(view(Some(32.0), Some(ThemeBrush::PrimaryText.into())))
                            .unwrap();
                    }
                    1 => {
                        let (_, color) = runtime.adapter().font_icon_state(object).unwrap();
                        assert_ne!(color, RED);
                        assert_ne!(color, CHANGED_PARENT);
                        light_color.set(color);
                        root_element
                            .SetRequestedTheme(native::ElementTheme::Dark)
                            .unwrap();
                    }
                    2 => {
                        let (_, color) = runtime.adapter().font_icon_state(object).unwrap();
                        assert_ne!(color, light_color.get());
                        assert_ne!(color, CHANGED_PARENT);
                        runtime.update(view(Some(32.0), Some(BLUE.into()))).unwrap();
                        assert_state(&runtime, object, 32.0, BLUE);
                        runtime
                            .update(view(Some(32.0), Some(ThemeBrush::PrimaryText.into())))
                            .unwrap();
                    }
                    3 => {
                        assert_ne!(runtime.adapter().font_icon_state(object).unwrap().1, BLUE);
                        runtime.update(view(Some(32.0), None)).unwrap();
                        assert_state(&runtime, object, 32.0, CHANGED_PARENT);
                        runtime
                            .adapter()
                            .set_control_foreground(parent, PARENT)
                            .unwrap();
                        assert!(runtime.update(view(Some(32.0), None)).unwrap().is_empty());
                        assert_state(&runtime, object, 32.0, PARENT);
                        runtime.update(view(None, None)).unwrap();
                        assert_state(&runtime, object, default_size.get(), PARENT);
                        value.SetFontSize(28.0).unwrap();
                        value
                            .cast::<native::IIconElement>()
                            .unwrap()
                            .SetForeground(&solid_color_brush(RED).unwrap())
                            .unwrap();
                        assert!(runtime.update(view(None, None)).unwrap().is_empty());
                        assert_state(&runtime, object, 28.0, RED);
                    }
                    _ => unreachable!(),
                }
                let Some(Handle::Generated(GeneratedHandle::FontIcon(retained))) =
                    runtime.adapter().handles.get(&object)
                else {
                    panic!()
                };
                assert_eq!(retained, &value);
                assert_eq!(
                    runtime.graph().child(parent, RelationId::Content),
                    Some(object)
                );
            }
            if phase.get() == 3 {
                runtime.update(Border::new()).unwrap();
            } else {
                phase.set(phase.get() + 1);
                *state.borrow_mut() = Some(runtime);
            }
        })?;
        timer.Start()?;
        Ok((timer, tick))
    })
    .unwrap();
    assert!(completed.get());
}
