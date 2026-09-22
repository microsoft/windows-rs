#![windows_subsystem = "console"]

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Duration;
use windows::UI::Input::Preview::Injection::{
    InjectedInputMouseInfo, InjectedInputMouseOptions, InputInjector,
};
use windows::Win32::RECT;
use windows::Win32::winuser::{
    GetForegroundWindow, GetSystemMetrics, GetWindowRect, SM_CXVIRTUALSCREEN, SM_CYVIRTUALSCREEN,
    SM_XVIRTUALSCREEN, SM_YVIRTUALSCREEN,
};
use windows_collections::IIterable;
use windows_reactor::{
    App, AppContext, AppProxy, Component, ComponentContext, TextBlock, View, ViewContext,
};
use windows_reactor2 as reactor2;

#[derive(Clone)]
struct Input {
    app: AppProxy,
}

impl PartialEq for Input {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

struct Fixture {
    app: AppProxy,
    boundary_host: reactor2::ComponentHost<reactor2::native::WinUiAdapter>,
    boundary_sender: reactor2::ComponentSender<()>,
    tree_content_sender: reactor2::ComponentSender<()>,
    tree_sender: reactor2::ComponentSender<()>,
    boundary_window: reactor2::native::NativeWindow,
    pointer_runtime: reactor2::Runtime<reactor2::native::WinUiAdapter>,
    pointer_window: reactor2::native::NativeWindow,
    retirement_runtime: reactor2::Runtime<reactor2::native::WinUiAdapter>,
    retirement_window: reactor2::native::NativeWindow,
    retiring_object: reactor2::ObjectId,
    retirement_expected_order: Vec<reactor2::ObjectId>,
    retirement_verified: bool,
    received_pointer: Rc<RefCell<Option<reactor2::PointerEventInfo>>>,
    pointer_injected: bool,
    pointer_waits: usize,
    runtime: reactor2::Runtime<reactor2::native::WinUiAdapter>,
    window: reactor2::native::NativeWindow,
    iteration: usize,
    input_phase: u8,
    text: Rc<RefCell<Rc<str>>>,
    text_changed: reactor2::Callback<Rc<str>>,
    text_changed_count: Rc<Cell<usize>>,
    replacement_text_changed: reactor2::Callback<Rc<str>>,
    replacement_text_changed_count: Rc<Cell<usize>>,
}

struct RootSwitch(bool);

impl reactor2::Component for RootSwitch {
    type Input = ();
    type Message = ();

    fn create(_input: &Self::Input, _context: &reactor2::ComponentContext<Self::Message>) -> Self {
        Self(false)
    }

    fn update(
        &mut self,
        _message: Self::Message,
        _context: &reactor2::ComponentContext<Self::Message>,
    ) {
        self.0 = !self.0;
    }

    fn view(
        &self,
        _input: &Self::Input,
        _context: &mut reactor2::ComponentViewContext<'_, Self::Message>,
    ) -> reactor2::Visual {
        if self.0 {
            reactor2::Border::new()
                .content(reactor2::TextBlock::new().text("Border root"))
                .into()
        } else {
            reactor2::TextBlock::new().text("Text root").into()
        }
    }
}

struct TreeContent(usize);

impl reactor2::Component for TreeContent {
    type Input = Rc<str>;
    type Message = ();

    fn create(_input: &Self::Input, _context: &reactor2::ComponentContext<Self::Message>) -> Self {
        Self(0)
    }

    fn update(
        &mut self,
        _message: Self::Message,
        _context: &reactor2::ComponentContext<Self::Message>,
    ) {
        self.0 += 1;
    }

    fn view(
        &self,
        input: &Self::Input,
        _context: &mut reactor2::ComponentViewContext<'_, Self::Message>,
    ) -> reactor2::Visual {
        reactor2::StackPanel::new()
            .children([
                reactor2::TextBlock::new().text(input.clone()).into(),
                reactor2::TextBlock::new().text(self.0.to_string()).into(),
            ])
            .into()
    }
}

struct TreeComponents(bool);

impl reactor2::Component for TreeComponents {
    type Input = ();
    type Message = ();

    fn create(_input: &Self::Input, _context: &reactor2::ComponentContext<Self::Message>) -> Self {
        Self(false)
    }

    fn update(
        &mut self,
        _message: Self::Message,
        _context: &reactor2::ComponentContext<Self::Message>,
    ) {
        self.0 = !self.0;
    }

    fn view(
        &self,
        _input: &Self::Input,
        _context: &mut reactor2::ComponentViewContext<'_, Self::Message>,
    ) -> reactor2::Visual {
        let first = reactor2::TreeNode::new("first", "First")
            .expanded(true)
            .content(reactor2::component::<TreeContent>(
                "first-content",
                Rc::from("First"),
            ));
        let second = reactor2::TreeNode::new("second", "Second")
            .expanded(true)
            .content(reactor2::component::<TreeContent>(
                "second-content",
                Rc::from("Second"),
            ));
        if self.0 {
            reactor2::TreeView::new().nodes([second, first]).into()
        } else {
            reactor2::TreeView::new().nodes([first, second]).into()
        }
    }
}

impl Fixture {
    fn inject_pointer_click() -> Result<(), String> {
        let hwnd = unsafe { GetForegroundWindow() };
        if hwnd.is_null() {
            return Err("the Reactor2 pointer self-test window is not foreground".to_string());
        }
        let mut rect = RECT::default();
        if !unsafe { GetWindowRect(hwnd, &mut rect) }.as_bool() {
            return Err("could not read the Reactor2 pointer self-test window".to_string());
        }
        let injector = InputInjector::TryCreate().map_err(|error| error.to_string())?;
        let (origin_x, origin_y, width, height) = unsafe {
            (
                GetSystemMetrics(SM_XVIRTUALSCREEN),
                GetSystemMetrics(SM_YVIRTUALSCREEN),
                GetSystemMetrics(SM_CXVIRTUALSCREEN).max(2),
                GetSystemMetrics(SM_CYVIRTUALSCREEN).max(2),
            )
        };
        let x = ((f64::from(rect.left + 100 - origin_x) * 65535.0) / f64::from(width - 1)).round()
            as i32;
        let y = ((f64::from(rect.top + 100 - origin_y) * 65535.0) / f64::from(height - 1)).round()
            as i32;
        let inject = |options| -> windows_core::Result<()> {
            let info = InjectedInputMouseInfo::new()?;
            info.SetDeltaX(x)?;
            info.SetDeltaY(y)?;
            info.SetMouseOptions(
                InjectedInputMouseOptions::Absolute
                    | InjectedInputMouseOptions::VirtualDesk
                    | options,
            )?;
            let inputs: IIterable<InjectedInputMouseInfo> = vec![Some(info)].into();
            injector.InjectMouseInput(&inputs)
        };
        inject(InjectedInputMouseOptions::Move).map_err(|error| error.to_string())?;
        inject(InjectedInputMouseOptions::LeftDown).map_err(|error| error.to_string())?;
        inject(InjectedInputMouseOptions::LeftUp).map_err(|error| error.to_string())?;
        Ok(())
    }

