use std::cell::Cell;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use windows::UI::Input::Preview::Injection::{
    InjectedInputMouseInfo, InjectedInputMouseOptions, InputInjector,
};
use windows::Win32::winuser::{
    BringWindowToTop, ClientToScreen, DispatchMessageW, GetClientRect, GetForegroundWindow,
    GetMessageW, GetMonitorInfoW, GetSystemMetrics, GetWindowRect, KillTimer,
    MONITOR_DEFAULTTONEAREST, MONITORINFO, MSG, MonitorFromWindow, PostQuitMessage,
    SM_CXVIRTUALSCREEN, SM_CYVIRTUALSCREEN, SM_XVIRTUALSCREEN, SM_YVIRTUALSCREEN, SWP_NOSIZE,
    SWP_NOZORDER, SendInput, SetForegroundWindow, SetTimer, SetWindowPos, TranslateMessage,
    WM_TIMER,
};
use windows::Win32::{
    HWND, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, KEYEVENTF_UNICODE, POINT,
    RECT,
};
use windows_canvas::{
    CanvasCompositionExt, CanvasImageSource, ColorF, GpuDevice, animated_canvas, canvas,
};
use windows_collections::IIterable;
use windows_composition::{Compositor as CompositionCompositor, ContainerVisual, SpriteVisual};
use windows_reactor::test::{LiveProbe, schedule_live_probe, schedule_live_window_handle};
use windows_reactor::*;
#[cfg(feature = "self-contained")]
use windows_webview::{EventRegistration, WebView, webview_result};

pub type FixtureResult = Result<(), String>;

const F13: VirtualKey = VirtualKey(0x7C);

#[cfg(feature = "self-contained")]
pub(crate) struct WebViewLifecycle {
    complete: Callback<FixtureResult>,
    navigation: Option<EventRegistration>,
    webview: Option<WebView>,
}

#[cfg(feature = "self-contained")]
pub(crate) enum WebViewMessage {
    Initialized(Result<WebView, IntegrationError>),
    Navigated(bool),
    Script(Result<String, windows_core::Error>),
}

#[cfg(feature = "self-contained")]
impl Component for WebViewLifecycle {
    type Input = FixtureInput;
    type Message = WebViewMessage;

    fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            complete: input.complete.clone(),
            navigation: None,
            webview: None,
        }
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
        self.complete = input.complete.clone();
    }

    fn update(&mut self, message: Self::Message, context: &ComponentContext<Self>) {
        let complete = matches!(&message, WebViewMessage::Script(Ok(_)));
        let result = match message {
            WebViewMessage::Initialized(Ok(webview)) => {
                let sender = context.sender();
                match webview.on_navigation_completed(move |args| {
                    _ = sender.send(WebViewMessage::Navigated(args.is_success()));
                }) {
                    Ok(navigation) => {
                        self.navigation = Some(navigation);
                        let result = webview
                            .navigate_to_string("<!DOCTYPE html><title>Reactor WebView</title>");
                        self.webview = Some(webview);
                        result.map_err(|error| format!("navigation did not start: {error}"))
                    }
                    Err(error) => Err(format!("navigation subscription failed: {error}")),
                }
            }
            WebViewMessage::Initialized(Err(error)) => {
                Err(format!("Reactor WebView initialization failed: {error:?}"))
            }
            WebViewMessage::Navigated(true) => {
                let Some(webview) = &self.webview else {
                    return;
                };
                let sender = context.sender();
                webview
                    .execute_script("6 * 7", move |result| {
                        _ = sender.send(WebViewMessage::Script(result));
                    })
                    .map_err(|error| format!("script did not start: {error}"))
            }
            WebViewMessage::Navigated(false) => {
                Err("Reactor WebView navigation failed".to_string())
            }
            WebViewMessage::Script(Ok(value)) if value == "42" => Ok(()),
            WebViewMessage::Script(Ok(value)) => Err(format!(
                "Reactor WebView returned unexpected script result {value}"
            )),
            WebViewMessage::Script(Err(error)) => {
                Err(format!("Reactor WebView script failed: {error}"))
            }
        };

        if let Err(error) = result {
            _ = self.complete.call(Err(error));
        } else if complete {
            _ = self.complete.call(Ok(()));
        }
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        webview_result(context.callback(WebViewMessage::Initialized))
    }
}

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

pub(crate) struct NestedWindowOperation {
    complete: Callback<FixtureResult>,
    dispatch_started: bool,
    finished: bool,
    mount_during: bool,
    mount_returned: bool,
    dispatch_during: bool,
    dispatch_returned: bool,
}

pub(crate) enum NestedWindowMessage {
    MountDuring,
    MountReturned(FixtureResult),
    DispatchDuring,
    DispatchReturned(FixtureResult),
}

impl NestedWindowOperation {
    fn complete(&mut self, result: FixtureResult) {
        if self.finished {
            return;
        }
        self.finished = true;
        if !self.complete.call(result) {
            eprintln!("nested window operation fixture completion was rejected");
            std::process::exit(1);
        }
    }
}

