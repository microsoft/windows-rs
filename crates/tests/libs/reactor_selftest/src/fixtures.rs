use std::cell::Cell;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use windows::UI::Input::Preview::Injection::{
    InjectedInputMouseInfo, InjectedInputMouseOptions, InputInjector,
};
use windows::Win32::winuser::{
    BringWindowToTop, ClientToScreen, GetClientRect, GetSystemMetrics, SM_CXVIRTUALSCREEN,
    SM_CYVIRTUALSCREEN, SM_XVIRTUALSCREEN, SM_YVIRTUALSCREEN, SetForegroundWindow,
};
use windows::Win32::{HWND, POINT, RECT};
use windows_canvas::{CanvasImageSource, ColorF, GpuDevice, animated_canvas};
use windows_collections::IIterable;
use windows_composition::{
    Color as CompositionColor, Compositor as CompositionCompositor, ContainerVisual, SpriteVisual,
};
use windows_reactor::*;

pub type FixtureResult = Result<(), String>;

#[derive(Clone, PartialEq)]
pub struct FixtureInput {
    pub complete: Callback<FixtureResult>,
}

#[derive(Clone, PartialEq)]
pub(crate) struct ProbeInput {
    pub complete: Callback<FixtureResult>,
    pub probe: LiveProbe,
}

pub(crate) struct ProbeFixture {
    started: bool,
}

pub(crate) enum ProbeMessage {
    WindowReady,
}

impl Component for ProbeFixture {
    type Input = ProbeInput;
    type Message = ProbeMessage;

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self { started: false }
    }

    fn update(&mut self, _message: Self::Message, _context: &ComponentContext<Self>) {
        self.started = true;
    }

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.on_window_size(context.callback(|_| ProbeMessage::WindowReady));
        if self.started {
            let complete = input.complete.clone();
            let probe = input.probe;
            context.use_effect("run-probe", (), move || {
                if let Err(error) = schedule_live_probe(probe, move |result| {
                    if !complete.call(result) {
                        eprintln!("{probe:?} fixture completion was rejected");
                        std::process::exit(1);
                    }
                }) {
                    eprintln!("could not schedule {probe:?} fixture: {error}");
                    std::process::exit(1);
                }
                None
            });
        }
        TextBlock::new().text(format!("{:?}", input.probe)).into()
    }
}

enum FocusStage {
    Mounted,
    Removed,
}

pub(crate) enum FocusMessage {
    Focused(Result<bool, FocusError>),
}

pub(crate) struct FocusPublication {
    complete: Callback<FixtureResult>,
    reference: ElementRef<TextBox>,
    stage: FocusStage,
}

impl Component for FocusPublication {
    type Input = FixtureInput;
    type Message = FocusMessage;

    fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            complete: input.complete.clone(),
            reference: ElementRef::new(),
            stage: FocusStage::Mounted,
        }
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
        self.complete = input.complete.clone();
    }

    fn update(&mut self, message: Self::Message, _context: &ComponentContext<Self>) {
        let failure = match message {
            FocusMessage::Focused(Ok(_)) => {
                self.stage = FocusStage::Removed;
                None
            }
            FocusMessage::Focused(Err(error)) => Some(format!("focus request failed: {error:?}")),
        };
        if let Some(failure) = failure
            && !self.complete.call(Err(failure))
        {
            eprintln!("focus fixture completion was rejected");
            std::process::exit(1);
        }
    }

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        match self.stage {
            FocusStage::Mounted => {
                let reference = self.reference.clone();
                let sender = context.sender();
                context.use_effect("request-focus", (), move || {
                    assert!(reference.request_focus_result(move |result| {
                        sender.send(FocusMessage::Focused(result));
                    }));
                    None
                });
                TextBox::new().element_ref(&self.reference).into()
            }
            FocusStage::Removed => {
                let reference = self.reference.clone();
                let complete = input.complete.clone();
                context.use_effect("verify-retirement", (), move || {
                    let result = if reference.request_focus() {
                        Err("retired TextBox reference remained bound".to_string())
                    } else {
                        Ok(())
                    };
                    if !complete.call(result) {
                        eprintln!("focus fixture completion was rejected");
                        std::process::exit(1);
                    }
                    None
                });
                TextBlock::new().text("focus fixture complete").into()
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum KeyedMutationStage {
    Initial,
    Removed,
    Inserted,
    Moved,
    Retained,
}

pub(crate) struct KeyedNativeMutations {
    stage: KeyedMutationStage,
}

impl Component for KeyedNativeMutations {
    type Input = FixtureInput;
    type Message = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            stage: KeyedMutationStage::Initial,
        }
    }

    fn update(&mut self, (): Self::Message, _context: &ComponentContext<Self>) {
        self.stage = match self.stage {
            KeyedMutationStage::Initial => KeyedMutationStage::Removed,
            KeyedMutationStage::Removed => KeyedMutationStage::Inserted,
            KeyedMutationStage::Inserted => KeyedMutationStage::Moved,
            KeyedMutationStage::Moved => KeyedMutationStage::Retained,
            KeyedMutationStage::Retained => return,
        };
    }

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        const EXITING_KEY: usize = usize::MAX;

        let mut keys = match self.stage {
            KeyedMutationStage::Initial => (0..320).collect::<Vec<_>>(),
            KeyedMutationStage::Removed => (0..20).collect(),
            KeyedMutationStage::Inserted => (0..220).collect(),
            KeyedMutationStage::Moved | KeyedMutationStage::Retained => (0..220).rev().collect(),
        };
        if self.stage == KeyedMutationStage::Retained {
            keys.push(1_000);
        } else {
            keys.push(EXITING_KEY);
        }

        if self.stage == KeyedMutationStage::Retained {
            let complete = input.complete.clone();
            context.use_effect("complete-keyed-mutations", (), move || {
                if !complete.call(Ok(())) {
                    eprintln!("keyed mutation fixture completion was rejected");
                    std::process::exit(1);
                }
                None
            });
        } else {
            let sender = context.sender();
            context.use_effect("advance-keyed-mutations", self.stage, move || {
                if !sender.send(()) {
                    eprintln!("keyed mutation fixture update was rejected");
                    std::process::exit(1);
                }
                None
            });
        }

        Grid::new().keyed_children(keys.into_iter().map(|key| {
            let child: View = if key == EXITING_KEY {
                Border::new()
                    .exit_transition(ExitTransition::fade(Duration::from_millis(200)))
                    .content(TextBlock::new().text("exiting"))
            } else {
                TextBlock::new().text(key.to_string()).into()
            };
            KeyedView::new(key, child)
        }))
    }
}

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum PointerStage {
    Move,
    LeftDown,
    MoveOutside,
    LeftUp,
    RightDown,
    RightUp,
    Exit,
}

#[derive(Clone, Copy)]
pub(crate) enum PointerMessage {
    Entered,
    Moved(PointerEventInfo),
    Pressed(PointerEventInfo),
    Released,
    Exited,
    Retry(PointerStage),
}

pub(crate) struct PointerInjection {
    stage: PointerStage,
    entered: bool,
    attempt: u8,
    complete: Callback<FixtureResult>,
    injector: Result<Rc<InputInjector>, String>,
}

impl PointerInjection {
    fn fail(&self, detail: impl Into<String>) {
        if !self.complete.call(Err(detail.into())) {
            eprintln!("pointer fixture failure was rejected");
            std::process::exit(1);
        }
    }

    fn retry(&self, context: &ComponentContext<Self>) {
        let stage = self.stage;
        context.spawn_background(move |_| {
            std::thread::sleep(Duration::from_millis(100));
            PointerMessage::Retry(stage)
        });
    }

    fn advance(&mut self, stage: PointerStage, context: &ComponentContext<Self>) {
        self.stage = stage;
        self.attempt = 0;
        self.retry(context);
    }
}