    fn declaration(
        iteration: usize,
        text: Rc<str>,
        text_changed: Option<reactor2::Callback<Rc<str>>>,
    ) -> reactor2::Visual {
        let children = [
            reactor2::TreeNode::new("first-child", format!("First child {iteration}"))
                .content(reactor2::TextBlock::new().text("Nested content")),
            reactor2::TreeNode::new("second-child", "Second child"),
        ];
        let children = if iteration.is_multiple_of(2) {
            children
        } else {
            [children[1].clone(), children[0].clone()]
        };
        let roots = [
            reactor2::TreeNode::new("first", format!("First {iteration}"))
                .expanded(true)
                .content(reactor2::StackPanel::new().children(vec![
                    reactor2::TextBlock::new().text("Folder").into(),
                    reactor2::TextBlock::new().text(format!("{} changes", iteration % 7)).into(),
                ]))
                .children(children),
            reactor2::TreeNode::new("second", format!("Second {iteration}"))
                .content(reactor2::TextBlock::new().text("Leaf")),
        ];
        let roots = if iteration.is_multiple_of(2) {
            reactor2::TreeView::new().nodes(roots)
        } else {
            reactor2::TreeView::new().nodes([roots[1].clone(), roots[0].clone()])
        };
        let items = if iteration.is_multiple_of(2) {
            [
                reactor2::DataItem::new("first", format!("First item {iteration}")),
                reactor2::DataItem::new("second", "Second item"),
            ]
        } else {
            [
                reactor2::DataItem::new("second", "Second item"),
                reactor2::DataItem::new("first", format!("First item {iteration}")),
            ]
        };
        let text_box = reactor2::TextBox::new(text);
        let text_box = if let Some(text_changed) = text_changed {
            text_box.on_text_changed_callback(text_changed)
        } else {
            text_box
        };
        reactor2::StackPanel::new()
            .children(vec![
                text_box.into(),
                reactor2::Border::new()
                    .transitions_optional(
                        iteration
                            .is_multiple_of(2)
                            .then_some([reactor2::ThemeTransition::Reposition]),
                    )
                    .content(reactor2::TextBlock::new().text(format!("Iteration {iteration}")))
                    .into(),
                roots.into(),
                reactor2::ListView::new().items(items).into(),
            ])
            .into()
    }

