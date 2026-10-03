#![windows_subsystem = "console"]

mod generated_coverage;

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Duration;
use windows::UI::Input::Preview::Injection::{
    InjectedInputMouseInfo, InjectedInputMouseOptions, InputInjector,
};
use windows::Win32::winuser::{
    BringWindowToTop, ClientToScreen, GetClientRect, GetCursorPos, GetForegroundWindow,
    GetSystemMetrics, SM_CXVIRTUALSCREEN, SM_CYVIRTUALSCREEN, SM_XVIRTUALSCREEN, SM_YVIRTUALSCREEN,
    SetForegroundWindow,
};
use windows::Win32::{POINT, RECT};
use windows_collections::IIterable;
use windows_reactor as reactor;
use windows_reactor::{
    App, CommandBarFlyoutExt as _, ContentDialogExt as _, FlyoutExt as _, MenuExt as _,
    TooltipExt as _,
};

const ENCODED_IMAGE_PNG: &[u8] = &[
    0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48, 0x44, 0x52,
    0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x06, 0x00, 0x00, 0x00, 0x1f, 0x15, 0xc4,
    0x89, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x44, 0x41, 0x54, 0x08, 0xd7, 0x63, 0xf8, 0xcf, 0xc0, 0xf0,
    0x1f, 0x00, 0x05, 0x00, 0x01, 0xff, 0x89, 0x99, 0x3d, 0x1d, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45,
    0x4e, 0x44, 0xae, 0x42, 0x60, 0x82,
];
const LIVE_EVENT_TIMEOUT_TICKS: usize = 500;
const LIVE_QUIET_TICKS: usize = 50;

struct Fixture {
    boundary_host: reactor::ComponentHost<reactor::native::WinUiAdapter>,
    boundary_sender: reactor::ComponentSender<()>,
    tree_content_sender: reactor::ComponentSender<()>,
    tree_sender: reactor::ComponentSender<()>,
    boundary_window: reactor::native::NativeWindow,
    pointer_runtime: reactor::Runtime<reactor::native::WinUiAdapter>,
    pointer_window: Option<reactor::native::NativeWindow>,
    retirement_runtime: reactor::Runtime<reactor::native::WinUiAdapter>,
    retirement_window: reactor::native::NativeWindow,
    retiring_object: reactor::ObjectId,
    retirement_expected_order: Vec<reactor::ObjectId>,
    retirement_started: bool,
    retirement_verified: bool,
    content_dialog_runtime: Option<reactor::Runtime<reactor::native::WinUiAdapter>>,
    content_dialog_window: Option<reactor::native::NativeWindow>,
    content_dialog: Option<reactor::ObjectId>,
    content_dialog_result: Rc<Cell<Option<reactor::ContentDialogResult>>>,
    content_dialog_hide_requested: bool,
    content_dialog_verified: bool,
    content_dialog_waits: usize,
    flyout_activated: bool,
    flyout_shown: bool,
    flyout_runtime: Option<reactor::Runtime<reactor::native::WinUiAdapter>>,
    flyout_window: Option<reactor::native::NativeWindow>,
    menu_activated: bool,
    menu_shown: bool,
    menu_runtime: Option<reactor::Runtime<reactor::native::WinUiAdapter>>,
    menu_window: Option<reactor::native::NativeWindow>,
    command_bar_flyout_activated: bool,
    command_bar_flyout_shown: bool,
    command_bar_flyout_runtime: Option<reactor::Runtime<reactor::native::WinUiAdapter>>,
    command_bar_flyout_window: Option<reactor::native::NativeWindow>,
    popup_waits: usize,
    encoded_image_activated: bool,
    encoded_image_cleared: bool,
    encoded_image_result: Rc<Cell<Option<bool>>>,
    encoded_image_runtime: Option<reactor::Runtime<reactor::native::WinUiAdapter>>,
    encoded_image_validated: bool,
    encoded_image_waits: usize,
    encoded_image_window: Option<reactor::native::NativeWindow>,
    received_pointer_moved: Rc<Cell<bool>>,
    received_pointer_pressed: Rc<Cell<bool>>,
    received_pointer: Rc<RefCell<Option<reactor::PointerEventInfo>>>,
    pointer_activated: bool,
    pointer_injection_stage: u8,
    pointer_injection_waits: usize,
    pointer_waits: usize,
    runtime: reactor::Runtime<reactor::native::WinUiAdapter>,
    window: reactor::native::NativeWindow,
    iteration: usize,
    input_phase: u8,
    text: Rc<RefCell<Rc<str>>>,
    text_changed: reactor::Callback<Rc<str>>,
    text_changed_count: Rc<Cell<usize>>,
    replacement_text_changed: reactor::Callback<Rc<str>>,
    replacement_text_changed_count: Rc<Cell<usize>>,
}

struct RootSwitch(bool);

impl reactor::Component for RootSwitch {
    type Input = ();
    type Message = ();

    fn create(_input: &Self::Input, _context: &reactor::ComponentContext<Self>) -> Self {
        Self(false)
    }

    fn update(&mut self, _message: Self::Message, _context: &reactor::ComponentContext<Self>) {
        self.0 = !self.0;
    }

    fn view(
        &self,
        _input: &Self::Input,
        _context: &mut reactor::ViewContext<Self>,
    ) -> reactor::View {
        if self.0 {
            reactor::Border::new()
                .content(reactor::TextBlock::new().text("Border root"))
                .into()
        } else {
            reactor::TextBlock::new().text("Text root").into()
        }
    }
}

struct TreeContent(usize);

impl reactor::Component for TreeContent {
    type Input = Rc<str>;
    type Message = ();

    fn create(_input: &Self::Input, _context: &reactor::ComponentContext<Self>) -> Self {
        Self(0)
    }

    fn update(&mut self, _message: Self::Message, _context: &reactor::ComponentContext<Self>) {
        self.0 += 1;
    }

    fn view(
        &self,
        input: &Self::Input,
        _context: &mut reactor::ViewContext<Self>,
    ) -> reactor::View {
        reactor::StackPanel::new()
            .children([
                reactor::TextBlock::new().text(input.clone()).into(),
                reactor::TextBlock::new().text(self.0.to_string()).into(),
            ])
            .into()
    }
}

struct TreeComponents(bool);

impl reactor::Component for TreeComponents {
    type Input = ();
    type Message = ();

    fn create(_input: &Self::Input, _context: &reactor::ComponentContext<Self>) -> Self {
        Self(false)
    }

    fn update(&mut self, _message: Self::Message, _context: &reactor::ComponentContext<Self>) {
        self.0 = !self.0;
    }

    fn view(
        &self,
        _input: &Self::Input,
        _context: &mut reactor::ViewContext<Self>,
    ) -> reactor::View {
        let first = reactor::TreeNode::new("first", "First")
            .expanded(true)
            .content(reactor::component::<TreeContent>(
                "first-content",
                Rc::from("First"),
            ));
        let second = reactor::TreeNode::new("second", "Second")
            .expanded(true)
            .content(reactor::component::<TreeContent>(
                "second-content",
                Rc::from("Second"),
            ));
        if self.0 {
            reactor::TreeView::new().nodes([second, first]).into()
        } else {
            reactor::TreeView::new().nodes([first, second]).into()
        }
    }
}

impl Fixture {
    fn verify_generated_coverage() {
        for case in generated_coverage::cases() {
            let mut runtime = reactor::Runtime::new(reactor::native::WinUiAdapter::default());
            runtime
                .update((case.set)())
                .unwrap_or_else(|error| panic!("{} set failed: {error:?}", case.contract));
            runtime
                .dispatch_native_events()
                .unwrap_or_else(|error| panic!("{} set dispatch failed: {error:?}", case.contract));
            runtime
                .update((case.clear)())
                .unwrap_or_else(|error| panic!("{} clear failed: {error:?}", case.contract));
            runtime.dispatch_native_events().unwrap_or_else(|error| {
                panic!("{} clear dispatch failed: {error:?}", case.contract)
            });
            runtime
                .adapter()
                .validate_graph(runtime.graph())
                .unwrap_or_else(|error| {
                    panic!("{} graph validation failed: {error:?}", case.contract)
                });
        }
    }

