use windows_core::{Error, HRESULT};
use windows_reactor::*;

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

#[test]
fn generated_builders_convert_to_views() {
    let optional_text = Some("value");
    let optional_enabled = Some(true);
    let text = TextBlock::new()
        .text("hello")
        .font_size(28.0)
        .text_wrapping(TextWrapping::Wrap);
    let stack = StackPanel::new()
        .orientation(Orientation::Horizontal)
        .spacing(8.0);
    let button = Button::new().is_enabled(false);
    let text_box = TextBox::new()
        .text_optional(optional_text)
        .placeholder_text("hint")
        .is_enabled(optional_enabled);
    let number_box = NumberBox::new().value(None);
    let slider = Slider::new().value(Some(10.0));
    let toggle = ToggleSwitch::new().is_on(None);
    let grid = Grid::new()
        .rows_optional(Some([GridLength::Auto, GridLength::STAR]))
        .columns_optional(None::<[GridLength; 0]>);
    let border = Border::new()
        .padding(Thickness::uniform(24.0))
        .border_thickness(1.0)
        .corner_radius(CornerRadius::uniform(8.0))
        .background(ThemeBrush::CardBackground)
        .border_brush(ThemeBrush::CardStroke);

    let _: View = text.into();
    let _: View = stack.into();
    let _: View = button.into();
    let _: View = text_box.into();
    let _: View = number_box.into();
    let _: View = slider.into();
    let _: View = toggle.into();
    let _: View = grid.into();
    let _: View = border.into();
}

#[test]
fn generated_structural_capabilities_compose_views() {
    let _: View = Button::new().content(TextBlock::new().text("button"));
    let _: View = Border::new().content(TextBlock::new().text("card"));
    let _: View = StackPanel::new().keyed_children([
        ("first", TextBlock::new().text("one")),
        ("second", TextBlock::new().text("two")),
    ]);
    let repeater = ItemsRepeater::new()
        .item("first", TextBlock::new().text("one"))
        .items([(2_u64, View::component::<TestComponent>("two".to_string()))]);
    let _: View = ScrollViewer::new().content(repeater);
    let _: View = NavigationView::new()
        .content(TextBlock::new())
        .header(Button::new())
        .into();
    let _: View = NavigationView::new()
        .menu_items([
            ("first", NavigationViewItem::new().content("one")),
            ("second", NavigationViewItem::new().content("two")),
        ])
        .into();
    let _: View = NavigationView::new()
        .footer_menu_items([("settings", NavigationViewItem::new().content("Settings"))])
        .into();
    let _: View = View::keyed_fragment([
        ("first", TextBlock::new().text("one")),
        ("second", TextBlock::new().text("two")),
    ]);
    let _: KeyedView = ("key", TextBlock::new().text("value")).into();
    let _: View = TitleBar::new()
        .preferred_height(WindowTitleBarHeight::Tall)
        .into();
    let _: View = TitleBar::new().into();
}

struct TestComponent;

impl Component for TestComponent {
    type Message = ();
    type Input = String;

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {}

    fn view(&self, input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        TextBlock::new().text(input.clone()).into()
    }
}

#[test]
fn positional_children_accept_heterogeneous_tuples_without_leaf_conversions() {
    let _: View = StackPanel::new().children((
        TextBlock::new().text("one"),
        TextBox::new().placeholder_text("two"),
        Button::new().content(TextBlock::new().text("button")),
    ));
}

#[test]
fn positional_children_accept_fixed_arrays() {
    let _: View = StackPanel::new().children([
        TextBlock::new().text("first"),
        TextBlock::new().text("second"),
    ]);
}

struct WindowVisualComponent;

impl Component for WindowVisualComponent {
    type Message = ();
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {}

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_visuals(
            WindowVisuals::new()
                .theme(WindowTheme::Dark)
                .backdrop(WindowBackdrop::Mica)
                .client_size(1400.0, 900.0),
        );
        TextBlock::new().into()
    }
}

#[test]
fn window_visual_environment_is_public() {
    let _: View = View::component::<WindowVisualComponent>(());
}

#[derive(Clone)]
struct ClosingWindowInput {
    dropped: Arc<AtomicBool>,
}

impl PartialEq for ClosingWindowInput {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.dropped, &other.dropped)
    }
}

struct ClosingWindow {
    _close: ComponentTask,
    dropped: Arc<AtomicBool>,
}