impl Component for NestedWindowOperation {
    type Input = FixtureInput;
    type Message = NestedWindowMessage;

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        let sender = context.sender();
        assert!(context.run_window(move |_| {
            NestedWindowMessage::MountReturned(run_nested_message_loop(
                sender,
                NestedWindowMessage::MountDuring,
            ))
        }));
        Self {
            complete: input.complete.clone(),
            dispatch_started: false,
            finished: false,
            mount_during: false,
            mount_returned: false,
            dispatch_during: false,
            dispatch_returned: false,
        }
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
        self.complete = input.complete.clone();
    }

    fn update(&mut self, message: Self::Message, context: &ComponentContext<Self>) {
        match message {
            NestedWindowMessage::MountDuring => self.mount_during = true,
            NestedWindowMessage::MountReturned(Ok(())) => self.mount_returned = true,
            NestedWindowMessage::MountReturned(Err(error))
            | NestedWindowMessage::DispatchReturned(Err(error)) => {
                self.complete(Err(error));
                return;
            }
            NestedWindowMessage::DispatchDuring => self.dispatch_during = true,
            NestedWindowMessage::DispatchReturned(Ok(())) => self.dispatch_returned = true,
        }

        if self.mount_during && self.mount_returned && !self.dispatch_started {
            self.dispatch_started = true;
            let sender = context.sender();
            if !context.run_window(move |_| {
                NestedWindowMessage::DispatchReturned(run_nested_message_loop(
                    sender,
                    NestedWindowMessage::DispatchDuring,
                ))
            }) {
                self.complete(Err("dispatch window operation was not staged".to_string()));
            }
        }

        if self.dispatch_during && self.dispatch_returned {
            self.complete(Ok(()));
        }
    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        TextBlock::new().text("nested window operation").into()
    }
}