    fn schedule(context: &ComponentContext<Self>) {
        context.spawn_background(|_| {
            std::thread::sleep(Duration::from_millis(20));
        });
    }
}

impl Component for Fixture {
    type Input = Input;
    type Message = ();

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        let text = Rc::new(RefCell::new(Rc::<str>::from("Initial text")));
        let text_changed_count = Rc::new(Cell::new(0));
        let text_for_callback = Rc::clone(&text);
        let count_for_callback = Rc::clone(&text_changed_count);
        let text_changed = reactor2::Callback::new(move |value| {
            *text_for_callback.borrow_mut() = value;
            count_for_callback.set(count_for_callback.get() + 1);
        });
        let replacement_text_changed_count = Rc::new(Cell::new(0));
        let replacement_text_for_callback = Rc::clone(&text);
        let replacement_count_for_callback = Rc::clone(&replacement_text_changed_count);
        let replacement_text_changed = reactor2::Callback::new(move |value| {
            *replacement_text_for_callback.borrow_mut() = value;
            replacement_count_for_callback.set(replacement_count_for_callback.get() + 1);
        });
        let mut runtime = reactor2::Runtime::new(reactor2::native::WinUiAdapter::default());
        let sender = context.sender();
        runtime.set_native_event_waker(move || {
            sender.send(());
        });
        runtime
            .update(Self::declaration(
                0,
                Rc::clone(&text.borrow()),
                Some(text_changed.clone()),
            ))
            .unwrap();
        let root = runtime.graph().root().unwrap();
        let window = runtime.adapter().open_window(root).unwrap();
        let grid_reference = reactor2::ElementRef::<reactor2::Grid>::new();
        let image_reference = reactor2::ElementRef::<reactor2::Image>::new();
        let webview_reference = reactor2::ElementRef::<reactor2::WebView2>::new();
        let surface_reference = reactor2::ElementRef::<reactor2::SwapChainPanel>::new();
        let composition_events = Rc::new(Cell::new(0));
        let composition_callback = Rc::clone(&composition_events);
        let composition_observation = grid_reference.observe_composition_host(move |_| {
            composition_callback.set(composition_callback.get() + 1);
        });
        let image_events = Rc::new(Cell::new(0));
        let image_callback = Rc::clone(&image_events);
        let image_observation = image_reference.observe_rasterization_scale(move |_| {
            image_callback.set(image_callback.get() + 1);
        });
        let surface_events = Rc::new(Cell::new(0));
        let surface_callback = Rc::clone(&surface_events);
        let surface_observation = surface_reference.observe_surface(move |_| {
            surface_callback.set(surface_callback.get() + 1);
        });
        let mut reference_runtime =
            reactor2::Runtime::new(reactor2::native::WinUiAdapter::default());
        reference_runtime
            .update(
                reactor2::Grid::new()
                    .element_ref(&grid_reference)
                    .children([
                        reactor2::keyed(
                            "image",
                            reactor2::Image::new().element_ref(&image_reference),
                        ),
                        reactor2::keyed(
                            "webview",
                            reactor2::WebView2::new().element_ref(&webview_reference),
                        ),
                        reactor2::keyed(
                            "surface",
                            reactor2::SwapChainPanel::new().element_ref(&surface_reference),
                        ),
                    ]),
            )
            .unwrap();
        let reference_root = reference_runtime.graph().root().unwrap();
        let reference_window = reference_runtime
            .adapter()
            .open_window(reference_root)
            .unwrap();
        let composition_results = Rc::new(RefCell::new(Vec::new()));
        let composition_completion = Rc::clone(&composition_results);
        assert!(
            grid_reference.request_set_child_visual(None, move |result| {
                composition_completion.borrow_mut().push(result);
            })
        );
        let image_results = Rc::new(RefCell::new(Vec::new()));
        let image_completion = Rc::clone(&image_results);
        assert!(
            image_reference.request_set_native_source(None, move |result| {
                image_completion.borrow_mut().push(result);
            })
        );
        let surface_results = Rc::new(RefCell::new(Vec::new()));
        let surface_completion = Rc::clone(&surface_results);
        assert!(surface_reference.request_clear_swap_chain(move |result| {
            surface_completion.borrow_mut().push(result);
        }));
        let webview_results = Rc::new(Cell::new(0));
        let webview_completion = Rc::clone(&webview_results);
        assert!(webview_reference.request_core_web_view2(move |_| {
            webview_completion.set(webview_completion.get() + 1);
        }));
        reference_runtime.dispatch_native_events().unwrap();
        assert!(!composition_results.borrow().is_empty());
        assert!(!image_results.borrow().is_empty());
        assert!(!surface_results.borrow().is_empty());
        assert!(composition_events.get() != 0);
        assert!(surface_events.get() != 0);
        drop(composition_observation);
        drop(image_observation);
        drop(surface_observation);
        reference_runtime.dispatch_native_events().unwrap();
        reference_window.close().unwrap();
        drop(reference_runtime);
        assert_eq!(webview_results.get(), 1);
        let mut title_bar_runtime =
            reactor2::Runtime::new(reactor2::native::WinUiAdapter::default());
        title_bar_runtime.update(reactor2::Grid::new()).unwrap();
        let title_bar_root = title_bar_runtime.graph().root().unwrap();
        let title_bar_window = title_bar_runtime
            .adapter()
            .open_window(title_bar_root)
            .unwrap();
        assert!(matches!(
            title_bar_runtime.adapter().create_window(title_bar_root),
            Err(reactor2::native::WinUiError::DuplicateWindowRoot(root)) if root == title_bar_root
        ));
        title_bar_runtime
            .update(
                reactor2::Grid::new().children([reactor2::keyed(
                    "title",
                    reactor2::TitleBar::new()
                        .title("Reactor2 title-bar self-test")
                        .preferred_height(reactor2::WindowTitleBarHeight::Tall),
                )]),
            )
            .unwrap();
        title_bar_runtime
            .update(
                reactor2::Grid::new().children([reactor2::keyed(
                    "title",
                    reactor2::TitleBar::new()
                        .title("Reactor2 title-bar self-test")
                        .preferred_height(reactor2::WindowTitleBarHeight::Standard),
                )]),
            )
            .unwrap();
        title_bar_runtime.update(reactor2::Grid::new()).unwrap();
        title_bar_window.close().unwrap();
        let boundary_host = reactor2::ComponentHost::mount(
            reactor2::native::WinUiAdapter::default(),
            [
                reactor2::component::<RootSwitch>("switch", ()),
                reactor2::component::<TreeComponents>("tree", ()),
            ],
        )
        .unwrap();
        let boundary_sender = boundary_host
            .sender::<RootSwitch>(&reactor2::Key::from("switch"))
            .unwrap();
        let tree_sender = boundary_host
            .sender::<TreeComponents>(&reactor2::Key::from("tree"))
            .unwrap();
        let tree_content_sender = boundary_host
            .sender_at::<TreeContent>(&[
                reactor2::Key::from("tree"),
                reactor2::Key::from("first-content"),
            ])
            .unwrap();
        let boundary_root = boundary_host.runtime().graph().root().unwrap();
        let boundary_window = boundary_host
            .runtime()
            .adapter()
            .open_window(boundary_root)
            .unwrap();
        assert!(boundary_sender.send(()));
        assert!(tree_sender.send(()));
        assert!(tree_content_sender.send(()));
        let stale_calls = Rc::new(Cell::new(0));
        let stale_callback_calls = Rc::clone(&stale_calls);
        let mut replacement_runtime =
            reactor2::Runtime::new(reactor2::native::WinUiAdapter::default());
        replacement_runtime
            .update(reactor2::Grid::new().children([reactor2::keyed(
                "replace",
                reactor2::TextBox::new("Old").on_text_changed(move |_| {
                    stale_callback_calls.set(stale_callback_calls.get() + 1);
                }),
            )]))
            .unwrap();
        let replacement_root = replacement_runtime.graph().root().unwrap();
        let replacement_child = replacement_runtime
            .graph()
            .children(replacement_root, reactor2::RelationId::Children)
            .unwrap()[0];
        let replacement_window = replacement_runtime
            .adapter()
            .open_window(replacement_root)
            .unwrap();
        replacement_runtime
            .adapter()
            .simulate_text_input(replacement_child, "First", 0, 0)
            .unwrap();
        replacement_runtime
            .adapter()
            .simulate_text_input(replacement_child, "Stale", 0, 0)
            .unwrap();
        let mut active = replacement_runtime.next_native_event().unwrap().unwrap();
        active.invoke();
        replacement_runtime
            .update_subtree(replacement_child, reactor2::Border::new())
            .unwrap();
        drop(active);
        assert_eq!(replacement_runtime.dispatch_native_events().unwrap(), 0);
        assert_eq!(stale_calls.get(), 1);
        replacement_runtime
            .adapter()
            .validate_graph(replacement_runtime.graph())
            .unwrap();
        replacement_window.close().unwrap();
        let button_calls = Rc::new(Cell::new(0));
        let button_callback_calls = Rc::clone(&button_calls);
        let mut button_runtime = reactor2::Runtime::new(reactor2::native::WinUiAdapter::default());
        button_runtime
            .update(
                reactor2::Button::new()
                    .content(reactor2::TextBlock::new().text("Click"))
                    .on_click(move || {
                        button_callback_calls.set(button_callback_calls.get() + 1);
                    }),
            )
            .unwrap();
        let button = button_runtime.graph().root().unwrap();
        let button_window = button_runtime.adapter().open_window(button).unwrap();
        button_runtime.adapter().simulate_click(button).unwrap();
        assert_eq!(button_runtime.dispatch_native_events().unwrap(), 1);
        assert_eq!(button_calls.get(), 1);
        button_runtime
            .adapter()
            .validate_graph(button_runtime.graph())
            .unwrap();
        button_window.close().unwrap();
        let pointer_value = reactor2::PointerEventInfo {
            x: 12.5,
            y: 24.5,
            window_x: 112.5,
            window_y: 224.5,
            pointer_id: 42,
            is_captured: true,
            is_right_button_pressed: true,
            ..Default::default()
        };
        let received_pointer = Rc::new(RefCell::new(None));
        let received_pointer_callback = Rc::clone(&received_pointer);
        let stale_pointer_calls = Rc::new(Cell::new(0));
        let stale_pointer_callback = Rc::clone(&stale_pointer_calls);
        let mut pointer_runtime = reactor2::Runtime::new(reactor2::native::WinUiAdapter::default());
        pointer_runtime
            .update(
                reactor2::Border::new()
                    .width(200.0)
                    .height(200.0)
                    .background(reactor2::Color::rgb(255, 255, 255))
                    .on_pointer_released(move |_| {
                        stale_pointer_callback.set(stale_pointer_callback.get() + 1);
                    }),
            )
            .unwrap();
        let pointer_border = pointer_runtime.graph().root().unwrap();
        let pointer_window = pointer_runtime
            .adapter()
            .open_window_with_policy(
                pointer_border,
                &reactor2::native::WindowPolicy::new()
                    .title("Reactor2 pointer self-test")
                    .client_size(200.0, 200.0),
            )
            .unwrap();
        pointer_runtime
            .adapter()
            .simulate_pointer_released(pointer_border, pointer_value)
            .unwrap();
        pointer_runtime
            .adapter()
            .simulate_pointer_released(pointer_border, pointer_value)
            .unwrap();
        let mut active = pointer_runtime.next_native_event().unwrap().unwrap();
        active.invoke();
        pointer_runtime
            .update(
                reactor2::Border::new()
                    .width(200.0)
                    .height(200.0)
                    .background(reactor2::Color::rgb(255, 255, 255))
                    .on_pointer_released(move |value| {
                        *received_pointer_callback.borrow_mut() = Some(value);
                    }),
            )
            .unwrap();
        drop(active);
        assert_eq!(pointer_runtime.dispatch_native_events().unwrap(), 0);
        assert_eq!(stale_pointer_calls.get(), 1);
        pointer_runtime
            .adapter()
            .simulate_pointer_released(pointer_border, pointer_value)
            .unwrap();
        assert_eq!(pointer_runtime.dispatch_native_events().unwrap(), 1);
        assert_eq!(*received_pointer.borrow(), Some(pointer_value));
        *received_pointer.borrow_mut() = None;
        pointer_runtime
            .adapter()
            .validate_graph(pointer_runtime.graph())
            .unwrap();
        let mut generated_runtime =
            reactor2::Runtime::new(reactor2::native::WinUiAdapter::default());
        let slider_changed = Rc::new(Cell::new(0.0));
        let slider_changed_count = Rc::new(Cell::new(0));
        let slider_changed_callback = Rc::clone(&slider_changed);
        let slider_count_callback = Rc::clone(&slider_changed_count);
        let toggle_changed = Rc::new(Cell::new(false));
        let toggle_changed_count = Rc::new(Cell::new(0));
        let toggle_changed_callback = Rc::clone(&toggle_changed);
        let toggle_count_callback = Rc::clone(&toggle_changed_count);
        let focus_reference = reactor2::ElementRef::default();
        generated_runtime
            .update(
                reactor2::StackPanel::new()
                    .spacing(8.0)
                    .orientation(reactor2::Orientation::Horizontal)
                    .children([
                        reactor2::Slider::new()
                            .minimum(-10.0)
                            .maximum(10.0)
                            .value(2.5)
                            .on_value_changed(move |value| {
                                slider_changed_callback.set(value);
                                slider_count_callback.set(slider_count_callback.get() + 1);
                            })
                            .into(),
                        reactor2::ToggleSwitch::new()
                            .is_on(true)
                            .on_toggled(move |value| {
                                toggle_changed_callback.set(value);
                                toggle_count_callback.set(toggle_count_callback.get() + 1);
                            })
                            .into(),
                        reactor2::CheckBox::new()
                            .is_checked(Some(true))
                            .content(reactor2::TextBlock::new().text("Enabled"))
                            .into(),
                        reactor2::ScrollViewer::new()
                            .content(
                                reactor2::Canvas::new().children([reactor2::TextBlock::new()
                                    .text("Scrollable canvas")
                                    .canvas_left(12.0)
                                    .canvas_top(24.0)
                                    .into()]),
                            )
                            .into(),
                        reactor2::SelectorBar::new()
                            .items([reactor2::keyed(
                                "only",
                                reactor2::SelectorBarItem::new().text("Only"),
                            )])
                            .into(),
                        reactor2::AppBarButton::new()
                            .label("Icon")
                            .icon(reactor2::SymbolIcon::new())
                            .into(),
                        reactor2::Button::new()
                            .element_ref(&focus_reference)
                            .is_enabled(true)
                            .min_width(20.0)
                            .max_height(80.0)
                            .grid_row(2)
                            .relative_align_left()
                            .automation_name("capability button")
                            .into(),
                        reactor2::TextBlock::new()
                            .text("Styled")
                            .font_weight(reactor2::FontWeight::SEMI_BOLD)
                            .into(),
                    ]),
            )
            .unwrap();
        let generated_root = generated_runtime.graph().root().unwrap();
        let generated_window = generated_runtime
            .adapter()
            .open_window(generated_root)
            .unwrap();
        let generated_children = generated_runtime
            .graph()
            .children(generated_root, reactor2::RelationId::Children)
            .unwrap();
        let slider = generated_children[0];
        let toggle = generated_children[1];
        let check_box = generated_children[2];
        let scroll_viewer = generated_children[3];
        let canvas = generated_runtime
            .graph()
            .child(scroll_viewer, reactor2::RelationId::Content)
            .unwrap();
        let canvas_child = generated_runtime
            .graph()
            .children(canvas, reactor2::RelationId::Children)
            .unwrap()[0];
        let capability_button = generated_children[6];
        let styled_text = generated_children[7];
        assert_eq!(
            generated_runtime
                .adapter()
                .stack_panel_state(generated_root)
                .unwrap(),
            (8.0, true)
        );
        assert_eq!(
            generated_runtime.adapter().slider_state(slider).unwrap(),
            (-10.0, 10.0, 2.5)
        );
        assert!(
            generated_runtime
                .adapter()
                .check_box_state(check_box)
                .unwrap()
        );
        assert!(
            generated_runtime
                .adapter()
                .toggle_switch_state(toggle)
                .unwrap()
        );
        assert_eq!(
            generated_runtime
                .adapter()
                .canvas_position(canvas_child)
                .unwrap(),
            (12.0, 24.0)
        );
        assert_eq!(
            generated_runtime
                .adapter()
                .capability_state(capability_button)
                .unwrap(),
            (20.0, 80.0, 2, true, "capability button".to_string(), true,)
        );
        assert_eq!(
            generated_runtime
                .adapter()
                .text_block_font_weight(styled_text)
                .unwrap(),
            600
        );
        let mut early_virtual_runtime =
            reactor2::Runtime::new(reactor2::native::WinUiAdapter::default());
        early_virtual_runtime
            .update(
                reactor2::ItemsRepeater::new().virtual_source(reactor2::VirtualSource::new(
                    1,
                    10_000,
                    reactor2::Key::from,
                    |index| -> reactor2::Visual {
                        reactor2::TextBlock::new()
                            .text(format!("virtual {index}"))
                            .into()
                    },
                )),
            )
            .unwrap();
        let early_virtual_root = early_virtual_runtime.graph().root().unwrap();
        let early_virtual_window = early_virtual_runtime
            .adapter()
            .open_window(early_virtual_root)
            .unwrap();
        early_virtual_runtime
            .adapter()
            .realize_virtual_item(early_virtual_root, 9_999)
            .unwrap();
        assert_eq!(early_virtual_runtime.dispatch_native_events().unwrap(), 0);
        assert!(early_virtual_runtime.graph().object_count() < 100);
        assert!(
            early_virtual_runtime
                .adapter()
                .virtual_shell_count(early_virtual_root)
                .unwrap()
                < 100
        );
        early_virtual_runtime
            .adapter()
            .validate_graph(early_virtual_runtime.graph())
            .unwrap();
        early_virtual_window.close().unwrap();