impl Component for PointerInjection {
    type Input = FixtureInput;
    type Message = PointerMessage;

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        let fixture = Self {
            stage: PointerStage::Move,
            entered: false,
            attempt: 0,
            complete: input.complete.clone(),
            injector: InputInjector::TryCreate()
                .map(Rc::new)
                .map_err(|error| error.to_string()),
        };
        fixture.retry(context);
        fixture
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
        self.complete = input.complete.clone();
    }

    fn update(&mut self, message: Self::Message, context: &ComponentContext<Self>) {
        match message {
            PointerMessage::Entered => self.entered = true,
            PointerMessage::Moved(info) if self.stage == PointerStage::Move => {
                if !self.entered {
                    self.fail("injected move did not raise PointerEntered");
                } else if info.x <= 0.0
                    || info.y <= 0.0
                    || info.window_x <= info.x
                    || info.window_y <= info.y
                {
                    self.fail(format!(
                        "invalid pointer coordinates: element=({}, {}), window=({}, {})",
                        info.x, info.y, info.window_x, info.window_y
                    ));
                } else {
                    self.advance(PointerStage::LeftDown, context);
                }
            }
            PointerMessage::Pressed(info) if self.stage == PointerStage::LeftDown => {
                if !info.is_left_button_pressed {
                    self.fail("left press did not report the left button");
                } else if !info.capture_succeeded {
                    self.fail("pointer capture failed on left press");
                } else {
                    self.advance(PointerStage::MoveOutside, context);
                }
            }
            PointerMessage::Moved(info)
                if self.stage == PointerStage::MoveOutside && info.y > 300.0 =>
            {
                self.advance(PointerStage::LeftUp, context);
            }
            PointerMessage::Released if self.stage == PointerStage::LeftUp => {
                self.advance(PointerStage::RightDown, context);
            }
            PointerMessage::Pressed(info) if self.stage == PointerStage::RightDown => {
                if !info.is_right_button_pressed {
                    self.fail("right press did not report the right button");
                } else {
                    self.advance(PointerStage::RightUp, context);
                }
            }
            PointerMessage::Released if self.stage == PointerStage::RightUp => {
                self.advance(PointerStage::Exit, context);
            }
            PointerMessage::Exited if self.stage == PointerStage::Exit => {
                if !self.complete.call(Ok(())) {
                    eprintln!("pointer fixture completion was rejected");
                    std::process::exit(1);
                }
            }
            PointerMessage::Retry(stage) if stage == self.stage => {
                self.attempt += 1;
                if self.attempt == 20 {
                    self.fail(format!(
                        "injected input did not advance stage {}",
                        pointer_stage_name(stage)
                    ));
                } else {
                    self.retry(context);
                }
            }
            _ => {}
        }
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let stage = self.stage;
        let attempt = self.attempt;
        let complete = self.complete.clone();
        let injector = self.injector.clone();
        context.use_effect("inject", (stage, attempt), move || {
            let result = injector
                .and_then(|injector| schedule_pointer_stage(stage, injector, complete.clone()));
            if let Err(error) = result
                && !complete.call(Err(error))
            {
                eprintln!("pointer injection failure was rejected");
                std::process::exit(1);
            }
            None
        });
        StackPanel::new().children((Border::new()
            .width(6000.0)
            .height(300.0)
            .margin(Thickness::uniform(20.0))
            .background(Color::rgb(32, 32, 40))
            .capture_pointer_on_press(true)
            .on_pointer_entered(context.callback(|_| PointerMessage::Entered))
            .on_pointer_moved(context.callback(PointerMessage::Moved))
            .on_pointer_pressed(context.callback(PointerMessage::Pressed))
            .on_pointer_released(context.callback(|_| PointerMessage::Released))
            .on_pointer_exited(context.callback(|_| PointerMessage::Exited))
            .content(TextBlock::new().text("pointer target")),))
    }
}

fn pointer_stage_name(stage: PointerStage) -> &'static str {
    match stage {
        PointerStage::Move => "move",
        PointerStage::LeftDown => "left-down",
        PointerStage::MoveOutside => "captured-move",
        PointerStage::LeftUp => "left-up",
        PointerStage::RightDown => "right-down",
        PointerStage::RightUp => "right-up",
        PointerStage::Exit => "exit",
    }
}

fn schedule_pointer_stage(
    stage: PointerStage,
    injector: Rc<InputInjector>,
    complete: Callback<FixtureResult>,
) -> Result<(), String> {
    schedule_live_window_handle(move |result| {
        let result = result.and_then(|handle| inject_pointer_stage(stage, handle, &injector));
        if let Err(error) = result
            && !complete.call(Err(error))
        {
            eprintln!("pointer injection failure was rejected");
            std::process::exit(1);
        }
    })
    .map_err(|error| error.to_string())
}