fn run_nested_message_loop(
    sender: LocalSender<NestedWindowMessage>,
    message: NestedWindowMessage,
) -> FixtureResult {
    let mut timer = unsafe { SetTimer(None, 0, 10, None) };
    if timer == 0 {
        return Err("could not start nested-loop timer".to_string());
    }
    let callback_seen = Rc::new(Cell::new(false));
    let mut message = Some(message);
    let mut sent = false;

    loop {
        let mut native = MSG::default();
        let status = unsafe { GetMessageW(&mut native, None, 0, 0) };
        if status.0 == -1 {
            unsafe {
                _ = KillTimer(None, timer);
            }
            return Err("nested GetMessageW failed".to_string());
        }
        if !status.as_bool() {
            unsafe {
                PostQuitMessage(native.wParam.0 as i32);
            }
            return Err("nested loop received WM_QUIT".to_string());
        }
        if native.message == WM_TIMER as u32 && native.wParam.0 == timer {
            unsafe {
                _ = KillTimer(None, timer);
            }
            if sent {
                if !callback_seen.get() {
                    return Err("nested dispatcher callback did not run".to_string());
                }
                return Ok(());
            }
            if !sender.send(message.take().unwrap()) {
                return Err("nested component message was rejected".to_string());
            }
            let observed = Rc::clone(&callback_seen);
            schedule_live_window_handle(move |result| {
                if result.is_err() {
                    observed.set(true);
                }
            })
            .map_err(|error| format!("nested dispatcher callback was rejected: {error}"))?;
            sent = true;
            timer = unsafe { SetTimer(None, 0, 100, None) };
            if timer == 0 {
                return Err("could not start nested-loop verification timer".to_string());
            }
            continue;
        }
        unsafe {
            _ = TranslateMessage(&native);
            DispatchMessageW(&native);
        }
    }
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
pub(crate) enum KeyboardClick {
    First,
    Second,
    Focused,
}

pub(crate) struct KeyboardInput {
    complete: Callback<FixtureResult>,
    reference: ElementRef<Border>,
    blur_reference: ElementRef<Border>,
    injector: Result<Rc<InputInjector>, String>,
    click_stage: Option<(KeyboardClick, PointerStage)>,
    click_attempt: u8,
    focus_observed: bool,
    refocus_observed: bool,
    second_click_complete: bool,
    same_target_click_started: bool,
    same_target_click_complete: bool,
    same_target_lost_observed: bool,
    same_target_refocus_observed: bool,
    second_defocus_requested: bool,
    second_defocus_complete: bool,
    second_lost_focus_observed: bool,
    programmatic_refocus_complete: bool,
    programmatic_refocus_observed: bool,
    tab_cycle_started: bool,
    tab_defocus_complete: bool,
    tab_lost_observed: bool,
    tab_injection_complete: bool,
    keyboard_refocus_observed: bool,
    key_observed: bool,
    key_up_observed: bool,
    character_observed: bool,
    injection_complete: bool,
    defocus_requested: bool,
    defocus_complete: bool,
    lost_focus_observed: bool,
    hwnd: Option<isize>,
}

pub(crate) enum KeyboardMessage {
    GotFocus(FocusEventInfo),
    Key(KeyEventInfo),
    KeyUp(KeyEventInfo),
    Character(CharacterEventInfo),
    Defocused(Result<bool, FocusError>),
    LostFocus(FocusEventInfo),
    WindowHandle(Result<isize, String>),
    Activated(Result<isize, String>),
    InitialFocused(Result<bool, FocusError>),
    PointerPressed,
    PointerReleased,
    ClickRetry(KeyboardClick, PointerStage),
    SameTargetSettled,
    SecondDefocused(Result<bool, FocusError>),
    ProgrammaticRefocused(Result<bool, FocusError>),
    TabDefocused(Result<bool, FocusError>),
    TabInjected(Result<(), String>),
    FocusTimeout(bool),
    Injected(Result<(), String>),
}

impl KeyboardInput {
    fn start_click(&mut self, click: KeyboardClick, context: &ComponentContext<Self>) {
        self.click_stage = Some((click, PointerStage::LeftDown));
        self.click_attempt = 0;
        self.retry_click(context);
    }

    fn advance_click(&mut self, stage: PointerStage, context: &ComponentContext<Self>) {
        let Some((click, _)) = self.click_stage else {
            return;
        };
        self.click_stage = Some((click, stage));
        self.click_attempt = 0;
        self.retry_click(context);
    }

    fn retry_click(&self, context: &ComponentContext<Self>) {
        let Some((click, stage)) = self.click_stage else {
            return;
        };
        context.spawn_background(move |_| {
            std::thread::sleep(Duration::from_millis(100));
            KeyboardMessage::ClickRetry(click, stage)
        });
    }

    fn complete_if_ready(&self) {
        if self.focus_observed
            && self.key_observed
            && self.key_up_observed
            && self.character_observed
            && self.injection_complete
            && self.defocus_complete
            && self.lost_focus_observed
            && self.refocus_observed
            && self.same_target_click_complete
            && self.same_target_refocus_observed
            && self.second_defocus_complete
            && self.second_lost_focus_observed
            && self.programmatic_refocus_complete
            && self.programmatic_refocus_observed
            && self.tab_defocus_complete
            && self.tab_lost_observed
            && self.tab_injection_complete
            && self.keyboard_refocus_observed
            && !self.complete.call(Ok(()))
        {
            eprintln!("keyboard fixture completion was rejected");
            std::process::exit(1);
        }
    }
}

impl Component for KeyboardInput {
    type Input = FixtureInput;
    type Message = KeyboardMessage;

    fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            complete: input.complete.clone(),
            reference: ElementRef::new(),
            blur_reference: ElementRef::new(),
            injector: InputInjector::TryCreate()
                .map(Rc::new)
                .map_err(|error| error.to_string()),
            click_stage: None,
            click_attempt: 0,
            focus_observed: false,
            refocus_observed: false,
            second_click_complete: false,
            same_target_click_started: false,
            same_target_click_complete: false,
            same_target_lost_observed: false,
            same_target_refocus_observed: false,
            second_defocus_requested: false,
            second_defocus_complete: false,
            second_lost_focus_observed: false,
            programmatic_refocus_complete: false,
            programmatic_refocus_observed: false,
            tab_cycle_started: false,
            tab_defocus_complete: false,
            tab_lost_observed: false,
            tab_injection_complete: false,
            keyboard_refocus_observed: false,
            key_observed: false,
            key_up_observed: false,
            character_observed: false,
            injection_complete: false,
            defocus_requested: false,
            defocus_complete: false,
            lost_focus_observed: false,
            hwnd: None,
        }
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
        self.complete = input.complete.clone();
    }

    fn update(&mut self, message: Self::Message, context: &ComponentContext<Self>) {
        let mut failure = match message {
            KeyboardMessage::GotFocus(info) => {
                if !info.is_direct || info.state == ElementFocusState::Unfocused {
                    Some(format!("unexpected focus payload: {info:?}"))
                } else if self.tab_lost_observed && !self.keyboard_refocus_observed {
                    if info.state != ElementFocusState::Keyboard {
                        Some(format!(
                            "keyboard navigation reported unexpected focus state: {info:?}"
                        ))
                    } else {
                        self.keyboard_refocus_observed = true;
                        None
                    }
                } else if !self.focus_observed {
                    if info.state != ElementFocusState::Pointer {
                        Some(format!(
                            "first click reported unexpected focus state: {info:?}"
                        ))
                    } else {
                        self.focus_observed = true;
                        let hwnd = self.hwnd.unwrap();
                        context.spawn_background(move |_| {
                            std::thread::sleep(Duration::from_millis(100));
                            KeyboardMessage::Injected(inject_keyboard(hwnd))
                        });
                        None
                    }
                } else if !self.refocus_observed {
                    if info.state != ElementFocusState::Pointer {
                        Some(format!(
                            "second click reported unexpected focus state: {info:?}"
                        ))
                    } else {
                        self.refocus_observed = true;
                        None
                    }
                } else if self.same_target_click_started && !self.same_target_refocus_observed {
                    if info.state != ElementFocusState::Pointer {
                        Some(format!(
                            "focused click reported unexpected focus state: {info:?}"
                        ))
                    } else {
                        self.same_target_refocus_observed = true;
                        None
                    }
                } else if self.second_lost_focus_observed
                    && info.state == ElementFocusState::Programmatic
                {
                    self.programmatic_refocus_observed = true;
                    None
                } else {
                    Some(format!("unexpected additional focus payload: {info:?}"))
                }
            }
            KeyboardMessage::Key(info) => {
                if info.key != F13
                    || info.original_key != F13
                    || info.status.is_released
                    || info.modifiers != InputModifiers::NONE
                {
                    Some(format!("unexpected key payload: {info:?}"))
                } else {
                    self.key_observed = true;
                    None
                }
            }
            KeyboardMessage::KeyUp(info) => {
                if info.key != F13
                    || info.original_key != F13
                    || !info.status.is_released
                    || info.modifiers != InputModifiers::NONE
                {
                    Some(format!("unexpected key-up payload: {info:?}"))
                } else {
                    self.key_up_observed = true;
                    None
                }
            }
            KeyboardMessage::Character(info) => {
                if info.character != b'x'.into() || info.status.is_released {
                    Some(format!("unexpected character payload: {info:?}"))
                } else {
                    self.character_observed = true;
                    None
                }
            }
            KeyboardMessage::Defocused(Ok(true)) => {
                self.defocus_complete = true;
                None
            }
            KeyboardMessage::Defocused(Ok(false)) => {
                Some("WinUI rejected the keyboard defocus request".to_string())
            }
            KeyboardMessage::Defocused(Err(error)) => {
                Some(format!("keyboard defocus request failed: {error:?}"))
            }
            KeyboardMessage::LostFocus(info) => {
                if !info.is_direct || info.state != ElementFocusState::Unfocused {
                    Some(format!("unexpected lost-focus payload: {info:?}"))
                } else if self.tab_cycle_started {
                    self.tab_lost_observed = true;
                    let hwnd = self.hwnd.unwrap();
                    context.spawn_background(move |_| {
                        std::thread::sleep(Duration::from_millis(100));
                        KeyboardMessage::TabInjected(inject_reverse_tab(hwnd))
                    });
                    None
                } else if self.second_defocus_requested {
                    self.second_lost_focus_observed = true;
                    let sender = context.sender();
                    if !self.reference.request_focus_result(move |result| {
                        sender.send(KeyboardMessage::ProgrammaticRefocused(result));
                    }) {
                        Some("keyboard target was not published".to_string())
                    } else {
                        None
                    }
                } else if self.lost_focus_observed && self.same_target_click_started {
                    self.same_target_lost_observed = true;
                    None
                } else {
                    self.lost_focus_observed = true;
                    self.start_click(KeyboardClick::Second, context);
                    None
                }
            }
            KeyboardMessage::WindowHandle(Ok(hwnd)) => {
                context.spawn_background(move |_| {
                    unsafe {
                        let _ = SetForegroundWindow(HWND(hwnd as *mut _));
                    }
                    std::thread::sleep(Duration::from_millis(100));
                    let result = (unsafe { GetForegroundWindow() } == HWND(hwnd as *mut _))
                        .then_some(hwnd)
                        .ok_or_else(|| "self-test window could not become foreground".to_string());
                    KeyboardMessage::Activated(result)
                });
                None
            }
            KeyboardMessage::WindowHandle(Err(error)) => Some(error),
            KeyboardMessage::Activated(Ok(hwnd)) => {
                self.hwnd = Some(hwnd);
                let sender = context.sender();
                if !self.blur_reference.request_focus_result(move |result| {
                    sender.send(KeyboardMessage::InitialFocused(result));
                }) {
                    Some("keyboard blur target was not published".to_string())
                } else {
                    None
                }
            }
            KeyboardMessage::Activated(Err(error)) => Some(error),
            KeyboardMessage::InitialFocused(Ok(true)) => {
                self.start_click(KeyboardClick::First, context);
                None
            }
            KeyboardMessage::InitialFocused(Ok(false)) => {
                Some("WinUI rejected the initial focus-readiness request".to_string())
            }
            KeyboardMessage::InitialFocused(Err(error)) => {
                Some(format!("initial focus-readiness request failed: {error:?}"))
            }
            KeyboardMessage::PointerPressed
                if matches!(self.click_stage, Some((_, PointerStage::LeftDown))) =>
            {
                self.advance_click(PointerStage::LeftUp, context);
                None
            }
            KeyboardMessage::PointerReleased
                if matches!(self.click_stage, Some((_, PointerStage::LeftUp))) =>
            {
                let (click, _) = self.click_stage.take().unwrap();
                match click {
                    KeyboardClick::First => schedule_focus_timeout(context, false),
                    KeyboardClick::Second => {
                        self.second_click_complete = true;
                        schedule_focus_timeout(context, true);
                    }
                    KeyboardClick::Focused => {
                        self.same_target_click_complete = true;
                        context.spawn_background(move |_| {
                            std::thread::sleep(Duration::from_millis(200));
                            KeyboardMessage::SameTargetSettled
                        });
                    }
                }
                None
            }
            KeyboardMessage::PointerPressed | KeyboardMessage::PointerReleased => None,
            KeyboardMessage::ClickRetry(click, stage)
                if self.click_stage == Some((click, stage)) =>
            {
                self.click_attempt += 1;
                if self.click_attempt == 20 {
                    Some(format!(
                        "keyboard click did not advance {}",
                        pointer_stage_name(stage)
                    ))
                } else {
                    self.retry_click(context);
                    None
                }
            }
            KeyboardMessage::ClickRetry(_, _) => None,
            KeyboardMessage::SameTargetSettled => {
                if self.same_target_lost_observed && !self.same_target_refocus_observed {
                    Some("clicking the focused target did not restore focus".to_string())
                } else {
                    self.same_target_refocus_observed = true;
                    self.second_defocus_requested = true;
                    let sender = context.sender();
                    if !self.blur_reference.request_focus_result(move |result| {
                        sender.send(KeyboardMessage::SecondDefocused(result));
                    }) {
                        Some("keyboard blur target was not published".to_string())
                    } else {
                        None
                    }
                }
            }
            KeyboardMessage::SecondDefocused(Ok(true)) => {
                self.second_defocus_complete = true;
                None
            }
            KeyboardMessage::SecondDefocused(Ok(false)) => {
                Some("WinUI rejected the second keyboard defocus request".to_string())
            }
            KeyboardMessage::SecondDefocused(Err(error)) => {
                Some(format!("second keyboard defocus request failed: {error:?}"))
            }
            KeyboardMessage::ProgrammaticRefocused(Ok(true)) => {
                self.programmatic_refocus_complete = true;
                None
            }
            KeyboardMessage::ProgrammaticRefocused(Ok(false)) => {
                Some("WinUI rejected the programmatic refocus request".to_string())
            }
            KeyboardMessage::ProgrammaticRefocused(Err(error)) => {
                Some(format!("programmatic refocus request failed: {error:?}"))
            }
            KeyboardMessage::TabDefocused(Ok(true)) => {
                self.tab_defocus_complete = true;
                None
            }
            KeyboardMessage::TabDefocused(Ok(false)) => {
                Some("WinUI rejected the Tab-cycle defocus request".to_string())
            }
            KeyboardMessage::TabDefocused(Err(error)) => {
                Some(format!("Tab-cycle defocus request failed: {error:?}"))
            }
            KeyboardMessage::TabInjected(Ok(())) => {
                self.tab_injection_complete = true;
                None
            }
            KeyboardMessage::TabInjected(Err(error)) => Some(error),
            KeyboardMessage::FocusTimeout(false) if !self.focus_observed => {
                Some("first click did not focus the keyboard target".to_string())
            }
            KeyboardMessage::FocusTimeout(true) if !self.refocus_observed => {
                Some("second click did not refocus the keyboard target".to_string())
            }
            KeyboardMessage::FocusTimeout(_) => None,
            KeyboardMessage::Injected(Ok(())) => {
                self.injection_complete = true;
                None
            }
            KeyboardMessage::Injected(Err(error)) => Some(error),
        };
        if failure.is_none()
            && self.key_observed
            && self.key_up_observed
            && self.character_observed
            && self.injection_complete
            && !self.defocus_requested
        {
            self.defocus_requested = true;
            let sender = context.sender();
            if !self.blur_reference.request_focus_result(move |result| {
                sender.send(KeyboardMessage::Defocused(result));
            }) {
                failure = Some("keyboard blur target was not published".to_string());
            }
        }
        if failure.is_none()
            && self.programmatic_refocus_complete
            && self.programmatic_refocus_observed
            && !self.tab_cycle_started
        {
            self.tab_cycle_started = true;
            let sender = context.sender();
            if !self.blur_reference.request_focus_result(move |result| {
                sender.send(KeyboardMessage::TabDefocused(result));
            }) {
                failure = Some("keyboard blur target was not published".to_string());
            }
        }
        if failure.is_none()
            && self.refocus_observed
            && self.second_click_complete
            && !self.same_target_click_started
        {
            self.same_target_click_started = true;
            self.start_click(KeyboardClick::Focused, context);
        }
        if let Some(failure) = failure {
            if !self.complete.call(Err(failure)) {
                eprintln!("keyboard fixture failure was rejected");
                std::process::exit(1);
            }
            return;
        }
        self.complete_if_ready();
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let sender = context.sender();
        context.use_effect("request-focus", (), move || {
            schedule_live_window_handle(move |result| {
                sender.send(KeyboardMessage::WindowHandle(result));
            })
            .unwrap();
            None
        });
        let click = (self.click_stage, self.click_attempt);
        let injector = self.injector.clone();
        let complete = self.complete.clone();
        context.use_effect("keyboard-click", click, move || {
            let (click_stage, _) = click;
            if let Some((_, stage)) = click_stage {
                let result = injector.and_then(|injector| {
                    schedule_keyboard_click_stage(stage, injector, complete.clone())
                });
                if let Err(error) = result
                    && !complete.call(Err(error))
                {
                    eprintln!("keyboard pointer injection failure was rejected");
                    std::process::exit(1);
                }
            }
            None
        });

        StackPanel::new().children((
            Border::new()
                .height(240.0)
                .is_tab_stop(true)
                .focus_on_pointer_release(true)
                .background(Color::transparent())
                .element_ref(&self.reference)
                .on_got_focus(context.callback(KeyboardMessage::GotFocus))
                .on_lost_focus(context.callback(KeyboardMessage::LostFocus))
                .on_pointer_pressed(context.callback(|_| KeyboardMessage::PointerPressed))
                .on_pointer_released(context.callback(|_| KeyboardMessage::PointerReleased))
                .on_preview_key_down(context.routed_callback(|info: KeyEventInfo| {
                    if info.key == F13 {
                        RoutedMessage::handled(KeyboardMessage::Key(info))
                    } else {
                        RoutedMessage::bubble_without_message()
                    }
                }))
                .on_key_up(context.routed_callback(|info: KeyEventInfo| {
                    if info.key == F13 {
                        RoutedMessage::handled(KeyboardMessage::KeyUp(info))
                    } else {
                        RoutedMessage::bubble_without_message()
                    }
                }))
                .on_character_received(context.routed_callback(|info: CharacterEventInfo| {
                    RoutedMessage::handled(KeyboardMessage::Character(info))
                }))
                .content(canvas(|context| {
                    context.clear(ColorF::from_rgb8(32, 32, 40));
                    Ok(())
                })),
            Border::new()
                .is_tab_stop(true)
                .element_ref(&self.blur_reference)
                .content("Keyboard blur target"),
        ))
    }
}