        let mut destroyed_virtual_runtime =
            reactor2::Runtime::new(reactor2::native::WinUiAdapter::default());
        destroyed_virtual_runtime
            .update(reactor2::Grid::new().children([reactor2::keyed(
                "repeater",
                reactor2::ItemsRepeater::new().item("row", reactor2::TextBlock::new().text("row")),
            )]))
            .unwrap();
        let destroyed_virtual_root = destroyed_virtual_runtime.graph().root().unwrap();
        let destroyed_repeater = destroyed_virtual_runtime
            .graph()
            .children(destroyed_virtual_root, reactor2::RelationId::Children)
            .unwrap()[0];
        let destroyed_virtual_window = destroyed_virtual_runtime
            .adapter()
            .open_window(destroyed_virtual_root)
            .unwrap();
        destroyed_virtual_runtime
            .adapter()
            .realize_virtual_item(destroyed_repeater, 0)
            .unwrap();
        assert_eq!(
            destroyed_virtual_runtime.dispatch_native_events().unwrap(),
            0
        );
        destroyed_virtual_runtime
            .update(reactor2::Grid::new())
            .unwrap();
        destroyed_virtual_runtime
            .adapter()
            .validate_graph(destroyed_virtual_runtime.graph())
            .unwrap();
        destroyed_virtual_window.close().unwrap();