fn inject_pointer_stage(
    stage: PointerStage,
    handle: isize,
    injector: &InputInjector,
) -> Result<(), String> {
    let hwnd = HWND(handle as *mut _);
    unsafe {
        let _ = SetForegroundWindow(hwnd);
        let _ = BringWindowToTop(hwnd);
    }
    let ((x, y), options) = match stage {
        PointerStage::Move => (
            client_screen_point(hwnd, 0.5, 0.1)?,
            InjectedInputMouseOptions::Move,
        ),
        PointerStage::LeftDown => (
            client_screen_point(hwnd, 0.5, 0.1)?,
            InjectedInputMouseOptions::LeftDown,
        ),
        PointerStage::MoveOutside => (
            client_screen_point(hwnd, 0.5, 0.75)?,
            InjectedInputMouseOptions::Move,
        ),
        PointerStage::LeftUp => (
            client_screen_point(hwnd, 0.5, 0.75)?,
            InjectedInputMouseOptions::LeftUp,
        ),
        PointerStage::RightDown => (
            client_screen_point(hwnd, 0.5, 0.1)?,
            InjectedInputMouseOptions::RightDown,
        ),
        PointerStage::RightUp => (
            client_screen_point(hwnd, 0.5, 0.1)?,
            InjectedInputMouseOptions::RightUp,
        ),
        PointerStage::Exit => (virtual_screen_origin(), InjectedInputMouseOptions::Move),
    };
    inject_at(injector, x, y, options).map_err(|error| error.to_string())
}

fn client_screen_point(hwnd: HWND, x_fraction: f64, y_fraction: f64) -> Result<(i32, i32), String> {
    let mut rect = RECT::default();
    if !unsafe { GetClientRect(hwnd, &mut rect) }.as_bool() {
        return Err("could not read the pointer fixture client rect".to_string());
    }
    let mut point = POINT {
        x: (f64::from(rect.right) * x_fraction) as i32,
        y: (f64::from(rect.bottom) * y_fraction) as i32,
    };
    if !unsafe { ClientToScreen(hwnd, &mut point) }.as_bool() {
        return Err("could not convert pointer fixture coordinates".to_string());
    }
    Ok((point.x, point.y))
}

fn virtual_screen_origin() -> (i32, i32) {
    unsafe {
        (
            GetSystemMetrics(SM_XVIRTUALSCREEN),
            GetSystemMetrics(SM_YVIRTUALSCREEN),
        )
    }
}

fn inject_at(
    injector: &InputInjector,
    screen_x: i32,
    screen_y: i32,
    options: InjectedInputMouseOptions,
) -> windows_core::Result<()> {
    let (origin_x, origin_y, width, height) = unsafe {
        (
            GetSystemMetrics(SM_XVIRTUALSCREEN),
            GetSystemMetrics(SM_YVIRTUALSCREEN),
            GetSystemMetrics(SM_CXVIRTUALSCREEN).max(2),
            GetSystemMetrics(SM_CYVIRTUALSCREEN).max(2),
        )
    };
    let x = ((f64::from(screen_x - origin_x) * 65535.0) / f64::from(width - 1)).round() as i32;
    let y = ((f64::from(screen_y - origin_y) * 65535.0) / f64::from(height - 1)).round() as i32;
    let info = InjectedInputMouseInfo::new()?;
    info.SetDeltaX(x)?;
    info.SetDeltaY(y)?;
    info.SetMouseOptions(
        InjectedInputMouseOptions::Absolute | InjectedInputMouseOptions::VirtualDesk | options,
    )?;
    let inputs: IIterable<InjectedInputMouseInfo> = vec![Some(info)].into();
    injector.InjectMouseInput(&inputs)
}

pub(crate) enum WindowFixtureMessage {
    Closed(Arc<AtomicBool>),
    Verified(FixtureResult),
}

pub(crate) struct WindowLifecycle {
    complete: Callback<FixtureResult>,
}

impl Component for WindowLifecycle {
    type Input = FixtureInput;
    type Message = WindowFixtureMessage;

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        let closed = context.sender().callback(WindowFixtureMessage::Closed);
        if !context.open_window(View::component::<ClosingWindow>(ClosingWindowInput {
            closed,
        })) && !input
            .complete
            .call(Err("secondary window request was rejected".to_string()))
        {
            eprintln!("window lifecycle fixture failure was rejected");
            std::process::exit(1);
        }
        Self {
            complete: input.complete.clone(),
        }
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
        self.complete = input.complete.clone();
    }

    fn update(&mut self, message: Self::Message, context: &ComponentContext<Self>) {
        match message {
            WindowFixtureMessage::Closed(task_cancelled) => {
                context.spawn_background(move |_| {
                    let deadline = Instant::now() + Duration::from_secs(1);
                    while !task_cancelled.load(Ordering::Acquire) && Instant::now() < deadline {
                        std::thread::yield_now();
                    }
                    WindowFixtureMessage::Verified(
                        task_cancelled
                            .load(Ordering::Acquire)
                            .then_some(())
                            .ok_or_else(|| "scope task was not cancelled".to_string()),
                    )
                });
            }
            WindowFixtureMessage::Verified(result) => {
                if !self.complete.call(result) {
                    eprintln!("window lifecycle fixture completion was rejected");
                    std::process::exit(1);
                }
            }
        }
    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        TextBlock::new().text("window lifecycle").into()
    }
}