fn inject_keyboard(hwnd: isize) -> Result<(), String> {
    if unsafe { GetForegroundWindow() } != HWND(hwnd as *mut _) {
        return Err("self-test window lost foreground focus".to_string());
    }
    let key = |virtual_key, scan_code, flags| INPUT {
        r#type: INPUT_KEYBOARD as u32,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: virtual_key,
                wScan: scan_code,
                dwFlags: flags,
                ..Default::default()
            },
        },
    };
    let inputs = [
        key(F13.0 as u16, 0, 0),
        key(F13.0 as u16, 0, KEYEVENTF_KEYUP as u32),
        key(0, b'x'.into(), KEYEVENTF_UNICODE as u32),
        key(0, b'x'.into(), (KEYEVENTF_UNICODE | KEYEVENTF_KEYUP) as u32),
    ];
    let inserted = unsafe { SendInput(&inputs, size_of::<INPUT>() as i32) };
    if inserted == inputs.len() as u32 {
        Ok(())
    } else {
        Err(format!(
            "SendInput inserted {inserted} of {} keyboard events",
            inputs.len()
        ))
    }
}

fn inject_reverse_tab(hwnd: isize) -> Result<(), String> {
    if unsafe { GetForegroundWindow() } != HWND(hwnd as *mut _) {
        return Err("self-test window lost foreground focus".to_string());
    }
    let key = |virtual_key, flags| INPUT {
        r#type: INPUT_KEYBOARD as u32,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: virtual_key,
                dwFlags: flags,
                ..Default::default()
            },
        },
    };
    let inputs = [
        key(0x10, 0),
        key(0x09, 0),
        key(0x09, KEYEVENTF_KEYUP as u32),
        key(0x10, KEYEVENTF_KEYUP as u32),
    ];
    let inserted = unsafe { SendInput(&inputs, size_of::<INPUT>() as i32) };
    if inserted == inputs.len() as u32 {
        Ok(())
    } else {
        Err(format!(
            "SendInput inserted {inserted} of {} keyboard navigation events",
            inputs.len()
        ))
    }
}