    fn inject_pointer(hwnd: *mut core::ffi::c_void, stage: u8) -> Result<(), String> {
        let hwnd: windows::Win32::HWND = hwnd.cast();
        unsafe {
            _ = SetForegroundWindow(hwnd);
            _ = BringWindowToTop(hwnd);
        }
        let mut point = POINT::default();
        if !unsafe { GetCursorPos(&mut point) }.as_bool() {
            return Err("could not read the current pointer position".to_string());
        }
        if stage == 0 {
            let mut rect = RECT::default();
            if !unsafe { GetClientRect(hwnd, &mut rect) }.as_bool() {
                return Err("could not read the Reactor pointer self-test client area".to_string());
            }
            let mut center = POINT {
                x: (rect.left + rect.right) / 2,
                y: (rect.top + rect.bottom) / 2,
            };
            if !unsafe { ClientToScreen(hwnd, &mut center) }.as_bool() {
                return Err(
                    "could not locate the Reactor pointer self-test client area".to_string()
                );
            }
            let mut target = POINT {
                x: if point.x <= center.x {
                    rect.left + (rect.right - rect.left) * 3 / 4
                } else {
                    rect.left + (rect.right - rect.left) / 4
                },
                y: if point.y <= center.y {
                    rect.top + (rect.bottom - rect.top) * 3 / 4
                } else {
                    rect.top + (rect.bottom - rect.top) / 4
                },
            };
            if !unsafe { ClientToScreen(hwnd, &mut target) }.as_bool() {
                return Err("could not locate the Reactor pointer self-test target".to_string());
            }
            point = target;
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
        let x = ((f64::from(point.x - origin_x) * 65535.0) / f64::from(width - 1)).round() as i32;
        let y = ((f64::from(point.y - origin_y) * 65535.0) / f64::from(height - 1)).round() as i32;
        let options = match stage {
            0 => InjectedInputMouseOptions::Move | InjectedInputMouseOptions::MoveNoCoalesce,
            1 => InjectedInputMouseOptions::LeftDown,
            2 => InjectedInputMouseOptions::LeftUp,
            _ => return Err(format!("invalid pointer injection stage {stage}")),
        };
        let info = InjectedInputMouseInfo::new().map_err(|error| error.to_string())?;
        info.SetDeltaX(x).map_err(|error| error.to_string())?;
        info.SetDeltaY(y).map_err(|error| error.to_string())?;
        info.SetMouseOptions(
            InjectedInputMouseOptions::Absolute | InjectedInputMouseOptions::VirtualDesk | options,
        )
        .map_err(|error| error.to_string())?;
        let inputs: IIterable<InjectedInputMouseInfo> = vec![Some(info)].into();
        injector
            .InjectMouseInput(&inputs)
            .map_err(|error| error.to_string())?;
        Ok(())
    }

    fn declaration(
        iteration: usize,
        text: Rc<str>,
        text_changed: Option<reactor::Callback<Rc<str>>>,
    ) -> reactor::View {
        let children = [
            reactor::TreeNode::new("first-child", format!("First child {iteration}"))
                .content(reactor::TextBlock::new().text("Nested content")),
            reactor::TreeNode::new("second-child", "Second child"),
        ];
        let children = if iteration.is_multiple_of(2) {
            children
        } else {
            [children[1].clone(), children[0].clone()]
        };
        let roots = [
            reactor::TreeNode::new("first", format!("First {iteration}"))
                .expanded(true)
                .content(reactor::StackPanel::new().children(vec![
                    reactor::TextBlock::new().text("Folder").into(),
                    reactor::TextBlock::new().text(format!("{} changes", iteration % 7)).into(),
                ]))
                .children(children),
            reactor::TreeNode::new("second", format!("Second {iteration}"))
                .content(reactor::TextBlock::new().text("Leaf")),
        ];
        let roots = if iteration.is_multiple_of(2) {
            reactor::TreeView::new().nodes(roots)
        } else {
            reactor::TreeView::new().nodes([roots[1].clone(), roots[0].clone()])
        };
        let items = if iteration.is_multiple_of(2) {
            [
                reactor::DataItem::new("first", format!("First item {iteration}")),
                reactor::DataItem::new("second", "Second item"),
            ]
        } else {
            [
                reactor::DataItem::new("second", "Second item"),
                reactor::DataItem::new("first", format!("First item {iteration}")),
            ]
        };
        let text_box = reactor::TextBox::new(text);
        let text_box = if let Some(text_changed) = text_changed {
            text_box.on_text_changed(text_changed)
        } else {
            text_box
        };
        let text_box = if iteration.is_multiple_of(2) {
            text_box
                .placeholder_text("Enter text")
                .accepts_return(true)
                .text_wrapping(reactor::TextWrapping::Wrap)
        } else {
            text_box
        };
        reactor::StackPanel::new()
            .children(vec![
                text_box.into(),
                reactor::Border::new()
                    .transitions_optional(
                        iteration
                            .is_multiple_of(2)
                            .then_some([reactor::ThemeTransition::Reposition]),
                    )
                    .background(reactor::ThemeBrush::CardBackground)
                    .border_brush(reactor::ThemeBrush::CardStroke)
                    .opacity_transition(Duration::from_millis(80))
                    .scale(if iteration.is_multiple_of(2) {
                        1.0
                    } else {
                        0.98
                    })
                    .scale_transition(Duration::from_millis(80))
                    .content(reactor::TextBlock::new().text(format!("Iteration {iteration}")))
                    .into(),
                reactor::Button::new()
                    .background(reactor::ThemeBrush::Accent)
                    .style(reactor::ButtonStyle::Subtle)
                    .content(
                        reactor::TextBlock::new()
                            .text("Shared style")
                            .foreground(reactor::ThemeBrush::PrimaryText),
                    )
                    .into(),
                reactor::Button::new()
                    .resource_overrides(
                        reactor::ResourceOverrides::new()
                            .set("ControlCornerRadius", reactor::CornerRadius::uniform(6.0)),
                    )
                    .content("Resource override")
                    .into(),
                reactor::HyperlinkButton::new()
                    .navigate_uri("https://example.com")
                    .unwrap()
                    .content("Navigate")
                    .into(),
                reactor::Grid::new()
                    .background(reactor::ThemeBrush::SolidBackground)
                    .keyed_children([
                        reactor::keyed(
                            "rectangle",
                            reactor::Rectangle::new()
                                .fill(reactor::ThemeBrush::Accent)
                                .stroke(reactor::Color::rgb(255, 255, 255)),
                        ),
                        reactor::keyed(
                            "path",
                            reactor::PathIcon::new().data("M 0,0 L 8,0 8,8 0,8 Z"),
                        ),
                        reactor::keyed("bitmap", reactor::BitmapIcon::new()),
                    ])
                    .into(),
                roots.into(),
                reactor::ListView::new().items(items).into(),
            ])
            .into()
    }

    fn schedule(context: &reactor::ComponentContext<Self>) {
        context.spawn_background(|_| {
            std::thread::sleep(Duration::from_millis(20));
        });
    }

    fn retirement_children(reversed: bool, include_retiring: bool) -> reactor::Grid {
        let mut children = Vec::new();
        let indices: Box<dyn Iterator<Item = usize>> = if reversed {
            Box::new((0..64).rev())
        } else {
            Box::new(0..64)
        };
        for index in indices {
            if include_retiring && index == 32 {
                children.push(reactor::keyed(
                    "retiring-owned",
                    reactor::Button::new()
                        .exit_fade(Duration::from_millis(50))
                        .content(reactor::TextBlock::new().text("retiring-owned")),
                ));
            }
            children.push(reactor::keyed(
                format!("owned-{index}"),
                reactor::TextBlock::new().text(index.to_string()),
            ));
        }
        reactor::Grid::new().keyed_children(children)
    }
}

impl reactor::Component for Fixture {
    type Input = ();
    type Message = ();

