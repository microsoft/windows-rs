use std::sync::mpsc::{Receiver, Sender, channel};
use std::time::{Duration, Instant};
use windows::Win32::*;
use windows_core::{HRESULT, w};
use windows_reactor::*;

const ICON: &str = concat!(env!("CARGO_MANIFEST_DIR"), "\\icon.ico");
// UIAutomationClient.h identifiers are not included in the generated Win32 projection.
const PROCESS_ID: i32 = 30002;
const CONTROL_TYPE: i32 = 30003;
const MENU_ITEM: i32 = 50011;
const INVOKE_PATTERN: i32 = 10000;
const ELEMENT_UNAVAILABLE: HRESULT = HRESULT(0x80040201u32 as i32);

enum Message {
    Ready,
    Selected(Key),
    Advance,
}

struct MenuApp {
    phase: u8,
    completed: Sender<u8>,
}

impl Application for MenuApp {
    type Input = Sender<u8>;
    type Message = Message;

    fn create(input: &Self::Input, context: &ApplicationContext<Self>) -> Self {
        context.show_window("fixture");
        assert!(context.sender().send(Message::Ready));
        Self {
            phase: 0,
            completed: input.clone(),
        }
    }

    fn update(&mut self, message: Message, _: &ApplicationContext<Self>) {
        match message {
            Message::Ready => assert_eq!(self.phase, 0),
            Message::Selected(key) => {
                let expected = match self.phase {
                    0 => "first",
                    1 => "second",
                    3 => "updated",
                    5 => "exit",
                    _ => panic!("unexpected menu selection"),
                };
                assert_eq!(key.as_str(), Some(expected));
                self.phase += 1;
            }
            Message::Advance => {
                assert!(matches!(self.phase, 2 | 4));
                self.phase += 1;
            }
        }
        self.completed.send(self.phase).unwrap();
    }

    fn view(&self, _: &Self::Input, context: &ApplicationViewContext<Self>) -> ApplicationView {
        let mut view = ApplicationView::new();
        if self.phase < 6 {
            view = view.window::<MenuWindow>("fixture", ());
        }
        if self.phase < 5 {
            let (key, label) = match self.phase {
                0 => ("first", "First command"),
                1 => ("old", "Old command"),
                2 => ("reopen", "Reopened command"),
                3 => ("updated", "Updated command"),
                4 => ("remove", "Removed command"),
                _ => unreachable!(),
            };
            view = view.notify_icon(
                "first",
                NotifyIcon::new(ICON)
                    .on_activate(context.callback(|_| Message::Advance))
                    .menu(Menu::new(
                        [MenuItem::item(key, label)],
                        context.callback(Message::Selected),
                    )),
            );
        }
        if matches!(self.phase, 1 | 5) {
            let (key, label) = if self.phase == 1 {
                ("second", "Second command")
            } else {
                ("exit", "Exit command")
            };
            view = view.notify_icon(
                "second",
                NotifyIcon::new(ICON).menu(Menu::new(
                    [MenuItem::item(key, label)],
                    context.callback(Message::Selected),
                )),
            );
        }
        view
    }
}

struct MenuWindow;

impl Component for MenuWindow {
    type Input = ();
    type Message = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        context.window_frame("Application menus fixture", "Testing notification menus")
    }
}

fn icon_windows() -> Vec<HWND> {
    let mut windows = Vec::new();
    let mut after = None;
    loop {
        unsafe {
            let hwnd = FindWindowExW(None, after, None, w!("windows-notifyicon"));
            if hwnd.is_null() {
                return windows;
            }
            let mut process = 0;
            GetWindowThreadProcessId(hwnd, Some(&mut process));
            if process == GetCurrentProcessId() {
                windows.push(hwnd);
            }
            after = Some(hwnd);
        }
    }
}

fn shell_event(automation: &Automation, hwnd: HWND, event: u32) {
    unsafe {
        let mut process = 0;
        GetWindowThreadProcessId(hwnd, Some(&mut process));
        assert_eq!(process, GetCurrentProcessId());
        if event == WM_CONTEXTMENU as u32 {
            GetWindowThreadProcessId(GetForegroundWindow(), Some(&mut process));
            if process != GetCurrentProcessId() {
                let deadline = Instant::now() + Duration::from_secs(5);
                let foreground = loop {
                    let hwnd = FindWindowW(None, w!("Application menus fixture"));
                    if !hwnd.is_null() && IsWindowVisible(hwnd).as_bool() {
                        break hwnd;
                    }
                    assert!(Instant::now() < deadline, "fixture window was not created");
                    std::thread::sleep(Duration::from_millis(20));
                };
                GetWindowThreadProcessId(foreground, Some(&mut process));
                assert_eq!(process, GetCurrentProcessId());
                // Synthetic Shell messages do not carry real user foreground activation.
                automation
                    .client
                    .ElementFromHandle(foreground.cast())
                    .unwrap()
                    .SetFocus()
                    .unwrap();
                loop {
                    GetWindowThreadProcessId(GetForegroundWindow(), Some(&mut process));
                    if process == GetCurrentProcessId() {
                        break;
                    }
                    assert!(
                        Instant::now() < deadline,
                        "fixture did not acquire foreground"
                    );
                    std::thread::sleep(Duration::from_millis(20));
                }
            }
        }
        PostMessageW(
            Some(hwnd),
            WM_USER as u32 + 1,
            32 | (32 << 16),
            event as isize,
        )
        .unwrap();
    }
}