#[derive(Clone, PartialEq)]
struct ClosingWindowInput {
    closed: Callback<Arc<AtomicBool>>,
}

enum ClosingWindowMessage {
    Close,
    Retitle,
}

struct ClosingWindow {
    retitled: bool,
    task_cancelled: Arc<AtomicBool>,
}

impl Component for ClosingWindow {
    type Input = ClosingWindowInput;
    type Message = ClosingWindowMessage;

    fn create(_input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        let task_cancelled = Arc::new(AtomicBool::new(false));
        let cancelled = Arc::clone(&task_cancelled);
        context.spawn_background(move |cancellation| {
            while !cancellation.is_cancelled() {
                std::thread::yield_now();
            }
            cancelled.store(true, Ordering::Release);
            ClosingWindowMessage::Close
        });
        Self {
            retitled: false,
            task_cancelled,
        }
    }

    fn update(&mut self, message: Self::Message, context: &ComponentContext<Self>) {
        match message {
            ClosingWindowMessage::Close => {
                if !context.window().request_close() {
                    eprintln!("secondary window close was rejected");
                    std::process::exit(1);
                }
            }
            ClosingWindowMessage::Retitle => self.retitled = true,
        }
    }

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title(if self.retitled {
            "closing child window - latest"
        } else {
            "closing child window - initial"
        });
        let sender = context.sender();
        let retitled = self.retitled;
        context.use_effect("advance-window", retitled, move || {
            let message = if retitled {
                ClosingWindowMessage::Close
            } else {
                ClosingWindowMessage::Retitle
            };
            if !sender.send(message) {
                eprintln!("secondary window update was rejected");
                std::process::exit(1);
            }
            None
        });
        let task_cancelled = Arc::clone(&self.task_cancelled);
        let closed = input.closed.clone();
        context.use_effect("cleanup", (), move || {
            Some(Box::new(move || {
                if !closed.call(task_cancelled) {
                    eprintln!("secondary window cleanup result was rejected");
                    std::process::exit(1);
                }
            }))
        });
        StackPanel::new().children((
            TitleBar::new().title("Closing child window"),
            TextBlock::new().text("closing child window"),
        ))
    }
}

pub(crate) enum ImageMessage {
    Scale(f64),
    Cleared(Result<(), ImageSourceError>),
}

pub(crate) struct ImageSourceLifecycle {
    device: Option<GpuDevice>,
    image: ElementRef<Image>,
    removed: bool,
    surface: Option<CanvasImageSource>,
}

impl Component for ImageSourceLifecycle {
    type Input = FixtureInput;
    type Message = ImageMessage;

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            device: None,
            image: ElementRef::new(),
            removed: false,
            surface: None,
        }
    }

    fn update(&mut self, message: Self::Message, context: &ComponentContext<Self>) {
        match message {
            ImageMessage::Scale(scale) if self.surface.is_none() => {
                let result = (|| {
                    if scale <= 0.0 {
                        return Err("invalid rasterization scale".to_string());
                    }
                    let device = GpuDevice::new_or_warp().map_err(|error| error.to_string())?;
                    let surface = CanvasImageSource::new(&device, 64.0, 64.0, scale as f32)
                        .map_err(|error| error.to_string())?;
                    if !surface.attach(&self.image) {
                        return Err("native ImageSource attachment was rejected".to_string());
                    }
                    self.device = Some(device);
                    self.surface = Some(surface);
                    let sender = context.sender();
                    if !self.image.request_set_native_source(None, move |result| {
                        sender.send(ImageMessage::Cleared(result));
                    }) {
                        return Err("native ImageSource clear was rejected".to_string());
                    }
                    Ok(())
                })();
                if let Err(error) = result {
                    eprintln!("ImageSource fixture failed: {error}");
                    std::process::exit(1);
                }
            }
            ImageMessage::Cleared(Ok(())) => self.removed = true,
            ImageMessage::Cleared(Err(error)) => {
                eprintln!("ImageSource clear failed: {error:?}");
                std::process::exit(1);
            }
            ImageMessage::Scale(_) => {}
        }
    }

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        if self.removed {
            let image = self.image.clone();
            let complete = input.complete.clone();
            context.use_effect("verify-image-retirement", (), move || {
                let result = (!image.request_set_native_source(None, |_| {}))
                    .then_some(())
                    .ok_or_else(|| "retired Image reference remained bound".to_string());
                if !complete.call(result) {
                    eprintln!("ImageSource fixture completion was rejected");
                    std::process::exit(1);
                }
                None
            });
            return TextBlock::new().text("image retired").into();
        }
        let image = self.image.clone();
        let sender = context.sender();
        context.use_effect("observe-image-scale", (), move || {
            let observation = image.observe_rasterization_scale(move |scale| {
                sender.send(ImageMessage::Scale(scale));
            });
            Some(Box::new(move || drop(observation)))
        });
        Image::new()
            .element_ref(&self.image)
            .width(64.0)
            .height(64.0)
            .into()
    }
}