        let mut replaced_virtual_runtime =
            reactor2::Runtime::new(reactor2::native::WinUiAdapter::default());
        replaced_virtual_runtime
            .update(reactor2::Grid::new().children([reactor2::keyed(
                "slot",
                reactor2::ItemsRepeater::new().item("row", reactor2::TextBlock::new().text("row")),
            )]))
            .unwrap();
        let replaced_virtual_root = replaced_virtual_runtime.graph().root().unwrap();
        let replaced_repeater = replaced_virtual_runtime
            .graph()
            .children(replaced_virtual_root, reactor2::RelationId::Children)
            .unwrap()[0];
        let replaced_virtual_window = replaced_virtual_runtime
            .adapter()
            .open_window(replaced_virtual_root)
            .unwrap();
        replaced_virtual_runtime
            .adapter()
            .realize_virtual_item(replaced_repeater, 0)
            .unwrap();
        assert_eq!(
            replaced_virtual_runtime.dispatch_native_events().unwrap(),
            0
        );
        replaced_virtual_runtime
            .update(
                reactor2::Grid::new().children([reactor2::keyed("slot", reactor2::Border::new())]),
            )
            .unwrap();
        replaced_virtual_runtime
            .adapter()
            .validate_graph(replaced_virtual_runtime.graph())
            .unwrap();
        replaced_virtual_window.close().unwrap();
        let focused = generated_runtime.focus(&focus_reference).unwrap();
        if !std::env::args().any(|argument| argument == "--headless") {
            assert!(focused, "WinUI rejected generic focus");
        }
        assert_eq!(generated_runtime.dispatch_native_events().unwrap(), 0);
        assert_eq!(slider_changed_count.get(), 0);
        assert_eq!(toggle_changed_count.get(), 0);
        generated_runtime
            .update(
                reactor2::StackPanel::new()
                    .spacing(8.0)
                    .orientation(reactor2::Orientation::Horizontal)
                    .children([
                        reactor2::Slider::new()
                            .minimum(-10.0)
                            .maximum(10.0)
                            .value(6.0)
                            .on_value_changed({
                                let value = Rc::clone(&slider_changed);
                                let count = Rc::clone(&slider_changed_count);
                                move |next| {
                                    value.set(next);
                                    count.set(count.get() + 1);
                                }
                            })
                            .into(),
                        reactor2::ToggleSwitch::new()
                            .is_on(false)
                            .on_toggled({
                                let value = Rc::clone(&toggle_changed);
                                let count = Rc::clone(&toggle_changed_count);
                                move |next| {
                                    value.set(next);
                                    count.set(count.get() + 1);
                                }
                            })
                            .into(),
                        reactor2::CheckBox::new()
                            .is_checked(Some(true))
                            .content(reactor2::TextBlock::new().text("Enabled"))
                            .into(),
                        reactor2::ScrollViewer::new()
                            .content(
                                reactor2::Canvas::new().children([reactor2::TextBlock::new()
                                    .text("Scrollable canvas")
                                    .canvas_left(12.0)
                                    .canvas_top(24.0)
                                    .into()]),
                            )
                            .into(),
                        reactor2::SelectorBar::new()
                            .items([
                                reactor2::keyed(
                                    "first",
                                    reactor2::SelectorBarItem::new().text("First"),
                                ),
                                reactor2::keyed(
                                    "second",
                                    reactor2::SelectorBarItem::new().text("Second"),
                                ),
                            ])
                            .into(),
                        reactor2::AppBarButton::new()
                            .label("Icon")
                            .icon(reactor2::SymbolIcon::new())
                            .into(),
                    ]),
            )
            .unwrap();
        assert_eq!(generated_runtime.dispatch_native_events().unwrap(), 0);
        assert_eq!(slider_changed_count.get(), 0);
        assert_eq!(toggle_changed_count.get(), 0);
        assert_eq!(
            generated_runtime.adapter().slider_state(slider).unwrap().2,
            6.0
        );
        assert!(
            generated_runtime
                .graph()
                .properties(slider)
                .unwrap()
                .contains(&reactor2::Property {
                    id: reactor2::PropertyId::Value,
                    value: reactor2::PropertyValue::F64(6.0),
                })
        );
        assert!(
            !generated_runtime
                .adapter()
                .toggle_switch_state(toggle)
                .unwrap()
        );
        generated_runtime
            .adapter()
            .set_slider_value(slider, 7.5)
            .unwrap();
        generated_runtime
            .adapter()
            .set_toggle_switch_is_on(toggle, true)
            .unwrap();
        assert_eq!(generated_runtime.dispatch_native_events().unwrap(), 2);
        assert_eq!(slider_changed.get(), 7.5);
        assert_eq!(slider_changed_count.get(), 1);
        assert!(toggle_changed.get());
        assert_eq!(toggle_changed_count.get(), 1);
        assert!(
            generated_runtime
                .graph()
                .properties(slider)
                .unwrap()
                .contains(&reactor2::Property {
                    id: reactor2::PropertyId::Value,
                    value: reactor2::PropertyValue::F64(7.5),
                })
        );
        assert!(
            generated_runtime
                .graph()
                .properties(toggle)
                .unwrap()
                .contains(&reactor2::Property {
                    id: reactor2::PropertyId::IsOn,
                    value: reactor2::PropertyValue::Bool(true),
                })
        );
        generated_runtime
            .adapter()
            .validate_graph(generated_runtime.graph())
            .unwrap();
        generated_runtime
            .update(
                reactor2::StackPanel::new().children([
                    reactor2::Slider::new().into(),
                    reactor2::ToggleSwitch::new().into(),
                    reactor2::CheckBox::new()
                        .content(reactor2::TextBlock::new().text("Enabled"))
                        .into(),
                    reactor2::ScrollViewer::new()
                        .content(reactor2::Canvas::new().children([
                            reactor2::TextBlock::new().text("Scrollable canvas").into(),
                        ]))
                        .into(),
                    reactor2::SelectorBar::new()
                        .items([
                            reactor2::keyed(
                                "first",
                                reactor2::SelectorBarItem::new().text("First"),
                            ),
                            reactor2::keyed(
                                "second",
                                reactor2::SelectorBarItem::new().text("Second"),
                            ),
                        ])
                        .into(),
                    reactor2::AppBarButton::new()
                        .label("Icon")
                        .icon(reactor2::SymbolIcon::new())
                        .into(),
                ]),
            )
            .unwrap();
        generated_runtime
            .adapter()
            .validate_graph(generated_runtime.graph())
            .unwrap();
        assert_eq!(
            generated_runtime
                .adapter()
                .stack_panel_state(generated_root)
                .unwrap(),
            (0.0, false)
        );
        assert_eq!(
            generated_runtime.adapter().slider_state(slider).unwrap(),
            (0.0, 100.0, 0.0)
        );
        assert!(
            !generated_runtime
                .adapter()
                .check_box_state(check_box)
                .unwrap()
        );
        let cleared_position = generated_runtime
            .adapter()
            .canvas_position(canvas_child)
            .unwrap();
        assert_eq!(cleared_position, (0.0, 0.0));
        generated_window.close().unwrap();

        let rich_value = Rc::new(RefCell::new(Rc::<str>::from("")));
        let rich_count = Rc::new(Cell::new(0));
        let rich_callback = {
            let value = Rc::clone(&rich_value);
            let count = Rc::clone(&rich_count);
            reactor2::Callback::new(move |next: Rc<str>| {
                *value.borrow_mut() = next;
                count.set(count.get() + 1);
            })
        };
        let mut document_runtime =
            reactor2::Runtime::new(reactor2::native::WinUiAdapter::default());
        document_runtime
            .update(
                reactor2::Grid::new()
                    .rows([reactor2::GridLength::Auto, reactor2::GridLength::STAR])
                    .columns([reactor2::GridLength::Pixel(120.0).min(40.0)])
                    .children([reactor2::keyed(
                        "document",
                        reactor2::RichEditBox::new()
                            .text("first\r\nsecond")
                            .is_read_only(true)
                            .on_text_changed_callback(rich_callback.clone()),
                    )]),
            )
            .unwrap();
        let document_root = document_runtime.graph().root().unwrap();
        let document = document_runtime
            .graph()
            .children(document_root, reactor2::RelationId::Children)
            .unwrap()[0];
        let document_window = document_runtime
            .adapter()
            .open_window(document_root)
            .unwrap();
        assert_eq!(
            document_runtime
                .adapter()
                .grid_definition_counts(document_root)
                .unwrap(),
            (2, 1)
        );
        assert_eq!(
            document_runtime
                .adapter()
                .rich_edit_state(document)
                .unwrap(),
            ("first\nsecond".to_string(), true)
        );
        assert_eq!(document_runtime.dispatch_native_events().unwrap(), 0);
        document_runtime
            .update(
                reactor2::Grid::new()
                    .rows([reactor2::GridLength::Pixel(24.0)])
                    .children([reactor2::keyed(
                        "document",
                        reactor2::RichEditBox::new()
                            .text("application")
                            .is_read_only(true)
                            .on_text_changed_callback(rich_callback.clone()),
                    )]),
            )
            .unwrap();
        assert_eq!(document_runtime.dispatch_native_events().unwrap(), 0);
        assert_eq!(
            document_runtime
                .adapter()
                .grid_definition_counts(document_root)
                .unwrap(),
            (1, 0)
        );
        assert_eq!(
            document_runtime
                .adapter()
                .rich_edit_state(document)
                .unwrap(),
            ("application".to_string(), true)
        );
        document_runtime
            .update(
                reactor2::Grid::new()
                    .rows([reactor2::GridLength::Pixel(24.0)])
                    .children([reactor2::keyed(
                        "document",
                        reactor2::RichEditBox::new()
                            .text("application")
                            .is_read_only(false)
                            .on_text_changed_callback(rich_callback),
                    )]),
            )
            .unwrap();
        assert_eq!(document_runtime.dispatch_native_events().unwrap(), 0);
        document_runtime
            .adapter()
            .simulate_rich_edit_input(document, "native\r\ntext")
            .unwrap();
        assert_eq!(document_runtime.dispatch_native_events().unwrap(), 1);
        assert_eq!(rich_count.get(), 1);
        assert_eq!(rich_value.borrow().as_ref(), "native\ntext");
        let clear_callback = {
            let value = Rc::clone(&rich_value);
            let count = Rc::clone(&rich_count);
            reactor2::Callback::new(move |next| {
                *value.borrow_mut() = next;
                count.set(count.get() + 1);
            })
        };
        let cleared_document = || {
            reactor2::Grid::new()
                .rows([reactor2::GridLength::Pixel(24.0)])
                .children([reactor2::keyed(
                    "document",
                    reactor2::RichEditBox::new()
                        .is_read_only(false)
                        .on_text_changed_callback(clear_callback.clone()),
                )])
        };
        document_runtime.update(cleared_document()).unwrap();
        assert_eq!(document_runtime.dispatch_native_events().unwrap(), 0);
        assert_eq!(rich_count.get(), 1);
        assert_eq!(
            document_runtime
                .adapter()
                .rich_edit_state(document)
                .unwrap()
                .0,
            ""
        );
        assert!(
            document_runtime
                .graph()
                .properties(document)
                .unwrap()
                .iter()
                .all(|property| property.id != reactor2::PropertyId::Document)
        );
        assert!(
            document_runtime
                .update(cleared_document())
                .unwrap()
                .is_empty()
        );
        assert_eq!(document_runtime.dispatch_native_events().unwrap(), 0);
        assert_eq!(rich_count.get(), 1);
        document_window.close().unwrap();