fn schedule_keyboard_click_stage(
    stage: PointerStage,
    injector: Rc<InputInjector>,
    complete: Callback<FixtureResult>,
) -> Result<(), String> {
    schedule_live_window_handle(move |result| {
        let result = result.and_then(|handle| {
            let hwnd = HWND(handle as *mut _);
            if stage == PointerStage::LeftDown {
                fit_window_to_work_area(hwnd)?;
                let (x, y) = virtual_screen_origin();
                inject_at(
                    &injector,
                    x,
                    y,
                    InjectedInputMouseOptions::Move | InjectedInputMouseOptions::MoveNoCoalesce,
                )
                .map_err(|error| error.to_string())?;
                inject_at(
                    &injector,
                    x,
                    y,
                    InjectedInputMouseOptions::LeftUp | InjectedInputMouseOptions::RightUp,
                )
                .map_err(|error| error.to_string())?;
            }
            unsafe {
                let _ = SetForegroundWindow(hwnd);
                let _ = BringWindowToTop(hwnd);
            }
            let (x, y) = client_screen_point(hwnd, 0.5, 0.1)?;
            let options = match stage {
                PointerStage::LeftDown => {
                    inject_at(
                        &injector,
                        x,
                        y,
                        InjectedInputMouseOptions::Move | InjectedInputMouseOptions::MoveNoCoalesce,
                    )
                    .map_err(|error| error.to_string())?;
                    InjectedInputMouseOptions::LeftDown
                }
                PointerStage::LeftUp => InjectedInputMouseOptions::LeftUp,
                _ => return Err("invalid keyboard click stage".to_string()),
            };
            inject_at(&injector, x, y, options).map_err(|error| error.to_string())
        });
        if let Err(error) = result
            && !complete.call(Err(error))
        {
            eprintln!("keyboard pointer injection failure was rejected");
            std::process::exit(1);
        }
    })
    .map_err(|error| error.to_string())
}