    fn create(_input: &Self::Input, context: &reactor::ComponentContext<Self>) -> Self {
        Self::verify_generated_coverage();
        let text = Rc::new(RefCell::new(Rc::<str>::from("Initial text")));
        let text_changed_count = Rc::new(Cell::new(0));
        let text_for_callback = Rc::clone(&text);
        let count_for_callback = Rc::clone(&text_changed_count);
        let text_changed = reactor::Callback::new(move |value| {
            *text_for_callback.borrow_mut() = value;
            count_for_callback.set(count_for_callback.get() + 1);
        });
        let replacement_text_changed_count = Rc::new(Cell::new(0));
        let replacement_text_for_callback = Rc::clone(&text);
        let replacement_count_for_callback = Rc::clone(&replacement_text_changed_count);
        let replacement_text_changed = reactor::Callback::new(move |value| {
            *replacement_text_for_callback.borrow_mut() = value;
            replacement_count_for_callback.set(replacement_count_for_callback.get() + 1);
        });
        let mut runtime = reactor::Runtime::new(reactor::native::WinUiAdapter::default());
        let sender = context.sender();
        runtime.set_native_event_waker(move || {
            _ = sender.send(());
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
        let grid_reference = reactor::ElementRef::<reactor::Grid>::new();
        let image_reference = reactor::ElementRef::<reactor::Image>::new();
        let webview_reference = reactor::ElementRef::<reactor::WebView2>::new();
        let surface_reference = reactor::ElementRef::<reactor::SwapChainPanel>::new();
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
        let mut reference_runtime = reactor::Runtime::new(reactor::native::WinUiAdapter::default());
        reference_runtime
            .update(
                reactor::Grid::new()
                    .element_ref(&grid_reference)
                    .keyed_children([
                        reactor::keyed(
                            "image",
                            reactor::Image::new().element_ref(&image_reference),
                        ),
                        reactor::keyed(
                            "webview",
                            reactor::WebView2::new().element_ref(&webview_reference),
                        ),
                        reactor::keyed(
                            "surface",
                            reactor::SwapChainPanel::new().element_ref(&surface_reference),
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
        let title_bar_requests = Rc::new(Cell::new(0));
        let back_requests = Rc::clone(&title_bar_requests);
        let pane_requests = Rc::clone(&title_bar_requests);
        let mut title_bar_runtime = reactor::Runtime::new(reactor::native::WinUiAdapter::default());
        title_bar_runtime.update(reactor::Grid::new()).unwrap();
        let title_bar_root = title_bar_runtime.graph().root().unwrap();
        let title_bar_window = title_bar_runtime
            .adapter()
            .open_window(title_bar_root)
            .unwrap();
        assert!(matches!(
            title_bar_runtime.adapter().create_window(title_bar_root),
            Err(reactor::native::WinUiError::DuplicateWindowRoot(root)) if root == title_bar_root
        ));
        let title_bar_icons = [
            reactor::Icon::symbol(reactor::Symbol::Home),
            reactor::Icon::font("\u{E721}"),
            reactor::Icon::bitmap("ms-appx:///Assets/icon.png", true).unwrap(),
            reactor::Icon::image_uri("ms-appx:///Assets/icon.svg").unwrap(),
            reactor::Icon::image_data(reactor::EncodedImage::from_static(ENCODED_IMAGE_PNG)),
            reactor::Icon::path("M 0,0 L 8,8"),
        ];
        for (index, icon) in title_bar_icons.into_iter().enumerate() {
            let title_bar = reactor::TitleBar::new()
                .title("Reactor title-bar self-test")
                .icon(icon);
            let title_bar = if index == 0 {
                title_bar
                    .subtitle("Coverage")
                    .left_header(reactor::TextBlock::new().text("Left header"))
                    .is_back_button_visible(true)
                    .is_back_button_enabled(true)
                    .is_pane_toggle_button_visible(true)
                    .on_back_requested({
                        let back_requests = Rc::clone(&back_requests);
                        move || {
                            back_requests.set(back_requests.get() + 1);
                        }
                    })
                    .on_pane_toggle_requested({
                        let pane_requests = Rc::clone(&pane_requests);
                        move || {
                            pane_requests.set(pane_requests.get() + 1);
                        }
                    })
                    .preferred_height(reactor::WindowTitleBarHeight::Tall)
            } else {
                title_bar
            };
            title_bar_runtime
                .update(reactor::Grid::new().keyed_children([reactor::keyed("title", title_bar)]))
                .unwrap();
        }
        title_bar_runtime
            .update(
                reactor::Grid::new().keyed_children([reactor::keyed(
                    "title",
                    reactor::TitleBar::new()
                        .title("Reactor title-bar self-test")
                        .preferred_height(reactor::WindowTitleBarHeight::Standard),
                )]),
            )
            .unwrap();
        assert_eq!(title_bar_requests.get(), 0);
        title_bar_runtime.update(reactor::Grid::new()).unwrap();
        title_bar_window.close().unwrap();
        let boundary_host = reactor::ComponentHost::mount(
            reactor::native::WinUiAdapter::default(),
            [
                reactor::component::<RootSwitch>("switch", ()),
                reactor::component::<TreeComponents>("tree", ()),
            ],
        )
        .unwrap();
        let boundary_sender = boundary_host
            .sender::<RootSwitch>(&reactor::Key::from("switch"))
            .unwrap();
        let tree_sender = boundary_host
            .sender::<TreeComponents>(&reactor::Key::from("tree"))
            .unwrap();
        let tree_content_sender = boundary_host
            .sender_at::<TreeContent>(&[
                reactor::Key::from("tree"),
                reactor::Key::from("first-content"),
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
            reactor::Runtime::new(reactor::native::WinUiAdapter::default());
        replacement_runtime
            .update(reactor::Grid::new().keyed_children([reactor::keyed(
                "replace",
                reactor::TextBox::new("Old").on_text_changed(move |_| {
                    stale_callback_calls.set(stale_callback_calls.get() + 1);
                }),
            )]))
            .unwrap();
        let replacement_root = replacement_runtime.graph().root().unwrap();
        let replacement_child = replacement_runtime
            .graph()
            .children(replacement_root, reactor::RelationId::Children)
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
            .update_subtree(replacement_child, reactor::Border::new())
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
        let mut button_runtime = reactor::Runtime::new(reactor::native::WinUiAdapter::default());
        button_runtime
            .update(
                reactor::Button::new()
                    .content(reactor::TextBlock::new().text("Click"))
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
        let controlled_calls = Rc::new(Cell::new(0));
        let checked_value = Rc::new(Cell::new(None));
        let toggle_value = Rc::new(Cell::new(None));
        let expanded_value = Rc::new(Cell::new(true));
        let pane_value = Rc::new(Cell::new(true));
        let checked_callback = {
            let calls = Rc::clone(&controlled_calls);
            let value = Rc::clone(&checked_value);
            reactor::Callback::new(move |current| {
                calls.set(calls.get() + 1);
                value.set(current);
            })
        };
        let toggle_callback = {
            let calls = Rc::clone(&controlled_calls);
            let value = Rc::clone(&toggle_value);
            reactor::Callback::new(move |current| {
                calls.set(calls.get() + 1);
                value.set(current);
            })
        };
        let expanded_callback = {
            let calls = Rc::clone(&controlled_calls);
            let value = Rc::clone(&expanded_value);
            reactor::Callback::new(move |current| {
                calls.set(calls.get() + 1);
                value.set(current);
            })
        };
        let pane_callback = {
            let calls = Rc::clone(&controlled_calls);
            let value = Rc::clone(&pane_value);
            reactor::Callback::new(move |current| {
                calls.set(calls.get() + 1);
                value.set(current);
            })
        };
        let controlled = |current| {
            reactor::StackPanel::new().children([
                reactor::CheckBox::new()
                    .is_checked(Some(current))
                    .on_is_checked_changed(checked_callback.clone())
                    .into(),
                reactor::ToggleButton::new()
                    .is_checked(Some(current))
                    .on_is_checked_changed(toggle_callback.clone())
                    .into(),
                reactor::Expander::new()
                    .is_expanded(current)
                    .on_is_expanded_changed(expanded_callback.clone())
                    .into(),
                reactor::NavigationView::new()
                    .is_pane_open(current)
                    .on_is_pane_open_changed(pane_callback.clone())
                    .into(),
            ])
        };
        let mut controlled_runtime =
            reactor::Runtime::new(reactor::native::WinUiAdapter::default());
        controlled_runtime.update(controlled(false)).unwrap();
        let controlled_root = controlled_runtime.graph().root().unwrap();
        let controlled_objects = controlled_runtime
            .graph()
            .children(controlled_root, reactor::RelationId::Children)
            .unwrap()
            .to_vec();
        controlled_runtime.update(controlled(true)).unwrap();
        assert_eq!(controlled_runtime.dispatch_native_events().unwrap(), 0);
        assert_eq!(controlled_calls.get(), 0);
        controlled_runtime
            .adapter()
            .simulate_is_checked(controlled_objects[0], Some(false))
            .unwrap();
        controlled_runtime
            .adapter()
            .simulate_is_checked(controlled_objects[1], Some(false))
            .unwrap();
        controlled_runtime
            .adapter()
            .simulate_is_expanded(controlled_objects[2], false)
            .unwrap();
        controlled_runtime
            .adapter()
            .simulate_is_pane_open(controlled_objects[3], false)
            .unwrap();
        assert_eq!(controlled_runtime.dispatch_native_events().unwrap(), 4);
        assert_eq!(controlled_calls.get(), 4);
        assert_eq!(checked_value.get(), Some(false));
        assert_eq!(toggle_value.get(), Some(false));
        assert!(!expanded_value.get());
        assert!(!pane_value.get());
        controlled_runtime
            .adapter()
            .validate_graph(controlled_runtime.graph())
            .unwrap();
        let stale_date_calls = Rc::new(Cell::new(0));
        let stale_date_callback = Rc::clone(&stale_date_calls);
        let date_value = Rc::new(Cell::new(None));
        let mut calendar_runtime = reactor::Runtime::new(reactor::native::WinUiAdapter::default());
        calendar_runtime
            .update(
                reactor::CalendarDatePicker::new().on_date_changed(move |_| {
                    stale_date_callback.set(stale_date_callback.get() + 1);
                }),
            )
            .unwrap();
        let calendar = calendar_runtime.graph().root().unwrap();
        calendar_runtime
            .adapter()
            .simulate_calendar_date(calendar, reactor::DateTime::from_unix_secs(1_700_000_000))
            .unwrap();
        calendar_runtime
            .adapter()
            .simulate_calendar_date(calendar, reactor::DateTime::from_unix_secs(1_705_000_000))
            .unwrap();
        let mut active_date = calendar_runtime.next_native_event().unwrap().unwrap();
        active_date.invoke();
        let date_value_for_callback = Rc::clone(&date_value);
        calendar_runtime
            .update(
                reactor::CalendarDatePicker::new()
                    .on_date_changed(move |value| date_value_for_callback.set(value)),
            )
            .unwrap();
        drop(active_date);
        assert_eq!(calendar_runtime.dispatch_native_events().unwrap(), 0);
        assert_eq!(stale_date_calls.get(), 1);
        let expected_date = reactor::DateTime::from_unix_secs(1_710_000_000);
        calendar_runtime
            .adapter()
            .simulate_calendar_date(calendar, expected_date)
            .unwrap();
        assert_eq!(calendar_runtime.dispatch_native_events().unwrap(), 1);
        assert_eq!(date_value.get(), Some(expected_date));
        calendar_runtime
            .update(reactor::CalendarDatePicker::new())
            .unwrap();
        calendar_runtime
            .adapter()
            .simulate_calendar_date(calendar, reactor::DateTime::from_unix_secs(1_720_000_000))
            .unwrap();
        assert_eq!(calendar_runtime.dispatch_native_events().unwrap(), 0);
        let pointer_value = reactor::PointerEventInfo {
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
        let received_pointer_moved = Rc::new(Cell::new(false));
        let received_pointer_moved_callback = Rc::clone(&received_pointer_moved);
        let received_pointer_pressed = Rc::new(Cell::new(false));
        let received_pointer_pressed_callback = Rc::clone(&received_pointer_pressed);
        let stale_pointer_calls = Rc::new(Cell::new(0));
        let stale_pointer_callback = Rc::clone(&stale_pointer_calls);
        let pointer_motion_events = Rc::new(RefCell::new(Vec::new()));
        let pointer_motion_callback = |event| {
            let events = Rc::clone(&pointer_motion_events);
            move |_| events.borrow_mut().push(event)
        };
        let mut pointer_runtime = reactor::Runtime::new(reactor::native::WinUiAdapter::default());
        let pressed_pointer = Rc::new(RefCell::new(None));
        let pressed_pointer_callback = Rc::clone(&pressed_pointer);
        let routed_keys = Rc::new(RefCell::new(Vec::new()));
        let preview_keys = Rc::clone(&routed_keys);
        let released_keys = Rc::clone(&routed_keys);
        let routed_characters = Rc::new(RefCell::new(Vec::new()));
        let character_values = Rc::clone(&routed_characters);
        let focus_values = Rc::new(RefCell::new(Vec::new()));
        let got_focus_values = Rc::clone(&focus_values);
        let lost_focus_values = Rc::clone(&focus_values);
        let drag_values = Rc::new(RefCell::new(Vec::new()));
        let drag_enter_values = Rc::clone(&drag_values);
        let drag_over_values = Rc::clone(&drag_values);
        let dropped_values = Rc::new(RefCell::new(Vec::new()));
        let dropped_callback = Rc::clone(&dropped_values);
        let drop_policy = reactor::DragDropPolicy::new()
            .storage_items(reactor::DragDropAction::new(
                reactor::DragDropOperation::Move,
            ))
            .text(
                reactor::DragDropAction::new(reactor::DragDropOperation::Copy).caption("Copy text"),
            );
        pointer_runtime
            .update(
                reactor::Border::new()
                    .width(200.0)
                    .height(200.0)
                    .background(reactor::Color::rgb(255, 255, 255))
                    .capture_pointer_on_press(true)
                    .focus_on_pointer_release(true)
                    .on_pointer_pressed(move |value| {
                        *pressed_pointer_callback.borrow_mut() = Some(value);
                    })
                    .on_pointer_moved(pointer_motion_callback(reactor::EventId::PointerMoved))
                    .on_pointer_entered(pointer_motion_callback(reactor::EventId::PointerEntered))
                    .on_pointer_exited(pointer_motion_callback(reactor::EventId::PointerExited))
                    .on_pointer_released(move |_| {
                        stale_pointer_callback.set(stale_pointer_callback.get() + 1);
                    })
                    .on_preview_key_down(reactor::RoutedCallback::new(move |value| {
                        preview_keys.borrow_mut().push(value);
                        true
                    }))
                    .on_key_up(reactor::RoutedCallback::new(move |value| {
                        released_keys.borrow_mut().push(value);
                        false
                    }))
                    .on_character_received(reactor::RoutedCallback::new(move |value| {
                        character_values.borrow_mut().push(value);
                        true
                    }))
                    .on_got_focus(move |value| got_focus_values.borrow_mut().push(value))
                    .on_lost_focus(move |value| lost_focus_values.borrow_mut().push(value))
                    .drop_policy(drop_policy.clone())
                    .on_drag_enter(move |value| drag_enter_values.borrow_mut().push(value))
                    .on_drag_over(move |value| drag_over_values.borrow_mut().push(value))
                    .on_drop(move |value| dropped_callback.borrow_mut().push(value)),
            )
            .unwrap();
        let pointer_border = pointer_runtime.graph().root().unwrap();
        assert_eq!(
            pointer_runtime
                .adapter()
                .pointer_policy(pointer_border)
                .unwrap(),
            (true, true)
        );
        assert_eq!(
            pointer_runtime
                .adapter()
                .drop_policy(pointer_border)
                .unwrap(),
            Some(Rc::new(drop_policy))
        );
        let pressed_value = reactor::PointerEventInfo {
            capture_succeeded: Some(true),
            ..pointer_value
        };
        pointer_runtime
            .adapter()
            .simulate_pointer_event(
                pointer_border,
                reactor::EventId::PointerPressed,
                pressed_value,
            )
            .unwrap();
        for event in [
            reactor::EventId::PointerMoved,
            reactor::EventId::PointerEntered,
            reactor::EventId::PointerExited,
        ] {
            pointer_runtime
                .adapter()
                .simulate_pointer_event(pointer_border, event, pointer_value)
                .unwrap();
        }
        assert_eq!(pointer_runtime.dispatch_native_events().unwrap(), 4);
        assert_eq!(*pressed_pointer.borrow(), Some(pressed_value));
        let key_value = reactor::KeyEventInfo {
            key: reactor::VirtualKey::ENTER,
            original_key: reactor::VirtualKey::ENTER,
            status: reactor::PhysicalKeyStatus::default(),
            modifiers: reactor::InputModifiers::CONTROL,
        };
        assert!(
            pointer_runtime
                .adapter()
                .simulate_key_event(pointer_border, reactor::EventId::PreviewKeyDown, key_value,)
                .unwrap()
        );
        assert!(
            !pointer_runtime
                .adapter()
                .simulate_key_event(pointer_border, reactor::EventId::KeyUp, key_value)
                .unwrap()
        );
        let character_value = reactor::CharacterEventInfo {
            character: b'A' as u16,
            status: reactor::PhysicalKeyStatus::default(),
            modifiers: reactor::InputModifiers::SHIFT,
        };
        assert!(
            pointer_runtime
                .adapter()
                .simulate_character_event(pointer_border, character_value)
                .unwrap()
        );
        assert_eq!(&*routed_keys.borrow(), &[key_value, key_value]);
        assert_eq!(&*routed_characters.borrow(), &[character_value]);
        let got_focus = reactor::FocusEventInfo {
            state: reactor::ElementFocusState::Pointer,
            is_direct: true,
        };
        let lost_focus = reactor::FocusEventInfo {
            state: reactor::ElementFocusState::Unfocused,
            is_direct: false,
        };
        pointer_runtime
            .adapter()
            .simulate_focus_event(pointer_border, reactor::EventId::GotFocus, got_focus)
            .unwrap();
        pointer_runtime
            .adapter()
            .simulate_focus_event(pointer_border, reactor::EventId::LostFocus, lost_focus)
            .unwrap();
        assert_eq!(pointer_runtime.dispatch_native_events().unwrap(), 2);
        assert_eq!(&*focus_values.borrow(), &[got_focus, lost_focus]);
        pointer_runtime
            .adapter()
            .simulate_drag_kind(
                pointer_border,
                reactor::EventId::DragEnter,
                reactor::DragKind::Text,
            )
            .unwrap();
        pointer_runtime
            .adapter()
            .simulate_drag_kind(
                pointer_border,
                reactor::EventId::DragOver,
                reactor::DragKind::StorageItems,
            )
            .unwrap();
        pointer_runtime
            .adapter()
            .simulate_dropped_data(
                pointer_border,
                reactor::DroppedData::Text("value".to_string()),
            )
            .unwrap();
        assert_eq!(pointer_runtime.dispatch_native_events().unwrap(), 3);
        assert_eq!(
            &*drag_values.borrow(),
            &[reactor::DragKind::Text, reactor::DragKind::StorageItems]
        );
        assert_eq!(
            &*dropped_values.borrow(),
            &[reactor::DroppedData::Text("value".to_string())]
        );
        assert_eq!(
            &*pointer_motion_events.borrow(),
            &[
                reactor::EventId::PointerMoved,
                reactor::EventId::PointerEntered,
                reactor::EventId::PointerExited
            ]
        );
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
                reactor::Border::new()
                    .width(200.0)
                    .height(200.0)
                    .background(reactor::Color::rgb(255, 255, 255))
                    .capture_pointer_on_press(true)
                    .focus_on_pointer_release(true)
                    .on_pointer_moved(move |_| received_pointer_moved_callback.set(true))
                    .on_pointer_pressed(move |_| received_pointer_pressed_callback.set(true))
                    .on_pointer_released(move |value| {
                        *received_pointer_callback.borrow_mut() = Some(value);
                    }),
            )
            .unwrap();
        assert_eq!(
            pointer_runtime
                .adapter()
                .pointer_policy(pointer_border)
                .unwrap(),
            (true, true)
        );
        assert_eq!(
            pointer_runtime
                .adapter()
                .drop_policy(pointer_border)
                .unwrap(),
            None
        );
        assert!(
            !pointer_runtime
                .adapter()
                .simulate_key_event(pointer_border, reactor::EventId::PreviewKeyDown, key_value,)
                .unwrap()
        );
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
        let reordered_values = Rc::new(RefCell::new(Vec::new()));
        let reordered_callback = Rc::clone(&reordered_values);
        let mut grid_runtime = reactor::Runtime::new(reactor::native::WinUiAdapter::default());
        grid_runtime
            .update(
                reactor::GridView::new()
                    .items([
                        reactor::GridViewItem::new().tag("first").into(),
                        reactor::GridViewItem::new().tag("second").into(),
                    ])
                    .on_reordered(move |value| reordered_callback.borrow_mut().push(value)),
            )
            .unwrap();
        let grid = grid_runtime.graph().root().unwrap();
        grid_runtime
            .adapter()
            .simulate_item_tags(grid, vec!["second".to_string(), "first".to_string()])
            .unwrap();
        assert_eq!(grid_runtime.dispatch_native_events().unwrap(), 1);
        assert_eq!(
            &*reordered_values.borrow(),
            &[vec!["second".to_string(), "first".to_string()]]
        );
        grid_runtime
            .adapter()
            .validate_graph(grid_runtime.graph())
            .unwrap();
        let first_list_selections = Rc::new(RefCell::new(Vec::new()));
        let first_list_selection_callback = Rc::clone(&first_list_selections);
        let first_list_reorders = Rc::new(RefCell::new(Vec::new()));
        let first_list_reorder_callback = Rc::clone(&first_list_reorders);
        let mut list_runtime = reactor::Runtime::new(reactor::native::WinUiAdapter::default());
        list_runtime
            .update(
                reactor::ListView::new()
                    .items([
                        reactor::DataItem::new("first", "First"),
                        reactor::DataItem::new("second", "Second"),
                    ])
                    .selected_index(Some(0))
                    .selection_mode(reactor::ListViewSelectionMode::Extended)
                    .can_drag_items(true)
                    .can_reorder_items(true)
                    .allow_drop(true)
                    .on_selection_changed(move |value| {
                        first_list_selection_callback.borrow_mut().push(value);
                    })
                    .on_reordered(move |value| {
                        first_list_reorder_callback.borrow_mut().push(value);
                    }),
            )
            .unwrap();
        let list = list_runtime.graph().root().unwrap();
        assert_eq!(
            list_runtime.adapter().list_view_state(list).unwrap(),
            (
                Some(0),
                reactor::ListViewSelectionMode::Extended,
                true,
                true,
                true,
            )
        );
        list_runtime
            .adapter()
            .simulate_list_selection(list, None)
            .unwrap();
        list_runtime
            .adapter()
            .simulate_item_tags(list, vec!["second".to_string(), "first".to_string()])
            .unwrap();
        let mut active_list_event = list_runtime.next_native_event().unwrap().unwrap();
        active_list_event.invoke();
        let second_list_selections = Rc::new(RefCell::new(Vec::new()));
        let second_list_selection_callback = Rc::clone(&second_list_selections);
        let second_list_reorders = Rc::new(RefCell::new(Vec::new()));
        let second_list_reorder_callback = Rc::clone(&second_list_reorders);
        list_runtime
            .update(
                reactor::ListView::new()
                    .items([
                        reactor::DataItem::new("first", "First"),
                        reactor::DataItem::new("second", "Second"),
                    ])
                    .selected_index(None)
                    .selection_mode(reactor::ListViewSelectionMode::Extended)
                    .on_selection_changed(move |value| {
                        second_list_selection_callback.borrow_mut().push(value);
                    })
                    .on_reordered(move |value| {
                        second_list_reorder_callback.borrow_mut().push(value);
                    }),
            )
            .unwrap();
        drop(active_list_event);
        assert_eq!(list_runtime.dispatch_native_events().unwrap(), 0);
        assert_eq!(&*first_list_selections.borrow(), &[None]);
        assert!(first_list_reorders.borrow().is_empty());
        assert_eq!(
            list_runtime.adapter().list_view_state(list).unwrap(),
            (
                None,
                reactor::ListViewSelectionMode::Extended,
                false,
                false,
                false,
            )
        );
        list_runtime
            .adapter()
            .simulate_list_selection(list, Some(0))
            .unwrap();
        list_runtime
            .adapter()
            .simulate_item_tags(list, vec!["first".to_string(), "second".to_string()])
            .unwrap();
        assert_eq!(list_runtime.dispatch_native_events().unwrap(), 2);
        assert_eq!(&*second_list_selections.borrow(), &[Some(0)]);
        assert_eq!(
            &*second_list_reorders.borrow(),
            &[vec!["first".to_string(), "second".to_string()]]
        );
        list_runtime
            .update(
                reactor::ListView::new()
                    .items([
                        reactor::DataItem::new("first", "First"),
                        reactor::DataItem::new("second", "Second"),
                    ])
                    .selection_mode(reactor::ListViewSelectionMode::Multiple),
            )
            .unwrap();
        assert_eq!(
            list_runtime.adapter().list_view_state(list).unwrap(),
            (
                None,
                reactor::ListViewSelectionMode::Multiple,
                false,
                false,
                false,
            )
        );
        list_runtime
            .adapter()
            .simulate_list_selection(list, Some(0))
            .unwrap();
        list_runtime
            .adapter()
            .simulate_item_tags(list, vec!["removed".to_string()])
            .unwrap();
        assert_eq!(list_runtime.dispatch_native_events().unwrap(), 0);
        assert_eq!(
            list_runtime.adapter().list_view_state(list).unwrap(),
            (
                Some(0),
                reactor::ListViewSelectionMode::Multiple,
                false,
                false,
                false,
            )
        );
        list_runtime
            .update(reactor::ListView::new().items([
                reactor::DataItem::new("first", "First"),
                reactor::DataItem::new("second", "Second"),
            ]))
            .unwrap();
        assert_eq!(
            list_runtime.adapter().list_view_state(list).unwrap(),
            (
                None,
                reactor::ListViewSelectionMode::Single,
                false,
                false,
                false,
            )
        );
        list_runtime
            .adapter()
            .validate_graph(list_runtime.graph())
            .unwrap();
        let first_tree_values = Rc::new(RefCell::new(Vec::new()));
        let first_tree_callback = Rc::clone(&first_tree_values);
        let mut tree_runtime = reactor::Runtime::new(reactor::native::WinUiAdapter::default());
        tree_runtime
            .update(
                reactor::TreeView::new()
                    .selection_mode(reactor::TreeViewSelectionMode::Multiple)
                    .nodes([reactor::TreeNode::new("root", "Root")])
                    .on_item_invoked(move |value| {
                        first_tree_callback.borrow_mut().push(value);
                    }),
            )
            .unwrap();
        let tree = tree_runtime.graph().root().unwrap();
        assert_eq!(
            tree_runtime
                .adapter()
                .tree_view_selection_mode(tree)
                .unwrap(),
            reactor::TreeViewSelectionMode::Multiple
        );
        tree_runtime
            .adapter()
            .simulate_tree_item_invoked(tree, "Stale")
            .unwrap();
        let active_tree_event = tree_runtime.next_native_event().unwrap().unwrap();
        let second_tree_values = Rc::new(RefCell::new(Vec::new()));
        let second_tree_callback = Rc::clone(&second_tree_values);
        tree_runtime
            .update(
                reactor::TreeView::new()
                    .selection_mode(reactor::TreeViewSelectionMode::Single)
                    .nodes([reactor::TreeNode::new("root", "Root")])
                    .on_item_invoked(move |value| {
                        second_tree_callback.borrow_mut().push(value);
                    }),
            )
            .unwrap();
        drop(active_tree_event);
        assert_eq!(tree_runtime.dispatch_native_events().unwrap(), 0);
        assert!(first_tree_values.borrow().is_empty());
        tree_runtime
            .adapter()
            .simulate_tree_item_invoked(tree, "Root")
            .unwrap();
        assert_eq!(tree_runtime.dispatch_native_events().unwrap(), 1);
        assert_eq!(&*second_tree_values.borrow(), &[Rc::from("Root")]);
        tree_runtime
            .update(reactor::TreeView::new().nodes([reactor::TreeNode::new("root", "Root")]))
            .unwrap();
        tree_runtime
            .adapter()
            .simulate_tree_item_invoked(tree, "Removed")
            .unwrap();
        assert_eq!(tree_runtime.dispatch_native_events().unwrap(), 0);
        assert_eq!(
            tree_runtime
                .adapter()
                .tree_view_selection_mode(tree)
                .unwrap(),
            reactor::TreeViewSelectionMode::Single
        );
        tree_runtime
            .adapter()
            .validate_graph(tree_runtime.graph())
            .unwrap();
        let breadcrumb_values = Rc::new(RefCell::new(Vec::new()));
        let breadcrumb_callback = Rc::clone(&breadcrumb_values);
        let mut breadcrumb_runtime =
            reactor::Runtime::new(reactor::native::WinUiAdapter::default());
        breadcrumb_runtime
            .update(
                reactor::BreadcrumbBar::new()
                    .items_source(["Root", "Current"])
                    .on_item_clicked(move |value| breadcrumb_callback.borrow_mut().push(value)),
            )
            .unwrap();
        let breadcrumb = breadcrumb_runtime.graph().root().unwrap();
        breadcrumb_runtime
            .adapter()
            .simulate_string_event(breadcrumb, reactor::EventId::ItemClicked, "Stale")
            .unwrap();
        let active_breadcrumb_event = breadcrumb_runtime.next_native_event().unwrap().unwrap();
        let replacement_values = Rc::new(RefCell::new(Vec::new()));
        let replacement_callback = {
            let values = Rc::clone(&replacement_values);
            reactor::Callback::new(move |value| values.borrow_mut().push(value))
        };
        breadcrumb_runtime
            .update(
                reactor::BreadcrumbBar::new()
                    .items_source(["Root", "Current"])
                    .on_item_clicked(replacement_callback),
            )
            .unwrap();
        drop(active_breadcrumb_event);
        assert_eq!(breadcrumb_runtime.dispatch_native_events().unwrap(), 0);
        assert!(breadcrumb_values.borrow().is_empty());
        breadcrumb_runtime
            .adapter()
            .simulate_string_event(breadcrumb, reactor::EventId::ItemClicked, "Current")
            .unwrap();
        assert_eq!(breadcrumb_runtime.dispatch_native_events().unwrap(), 1);
        assert_eq!(&*replacement_values.borrow(), &[Rc::from("Current")]);
        breadcrumb_runtime
            .update(reactor::BreadcrumbBar::new().items_source(["Root", "Current"]))
            .unwrap();
        breadcrumb_runtime
            .adapter()
            .simulate_string_event(breadcrumb, reactor::EventId::ItemClicked, "Removed")
            .unwrap();
        assert_eq!(breadcrumb_runtime.dispatch_native_events().unwrap(), 0);
        let suggestion_values = Rc::new(RefCell::new(Vec::new()));
        let suggestion_callback = Rc::clone(&suggestion_values);
        let mut suggestion_runtime =
            reactor::Runtime::new(reactor::native::WinUiAdapter::default());
        suggestion_runtime
            .update(
                reactor::AutoSuggestBox::new()
                    .items_source(["First", "Second"])
                    .on_suggestion_chosen(move |value| {
                        suggestion_callback.borrow_mut().push(value);
                    }),
            )
            .unwrap();
        let suggestions = suggestion_runtime.graph().root().unwrap();
        suggestion_runtime
            .adapter()
            .simulate_string_event(suggestions, reactor::EventId::SuggestionChosen, "Second")
            .unwrap();
        assert_eq!(suggestion_runtime.dispatch_native_events().unwrap(), 1);
        assert_eq!(&*suggestion_values.borrow(), &[Rc::from("Second")]);
        suggestion_runtime
            .adapter()
            .validate_graph(suggestion_runtime.graph())
            .unwrap();
        let closed_tabs = Rc::new(RefCell::new(Vec::new()));
        let closed_callback = Rc::clone(&closed_tabs);
        let reordered_tabs = Rc::new(RefCell::new(Vec::new()));
        let reordered_callback = Rc::clone(&reordered_tabs);
        let mut tab_runtime = reactor::Runtime::new(reactor::native::WinUiAdapter::default());
        tab_runtime
            .update(
                reactor::TabView::new()
                    .tab_items([
                        reactor::TabViewItem::new().tag("first").into(),
                        reactor::TabViewItem::new().tag("second").into(),
                    ])
                    .on_close_requested(move |value| closed_callback.borrow_mut().push(value))
                    .on_reordered(move |value| reordered_callback.borrow_mut().push(value)),
            )
            .unwrap();
        let tabs = tab_runtime.graph().root().unwrap();
        tab_runtime
            .adapter()
            .simulate_string_event(tabs, reactor::EventId::TabCloseRequested, "second")
            .unwrap();
        tab_runtime
            .adapter()
            .simulate_string_list_event(
                tabs,
                reactor::EventId::TabItemsChanged,
                ["second", "first"],
            )
            .unwrap();
        assert_eq!(tab_runtime.dispatch_native_events().unwrap(), 2);
        assert_eq!(&*closed_tabs.borrow(), &[Rc::from("second")]);
        assert_eq!(
            &*reordered_tabs.borrow(),
            &[vec!["second".to_string(), "first".to_string()]]
        );
        tab_runtime
            .adapter()
            .validate_graph(tab_runtime.graph())
            .unwrap();
        let color_values = Rc::new(RefCell::new(Vec::new()));
        let color_callback = Rc::clone(&color_values);
        let color = reactor::Color::rgb(10, 20, 30);
        let mut color_runtime = reactor::Runtime::new(reactor::native::WinUiAdapter::default());
        color_runtime
            .update(
                reactor::ColorPicker::new()
                    .color(color)
                    .on_color_changed(move |value| color_callback.borrow_mut().push(value)),
            )
            .unwrap();
        let picker = color_runtime.graph().root().unwrap();
        assert_eq!(
            color_runtime.adapter().color_picker_color(picker).unwrap(),
            color
        );
        assert_eq!(color_runtime.dispatch_native_events().unwrap(), 0);
        let changed_color = reactor::Color::rgb(30, 20, 10);
        color_runtime
            .adapter()
            .simulate_color_changed(picker, changed_color)
            .unwrap();
        assert_eq!(color_runtime.dispatch_native_events().unwrap(), 1);
        assert_eq!(&*color_values.borrow(), &[changed_color]);
        assert_eq!(
            color_runtime.adapter().color_picker_color(picker).unwrap(),
            changed_color
        );

        let modes = Rc::new(RefCell::new(Vec::new()));
        let mode_callback = Rc::clone(&modes);
        let mut navigation_runtime =
            reactor::Runtime::new(reactor::native::WinUiAdapter::default());
        navigation_runtime
            .update(
                reactor::NavigationView::new().on_display_mode_changed(move |value| {
                    mode_callback.borrow_mut().push(value);
                }),
            )
            .unwrap();
        let navigation = navigation_runtime.graph().root().unwrap();
        navigation_runtime
            .adapter()
            .simulate_navigation_display_mode_changed(
                navigation,
                reactor::NavigationViewDisplayMode::Compact,
            )
            .unwrap();
        assert_eq!(navigation_runtime.dispatch_native_events().unwrap(), 1);
        assert_eq!(
            &*modes.borrow(),
            &[reactor::NavigationViewDisplayMode::Compact]
        );

        let selected_dates = Rc::new(RefCell::new(Vec::new()));
        let date_callback = Rc::clone(&selected_dates);
        let mut date_runtime = reactor::Runtime::new(reactor::native::WinUiAdapter::default());
        date_runtime
            .update(
                reactor::DatePicker::new()
                    .on_selected_date_changed(move |value| date_callback.borrow_mut().push(value)),
            )
            .unwrap();
        let date_picker = date_runtime.graph().root().unwrap();
        let date = reactor::DateTime::from_unix_secs(1_700_000_000);
        date_runtime
            .adapter()
            .simulate_selected_date_changed(date_picker, Some(date))
            .unwrap();
        assert_eq!(date_runtime.dispatch_native_events().unwrap(), 1);
        assert_eq!(&*selected_dates.borrow(), &[Some(date)]);

        let selected_times = Rc::new(RefCell::new(Vec::new()));
        let time_callback = Rc::clone(&selected_times);
        let mut time_runtime = reactor::Runtime::new(reactor::native::WinUiAdapter::default());
        time_runtime
            .update(
                reactor::TimePicker::new()
                    .on_selected_time_changed(move |value| time_callback.borrow_mut().push(value)),
            )
            .unwrap();
        let time_picker = time_runtime.graph().root().unwrap();
        let time = reactor::TimeSpan::from_millis(90_000);
        time_runtime
            .adapter()
            .simulate_selected_time_changed(time_picker, Some(time))
            .unwrap();
        assert_eq!(time_runtime.dispatch_native_events().unwrap(), 1);
        assert_eq!(&*selected_times.borrow(), &[Some(time)]);

        let mut image_runtime = reactor::Runtime::new(reactor::native::WinUiAdapter::default());
        image_runtime
            .update(
                reactor::Image::new()
                    .source("https://example.com/image.png")
                    .unwrap(),
            )
            .unwrap();
        let image = image_runtime.graph().root().unwrap();
        assert!(image_runtime.adapter().image_has_source(image).unwrap());
        assert!(!image_runtime.adapter().image_source_is_svg(image).unwrap());
        image_runtime
            .update(
                reactor::Image::new()
                    .source("file:///C:/reactor-selftest.svg")
                    .unwrap(),
            )
            .unwrap();
        assert!(image_runtime.adapter().image_source_is_svg(image).unwrap());
        image_runtime
            .update(
                reactor::Image::new()
                    .source_data(reactor::EncodedImage::from_static(ENCODED_IMAGE_PNG)),
            )
            .unwrap();
        assert!(image_runtime.adapter().image_has_source(image).unwrap());
        image_runtime.update(reactor::Image::new()).unwrap();
        assert!(!image_runtime.adapter().image_has_source(image).unwrap());

        let mut icon_runtime = reactor::Runtime::new(reactor::native::WinUiAdapter::default());
        icon_runtime
            .update(
                reactor::ImageIcon::new()
                    .source("https://example.com/icon.png")
                    .unwrap(),
            )
            .unwrap();
        let icon = icon_runtime.graph().root().unwrap();
        assert!(icon_runtime.adapter().image_has_source(icon).unwrap());
        assert!(!icon_runtime.adapter().image_source_is_svg(icon).unwrap());
        icon_runtime
            .update(
                reactor::ImageIcon::new()
                    .source("file:///C:/reactor-selftest.svg")
                    .unwrap(),
            )
            .unwrap();
        assert!(icon_runtime.adapter().image_source_is_svg(icon).unwrap());
        icon_runtime
            .update(
                reactor::ImageIcon::new()
                    .source_data(reactor::EncodedImage::from_static(ENCODED_IMAGE_PNG)),
            )
            .unwrap();
        assert!(icon_runtime.adapter().image_has_source(icon).unwrap());
        icon_runtime.update(reactor::ImageIcon::new()).unwrap();
        assert!(!icon_runtime.adapter().image_has_source(icon).unwrap());
        let encoded_image_result = Rc::new(Cell::new(None));
        let encoded_image_opened = Rc::clone(&encoded_image_result);
        let encoded_image_failed = Rc::clone(&encoded_image_result);
        let mut encoded_image_runtime =
            reactor::Runtime::new(reactor::native::WinUiAdapter::default());
        encoded_image_runtime
            .update(
                reactor::Image::new()
                    .width(1.0)
                    .height(1.0)
                    .on_opened(move || encoded_image_opened.set(Some(true)))
                    .on_failed(move || encoded_image_failed.set(Some(false))),
            )
            .unwrap();
        let encoded_image_root = encoded_image_runtime.graph().root().unwrap();
        let encoded_image_window = encoded_image_runtime
            .adapter()
            .create_window(encoded_image_root)
            .unwrap();
        let mut flyout_runtime = reactor::Runtime::new(reactor::native::WinUiAdapter::default());
        flyout_runtime
            .update(reactor::Button::new().content("Flyout target").flyout_with(
                reactor::Flyout::text("Flyout content").placement(reactor::FlyoutPlacement::Bottom),
            ))
            .unwrap();
        let flyout_root = flyout_runtime.graph().root().unwrap();
        let flyout_window = flyout_runtime.adapter().create_window(flyout_root).unwrap();
        let mut menu_runtime = reactor::Runtime::new(reactor::native::WinUiAdapter::default());
        menu_runtime
            .update(
                reactor::Button::new()
                    .content("Menu target")
                    .menu(reactor::Menu::new(
                        [
                            reactor::MenuItem::item("open", "Open"),
                            reactor::MenuItem::separator("separator"),
                            reactor::MenuItem::submenu(
                                "share",
                                "Share",
                                [reactor::MenuItem::item("email", "Email")],
                            ),
                        ],
                        |_| {},
                    )),
            )
            .unwrap();
        let menu_root = menu_runtime.graph().root().unwrap();
        let menu_window = menu_runtime.adapter().create_window(menu_root).unwrap();
        let mut command_bar_flyout_runtime =
            reactor::Runtime::new(reactor::native::WinUiAdapter::default());
        command_bar_flyout_runtime
            .update(
                reactor::Button::new()
                    .content("Command target")
                    .command_bar_flyout(reactor::CommandBarFlyout::new(
                        [reactor::CommandBarCommand::button_with_icon(
                            "save",
                            "Save",
                            reactor::Symbol::Save,
                        )],
                        [
                            reactor::CommandBarCommand::separator("separator"),
                            reactor::CommandBarCommand::button("copy", "Copy"),
                        ],
                        |_| {},
                    )),
            )
            .unwrap();
        let command_bar_flyout_root = command_bar_flyout_runtime.graph().root().unwrap();
        let command_bar_flyout_window = command_bar_flyout_runtime
            .adapter()
            .create_window(command_bar_flyout_root)
            .unwrap();

        let mut rich_runtime = reactor::Runtime::new(reactor::native::WinUiAdapter::default());
        rich_runtime
            .update(
                reactor::RichTextBlock::new().paragraphs(reactor::RichText::new([
                    reactor::RichTextParagraph::new([
                        reactor::RichTextInline::Run(reactor::RichTextRun::plain("First")),
                        reactor::RichTextInline::LineBreak,
                    ]),
                    reactor::RichTextParagraph::new([reactor::RichTextInline::Hyperlink(
                        reactor::RichTextHyperlink {
                            text: "Second".into(),
                            uri: "https://example.com".into(),
                        },
                    )]),
                ])),
            )
            .unwrap();
        let rich_text = rich_runtime.graph().root().unwrap();
        assert_eq!(
            rich_runtime.adapter().rich_text_shape(rich_text).unwrap(),
            vec![2, 1]
        );
        rich_runtime.update(reactor::RichTextBlock::new()).unwrap();
        assert!(
            rich_runtime
                .adapter()
                .rich_text_shape(rich_text)
                .unwrap()
                .is_empty()
        );
        let mut generated_runtime = reactor::Runtime::new(reactor::native::WinUiAdapter::default());
        let slider_changed = Rc::new(Cell::new(0.0));
        let slider_changed_count = Rc::new(Cell::new(0));
        let slider_changed_callback = Rc::clone(&slider_changed);
        let slider_count_callback = Rc::clone(&slider_changed_count);
        let toggle_changed = Rc::new(Cell::new(false));
        let toggle_changed_count = Rc::new(Cell::new(0));
        let toggle_changed_callback = Rc::clone(&toggle_changed);
        let toggle_count_callback = Rc::clone(&toggle_changed_count);
        let focus_reference = reactor::ElementRef::<reactor::Button>::default();
        generated_runtime
            .update(
                reactor::StackPanel::new()
                    .spacing(8.0)
                    .orientation(reactor::Orientation::Horizontal)
                    .children([
                        reactor::Slider::new()
                            .minimum(-10.0)
                            .maximum(10.0)
                            .value(2.5)
                            .on_value_changed(move |value| {
                                slider_changed_callback.set(value);
                                slider_count_callback.set(slider_count_callback.get() + 1);
                            })
                            .into(),
                        reactor::ToggleSwitch::new()
                            .is_on(true)
                            .on_toggled(move |value| {
                                toggle_changed_callback.set(value);
                                toggle_count_callback.set(toggle_count_callback.get() + 1);
                            })
                            .into(),
                        reactor::CheckBox::new()
                            .is_checked(Some(true))
                            .content(reactor::TextBlock::new().text("Enabled"))
                            .into(),
                        reactor::ScrollViewer::new()
                            .content(
                                reactor::Canvas::new().children([reactor::TextBlock::new()
                                    .text("Scrollable canvas")
                                    .canvas_left(12.0)
                                    .canvas_top(24.0)
                                    .into()]),
                            )
                            .into(),
                        reactor::SelectorBar::new()
                            .keyed_items([reactor::keyed(
                                "only",
                                reactor::SelectorBarItem::new().text("Only"),
                            )])
                            .into(),
                        reactor::AppBarButton::new()
                            .label("Icon")
                            .icon(reactor::Symbol::Home)
                            .into(),
                        reactor::Button::new()
                            .element_ref(&focus_reference)
                            .is_enabled(true)
                            .min_width(20.0)
                            .max_height(80.0)
                            .grid_row(2)
                            .relative_align_left()
                            .automation_name("capability button")
                            .into(),
                        reactor::TextBlock::new()
                            .text("Styled")
                            .font_weight(reactor::FontWeight::SEMI_BOLD)
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
            .children(generated_root, reactor::RelationId::Children)
            .unwrap();
        let slider = generated_children[0];
        let toggle = generated_children[1];
        let check_box = generated_children[2];
        let scroll_viewer = generated_children[3];
        let canvas = generated_runtime
            .graph()
            .child(scroll_viewer, reactor::RelationId::Content)
            .unwrap();
        let canvas_child = generated_runtime
            .graph()
            .children(canvas, reactor::RelationId::Children)
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
            reactor::Runtime::new(reactor::native::WinUiAdapter::default());
        early_virtual_runtime
            .update(
                reactor::ItemsRepeater::new().virtual_source(reactor::VirtualSource::new(
                    1,
                    10_000,
                    reactor::Key::from,
                    |index| -> reactor::View {
                        reactor::TextBlock::new()
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
            reactor::Runtime::new(reactor::native::WinUiAdapter::default());
        destroyed_virtual_runtime
            .update(reactor::Grid::new().keyed_children([reactor::keyed(
                "repeater",
                reactor::ItemsRepeater::new().item("row", reactor::TextBlock::new().text("row")),
            )]))
            .unwrap();
        let destroyed_virtual_root = destroyed_virtual_runtime.graph().root().unwrap();
        let destroyed_repeater = destroyed_virtual_runtime
            .graph()
            .children(destroyed_virtual_root, reactor::RelationId::Children)
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
            .update(reactor::Grid::new())
            .unwrap();
        title_bar_runtime
            .update(
                reactor::Grid::new().keyed_children([reactor::keyed(
                    "target",
                    reactor::Button::new().content("Hover target").tooltip_with(
                        reactor::Tooltip::rich(reactor::StackPanel::new().children([
                            reactor::TextBlock::new().text("Rich tooltip").into(),
                            reactor::TextBlock::new().text("Live content").into(),
                        ]))
                        .placement(reactor::TooltipPlacement::Bottom),
                    ),
                )]),
            )
            .unwrap();
        title_bar_runtime
            .update(
                reactor::Grid::new().keyed_children([reactor::keyed(
                    "target",
                    reactor::Button::new().content("Hover target").tooltip_with(
                        reactor::Tooltip::text("Updated tooltip")
                            .placement(reactor::TooltipPlacement::Mouse),
                    ),
                )]),
            )
            .unwrap();
        title_bar_runtime.update(reactor::Grid::new()).unwrap();
        destroyed_virtual_runtime
            .adapter()
            .validate_graph(destroyed_virtual_runtime.graph())
            .unwrap();
        destroyed_virtual_window.close().unwrap();

        let mut replaced_virtual_runtime =
            reactor::Runtime::new(reactor::native::WinUiAdapter::default());
        replaced_virtual_runtime
            .update(reactor::Grid::new().keyed_children([reactor::keyed(
                "slot",
                reactor::ItemsRepeater::new().item("row", reactor::TextBlock::new().text("row")),
            )]))
            .unwrap();
        let replaced_virtual_root = replaced_virtual_runtime.graph().root().unwrap();
        let replaced_repeater = replaced_virtual_runtime
            .graph()
            .children(replaced_virtual_root, reactor::RelationId::Children)
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
                reactor::Grid::new()
                    .keyed_children([reactor::keyed("slot", reactor::Border::new())]),
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
                reactor::StackPanel::new()
                    .spacing(8.0)
                    .orientation(reactor::Orientation::Horizontal)
                    .children([
                        reactor::Slider::new()
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
                        reactor::ToggleSwitch::new()
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
                        reactor::CheckBox::new()
                            .is_checked(Some(true))
                            .content(reactor::TextBlock::new().text("Enabled"))
                            .into(),
                        reactor::ScrollViewer::new()
                            .content(
                                reactor::Canvas::new().children([reactor::TextBlock::new()
                                    .text("Scrollable canvas")
                                    .canvas_left(12.0)
                                    .canvas_top(24.0)
                                    .into()]),
                            )
                            .into(),
                        reactor::SelectorBar::new()
                            .keyed_items([
                                reactor::keyed(
                                    "first",
                                    reactor::SelectorBarItem::new().text("First"),
                                ),
                                reactor::keyed(
                                    "second",
                                    reactor::SelectorBarItem::new().text("Second"),
                                ),
                            ])
                            .into(),
                        reactor::AppBarButton::new()
                            .label("Icon")
                            .icon(reactor::Symbol::Home)
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
                .contains(&reactor::Property {
                    id: reactor::PropertyId::Value,
                    value: reactor::PropertyValue::F64(6.0),
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
                .contains(&reactor::Property {
                    id: reactor::PropertyId::Value,
                    value: reactor::PropertyValue::F64(7.5),
                })
        );
        assert!(
            generated_runtime
                .graph()
                .properties(toggle)
                .unwrap()
                .contains(&reactor::Property {
                    id: reactor::PropertyId::IsOn,
                    value: reactor::PropertyValue::Bool(true),
                })
        );
        generated_runtime
            .adapter()
            .validate_graph(generated_runtime.graph())
            .unwrap();
        generated_runtime
            .update(
                reactor::StackPanel::new().children([
                    reactor::Slider::new().into(),
                    reactor::ToggleSwitch::new().into(),
                    reactor::CheckBox::new()
                        .content(reactor::TextBlock::new().text("Enabled"))
                        .into(),
                    reactor::ScrollViewer::new()
                        .content(
                            reactor::Canvas::new().children([reactor::TextBlock::new()
                                .text("Scrollable canvas")
                                .into()]),
                        )
                        .into(),
                    reactor::SelectorBar::new()
                        .keyed_items([
                            reactor::keyed("first", reactor::SelectorBarItem::new().text("First")),
                            reactor::keyed(
                                "second",
                                reactor::SelectorBarItem::new().text("Second"),
                            ),
                        ])
                        .into(),
                    reactor::AppBarButton::new()
                        .label("Icon")
                        .icon(reactor::Symbol::Home)
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
            reactor::Callback::new(move |next: Rc<str>| {
                *value.borrow_mut() = next;
                count.set(count.get() + 1);
            })
        };
        let mut document_runtime = reactor::Runtime::new(reactor::native::WinUiAdapter::default());
        document_runtime
            .update(
                reactor::Grid::new()
                    .rows([reactor::GridLength::Auto, reactor::GridLength::STAR])
                    .columns([reactor::GridLength::Pixel(120.0).min(40.0)])
                    .keyed_children([reactor::keyed(
                        "document",
                        reactor::RichEditBox::new()
                            .text("first\r\nsecond")
                            .is_read_only(true)
                            .on_text_changed(rich_callback.clone()),
                    )]),
            )
            .unwrap();
        let document_root = document_runtime.graph().root().unwrap();
        let document = document_runtime
            .graph()
            .children(document_root, reactor::RelationId::Children)
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
                reactor::Grid::new()
                    .rows([reactor::GridLength::Pixel(24.0)])
                    .keyed_children([reactor::keyed(
                        "document",
                        reactor::RichEditBox::new()
                            .text("application")
                            .is_read_only(true)
                            .on_text_changed(rich_callback.clone()),
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
                reactor::Grid::new()
                    .rows([reactor::GridLength::Pixel(24.0)])
                    .keyed_children([reactor::keyed(
                        "document",
                        reactor::RichEditBox::new()
                            .text("application")
                            .is_read_only(false)
                            .on_text_changed(rich_callback),
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
            reactor::Callback::new(move |next| {
                *value.borrow_mut() = next;
                count.set(count.get() + 1);
            })
        };
        let cleared_document = || {
            reactor::Grid::new()
                .rows([reactor::GridLength::Pixel(24.0)])
                .keyed_children([reactor::keyed(
                    "document",
                    reactor::RichEditBox::new()
                        .is_read_only(false)
                        .on_text_changed(clear_callback.clone()),
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
                .all(|property| property.id != reactor::PropertyId::Document)
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
            reactor::ItemsRepeater::new().virtual_source(reactor::VirtualSource::new(
                1,
                10_000,
                reactor::Key::from,
                move |index| -> reactor::View {
                    reactor::TextBlock::new()
                        .text(format!("{prefix} {index}"))
                        .into()
                },
            ))
        };
        let mut virtual_runtime = reactor::Runtime::new(reactor::native::WinUiAdapter::default());
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
            .children(virtual_root, reactor::RelationId::Items)
            .unwrap()
            .to_vec();
        assert!(!realized.is_empty());
        virtual_runtime.update(virtual_view("updated")).unwrap();
        assert_eq!(virtual_runtime.dispatch_native_events().unwrap(), 0);
        assert!(
            virtual_runtime
                .graph()
                .children(virtual_root, reactor::RelationId::Items)
                .unwrap()
                .len()
                <= 1
        );
        virtual_runtime
            .update(reactor::ItemsRepeater::new())
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
        let mut payload_runtime = reactor::Runtime::new(reactor::native::WinUiAdapter::default());
        payload_runtime
            .update(
                reactor::StackPanel::new().children([
                    reactor::PasswordBox::new()
                        .on_password_changed({
                            let value = Rc::clone(&password_value);
                            let count = Rc::clone(&password_count);
                            move |next| {
                                *value.borrow_mut() = next;
                                count.set(count.get() + 1);
                            }
                        })
                        .into(),
                    reactor::RatingControl::new()
                        .on_value_changed({
                            let value = Rc::clone(&rating_value);
                            let count = Rc::clone(&rating_count);
                            move |next| {
                                value.set(next);
                                count.set(count.get() + 1);
                            }
                        })
                        .into(),
                    reactor::ComboBox::new()
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
            .children(payload_root, reactor::RelationId::Children)
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
            reactor::Callback::new(move |next: Option<Rc<str>>| {
                value.borrow_mut().clone_from(&next);
                history.borrow_mut().push(next);
                count.set(count.get() + 1);
            })
        };
        let list_callback = {
            let value = Rc::clone(&list_value);
            let count = Rc::clone(&list_count);
            reactor::Callback::new(move |next| {
                *value.borrow_mut() = next;
                count.set(count.get() + 1);
            })
        };
        let selector_callback = {
            let value = Rc::clone(&selector_value);
            let count = Rc::clone(&selector_count);
            reactor::Callback::new(move |next| {
                *value.borrow_mut() = next;
                count.set(count.get() + 1);
            })
        };
        let selections = |reversed: bool, remove_selected: bool| {
            let navigation_items = if remove_selected {
                vec![reactor::keyed(
                    "first",
                    reactor::NavigationViewItem::new()
                        .tag("nav-first")
                        .is_selected(false),
                )]
            } else if reversed {
                vec![
                    reactor::keyed(
                        "second",
                        reactor::NavigationViewItem::new()
                            .tag("nav-second")
                            .is_selected(true),
                    ),
                    reactor::keyed(
                        "first",
                        reactor::NavigationViewItem::new()
                            .tag("nav-first")
                            .is_selected(false),
                    ),
                ]
            } else {
                vec![
                    reactor::keyed(
                        "first",
                        reactor::NavigationViewItem::new()
                            .tag("nav-first")
                            .is_selected(true),
                    ),
                    reactor::keyed(
                        "second",
                        reactor::NavigationViewItem::new()
                            .tag("nav-second")
                            .is_selected(false),
                    ),
                ]
            };
            let list_items = if remove_selected {
                vec![reactor::keyed(
                    "first",
                    reactor::ListBoxItem::new()
                        .tag("list-first")
                        .is_selected(false),
                )]
            } else if reversed {
                vec![
                    reactor::keyed(
                        "second",
                        reactor::ListBoxItem::new()
                            .tag("list-second")
                            .is_selected(true),
                    ),
                    reactor::keyed(
                        "first",
                        reactor::ListBoxItem::new()
                            .tag("list-first")
                            .is_selected(false),
                    ),
                ]
            } else {
                vec![
                    reactor::keyed(
                        "first",
                        reactor::ListBoxItem::new()
                            .tag("list-first")
                            .is_selected(true),
                    ),
                    reactor::keyed(
                        "second",
                        reactor::ListBoxItem::new()
                            .tag("list-second")
                            .is_selected(false),
                    ),
                ]
            };
            let selector_items = if remove_selected {
                vec![reactor::keyed(
                    "first",
                    reactor::SelectorBarItem::new()
                        .text("selector-first")
                        .is_selected(false),
                )]
            } else if reversed {
                vec![
                    reactor::keyed(
                        "second",
                        reactor::SelectorBarItem::new()
                            .text("selector-second")
                            .is_selected(true),
                    ),
                    reactor::keyed(
                        "first",
                        reactor::SelectorBarItem::new()
                            .text("selector-first")
                            .is_selected(false),
                    ),
                ]
            } else {
                vec![
                    reactor::keyed(
                        "first",
                        reactor::SelectorBarItem::new()
                            .text("selector-first")
                            .is_selected(true),
                    ),
                    reactor::keyed(
                        "second",
                        reactor::SelectorBarItem::new()
                            .text("selector-second")
                            .is_selected(false),
                    ),
                ]
            };
            reactor::StackPanel::new().children([
                reactor::NavigationView::new()
                    .keyed_menu_items(navigation_items)
                    .on_selected_tag_changed(navigation_callback.clone())
                    .into(),
                reactor::ListBox::new()
                    .keyed_items(list_items)
                    .on_selected_tag_changed(list_callback.clone())
                    .into(),
                reactor::SelectorBar::new()
                    .keyed_items(selector_items)
                    .on_selected_text_changed(selector_callback.clone())
                    .into(),
            ])
        };
        let mut selection_runtime = reactor::Runtime::new(reactor::native::WinUiAdapter::default());
        selection_runtime.update(selections(false, false)).unwrap();
        let selection_root = selection_runtime.graph().root().unwrap();
        let selection_window = selection_runtime
            .adapter()
            .open_window(selection_root)
            .unwrap();
        let selection_owners = selection_runtime
            .graph()
            .children(selection_root, reactor::RelationId::Children)
            .unwrap()
            .to_vec();
        let navigation_items = selection_runtime
            .graph()
            .children(selection_owners[0], reactor::RelationId::MenuItems)
            .unwrap()
            .to_vec();
        let list_items = selection_runtime
            .graph()
            .children(selection_owners[1], reactor::RelationId::Items)
            .unwrap()
            .to_vec();
        let selector_items = selection_runtime
            .graph()
            .children(selection_owners[2], reactor::RelationId::Items)
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

        let mut retirement_runtime =
            reactor::Runtime::new(reactor::native::WinUiAdapter::default());
        retirement_runtime
            .update(Self::retirement_children(false, true))
            .unwrap();
        let retirement_root = retirement_runtime.graph().root().unwrap();
        let retirement_window = retirement_runtime
            .adapter()
            .create_window(retirement_root)
            .unwrap();
        retirement_window.activate().unwrap();
        let initial_retirement_order = retirement_runtime
            .graph()
            .children(retirement_root, reactor::RelationId::Children)
            .unwrap()
            .to_vec();
        let retiring_object = initial_retirement_order[32];
        let retirement_expected_order = initial_retirement_order
            .iter()
            .copied()
            .filter(|object| *object != retiring_object)
            .rev()
            .collect::<Vec<_>>();
        let content_dialog_result = Rc::new(Cell::new(None));

        Self::schedule(context);
        Self {
            boundary_host,
            boundary_sender,
            tree_content_sender,
            tree_sender,
            boundary_window,
            pointer_runtime,
            pointer_window: None,
            retirement_runtime,
            retirement_window,
            retiring_object,
            retirement_expected_order,
            retirement_started: false,
            retirement_verified: false,
            content_dialog_runtime: None,
            content_dialog_window: None,
            content_dialog: None,
            content_dialog_result,
            content_dialog_hide_requested: false,
            content_dialog_verified: false,
            content_dialog_waits: 0,
            flyout_activated: false,
            flyout_shown: false,
            flyout_runtime: Some(flyout_runtime),
            flyout_window: Some(flyout_window),
            menu_activated: false,
            menu_shown: false,
            menu_runtime: Some(menu_runtime),
            menu_window: Some(menu_window),
            command_bar_flyout_activated: false,
            command_bar_flyout_shown: false,
            command_bar_flyout_runtime: Some(command_bar_flyout_runtime),
            command_bar_flyout_window: Some(command_bar_flyout_window),
            popup_waits: 0,
            encoded_image_activated: false,
            encoded_image_cleared: false,
            encoded_image_result,
            encoded_image_runtime: Some(encoded_image_runtime),
            encoded_image_validated: false,
            encoded_image_waits: 0,
            encoded_image_window: Some(encoded_image_window),
            received_pointer_moved,
            received_pointer_pressed,
            received_pointer,
            pointer_activated: false,
            pointer_injection_stage: 0,
            pointer_injection_waits: 0,
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

    fn update(&mut self, _message: Self::Message, context: &reactor::ComponentContext<Self>) {
        if !self.retirement_started {
            let root = self.retirement_runtime.graph().root().unwrap();
            if !self.retirement_runtime.adapter().is_attached(root).unwrap() {
                Self::schedule(context);
                return;
            }
            self.retirement_runtime
                .update(Self::retirement_children(true, false))
                .unwrap();
            let mut physical_with_retirement = self.retirement_expected_order.clone();
            physical_with_retirement.insert(32, self.retiring_object);
            assert_eq!(
                self.retirement_runtime
                    .adapter()
                    .owned_physical_children(root, reactor::RelationId::Children,)
                    .unwrap(),
                physical_with_retirement
            );
            assert_eq!(self.retirement_runtime.graph().retired_count(), 1);
            assert_eq!(self.retirement_runtime.adapter().retirement_count(), 1);
            assert!(
                self.retirement_runtime
                    .adapter()
                    .contains_object(self.retiring_object)
            );
            self.retirement_started = true;
            Self::schedule(context);
            return;
        }
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
                self.retirement_window.close().unwrap();
                assert_eq!(
                    self.retirement_runtime
                        .adapter()
                        .owned_physical_children(
                            self.retirement_runtime.graph().root().unwrap(),
                            reactor::RelationId::Children,
                        )
                        .unwrap(),
                    self.retirement_expected_order
                );
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
        if self.content_dialog_runtime.is_none() {
            let dialog_result = Rc::clone(&self.content_dialog_result);
            let dialog_view = move |open| {
                let dialog_result = Rc::clone(&dialog_result);
                reactor::Button::new()
                    .content("Dialog owner")
                    .content_dialog(
                        reactor::ContentDialog::new()
                            .title("Lifecycle")
                            .primary_button_text("Primary")
                            .secondary_button_text("Secondary")
                            .close_button_text("Close")
                            .is_primary_button_enabled(true)
                            .is_secondary_button_enabled(true)
                            .content(reactor::TextBlock::new().text("Live dialog"))
                            .is_open(open)
                            .on_closed(move |result| dialog_result.set(Some(result))),
                    )
            };
            let mut runtime = reactor::Runtime::new(reactor::native::WinUiAdapter::default());
            runtime.update(dialog_view(false)).unwrap();
            let root = runtime.graph().root().unwrap();
            let window = runtime.adapter().create_window(root).unwrap();
            window.activate().unwrap();
            runtime.update(dialog_view(true)).unwrap();
            self.content_dialog = Some(runtime.graph().content_dialog(root).unwrap().0);
            self.content_dialog_runtime = Some(runtime);
            self.content_dialog_window = Some(window);
            Self::schedule(context);
            return;
        }
        let content_dialog_runtime = self.content_dialog_runtime.as_mut().unwrap();
        content_dialog_runtime.dispatch_native_events().unwrap();
        if !self.content_dialog_verified {
            if !self.content_dialog_hide_requested {
                content_dialog_runtime
                    .adapter()
                    .hide_content_dialog(self.content_dialog.unwrap())
                    .unwrap();
                self.content_dialog_hide_requested = true;
                Self::schedule(context);
                return;
            }
            if self.content_dialog_result.get() == Some(reactor::ContentDialogResult::None) {
                content_dialog_runtime
                    .update(reactor::Button::new().content("Dialog owner"))
                    .unwrap();
                content_dialog_runtime
                    .adapter()
                    .validate_graph(content_dialog_runtime.graph())
                    .unwrap();
                self.content_dialog_window.take().unwrap().close().unwrap();
                self.content_dialog_verified = true;
            } else {
                self.content_dialog_waits += 1;
                if self.content_dialog_waits >= LIVE_EVENT_TIMEOUT_TICKS {
                    eprintln!("real WinUI ContentDialog.Closed event was not delivered");
                    std::process::exit(1);
                }
                Self::schedule(context);
                return;
            }
        }
        if let Some(flyout_runtime) = self.flyout_runtime.as_ref() {
            if !self.flyout_activated {
                self.flyout_window.as_ref().unwrap().activate().unwrap();
                self.flyout_activated = true;
                Self::schedule(context);
                return;
            }
            let target = flyout_runtime.graph().root().unwrap();
            if !self.flyout_shown {
                self.flyout_shown = true;
                if !flyout_runtime.adapter().show_flyout(target).unwrap() {
                    Self::schedule(context);
                    return;
                }
            } else if !flyout_runtime.adapter().is_flyout_open(target).unwrap() {
                self.popup_waits += 1;
                if self.popup_waits >= LIVE_EVENT_TIMEOUT_TICKS {
                    eprintln!("real WinUI flyout did not open");
                    std::process::exit(1);
                }
                Self::schedule(context);
                return;
            }
            self.popup_waits = 0;
            flyout_runtime.adapter().hide_flyout(target).unwrap();
            self.flyout_window.take().unwrap().close().unwrap();
            self.flyout_runtime.take();
        }
        if let Some(menu_runtime) = self.menu_runtime.as_ref() {
            if !self.menu_activated {
                self.menu_window.as_ref().unwrap().activate().unwrap();
                self.menu_activated = true;
                Self::schedule(context);
                return;
            }
            let target = menu_runtime.graph().root().unwrap();
            if !self.menu_shown {
                self.menu_shown = true;
                if !menu_runtime.adapter().show_menu(target).unwrap() {
                    Self::schedule(context);
                    return;
                }
            } else if !menu_runtime.adapter().is_menu_open(target).unwrap() {
                self.popup_waits += 1;
                if self.popup_waits >= LIVE_EVENT_TIMEOUT_TICKS {
                    eprintln!("real WinUI menu did not open");
                    std::process::exit(1);
                }
                Self::schedule(context);
                return;
            }
            self.popup_waits = 0;
            menu_runtime.adapter().hide_menu(target).unwrap();
            self.menu_window.take().unwrap().close().unwrap();
            self.menu_runtime.take();
        }
        if let Some(runtime) = self.command_bar_flyout_runtime.as_ref() {
            if !self.command_bar_flyout_activated {
                self.command_bar_flyout_window
                    .as_ref()
                    .unwrap()
                    .activate()
                    .unwrap();
                self.command_bar_flyout_activated = true;
                Self::schedule(context);
                return;
            }
            let target = runtime.graph().root().unwrap();
            if !self.command_bar_flyout_shown {
                self.command_bar_flyout_shown = true;
                if !runtime.adapter().show_command_bar_flyout(target).unwrap() {
                    Self::schedule(context);
                    return;
                }
            } else if !runtime
                .adapter()
                .is_command_bar_flyout_open(target)
                .unwrap()
            {
                self.popup_waits += 1;
                if self.popup_waits >= LIVE_EVENT_TIMEOUT_TICKS {
                    eprintln!("real WinUI command-bar flyout did not open");
                    std::process::exit(1);
                }
                Self::schedule(context);
                return;
            }
            self.popup_waits = 0;
            runtime.adapter().hide_command_bar_flyout(target).unwrap();
            self.command_bar_flyout_window
                .take()
                .unwrap()
                .close()
                .unwrap();
            self.command_bar_flyout_runtime.take();
        }
        if let Some(encoded_image_runtime) = self.encoded_image_runtime.as_mut() {
            if !self.encoded_image_activated {
                self.encoded_image_window
                    .as_ref()
                    .unwrap()
                    .activate()
                    .unwrap();
                let encoded_image_opened = Rc::clone(&self.encoded_image_result);
                let encoded_image_failed = Rc::clone(&self.encoded_image_result);
                encoded_image_runtime
                    .update(
                        reactor::Image::new()
                            .width(1.0)
                            .height(1.0)
                            .source_data(reactor::EncodedImage::from_static(ENCODED_IMAGE_PNG))
                            .on_opened(move || encoded_image_opened.set(Some(true)))
                            .on_failed(move || encoded_image_failed.set(Some(false))),
                    )
                    .unwrap();
                assert!(
                    encoded_image_runtime
                        .adapter()
                        .image_has_decode_failure_subscription(
                            encoded_image_runtime.graph().root().unwrap()
                        )
                );
                self.encoded_image_activated = true;
                Self::schedule(context);
                return;
            }
            encoded_image_runtime.dispatch_native_events().unwrap();
            match self.encoded_image_result.get() {
                Some(true) if !self.encoded_image_validated => {
                    let encoded_image_opened = Rc::clone(&self.encoded_image_result);
                    let encoded_image_failed = Rc::clone(&self.encoded_image_result);
                    self.encoded_image_result.set(None);
                    self.encoded_image_waits = 0;
                    self.encoded_image_validated = true;
                    encoded_image_runtime
                        .update(
                            reactor::Image::new()
                                .width(1.0)
                                .height(1.0)
                                .source_data(reactor::EncodedImage::from_static(&[0, 1, 2]))
                                .on_opened(move || encoded_image_opened.set(Some(true)))
                                .on_failed(move || encoded_image_failed.set(Some(false))),
                        )
                        .unwrap();
                    Self::schedule(context);
                    return;
                }
                Some(false) if self.encoded_image_validated && !self.encoded_image_cleared => {
                    let encoded_image_opened = Rc::clone(&self.encoded_image_result);
                    let encoded_image_failed = Rc::clone(&self.encoded_image_result);
                    encoded_image_runtime
                        .update(
                            reactor::Image::new()
                                .width(1.0)
                                .height(1.0)
                                .source_data(reactor::EncodedImage::from_static(&[3, 4, 5]))
                                .on_opened(move || encoded_image_opened.set(Some(true)))
                                .on_failed(move || encoded_image_failed.set(Some(false))),
                        )
                        .unwrap();
                    let encoded_image_opened = Rc::clone(&self.encoded_image_result);
                    let encoded_image_failed = Rc::clone(&self.encoded_image_result);
                    encoded_image_runtime
                        .update(
                            reactor::Image::new()
                                .width(1.0)
                                .height(1.0)
                                .on_opened(move || encoded_image_opened.set(Some(true)))
                                .on_failed(move || encoded_image_failed.set(Some(false))),
                        )
                        .unwrap();
                    assert!(
                        !encoded_image_runtime
                            .adapter()
                            .image_has_decode_failure_subscription(
                                encoded_image_runtime.graph().root().unwrap()
                            )
                    );
                    self.encoded_image_result.set(None);
                    self.encoded_image_waits = 0;
                    self.encoded_image_cleared = true;
                    Self::schedule(context);
                    return;
                }
                Some(_) if self.encoded_image_cleared => {
                    eprintln!("cleared encoded image delivered a stale decode event");
                    std::process::exit(1);
                }
                Some(true) => {
                    eprintln!("invalid encoded image unexpectedly raised ImageOpened");
                    std::process::exit(1);
                }
                Some(false) => {
                    eprintln!("valid encoded image unexpectedly raised ImageFailed");
                    std::process::exit(1);
                }
                None => {
                    self.encoded_image_waits += 1;
                    let timeout = if self.encoded_image_cleared {
                        LIVE_QUIET_TICKS
                    } else {
                        LIVE_EVENT_TIMEOUT_TICKS
                    };
                    if self.encoded_image_waits >= timeout {
                        if self.encoded_image_cleared {
                            self.encoded_image_window.take().unwrap().close().unwrap();
                            self.encoded_image_runtime.take();
                        } else if self.encoded_image_validated {
                            eprintln!("real WinUI encoded ImageFailed event was not delivered");
                            std::process::exit(1);
                        } else {
                            eprintln!("real WinUI encoded ImageOpened event was not delivered");
                            std::process::exit(1);
                        }
                    }
                    if self.encoded_image_runtime.is_some() {
                        Self::schedule(context);
                        return;
                    }
                }
            }
        }
        if !self.pointer_activated {
            let root = self.pointer_runtime.graph().root().unwrap();
            let window = self
                .pointer_runtime
                .adapter()
                .create_window_with_policy(
                    root,
                    &reactor::WindowPolicy::new()
                        .title("Reactor pointer self-test")
                        .client_size(200.0, 200.0),
                )
                .unwrap();
            window.activate().unwrap();
            self.pointer_window = Some(window);
            self.pointer_activated = true;
            Self::schedule(context);
            return;
        }
        self.pointer_runtime.dispatch_native_events().unwrap();
        if self.pointer_injection_stage < 3 {
            let window = self.pointer_window.as_ref().unwrap();
            let raw = window.raw_handle().unwrap();
            let hwnd: windows::Win32::HWND = raw.cast();
            if self.pointer_injection_stage == 0 && unsafe { GetForegroundWindow() } != hwnd {
                self.pointer_injection_waits += 1;
                window.activate().unwrap();
                if self.pointer_injection_waits >= LIVE_EVENT_TIMEOUT_TICKS {
                    eprintln!("real WinUI pointer window did not become foreground");
                    std::process::exit(1);
                }
                Self::schedule(context);
                return;
            }
            let ready = match self.pointer_injection_stage {
                0 => true,
                1 => self.received_pointer_moved.get(),
                2 => self.received_pointer_pressed.get(),
                _ => unreachable!(),
            };
            if !ready {
                self.pointer_injection_waits += 1;
                if self.pointer_injection_stage == 1
                    && self.pointer_injection_waits.is_multiple_of(10)
                {
                    let window = self.pointer_window.as_ref().unwrap();
                    window.activate().unwrap();
                    let hwnd = window.raw_handle().unwrap();
                    if let Err(error) = Self::inject_pointer(hwnd, 0) {
                        eprintln!("{error}");
                        std::process::exit(1);
                    }
                }
                if self.pointer_injection_waits >= LIVE_EVENT_TIMEOUT_TICKS {
                    eprintln!(
                        "real WinUI pointer stage {} was not delivered",
                        self.pointer_injection_stage - 1
                    );
                    std::process::exit(1);
                }
                Self::schedule(context);
                return;
            }
            match Self::inject_pointer(raw, self.pointer_injection_stage) {
                Ok(()) => {
                    self.pointer_injection_stage += 1;
                    self.pointer_injection_waits = 0;
                }
                Err(error) => {
                    eprintln!("{error}");
                    std::process::exit(1);
                }
            }
            Self::schedule(context);
            return;
        }
        if let Some(pointer) = *self.received_pointer.borrow() {
            assert!(pointer.pointer_id > 0);
            assert!(!pointer.is_captured);
            assert_eq!(pointer.capture_succeeded, None);
        } else {
            self.pointer_waits += 1;
            if self.pointer_waits >= LIVE_EVENT_TIMEOUT_TICKS {
                eprintln!("real WinUI PointerReleased event was not delivered");
                std::process::exit(1);
            }
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
            .children(root, reactor::RelationId::Children)
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
        assert_eq!(
            self.runtime
                .adapter()
                .text_box_appearance(text_box)
                .unwrap(),
            if self.iteration.is_multiple_of(2) {
                ("Enter text".to_string(), true, reactor::TextWrapping::Wrap)
            } else {
                (String::new(), false, reactor::TextWrapping::NoWrap)
            }
        );
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
            self.pointer_window.take().unwrap().close().unwrap();
            self.boundary_window.close().unwrap();
            self.window.close().unwrap();
            assert!(context.close_window());
        } else {
            assert!(self.boundary_sender.send(()));
            assert!(self.tree_sender.send(()));
            assert!(self.tree_content_sender.send(()));
            Self::schedule(context);
        }
    }

    fn view(
        &self,
        _input: &Self::Input,
        _context: &mut reactor::ViewContext<Self>,
    ) -> reactor::View {
        reactor::TextBlock::new()
            .text(format!("Reactor TreeView stress: {}", self.iteration))
            .into()
    }
}

fn main() -> windows_core::Result<()> {
    std::thread::spawn(|| {
        std::thread::sleep(Duration::from_secs(60));
        eprintln!("Reactor native self-test timed out");
        std::process::exit(1);
    });
    App::run_component::<Fixture>(())
}