        let virtual_view = |prefix: &'static str| {
            reactor2::ItemsRepeater::new().virtual_source(reactor2::VirtualSource::new(
                1,
                10_000,
                reactor2::Key::from,
                move |index| -> reactor2::Visual {
                    reactor2::TextBlock::new()
                        .text(format!("{prefix} {index}"))
                        .into()
                },
            ))
        };
        let mut virtual_runtime = reactor2::Runtime::new(reactor2::native::WinUiAdapter::default());
        virtual_runtime.update(virtual_view("first")).unwrap();
        let virtual_root = virtual_runtime.graph().root().unwrap();
        let virtual_window = virtual_runtime.adapter().open_window(virtual_root).unwrap();
        virtual_runtime
            .adapter()
            .realize_virtual_item(virtual_root, 9_999)
            .unwrap();
        assert_eq!(virtual_runtime.dispatch_native_events().unwrap(), 0);
        assert!(virtual_runtime.graph().object_count() < 100);
        assert!(
            virtual_runtime
                .adapter()
                .virtual_shell_count(virtual_root)
                .unwrap()
                < 100
        );
        let realized = virtual_runtime
            .graph()
            .children(virtual_root, reactor2::RelationId::Items)
            .unwrap()
            .to_vec();
        assert!(!realized.is_empty());
        virtual_runtime.update(virtual_view("updated")).unwrap();
        assert_eq!(virtual_runtime.dispatch_native_events().unwrap(), 0);
        assert!(
            virtual_runtime
                .graph()
                .children(virtual_root, reactor2::RelationId::Items)
                .unwrap()
                .len()
                <= 1
        );
        virtual_runtime
            .update(reactor2::ItemsRepeater::new())
            .unwrap();
        assert_eq!(virtual_runtime.dispatch_native_events().unwrap(), 0);
        assert_eq!(virtual_runtime.graph().object_count(), 1);
        virtual_runtime.update(virtual_view("again")).unwrap();
        assert_eq!(virtual_runtime.dispatch_native_events().unwrap(), 0);
        virtual_runtime
            .adapter()
            .validate_graph(virtual_runtime.graph())
            .unwrap();
        virtual_window.close().unwrap();

        let password_value = Rc::new(RefCell::new(Rc::<str>::from("")));
        let password_count = Rc::new(Cell::new(0));
        let rating_value = Rc::new(Cell::new(None));
        let rating_count = Rc::new(Cell::new(0));
        let selected_index = Rc::new(Cell::new(None));
        let selected_index_count = Rc::new(Cell::new(0));
        let mut payload_runtime = reactor2::Runtime::new(reactor2::native::WinUiAdapter::default());
        payload_runtime
            .update(
                reactor2::StackPanel::new().children([
                    reactor2::PasswordBox::new()
                        .on_password_changed({
                            let value = Rc::clone(&password_value);
                            let count = Rc::clone(&password_count);
                            move |next| {
                                *value.borrow_mut() = next;
                                count.set(count.get() + 1);
                            }
                        })
                        .into(),
                    reactor2::RatingControl::new()
                        .on_value_changed({
                            let value = Rc::clone(&rating_value);
                            let count = Rc::clone(&rating_count);
                            move |next| {
                                value.set(next);
                                count.set(count.get() + 1);
                            }
                        })
                        .into(),
                    reactor2::ComboBox::new()
                        .items_source(["First", "Second"])
                        .on_selection_changed({
                            let value = Rc::clone(&selected_index);
                            let count = Rc::clone(&selected_index_count);
                            move |next| {
                                value.set(next);
                                count.set(count.get() + 1);
                            }
                        })
                        .into(),
                ]),
            )
            .unwrap();
        let payload_root = payload_runtime.graph().root().unwrap();
        let payload_window = payload_runtime.adapter().open_window(payload_root).unwrap();
        let payload_children = payload_runtime
            .graph()
            .children(payload_root, reactor2::RelationId::Children)
            .unwrap();
        payload_runtime
            .adapter()
            .set_password(payload_children[0], "secret")
            .unwrap();
        payload_runtime
            .adapter()
            .set_rating_value(payload_children[1], 4.0)
            .unwrap();
        payload_runtime
            .adapter()
            .set_combo_box_selected_index(payload_children[2], Some(1))
            .unwrap();
        assert_eq!(payload_runtime.dispatch_native_events().unwrap(), 3);
        assert_eq!(password_value.borrow().as_ref(), "secret");
        assert_eq!(password_count.get(), 1);
        assert_eq!(rating_value.get(), Some(4.0));
        assert_eq!(rating_count.get(), 1);
        assert_eq!(selected_index.get(), Some(1));
        assert_eq!(selected_index_count.get(), 1);
        payload_window.close().unwrap();