fn schedule_focus_timeout(context: &ComponentContext<KeyboardInput>, second: bool) {
    context.spawn_background(move |_| {
        std::thread::sleep(Duration::from_secs(1));
        KeyboardMessage::FocusTimeout(second)
    });
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
    if stage == PointerStage::Move {
        fit_window_to_work_area(hwnd)?;
    }
    unsafe {
        let _ = SetForegroundWindow(hwnd);
        let _ = BringWindowToTop(hwnd);
    }
    if stage == PointerStage::Move {
        let (x, y) = virtual_screen_origin();
        inject_at(
            injector,
            x,
            y,
            InjectedInputMouseOptions::Move | InjectedInputMouseOptions::MoveNoCoalesce,
        )
        .map_err(|error| error.to_string())?;
        inject_at(
            injector,
            x,
            y,
            InjectedInputMouseOptions::LeftUp | InjectedInputMouseOptions::RightUp,
        )
        .map_err(|error| error.to_string())?;
    }
    let ((x, y), options) = match stage {
        PointerStage::Move => (
            client_screen_point(hwnd, 0.5, 0.1)?,
            InjectedInputMouseOptions::Move | InjectedInputMouseOptions::MoveNoCoalesce,
        ),
        PointerStage::LeftDown => (
            client_screen_point(hwnd, 0.5, 0.1)?,
            InjectedInputMouseOptions::LeftDown,
        ),
        PointerStage::MoveOutside => (
            client_screen_point(hwnd, 0.5, 0.75)?,
            InjectedInputMouseOptions::Move | InjectedInputMouseOptions::MoveNoCoalesce,
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
        PointerStage::Exit => (
            virtual_screen_origin(),
            InjectedInputMouseOptions::Move | InjectedInputMouseOptions::MoveNoCoalesce,
        ),
    };
    inject_at(injector, x, y, options).map_err(|error| error.to_string())
}

fn fit_window_to_work_area(hwnd: HWND) -> Result<(), String> {
    let mut window = RECT::default();
    if !unsafe { GetWindowRect(hwnd, &mut window) }.as_bool() {
        return Err("could not read the pointer fixture window rect".to_string());
    }
    let monitor = unsafe { MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST as u32) };
    let mut monitor_info = MONITORINFO {
        cbSize: size_of::<MONITORINFO>() as u32,
        ..Default::default()
    };
    if !unsafe { GetMonitorInfoW(monitor, &mut monitor_info) }.as_bool() {
        return Err("could not read the pointer fixture monitor work area".to_string());
    }

    let width = window.right - window.left;
    let height = window.bottom - window.top;
    let work = monitor_info.rcWork;
    let x = window
        .left
        .clamp(work.left, (work.right - width).max(work.left));
    let y = window
        .top
        .clamp(work.top, (work.bottom - height).max(work.top));
    if (x != window.left || y != window.top)
        && !unsafe { SetWindowPos(hwnd, None, x, y, 0, 0, (SWP_NOSIZE | SWP_NOZORDER) as u32) }
            .as_bool()
    {
        return Err("could not move the pointer fixture into the monitor work area".to_string());
    }
    Ok(())
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

pub(crate) enum EncodedImageMessage {
    Advance(u8),
    AwaitOverride,
    NativeCleared(Result<(), ImageSourceError>),
    Opened(u8),
    Failed(u8),
}

pub(crate) struct EncodedImageLifecycle {
    error: Option<&'static str>,
    image: ElementRef<Image>,
    stage: u8,
}

impl Component for EncodedImageLifecycle {
    type Input = FixtureInput;
    type Message = EncodedImageMessage;

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            error: None,
            image: ElementRef::new(),
            stage: 0,
        }
    }

    fn update(&mut self, message: Self::Message, context: &ComponentContext<Self>) {
        match message {
            EncodedImageMessage::NativeCleared(Ok(())) if self.stage == 0 => {
                context.spawn_background(|_| {
                    std::thread::sleep(Duration::from_millis(250));
                    EncodedImageMessage::AwaitOverride
                });
            }
            EncodedImageMessage::AwaitOverride if self.stage == 0 => {
                self.stage = 1;
            }
            EncodedImageMessage::NativeCleared(Err(_)) => {
                self.error = Some("native image source clear failed");
            }
            EncodedImageMessage::NativeCleared(Ok(())) => {
                self.error = Some("received an unexpected native image source completion");
            }
            EncodedImageMessage::AwaitOverride => {
                self.error = Some("received an unexpected native image source wait");
            }
            EncodedImageMessage::Advance(stage) if stage == self.stage => {
                self.stage += 1;
            }
            EncodedImageMessage::Opened(stage) if stage == self.stage && stage == 1 => {
                self.stage += 1;
            }
            EncodedImageMessage::Failed(2) if self.stage == 2 => {
                self.stage = 3;
            }
            EncodedImageMessage::Opened(_) => {
                self.error = Some("received an unexpected or duplicate ImageOpened event");
            }
            EncodedImageMessage::Failed(_) => {
                self.error = Some("received an unexpected or duplicate ImageFailed event");
            }
            EncodedImageMessage::Advance(_) => {}
        }
    }

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        if let Some(error) = self.error {
            let complete = input.complete.clone();
            context.use_effect("fail-encoded-image", (), move || {
                if !complete.call(Err(error.to_string())) {
                    eprintln!("encoded image fixture completion was rejected");
                    std::process::exit(1);
                }
                None
            });
            return TextBlock::new().text(error).into();
        }
        if self.stage == 4 {
            let complete = input.complete.clone();
            context.use_effect("complete-encoded-image", (), move || {
                if !complete.call(Ok(())) {
                    eprintln!("encoded image fixture completion was rejected");
                    std::process::exit(1);
                }
                None
            });
            return TextBlock::new().text("encoded image complete").into();
        }

        if self.stage == 0 {
            let image = self.image.clone();
            let sender = context.sender();
            context.use_effect("override-encoded-image", (), move || {
                if !image.request_set_native_source(None, move |result| {
                    sender.send(EncodedImageMessage::NativeCleared(result));
                }) {
                    eprintln!("native image source clear was rejected");
                    std::process::exit(1);
                }
                None
            });
        } else if self.stage == 3 {
            let stage = self.stage;
            let sender = context.sender();
            context.use_effect("advance-encoded-image", stage, move || {
                sender.send(EncodedImageMessage::Advance(stage));
                None
            });
        }
        let source = match self.stage {
            0 => EncodedImage::from_static(include_bytes!(
                "../../../../samples/reactor/icon-elements/image.png"
            )),
            1 => EncodedImage::from_static(include_bytes!(
                "../../../../samples/reactor/gallery/assets/Image.png"
            )),
            2 => EncodedImage::from_static(b"not an encoded image"),
            3 => EncodedImage::from_static(include_bytes!(
                "../../../../samples/reactor/gallery/assets/Image.png"
            )),
            _ => unreachable!(),
        };
        let stage = self.stage;
        Image::new()
            .source_data(source)
            .element_ref(&self.image)
            .on_opened(context.callback(move |()| EncodedImageMessage::Opened(stage)))
            .on_failed(context.callback(move |()| EncodedImageMessage::Failed(stage)))
            .width(64.0)
            .height(64.0)
            .into()
    }
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
        context.use_effect_guard("observe-image-scale", (), move || {
            image.observe_rasterization_scale(move |scale| {
                sender.send(ImageMessage::Scale(scale));
            })
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

pub(crate) enum TimerMessage {
    Fired,
    VerifyCancellation,
    UnexpectedDelivery,
}

pub(crate) struct TimerLifecycle {
    complete: Callback<FixtureResult>,
    timer: Option<ComponentTimer>,
}

impl TimerLifecycle {
    fn fail(&self, detail: &str) {
        if !self.complete.call(Err(detail.to_string())) {
            eprintln!("timer fixture failure was rejected");
            std::process::exit(1);
        }
    }
}

impl Component for TimerLifecycle {
    type Input = FixtureInput;
    type Message = TimerMessage;

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        let timer = context
            .set_timeout(Duration::from_millis(10), TimerMessage::Fired)
            .unwrap();
        Self {
            complete: input.complete.clone(),
            timer: Some(timer),
        }
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
        self.complete = input.complete.clone();
    }

    fn update(&mut self, message: Self::Message, context: &ComponentContext<Self>) {
        match message {
            TimerMessage::Fired => {
                self.timer = None;

                let cancelled = context
                    .set_timeout(Duration::from_millis(10), TimerMessage::UnexpectedDelivery)
                    .unwrap();
                cancelled.cancel();

                drop(
                    context
                        .set_timeout(Duration::from_millis(10), TimerMessage::UnexpectedDelivery)
                        .unwrap(),
                );

                self.timer = Some(
                    context
                        .set_timeout(Duration::from_millis(100), TimerMessage::VerifyCancellation)
                        .unwrap(),
                );
            }
            TimerMessage::VerifyCancellation => {
                self.timer = None;
                if !self.complete.call(Ok(())) {
                    eprintln!("timer fixture completion was rejected");
                    std::process::exit(1);
                }
            }
            TimerMessage::UnexpectedDelivery => {
                self.fail("a cancelled or dropped timer delivered its message");
            }
        }
    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        TextBlock::new().text("timer lifecycle").into()
    }
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
                    let device = GpuDevice::new_or_warp().map_err(|error| error.to_string())?;
                    let graphics = device
                        .create_graphics_device(&compositor)
                        .map_err(|error| error.to_string())?;
                    let surface = graphics
                        .create_drawing_surface(16.0, 16.0)
                        .map_err(|error| error.to_string())?;
                    if !surface
                        .draw(|session| {
                            session.clear(ColorF::CORNFLOWER_BLUE);
                            Ok(())
                        })
                        .map_err(|error| error.to_string())?
                    {
                        return Err(
                            "initial lifted composition surface draw lost its device".into()
                        );
                    }
                    surface.resize(24, 20).map_err(|error| error.to_string())?;
                    let replacement =
                        GpuDevice::new_or_warp().map_err(|error| error.to_string())?;
                    replacement
                        .replace_graphics_device(&graphics)
                        .map_err(|error| error.to_string())?;
                    if !surface
                        .draw(|session| {
                            session.clear(ColorF::TRANSPARENT);
                            Ok(())
                        })
                        .map_err(|error| error.to_string())?
                    {
                        return Err(
                            "rebound lifted composition surface draw lost its device".into()
                        );
                    }
                    background.set_brush(&compositor.create_surface_brush(&surface));
                    root.children().insert_at_bottom(&background);
                    let sender = context.sender();
                    if !self.host.request_set_child_visual(
                        Some(root.host_visual()),
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
                    Some(replacement.host_visual()),
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
        context.use_effect_guard("observe-composition", (), move || {
            host.observe_composition_host(move |event| {
                sender.send(CompositionMessage::Host(event));
            })
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