impl Component for ClosingWindow {
    type Input = ClosingWindowInput;
    type Message = ();

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        Self {
            _close: context.spawn_background(|_| {
                std::thread::sleep(Duration::from_millis(100));
            }),
            dropped: Arc::clone(&input.dropped),
        }
    }

    fn update(&mut self, _message: (), context: &ComponentContext<Self>) {
        assert!(context.window().request_close());
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_frame("Application lifetime test", "Closing...")
    }
}

impl Drop for ClosingWindow {
    fn drop(&mut self) {
        self.dropped.store(true, Ordering::Release);
    }
}

#[derive(Clone)]
struct HoldingWindowInput(Arc<AtomicBool>, Arc<AtomicBool>);

impl PartialEq for HoldingWindowInput {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0) && Arc::ptr_eq(&self.1, &other.1)
    }
}

struct HoldingWindow {
    dropped: Arc<AtomicBool>,
}

impl Component for HoldingWindow {
    type Input = HoldingWindowInput;
    type Message = ();

    fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        input.0.store(true, Ordering::Release);
        Self {
            dropped: Arc::clone(&input.1),
        }
    }

    fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {}

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_frame("Application exit test", "Waiting for explicit exit...")
    }
}

impl Drop for HoldingWindow {
    fn drop(&mut self) {
        self.dropped.store(true, Ordering::Release);
    }
}

#[derive(Clone)]
struct ReplacementWindowInput(Arc<AtomicBool>);

impl PartialEq for ReplacementWindowInput {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

struct ReplacementWindow {
    _close: ComponentTask,
}

impl Component for ReplacementWindow {
    type Input = ReplacementWindowInput;
    type Message = ();

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        input.0.store(true, Ordering::Release);
        Self {
            _close: context.spawn_background(|_| {
                std::thread::sleep(Duration::from_millis(100));
            }),
        }
    }

    fn update(&mut self, _message: (), context: &ComponentContext<Self>) {
        assert!(context.window().request_close());
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_frame("Replacement window", "Mounted")
    }
}

struct ReplacingWindow {
    mounted: Arc<AtomicBool>,
    _replace: ComponentTask,
}

impl Component for ReplacingWindow {
    type Input = ReplacementWindowInput;
    type Message = ();

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        Self {
            mounted: Arc::clone(&input.0),
            _replace: context.spawn_background(|_| {
                std::thread::sleep(Duration::from_millis(100));
            }),
        }
    }

    fn update(&mut self, _message: (), context: &ComponentContext<Self>) {
        assert!(context.open_window(View::component::<ReplacementWindow>(
            ReplacementWindowInput(Arc::clone(&self.mounted)),
        )));
        assert!(context.window().request_close());
    }

    fn view(&self, _input: &ReplacementWindowInput, context: &mut ViewContext<Self>) -> View {
        context.window_frame("Replacing window", "Opening replacement...")
    }
}

#[derive(Clone)]
struct TimedClosingWindowInput {
    closed: Arc<AtomicBool>,
    delay: Duration,
}

impl PartialEq for TimedClosingWindowInput {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.closed, &other.closed) && self.delay == other.delay
    }
}

struct TimedClosingWindow {
    closed: Arc<AtomicBool>,
    _close: ComponentTask,
}

impl Component for TimedClosingWindow {
    type Input = TimedClosingWindowInput;
    type Message = ();

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        let delay = input.delay;
        Self {
            closed: Arc::clone(&input.closed),
            _close: context.spawn_background(move |_| std::thread::sleep(delay)),
        }
    }

    fn update(&mut self, _message: (), context: &ComponentContext<Self>) {
        self.closed.store(true, Ordering::Release);
        assert!(context.window().request_close());
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_frame("Timed window", "Waiting to close...")
    }
}

struct AppResource {
    dropped: Arc<AtomicBool>,
    worker: Option<std::thread::JoinHandle<()>>,
}

impl Drop for AppResource {
    fn drop(&mut self) {
        self.worker.take().unwrap().join().unwrap();
        self.dropped.store(true, Ordering::Release);
    }
}

#[test]
#[ignore = "runs the interactive WinUI application loop"]
fn last_window_can_be_replaced_in_one_publication() {
    let mounted = Arc::new(AtomicBool::new(false));
    App::run_component::<ReplacingWindow>(ReplacementWindowInput(Arc::clone(&mounted))).unwrap();
    assert!(mounted.load(Ordering::Acquire));
}