        let navigation_value = Rc::new(RefCell::new(None));
        let navigation_history = Rc::new(RefCell::new(Vec::new()));
        let navigation_count = Rc::new(Cell::new(0));
        let list_value = Rc::new(RefCell::new(None));
        let list_count = Rc::new(Cell::new(0));
        let selector_value = Rc::new(RefCell::new(None));
        let selector_count = Rc::new(Cell::new(0));
        let navigation_callback = {
            let value = Rc::clone(&navigation_value);
            let history = Rc::clone(&navigation_history);
            let count = Rc::clone(&navigation_count);
            reactor2::Callback::new(move |next: Option<Rc<str>>| {
                value.borrow_mut().clone_from(&next);
                history.borrow_mut().push(next);
                count.set(count.get() + 1);
            })
        };
        let list_callback = {
            let value = Rc::clone(&list_value);
            let count = Rc::clone(&list_count);
            reactor2::Callback::new(move |next| {
                *value.borrow_mut() = next;
                count.set(count.get() + 1);
            })
        };
        let selector_callback = {
            let value = Rc::clone(&selector_value);
            let count = Rc::clone(&selector_count);
            reactor2::Callback::new(move |next| {
                *value.borrow_mut() = next;
                count.set(count.get() + 1);
            })
        };
        let selections = |reversed: bool, remove_selected: bool| {
            let navigation_items = if remove_selected {
                vec![reactor2::keyed(
                    "first",
                    reactor2::NavigationViewItem::new()
                        .tag("nav-first")
                        .is_selected(false),
                )]
            } else if reversed {
                vec![
                    reactor2::keyed(
                        "second",
                        reactor2::NavigationViewItem::new()
                            .tag("nav-second")
                            .is_selected(true),
                    ),
                    reactor2::keyed(
                        "first",
                        reactor2::NavigationViewItem::new()
                            .tag("nav-first")
                            .is_selected(false),
                    ),
                ]
            } else {
                vec![
                    reactor2::keyed(
                        "first",
                        reactor2::NavigationViewItem::new()
                            .tag("nav-first")
                            .is_selected(true),
                    ),
                    reactor2::keyed(
                        "second",
                        reactor2::NavigationViewItem::new()
                            .tag("nav-second")
                            .is_selected(false),
                    ),
                ]
            };
            let list_items = if remove_selected {
                vec![reactor2::keyed(
                    "first",
                    reactor2::ListBoxItem::new()
                        .tag("list-first")
                        .is_selected(false),
                )]
            } else if reversed {
                vec![
                    reactor2::keyed(
                        "second",
                        reactor2::ListBoxItem::new()
                            .tag("list-second")
                            .is_selected(true),
                    ),
                    reactor2::keyed(
                        "first",
                        reactor2::ListBoxItem::new()
                            .tag("list-first")
                            .is_selected(false),
                    ),
                ]
            } else {
                vec![
                    reactor2::keyed(
                        "first",
                        reactor2::ListBoxItem::new()
                            .tag("list-first")
                            .is_selected(true),
                    ),
                    reactor2::keyed(
                        "second",
                        reactor2::ListBoxItem::new()
                            .tag("list-second")
                            .is_selected(false),
                    ),
                ]
            };
            let selector_items = if remove_selected {
                vec![reactor2::keyed(
                    "first",
                    reactor2::SelectorBarItem::new()
                        .text("selector-first")
                        .is_selected(false),
                )]
            } else if reversed {
                vec![
                    reactor2::keyed(
                        "second",
                        reactor2::SelectorBarItem::new()
                            .text("selector-second")
                            .is_selected(true),
                    ),
                    reactor2::keyed(
                        "first",
                        reactor2::SelectorBarItem::new()
                            .text("selector-first")
                            .is_selected(false),
                    ),
                ]
            } else {
                vec![
                    reactor2::keyed(
                        "first",
                        reactor2::SelectorBarItem::new()
                            .text("selector-first")
                            .is_selected(true),
                    ),
                    reactor2::keyed(
                        "second",
                        reactor2::SelectorBarItem::new()
                            .text("selector-second")
                            .is_selected(false),
                    ),
                ]
            };
            reactor2::StackPanel::new().children([
                reactor2::NavigationView::new()
                    .menu_items(navigation_items)
                    .on_selected_tag_changed_callback(navigation_callback.clone())
                    .into(),
                reactor2::ListBox::new()
                    .items(list_items)
                    .on_selected_tag_changed_callback(list_callback.clone())
                    .into(),
                reactor2::SelectorBar::new()
                    .items(selector_items)
                    .on_selected_text_changed_callback(selector_callback.clone())
                    .into(),
            ])
        };
        let mut selection_runtime =
            reactor2::Runtime::new(reactor2::native::WinUiAdapter::default());
        selection_runtime.update(selections(false, false)).unwrap();
        let selection_root = selection_runtime.graph().root().unwrap();
        let selection_window = selection_runtime
            .adapter()
            .open_window(selection_root)
            .unwrap();
        let selection_owners = selection_runtime
            .graph()
            .children(selection_root, reactor2::RelationId::Children)
            .unwrap()
            .to_vec();
        let navigation_items = selection_runtime
            .graph()
            .children(selection_owners[0], reactor2::RelationId::MenuItems)
            .unwrap()
            .to_vec();
        let list_items = selection_runtime
            .graph()
            .children(selection_owners[1], reactor2::RelationId::Items)
            .unwrap()
            .to_vec();
        let selector_items = selection_runtime
            .graph()
            .children(selection_owners[2], reactor2::RelationId::Items)
            .unwrap()
            .to_vec();
        assert_eq!(selection_runtime.dispatch_native_events().unwrap(), 0);
        assert_eq!(navigation_count.get(), 0);
        assert_eq!(list_count.get(), 0);
        assert_eq!(selector_count.get(), 0);

        for (owner, item) in [
            (selection_owners[0], navigation_items[1]),
            (selection_owners[1], list_items[1]),
            (selection_owners[2], selector_items[1]),
        ] {
            selection_runtime
                .adapter()
                .select_item(owner, Some(item))
                .unwrap();
        }
        assert_eq!(selection_runtime.dispatch_native_events().unwrap(), 3);
        assert_eq!(navigation_value.borrow().as_deref(), Some("nav-second"));
        assert_eq!(list_value.borrow().as_deref(), Some("list-second"));
        assert_eq!(selector_value.borrow().as_deref(), Some("selector-second"));
        assert_eq!(navigation_count.get(), 1);
        assert_eq!(list_count.get(), 1);
        assert_eq!(selector_count.get(), 1);
        assert_eq!(
            navigation_history.borrow().as_slice(),
            [Some(Rc::from("nav-second"))]
        );

        selection_runtime
            .adapter()
            .select_item(selection_owners[0], Some(navigation_items[0]))
            .unwrap();
        selection_runtime
            .adapter()
            .select_item(selection_owners[0], Some(navigation_items[1]))
            .unwrap();
        assert_eq!(selection_runtime.dispatch_native_events().unwrap(), 2);
        assert_eq!(
            navigation_history.borrow().as_slice(),
            [
                Some(Rc::from("nav-second")),
                Some(Rc::from("nav-first")),
                Some(Rc::from("nav-second")),
            ]
        );
        assert_eq!(navigation_count.get(), 3);

        selection_runtime.update(selections(true, false)).unwrap();
        assert_eq!(selection_runtime.dispatch_native_events().unwrap(), 0);
        assert_eq!(
            selection_runtime
                .adapter()
                .selected_item(selection_owners[0])
                .unwrap(),
            Some(navigation_items[1])
        );
        assert_eq!(
            selection_runtime
                .adapter()
                .selected_item(selection_owners[1])
                .unwrap(),
            Some(list_items[1])
        );
        assert_eq!(
            selection_runtime
                .adapter()
                .selected_item(selection_owners[2])
                .unwrap(),
            Some(selector_items[1])
        );

        selection_runtime.update(selections(false, true)).unwrap();
        assert_eq!(selection_runtime.dispatch_native_events().unwrap(), 0);
        for owner in selection_owners {
            assert_eq!(
                selection_runtime.adapter().selected_item(owner).unwrap(),
                None
            );
        }
        assert_eq!(navigation_count.get(), 3);
        assert_eq!(list_count.get(), 1);
        assert_eq!(selector_count.get(), 1);
        selection_runtime
            .adapter()
            .validate_graph(selection_runtime.graph())
            .unwrap();
        selection_window.close().unwrap();

