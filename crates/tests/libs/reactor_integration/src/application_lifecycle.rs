use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use windows_core::{Error, HRESULT};
use windows_reactor::{
    App, Component, ComponentContext, ComponentTimer, TextBlock, View, ViewContext,
};

#[derive(Clone)]
struct WindowInput {
    closed: Arc<AtomicBool>,
    delay: Duration,
}

impl PartialEq for WindowInput {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.closed, &other.closed) && self.delay == other.delay
    }
}

struct ClosingWindow {
    closed: Arc<AtomicBool>,
    _timer: ComponentTimer,
}

impl Component for ClosingWindow {
    type Input = WindowInput;
    type Message = ();

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        Self {
            closed: Arc::clone(&input.closed),
            _timer: context.set_timeout(input.delay, ()),
        }
    }

    fn update(&mut self, _message: Self::Message, context: &ComponentContext<Self>) {
        self.closed.store(true, Ordering::Release);
        assert!(context.close_window());
    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        TextBlock::new().text("Closing").into()
    }
}

#[derive(Clone)]
struct ReplacementInput {
    mounted: Arc<AtomicBool>,
}

impl PartialEq for ReplacementInput {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.mounted, &other.mounted)
    }
}

struct ReplacementWindow {
    _timer: ComponentTimer,
}

impl Component for ReplacementWindow {
    type Input = ReplacementInput;
    type Message = ();

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        input.mounted.store(true, Ordering::Release);
        Self {
            _timer: context.set_timeout(Duration::from_millis(100), ()),
        }
    }

    fn update(&mut self, _message: Self::Message, context: &ComponentContext<Self>) {
        assert!(context.close_window());
    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        TextBlock::new().text("Replacement").into()
    }
}

struct ReplacingWindow {
    mounted: Arc<AtomicBool>,
    _timer: ComponentTimer,
}

impl Component for ReplacingWindow {
    type Input = ReplacementInput;
    type Message = ();

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        Self {
            mounted: Arc::clone(&input.mounted),
            _timer: context.set_timeout(Duration::from_millis(100), ()),
        }
    }

    fn update(&mut self, _message: Self::Message, context: &ComponentContext<Self>) {
        assert!(context.open_window::<ReplacementWindow>(ReplacementInput {
            mounted: Arc::clone(&self.mounted),
        }));
        assert!(context.close_window());
    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        TextBlock::new().text("Replacing").into()
    }
}

#[derive(Clone)]
struct MultipleWindowInput {
    first_closed: Arc<AtomicBool>,
    second_closed: Arc<AtomicBool>,
}

impl PartialEq for MultipleWindowInput {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.first_closed, &other.first_closed)
            && Arc::ptr_eq(&self.second_closed, &other.second_closed)
    }
}

struct MultipleWindowRoot {
    first_closed: Arc<AtomicBool>,
    _timer: ComponentTimer,
}

impl Component for MultipleWindowRoot {
    type Input = MultipleWindowInput;
    type Message = ();

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        assert!(context.open_window::<ClosingWindow>(WindowInput {
            closed: Arc::clone(&input.second_closed),
            delay: Duration::from_millis(300),
        }));
        Self {
            first_closed: Arc::clone(&input.first_closed),
            _timer: context.set_timeout(Duration::from_millis(100), ()),
        }
    }

    fn update(&mut self, _message: Self::Message, context: &ComponentContext<Self>) {
        self.first_closed.store(true, Ordering::Release);
        assert!(context.close_window());
    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        TextBlock::new().text("First").into()
    }
}

fn replacement() -> windows_core::Result<()> {
    let mounted = Arc::new(AtomicBool::new(false));
    App::run_component::<ReplacingWindow>(ReplacementInput {
        mounted: Arc::clone(&mounted),
    })?;
    assert!(mounted.load(Ordering::Acquire));
    Ok(())
}

fn multiple() -> windows_core::Result<()> {
    let first_closed = Arc::new(AtomicBool::new(false));
    let second_closed = Arc::new(AtomicBool::new(false));
    App::run_component::<MultipleWindowRoot>(MultipleWindowInput {
        first_closed: Arc::clone(&first_closed),
        second_closed: Arc::clone(&second_closed),
    })?;
    assert!(first_closed.load(Ordering::Acquire));
    assert!(second_closed.load(Ordering::Acquire));
    Ok(())
}

fn startup_error() -> windows_core::Result<()> {
    let expected = HRESULT(0x8000_4005_u32 as i32);
    let error =
        App::run_with(move |_| Err::<(), _>(Error::new(expected, "startup failed"))).unwrap_err();
    assert_eq!(error.code(), expected);
    Ok(())
}

fn main() -> windows_core::Result<()> {
    std::thread::spawn(|| {
        std::thread::sleep(Duration::from_secs(30));
        eprintln!("Reactor application lifecycle fixture timed out");
        std::process::exit(1);
    });
    match std::env::args().nth(1).as_deref() {
        Some("replacement") => replacement(),
        Some("multiple") => multiple(),
        Some("startup-error") => startup_error(),
        _ => panic!("expected replacement, multiple, or startup-error lifecycle case"),
    }
}