struct CompositionScene {
    _background: SpriteVisual,
    root: ContainerVisual,
}

pub(crate) enum CompositionMessage {
    Host(CompositionHostEvent),
    Attached(Result<(), CompositionHostError>),
    Replaced(Result<(), CompositionHostError>),
    Cleared(Result<(), CompositionHostError>),
}

pub(crate) struct CompositionLifecycle {
    host: ElementRef<Grid>,
    removed: bool,
    scene: Option<CompositionScene>,
}

impl Component for CompositionLifecycle {
    type Input = FixtureInput;
    type Message = CompositionMessage;

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            host: ElementRef::new(),
            removed: false,
            scene: None,
        }
    }

    fn update(&mut self, message: Self::Message, context: &ComponentContext<Self>) {
        match message {
            CompositionMessage::Host(CompositionHostEvent::Ready {
                compositor,
                width,
                height,
                scale,
            }) if self.scene.is_none() => {
                let result = (|| {
                    if width < 0.0 || height < 0.0 || scale <= 0.0 {
                        return Err("invalid composition host metrics".to_string());
                    }
                    let compositor = CompositionCompositor::from_host(compositor)
                        .map_err(|error| error.to_string())?;
                    let root = compositor.create_container_visual();
                    root.set_size(width as f32, height as f32);
                    let background = compositor.create_sprite_visual();
                    background.set_size(width as f32, height as f32);
                    background.set_brush(
                        &compositor.create_color_brush(CompositionColor::rgb(24, 24, 32)),
                    );
                    root.children().insert_at_bottom(&background);
                    let sender = context.sender();
                    if !self.host.request_set_child_visual(
                        Some(root.as_raw().into()),
                        move |result| {
                            sender.send(CompositionMessage::Attached(result));
                        },
                    ) {
                        return Err("composition attachment was rejected".to_string());
                    }
                    self.scene = Some(CompositionScene {
                        _background: background,
                        root,
                    });
                    Ok(())
                })();
                if let Err(error) = result {
                    eprintln!("composition fixture failed: {error}");
                    std::process::exit(1);
                }
            }
            CompositionMessage::Host(CompositionHostEvent::Metrics {
                width,
                height,
                scale,
            }) => {
                if width < 0.0 || height < 0.0 || scale <= 0.0 {
                    eprintln!("composition fixture received invalid metrics");
                    std::process::exit(1);
                }
                if let Some(scene) = &self.scene {
                    scene.root.set_size(width as f32, height as f32);
                }
            }
            CompositionMessage::Attached(Ok(())) => {
                let compositor = self.scene.as_ref().unwrap().root.compositor();
                let replacement = compositor.create_container_visual();
                let sender = context.sender();
                if !self.host.request_set_child_visual(
                    Some(replacement.as_raw().into()),
                    move |result| {
                        sender.send(CompositionMessage::Replaced(result));
                    },
                ) {
                    eprintln!("composition replacement was rejected");
                    std::process::exit(1);
                }
                self.scene = Some(CompositionScene {
                    _background: compositor.create_sprite_visual(),
                    root: replacement,
                });
            }
            CompositionMessage::Replaced(Ok(())) => {
                let sender = context.sender();
                if !self.host.request_set_child_visual(None, move |result| {
                    sender.send(CompositionMessage::Cleared(result));
                }) {
                    eprintln!("composition clear was rejected");
                    std::process::exit(1);
                }
            }
            CompositionMessage::Cleared(Ok(())) => self.removed = true,
            CompositionMessage::Attached(Err(error))
            | CompositionMessage::Replaced(Err(error))
            | CompositionMessage::Cleared(Err(error)) => {
                eprintln!("composition command failed: {error:?}");
                std::process::exit(1);
            }
            CompositionMessage::Host(_) => {}
        }
    }

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        if self.removed {
            let host = self.host.clone();
            let complete = input.complete.clone();
            context.use_effect("verify-composition-retirement", (), move || {
                let result = (!host.request_set_child_visual(None, |_| {}))
                    .then_some(())
                    .ok_or_else(|| "retired composition host remained bound".to_string());
                if !complete.call(result) {
                    eprintln!("composition fixture completion was rejected");
                    std::process::exit(1);
                }
                None
            });
            return TextBlock::new().text("composition retired").into();
        }
        let host = self.host.clone();
        let sender = context.sender();
        context.use_effect("observe-composition", (), move || {
            let observation = host.observe_composition_host(move |event| {
                sender.send(CompositionMessage::Host(event));
            });
            Some(Box::new(move || drop(observation)))
        });
        Grid::new().element_ref(&self.host).into()
    }
}