#[test]
#[ignore = "runs the interactive WinUI application loop"]
fn multiple_windows_exit_after_all_have_closed() {
    let first_closed = Arc::new(AtomicBool::new(false));
    let second_closed = Arc::new(AtomicBool::new(false));
    App::run_windows([
        View::component::<TimedClosingWindow>(TimedClosingWindowInput {
            closed: Arc::clone(&first_closed),
            delay: Duration::from_millis(100),
        }),
        View::component::<TimedClosingWindow>(TimedClosingWindowInput {
            closed: Arc::clone(&second_closed),
            delay: Duration::from_millis(300),
        }),
    ])
    .unwrap();
    assert!(first_closed.load(Ordering::Acquire));
    assert!(second_closed.load(Ordering::Acquire));
}

#[test]
#[ignore = "runs the interactive WinUI application loop"]
fn application_lifetime_is_independent_of_windows() {
    let completed = Arc::new(AtomicBool::new(false));
    let watchdog_completed = Arc::clone(&completed);
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_secs(20));
        if !watchdog_completed.load(Ordering::Acquire) {
            eprintln!("Reactor application lifetime test timed out");
            std::process::exit(1);
        }
    });

    let opened = Arc::new(AtomicBool::new(false));
    let window_dropped = Arc::new(AtomicBool::new(false));
    let active_window = Arc::new(AtomicBool::new(false));
    let active_window_dropped = Arc::new(AtomicBool::new(false));
    let explicit_exit = Arc::new(AtomicBool::new(false));
    let resource_dropped = Arc::new(AtomicBool::new(false));

    App::run_with({
        let opened = Arc::clone(&opened);
        let window_dropped = Arc::clone(&window_dropped);
        let active_window = Arc::clone(&active_window);
        let active_window_dropped = Arc::clone(&active_window_dropped);
        let explicit_exit = Arc::clone(&explicit_exit);
        let resource_dropped = Arc::clone(&resource_dropped);
        move |app| {
            let proxy = app.proxy();
            let worker = std::thread::spawn(move || {
                std::thread::sleep(Duration::from_millis(50));
                let opened_window = Arc::clone(&opened);
                let dropped = Arc::clone(&window_dropped);
                proxy
                    .dispatch(move |app| {
                        app.open_window(View::component::<ClosingWindow>(ClosingWindowInput {
                            dropped,
                        }))
                        .unwrap();
                        opened_window.store(true, Ordering::Release);
                    })
                    .unwrap();

                let deadline = Instant::now() + Duration::from_secs(5);
                while Instant::now() < deadline && !window_dropped.load(Ordering::Acquire) {
                    std::thread::sleep(Duration::from_millis(10));
                }
                std::thread::sleep(Duration::from_millis(500));

                let mounted = Arc::clone(&active_window);
                let dropped = Arc::clone(&active_window_dropped);
                proxy
                    .dispatch(move |app| {
                        app.open_window(View::component::<HoldingWindow>(HoldingWindowInput(
                            mounted, dropped,
                        )))
                        .unwrap();
                    })
                    .unwrap();
                let deadline = Instant::now() + Duration::from_secs(5);
                while Instant::now() < deadline && !active_window.load(Ordering::Acquire) {
                    std::thread::sleep(Duration::from_millis(10));
                }

                proxy
                    .dispatch(move |app| {
                        explicit_exit.store(true, Ordering::Release);
                        app.exit().unwrap();
                    })
                    .unwrap();
            });
            Ok(AppResource {
                dropped: resource_dropped,
                worker: Some(worker),
            })
        }
    })
    .unwrap();

    assert!(opened.load(Ordering::Acquire));
    assert!(window_dropped.load(Ordering::Acquire));
    assert!(active_window.load(Ordering::Acquire));
    assert!(active_window_dropped.load(Ordering::Acquire));
    assert!(explicit_exit.load(Ordering::Acquire));
    assert!(resource_dropped.load(Ordering::Acquire));
    completed.store(true, Ordering::Release);
}

#[test]
#[ignore = "runs the interactive WinUI application loop"]
fn application_startup_error_is_returned() {
    let expected = HRESULT(0x8000_4005_u32 as i32);
    let error =
        App::run_with(move |_| Err::<(), _>(Error::new(expected, "startup failed"))).unwrap_err();
    assert_eq!(error.code(), expected);
}

#[test]
#[ignore = "runs the interactive WinUI application loop"]
fn excessive_startup_windows_are_rejected() {
    assert!(
        App::run_with(|app| {
            app.open_windows(
                (0..100).map(|index| TextBlock::new().text(format!("Window {index}")).into()),
            )
        })
        .is_err()
    );
}

#[test]
fn app_proxy_is_send_and_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<AppProxy>();
}