fn integer(value: i32) -> VARIANT {
    VARIANT {
        Anonymous: VARIANT_0 {
            Anonymous: std::mem::ManuallyDrop::new(VARIANT_0_0 {
                vt: VT_I4 as u16,
                Anonymous: VARIANT_0_0_0 { lVal: value },
                ..Default::default()
            }),
        },
    }
}

struct Automation {
    client: IUIAutomation,
    process: IUIAutomationCondition,
    menu_item: IUIAutomationCondition,
}

impl Automation {
    fn new() -> windows_core::Result<Self> {
        unsafe {
            let client: IUIAutomation =
                CoCreateInstance(&CUIAutomation, None, CLSCTX_INPROC_SERVER)?;
            let process = client
                .CreatePropertyCondition(PROCESS_ID, &integer(GetCurrentProcessId() as i32))?;
            let menu_item = client.CreatePropertyCondition(CONTROL_TYPE, &integer(MENU_ITEM))?;
            Ok(Self {
                client,
                process,
                menu_item,
            })
        }
    }

    fn find(&self, name: &str) -> windows_core::Result<Option<IUIAutomationElement>> {
        unsafe {
            let windows = self
                .client
                .GetRootElement()?
                .FindAll(TreeScope_Children, &self.process)?;
            for index in 0..windows.Length()? {
                let window = windows.GetElement(index)?;
                let items = window.FindAll(TreeScope_Descendants, &self.menu_item)?;
                for index in 0..items.Length()? {
                    let item = items.GetElement(index)?;
                    if String::from_utf16(&item.CurrentName()?).unwrap() == name
                        && !item.CurrentIsOffscreen()?.as_bool()
                    {
                        return Ok(Some(item));
                    }
                }
            }
            Ok(None)
        }
    }

    fn wait(&self, name: &str, visible: bool) -> Option<IUIAutomationElement> {
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            match self.find(name) {
                Ok(item) if item.is_some() == visible => return item,
                Ok(_) => {}
                Err(error) if error.code() == ELEMENT_UNAVAILABLE => {}
                Err(error) => panic!("UI automation failed: {error}"),
            }
            assert!(
                Instant::now() < deadline,
                "menu visibility did not become {visible}: {name}"
            );
            std::thread::sleep(Duration::from_millis(20));
        }
    }

    fn invoke(&self, name: &str) {
        unsafe {
            self.wait(name, true)
                .unwrap()
                .GetCurrentPatternAs::<IUIAutomationInvokePattern>(INVOKE_PATTERN)
                .unwrap()
                .Invoke()
                .unwrap();
        }
    }
}

fn phase(completed: &Receiver<u8>, expected: u8) {
    assert_eq!(
        completed.recv_timeout(Duration::from_secs(5)).unwrap(),
        expected
    );
}

fn drive(completed: Receiver<u8>) -> windows_core::Result<()> {
    let automation = Automation::new()?;
    phase(&completed, 0);
    let first = icon_windows()[0];
    shell_event(&automation, first, WM_CONTEXTMENU as u32);
    automation.invoke("First command");
    phase(&completed, 1);
    let deadline = Instant::now() + Duration::from_secs(5);
    let second = loop {
        if let Some(second) = icon_windows().into_iter().find(|hwnd| *hwnd != first) {
            break second;
        }
        assert!(Instant::now() < deadline, "second icon was not created");
        std::thread::sleep(Duration::from_millis(20));
    };
    shell_event(&automation, first, WM_CONTEXTMENU as u32);
    let old = unsafe {
        automation
            .wait("Old command", true)
            .unwrap()
            .GetCurrentPatternAs::<IUIAutomationInvokePattern>(INVOKE_PATTERN)?
    };
    shell_event(&automation, first, WM_CONTEXTMENU as u32);
    shell_event(&automation, second, WM_CONTEXTMENU as u32);
    automation.wait("Second command", true);
    automation.wait("Old command", false);
    // A retained peer can reject Invoke with E_FAIL or ELEMENT_UNAVAILABLE after replacement.
    let result = unsafe { old.Invoke() };
    assert!(
        result.is_ok() || result == ELEMENT_UNAVAILABLE || result == HRESULT(0x80004005u32 as i32),
        "{result:?}"
    );
    automation.invoke("Second command");
    phase(&completed, 2);
    shell_event(&automation, first, WM_CONTEXTMENU as u32);
    automation.wait("Reopened command", true);
    shell_event(&automation, first, 0x400);
    phase(&completed, 3);
    automation.wait("Reopened command", false);
    shell_event(&automation, first, WM_CONTEXTMENU as u32);
    automation.invoke("Updated command");
    phase(&completed, 4);
    shell_event(&automation, first, WM_CONTEXTMENU as u32);
    automation.wait("Removed command", true);
    shell_event(&automation, first, 0x400);
    phase(&completed, 5);
    automation.wait("Removed command", false);
    let last = icon_windows()[0];
    shell_event(&automation, last, WM_CONTEXTMENU as u32);
    automation.invoke("Exit command");
    phase(&completed, 6);
    Ok(())
}

fn main() -> windows_core::Result<()> {
    std::thread::spawn(|| {
        std::thread::sleep(Duration::from_secs(30));
        eprintln!("Application menus fixture timed out");
        std::process::exit(1);
    });
    let (completed, receiver) = channel();
    let automation = std::thread::spawn(move || {
        unsafe {
            CoInitializeEx(None, COINIT_MULTITHREADED as u32)
                .ok()
                .unwrap();
        }
        let result = drive(receiver);
        unsafe {
            CoUninitialize();
        }
        result.unwrap();
    });
    App::run_application::<MenuApp>(completed)?;
    automation.join().unwrap();
    assert!(icon_windows().is_empty());
    Ok(())
}