pub(crate) enum SwapChainMessage {
    Rendered,
}

pub(crate) struct SwapChainLifecycle {
    rendered: bool,
}

impl Component for SwapChainLifecycle {
    type Input = FixtureInput;
    type Message = SwapChainMessage;

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self { rendered: false }
    }

    fn update(&mut self, _message: Self::Message, _context: &ComponentContext<Self>) {
        self.rendered = true;
    }

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        if self.rendered {
            let complete = input.complete.clone();
            context.use_effect("verify-swap-retirement", (), move || {
                if !complete.call(Ok(())) {
                    eprintln!("swap-chain fixture completion was rejected");
                    std::process::exit(1);
                }
                None
            });
            return TextBlock::new().text("swap chain retired").into();
        }
        let sent = Rc::new(Cell::new(false));
        let draw_sent = Rc::clone(&sent);
        let sender = context.sender();
        animated_canvas(move |draw| {
            draw.clear(ColorF::CORNFLOWER_BLUE);
            if !draw_sent.replace(true) && !sender.send(SwapChainMessage::Rendered) {
                return Err(windows_core::Error::new(
                    windows_core::HRESULT(0x80004005_u32 as _),
                    "swap-chain render result was rejected",
                ));
            }
            Ok(())
        })
    }
}

pub(crate) enum ThemeMessage {
    Scheme(ColorScheme),
    Switch,
}

pub(crate) struct ThemeResources {
    dark: bool,
}

impl Component for ThemeResources {
    type Input = FixtureInput;
    type Message = ThemeMessage;

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self { dark: false }
    }

    fn update(&mut self, message: Self::Message, _context: &ComponentContext<Self>) {
        match message {
            ThemeMessage::Switch => self.dark = true,
            ThemeMessage::Scheme(scheme) => {
                let _ = scheme;
            }
        }
    }

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_visuals(WindowVisuals::new().theme(if self.dark {
            WindowTheme::Dark
        } else {
            WindowTheme::Light
        }));
        let complete = input.complete.clone();
        context.on_color_scheme(context.callback(move |scheme| {
            if scheme == ColorScheme::Dark && !complete.call(Ok(())) {
                eprintln!("theme fixture completion was rejected");
                std::process::exit(1);
            }
            ThemeMessage::Scheme(scheme)
        }));
        if !self.dark {
            let sender = context.sender();
            context.use_effect("switch-theme", (), move || {
                if !sender.send(ThemeMessage::Switch) {
                    eprintln!("theme switch message was rejected");
                    std::process::exit(1);
                }
                None
            });
        }
        let resources = if self.dark {
            ResourceOverrides::new().set("ButtonForeground", Color::rgb(255, 255, 255))
        } else {
            ResourceOverrides::new().set("ButtonBackground", Color::rgb(32, 32, 32))
        };
        Button::new()
            .resource_overrides(resources)
            .content(TextBlock::new().text("theme target"))
    }
}
