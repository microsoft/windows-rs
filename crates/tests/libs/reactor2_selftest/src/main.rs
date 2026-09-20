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
    received_pointer: Rc<RefCell<Option<reactor2::PointerEventInfo>>>,
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
                .content(reactor2::TextBlock::new("Border root"))
                .into()
        } else {
            reactor2::TextBlock::new("Text root").into()
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
                reactor2::TextBlock::new(input.clone()).into(),
                reactor2::TextBlock::new(self.0.to_string()).into(),
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
                .content(reactor2::TextBlock::new("Nested content")),
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
                    reactor2::TextBlock::new("Folder").into(),
                    reactor2::TextBlock::new(format!("{} changes", iteration % 7)).into(),
                ]))
                .children(children),
            reactor2::TreeNode::new("second", format!("Second {iteration}"))
                .content(reactor2::TextBlock::new("Leaf")),
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
                    .content(reactor2::TextBlock::new(format!("Iteration {iteration}")))
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
        runtime.adapter_mut().set_event_waker(move || {
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
            .simulate_text_input(replacement_child, "Stale", 0, 0)
            .unwrap();
        replacement_runtime
            .update_subtree(replacement_child, reactor2::Border::new())
            .unwrap();
        let mut stale_events = Vec::new();
        replacement_runtime.drain_events(&mut stale_events).unwrap();
        assert!(stale_events.is_empty());
        assert_eq!(stale_calls.get(), 0);
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
                    .content(reactor2::TextBlock::new("Click"))
                    .on_click(move || {
                        button_callback_calls.set(button_callback_calls.get() + 1);
                    }),
            )
            .unwrap();
        let button = button_runtime.graph().root().unwrap();
        let button_window = button_runtime.adapter().open_window(button).unwrap();
        button_runtime.adapter().simulate_click(button).unwrap();
        let mut button_events = Vec::new();
        button_runtime.drain_events(&mut button_events).unwrap();
        for event in button_events {
            event.invoke();
        }
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
        let mut pointer_events = Vec::new();
        pointer_runtime.drain_events(&mut pointer_events).unwrap();
        assert!(pointer_events.is_empty());
        assert_eq!(stale_pointer_calls.get(), 0);
        pointer_runtime
            .adapter()
            .simulate_pointer_released(pointer_border, pointer_value)
            .unwrap();
        pointer_runtime.drain_events(&mut pointer_events).unwrap();
        for event in pointer_events {
            event.invoke();
        }
        assert_eq!(*received_pointer.borrow(), Some(pointer_value));
        *received_pointer.borrow_mut() = None;
        pointer_runtime
            .adapter()
            .validate_graph(pointer_runtime.graph())
            .unwrap();
        Self::inject_pointer_click().unwrap();
        let mut generated_runtime =
            reactor2::Runtime::new(reactor2::native::WinUiAdapter::default());
        let slider_changed = Rc::new(Cell::new(0.0));
        let slider_changed_callback = Rc::clone(&slider_changed);
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
                            .on_value_changed(move |value| slider_changed_callback.set(value))
                            .into(),
                        reactor2::CheckBox::new()
                            .is_checked(Some(true))
                            .content(reactor2::TextBlock::new("Enabled"))
                            .into(),
                        reactor2::ScrollViewer::new()
                            .content(
                                reactor2::Canvas::new().children([reactor2::TextBlock::new(
                                    "Scrollable canvas",
                                )
                                .canvas_left(12.0)
                                .canvas_top(24.0)
                                .into()]),
                            )
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
        let check_box = generated_children[1];
        let scroll_viewer = generated_children[2];
        let canvas = generated_runtime
            .graph()
            .child(scroll_viewer, reactor2::RelationId::Content)
            .unwrap();
        let canvas_child = generated_runtime
            .graph()
            .children(canvas, reactor2::RelationId::Children)
            .unwrap()[0];
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
        assert_eq!(
            generated_runtime
                .adapter()
                .canvas_position(canvas_child)
                .unwrap(),
            (12.0, 24.0)
        );
        generated_runtime
            .adapter()
            .set_slider_value(slider, 7.5)
            .unwrap();
        let mut generated_events = Vec::new();
        generated_runtime
            .drain_events(&mut generated_events)
            .unwrap();
        for event in generated_events {
            event.invoke();
        }
        assert_eq!(slider_changed.get(), 7.5);
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
        generated_runtime
            .adapter()
            .validate_graph(generated_runtime.graph())
            .unwrap();
        generated_runtime
            .update(
                reactor2::StackPanel::new().children([
                    reactor2::Slider::new().into(),
                    reactor2::CheckBox::new()
                        .content(reactor2::TextBlock::new("Enabled"))
                        .into(),
                    reactor2::ScrollViewer::new()
                        .content(
                            reactor2::Canvas::new()
                                .children([reactor2::TextBlock::new("Scrollable canvas").into()]),
                        )
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
            received_pointer,
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
        let mut pointer_events = Vec::new();
        self.pointer_runtime
            .drain_events(&mut pointer_events)
            .unwrap();
        for event in pointer_events {
            event.invoke();
        }
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
        let mut events = Vec::new();
        self.runtime.drain_events(&mut events).unwrap();
        for event in events {
            event.invoke();
        }
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