        let retirement_children = |reversed: bool, include_retiring: bool| {
            let mut children = Vec::new();
            let indices: Box<dyn Iterator<Item = usize>> = if reversed {
                Box::new((0..64).rev())
            } else {
                Box::new(0..64)
            };
            for index in indices {
                if include_retiring && index == 32 {
                    children.push(reactor2::keyed(
                        "retiring-owned",
                        reactor2::Button::new()
                            .exit_fade(Duration::from_millis(50))
                            .content(reactor2::TextBlock::new().text("retiring-owned")),
                    ));
                }
                children.push(reactor2::keyed(
                    format!("owned-{index}"),
                    reactor2::TextBlock::new().text(index.to_string()),
                ));
            }
            reactor2::Grid::new().children(children)
        };
        let mut retirement_runtime =
            reactor2::Runtime::new(reactor2::native::WinUiAdapter::default());
        retirement_runtime
            .update(retirement_children(false, true))
            .unwrap();
        let retirement_root = retirement_runtime.graph().root().unwrap();
        let initial_retirement_order = retirement_runtime
            .graph()
            .children(retirement_root, reactor2::RelationId::Children)
            .unwrap()
            .to_vec();
        let retiring_object = initial_retirement_order[32];
        let retirement_expected_order = initial_retirement_order
            .iter()
            .copied()
            .filter(|object| *object != retiring_object)
            .rev()
            .collect::<Vec<_>>();
        let retirement_window = retirement_runtime
            .adapter()
            .create_window(retirement_root)
            .unwrap();
        retirement_window.activate().unwrap();
        retirement_runtime
            .update(retirement_children(true, false))
            .unwrap();
        let mut physical_with_retirement = retirement_expected_order.clone();
        physical_with_retirement.insert(32, retiring_object);
        assert_eq!(
            retirement_runtime
                .adapter()
                .owned_physical_children(retirement_root, reactor2::RelationId::Children)
                .unwrap(),
            physical_with_retirement
        );
        assert_eq!(retirement_runtime.graph().retired_count(), 1);
        assert_eq!(retirement_runtime.adapter().retirement_count(), 1);
        assert!(
            retirement_runtime
                .adapter()
                .contains_object(retiring_object)
        );
        assert_eq!(
            retirement_runtime
                .adapter()
                .opacity(retiring_object)
                .unwrap(),
            0.0
        );

        Self::schedule(context);
        Self {
            app: input.app.clone(),
            boundary_host,
            boundary_sender,
            tree_content_sender,
            tree_sender,
            boundary_window,
            pointer_runtime,
            pointer_window,
            retirement_runtime,
            retirement_window,
            retiring_object,
            retirement_expected_order,
            retirement_verified: false,
            received_pointer,
            pointer_injected: false,
            pointer_waits: 0,
            runtime,
            window,
            iteration: 0,
            input_phase: 0,
            text,
            text_changed,
            text_changed_count,
            replacement_text_changed,
            replacement_text_changed_count,
        }
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
        self.app = input.app.clone();
    }

    fn update(&mut self, _message: Self::Message, context: &ComponentContext<Self>) {
        self.retirement_runtime.dispatch_native_events().unwrap();
        if !self.retirement_verified {
            if self.retirement_runtime.graph().retired_count() == 0 {
                assert_eq!(self.retirement_runtime.adapter().retirement_count(), 0);
                assert!(
                    !self
                        .retirement_runtime
                        .adapter()
                        .contains_object(self.retiring_object)
                );
                self.retirement_runtime
                    .adapter()
                    .validate_graph(self.retirement_runtime.graph())
                    .unwrap();
                assert_eq!(
                    self.retirement_runtime
                        .adapter()
                        .owned_physical_children(
                            self.retirement_runtime.graph().root().unwrap(),
                            reactor2::RelationId::Children,
                        )
                        .unwrap(),
                    self.retirement_expected_order
                );
                self.retirement_window.close().unwrap();
                self.retirement_verified = true;
            } else {
                assert!(
                    self.retirement_runtime
                        .adapter()
                        .contains_object(self.retiring_object)
                );
                Self::schedule(context);
                return;
            }
        }
        if !self.pointer_injected {
            Self::inject_pointer_click().unwrap();
            self.pointer_injected = true;
            Self::schedule(context);
            return;
        }
        self.pointer_runtime.dispatch_native_events().unwrap();
        if let Some(pointer) = *self.received_pointer.borrow() {
            assert!(pointer.pointer_id > 0);
            assert!(!pointer.is_captured);
            assert_eq!(pointer.capture_succeeded, None);
        } else {
            self.pointer_waits += 1;
            assert!(
                self.pointer_waits < 50,
                "real WinUI PointerReleased event was not delivered"
            );
            Self::schedule(context);
            return;
        }
        self.boundary_host.drain(usize::MAX).unwrap();
        self.boundary_host
            .runtime()
            .adapter()
            .validate_graph(self.boundary_host.runtime().graph())
            .unwrap();
        self.runtime.dispatch_native_events().unwrap();
        self.iteration += 1;
        let root = self.runtime.graph().root().unwrap();
        let text_box = self
            .runtime
            .graph()
            .children(root, reactor2::RelationId::Children)
            .unwrap()[0];
        match self.input_phase {
            0 => {
                self.runtime
                    .adapter()
                    .simulate_text_input(text_box, "Native input", 3, 2)
                    .unwrap();
                self.input_phase = 1;
                Self::schedule(context);
                return;
            }
            1 if self.text_changed_count.get() == 0 => {
                Self::schedule(context);
                return;
            }
            1 => {
                assert_eq!(self.text_changed_count.get(), 1);
                assert_eq!(self.text.borrow().as_ref(), "Native input");
                self.input_phase = 2;
            }
            2 => {
                *self.text.borrow_mut() = Rc::from("X");
                self.input_phase = 3;
            }
            3 => {
                assert_eq!(self.text_changed_count.get(), 1);
                assert_eq!(self.text.borrow().as_ref(), "X");
                self.runtime
                    .adapter()
                    .simulate_text_input(text_box, "Stale input", 2, 0)
                    .unwrap();
                self.input_phase = 4;
            }
            5 => {
                assert_eq!(self.text_changed_count.get(), 1);
                assert_eq!(self.replacement_text_changed_count.get(), 1);
                assert_eq!(self.text.borrow().as_ref(), "Replacement input");
                self.input_phase = 6;
            }
            _ => {}
        }
        let text_changed = if self.input_phase >= 4 {
            self.replacement_text_changed.clone()
        } else {
            self.text_changed.clone()
        };
        self.runtime
            .update(Self::declaration(
                self.iteration,
                Rc::clone(&self.text.borrow()),
                Some(text_changed),
            ))
            .unwrap();
        if self.input_phase == 2 {
            assert_eq!(
                self.runtime.adapter().text_box_state(text_box).unwrap(),
                ("Native input".to_string(), 3, 2)
            );
            assert_eq!(self.text_changed_count.get(), 1);
        } else if self.input_phase == 3 {
            assert_eq!(
                self.runtime.adapter().text_box_state(text_box).unwrap(),
                ("X".to_string(), 1, 0)
            );
            assert_eq!(self.text_changed_count.get(), 1);
        } else if self.input_phase == 4 {
            self.runtime
                .adapter()
                .simulate_text_input(text_box, "Replacement input", 5, 0)
                .unwrap();
            self.input_phase = 5;
            Self::schedule(context);
            return;
        }
        self.runtime
            .adapter()
            .validate_graph(self.runtime.graph())
            .unwrap();
        if self.iteration == 100 {
            self.pointer_window.close().unwrap();
            self.boundary_window.close().unwrap();
            self.window.close().unwrap();
            self.app.exit().unwrap();
        } else {
            assert!(self.boundary_sender.send(()));
            assert!(self.tree_sender.send(()));
            assert!(self.tree_content_sender.send(()));
            Self::schedule(context);
        }
    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        TextBlock::new()
            .text(format!("Reactor2 TreeView stress: {}", self.iteration))
            .into()
    }
}

fn main() -> windows_core::Result<()> {
    std::thread::spawn(|| {
        std::thread::sleep(Duration::from_secs(15));
        eprintln!("Reactor2 native self-test timed out");
        std::process::exit(1);
    });
    App::run_with(|app: &AppContext| {
        app.open_window(View::component::<Fixture>(Input { app: app.proxy() }))
    })
}
