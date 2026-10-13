use std::cell::Cell;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::time::{Duration, Instant};
use windows::Win32::*;
use windows_core::{HRESULT, w};
use windows_reactor::*;

const ICON: &str = concat!(env!("CARGO_MANIFEST_DIR"), "\\icon.ico");
const PROCESS_ID: i32 = 30002;
const CONTROL_TYPE: i32 = 30003;
const BUTTON: i32 = 50000;
const MENU_ITEM: i32 = 50011;
const INVOKE_PATTERN: i32 = 10000;
const ELEMENT_UNAVAILABLE: HRESULT = HRESULT(0x80040201u32 as i32);

enum Message {
    Ready,
    Advance,
    Selected(u8, String),
}

struct MenuApp {
    kind: String,
    phase: u8,
    completed: Sender<u8>,
}

fn menu(phase: u8, callback: Callback<Key>) -> Menu {
    let label = if phase == 3 {
        "Changed command"
    } else {
        "Command"
    };
    Menu::new(
        [
            MenuItem::item("command", label),
            MenuItem::submenu("more", "More", [MenuItem::item("nested", "Nested")]),
        ],
        callback,
    )
}

impl Application for MenuApp {
    type Input = (String, Sender<u8>);
    type Message = Message;

    fn create(input: &Self::Input, context: &ApplicationContext<Self>) -> Self {
        context.show_window("fixture");
        Self {
            kind: input.0.clone(),
            phase: 0,
            completed: input.1.clone(),
        }
    }

    fn update(&mut self, message: Message, _: &ApplicationContext<Self>) {
        match message {
            Message::Ready => assert_eq!(self.phase, 0),
            Message::Advance => {
                assert!(matches!(self.phase, 0 | 2 | 3 | 4 | 5 | 6));
                self.phase += 1;
            }
            Message::Selected(captured, key) => {
                assert_eq!(captured, self.phase);
                assert_eq!(key, "command");
                assert!(
                    matches!(self.phase, 1 | 7),
                    "retired popup delivered a click in phase {}",
                    self.phase
                );
                self.phase += 1;
            }
        }
        self.completed.send(self.phase).unwrap();
    }

    fn view(&self, _: &Self::Input, context: &ApplicationViewContext<Self>) -> ApplicationView {
        if self.phase == 8 {
            return ApplicationView::new();
        }
        let phase = self.phase;
        let mut icon = NotifyIcon::new(ICON).on_activate(context.callback(|_| Message::Advance));
        if self.kind == "tray" && phase != 4 {
            icon = icon.menu(menu(
                phase,
                context.callback(move |key: Key| {
                    Message::Selected(phase, key.as_str().unwrap().to_owned())
                }),
            ));
        }
        ApplicationView::new()
            .notify_icon(if phase < 6 { "driver" } else { "replacement" }, icon)
            .window::<MenuWindow>(
                "fixture",
                WindowInput {
                    kind: self.kind.clone(),
                    phase,
                    ready: context.callback(|()| Message::Ready),
                    selected: context.callback(|(phase, key)| Message::Selected(phase, key)),
                },
            )
    }
}

#[derive(Clone, PartialEq)]
struct WindowInput {
    kind: String,
    phase: u8,
    ready: Callback<()>,
    selected: Callback<(u8, String)>,
}

struct MenuWindow;

impl Component for MenuWindow {
    type Input = WindowInput;
    type Message = ();

    fn create(_: &Self::Input, _: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        // Initial placement is published after the window is activated.
        let ready = input.ready.clone();
        context.on_window_placement(move |_: WindowPlacement| ready.call(()));
        let phase = input.phase;
        let selected = input.selected.clone();
        let callback = Callback::new(move |key: Key| {
            selected.call((phase, key.as_str().unwrap().to_owned()));
        });
        let attached = phase != 4;
        let target: View = match input.kind.as_str() {
            "menu" => {
                let button = Button::new().content("Open menu");
                if attached {
                    button.menu(menu(phase, callback))
                } else {
                    button.into()
                }
            }
            "dropdown" => {
                let button = DropDownButton::new().content("Open menu");
                if attached {
                    button.menu(menu(phase, callback))
                } else {
                    button.into()
                }
            }
            "menubar" => {
                let item = MenuBarItem::new().title("Open menu");
                let item = if attached {
                    item.menu(menu(phase, callback))
                } else {
                    item.into()
                };
                MenuBar::new().items([item]).into()
            }
            "command" => {
                let button = Button::new().content("Open menu");
                if attached {
                    button.command_bar_flyout(CommandBarFlyout::new(
                        [CommandBarCommand::button(
                            "command",
                            if phase == 3 {
                                "Changed command"
                            } else {
                                "Command"
                            },
                        )],
                        [],
                        callback,
                    ))
                } else {
                    button.into()
                }
            }
            "tray" => TextBlock::new().text("Notification menu").into(),
            _ => unreachable!(),
        };
        context.window_visuals(WindowVisuals::new().client_size(450.0, 350.0));
        context.window_frame(
            "Menu lifetimes fixture",
            StackPanel::new()
                .margin(20.0)
                .spacing(12.0)
                .keyed_children([
                    keyed("phase", TextBlock::new().text(format!("Phase {phase}"))),
                    keyed("focus", Button::new().content("Driver focus")),
                    keyed(if phase < 6 { "target" } else { "replacement" }, target),
                ]),
        )
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
    command: IUIAutomationCondition,
    phase: Cell<u8>,
}

impl Automation {
    fn new(kind: &str) -> windows_core::Result<Self> {
        unsafe {
            let client: IUIAutomation =
                CoCreateInstance(&CUIAutomation, None, CLSCTX_INPROC_SERVER)?;
            let process = client
                .CreatePropertyCondition(PROCESS_ID, &integer(GetCurrentProcessId() as i32))?;
            let command = client.CreatePropertyCondition(CONTROL_TYPE, &integer(MENU_ITEM))?;
            let command = if kind == "command" {
                client.CreateOrCondition(
                    &command,
                    &client.CreatePropertyCondition(CONTROL_TYPE, &integer(BUTTON))?,
                )?
            } else {
                command
            };
            Ok(Self {
                client,
                process,
                command,
                phase: Cell::new(0),
            })
        }
    }

    fn find(&self, name: &str, command: bool) -> windows_core::Result<Vec<IUIAutomationElement>> {
        unsafe {
            let condition = if command {
                self.command.clone()
            } else {
                self.client.CreateTrueCondition()?
            };
            let roots = self
                .client
                .GetRootElement()?
                .FindAll(TreeScope_Children, &self.process)?;
            let mut found = Vec::new();
            for index in 0..roots.Length()? {
                let items = roots
                    .GetElement(index)?
                    .FindAll(TreeScope_Descendants, &condition)?;
                for index in 0..items.Length()? {
                    let item = items.GetElement(index)?;
                    if String::from_utf16(&item.CurrentName()?).unwrap() == name
                        && !item.CurrentIsOffscreen()?.as_bool()
                    {
                        found.push(item);
                    }
                }
            }
            Ok(found)
        }
    }

    fn wait(&self, name: &str, command: bool, visible: bool) -> Option<IUIAutomationElement> {
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            match self.find(name, command) {
                Ok(items) if !items.is_empty() == visible => return items.into_iter().next(),
                Ok(_) => {}
                Err(error) if error.code() == ELEMENT_UNAVAILABLE => {}
                Err(error) => panic!("UI automation failed: {error}"),
            }
            assert!(
                Instant::now() < deadline,
                "phase {}: visibility did not become {visible}: {name}",
                self.phase.get()
            );
            std::thread::sleep(Duration::from_millis(20));
        }
    }

    fn invoke(&self, name: &str, command: bool) -> windows_core::Result<()> {
        unsafe {
            self.wait(name, command, true)
                .unwrap()
                .GetCurrentPatternAs::<IUIAutomationInvokePattern>(INVOKE_PATTERN)?
                .Invoke()
                .ok()
        }
    }
}

fn icon_window() -> HWND {
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        let mut after = None;
        loop {
            unsafe {
                let hwnd = FindWindowExW(None, after, None, w!("windows-notifyicon"));
                if hwnd.is_null() {
                    break;
                }
                let mut process = 0;
                GetWindowThreadProcessId(hwnd, Some(&mut process));
                if process == GetCurrentProcessId() {
                    return hwnd;
                }
                after = Some(hwnd);
            }
        }
        assert!(
            Instant::now() < deadline,
            "notification callback window was not created"
        );
        std::thread::sleep(Duration::from_millis(20));
    }
}

fn shell_event(hwnd: HWND, event: u32) -> windows_core::Result<()> {
    unsafe {
        let mut process = 0;
        GetWindowThreadProcessId(hwnd, Some(&mut process));
        assert_eq!(process, GetCurrentProcessId());
        PostMessageW(
            Some(hwnd),
            WM_USER as u32 + 1,
            32 | (32 << 16),
            event as isize,
        )
        .ok()
    }
}

fn phase(automation: &Automation, completed: &Receiver<u8>, expected: u8) {
    assert_eq!(
        completed.recv_timeout(Duration::from_secs(5)).unwrap(),
        expected
    );
    automation.phase.set(expected);
    if expected < 8 {
        automation.wait(&format!("Phase {expected}"), false, true);
    }
}

fn open(
    automation: &Automation,
    kind: &str,
    icon: HWND,
    label: &str,
) -> windows_core::Result<IUIAutomationElement> {
    let visible = automation.find(label, true)?;
    if !visible.is_empty() {
        assert_eq!(visible.len(), 1, "duplicate menu entries");
        return Ok(visible.into_iter().next().unwrap());
    }
    if kind == "tray" {
        shell_event(icon, WM_CONTEXTMENU as u32)?;
    } else {
        automation.invoke("Open menu", false)?;
    }
    let item = automation.wait(label, true, true).unwrap();
    assert_eq!(
        automation.find(label, true)?.len(),
        1,
        "duplicate menu entries"
    );
    Ok(item)
}

fn stale(pattern: &IUIAutomationInvokePattern) {
    let result = unsafe { pattern.Invoke() };
    assert!(
        result.is_ok() || result == ELEMENT_UNAVAILABLE || result == HRESULT(0x80004005u32 as i32),
        "{result:?}",
    );
}

fn closed(automation: &Automation, kind: &str, label: &str) {
    automation.wait(label, true, false);
    if kind != "command" {
        automation.wait("More", true, false);
    }
}

fn drive(kind: &str, completed: Receiver<u8>) -> windows_core::Result<()> {
    let automation = Automation::new(kind)?;
    phase(&automation, &completed, 0);
    unsafe {
        let hwnd = FindWindowW(None, w!("Menu lifetimes fixture"));
        let mut process = 0;
        GetWindowThreadProcessId(hwnd, Some(&mut process));
        assert_eq!(process, GetCurrentProcessId());
        let window = automation.client.ElementFromHandle(hwnd.cast())?;
        window.SetFocus().ok()?;
        automation
            .wait("Driver focus", false, true)
            .unwrap()
            .SetFocus()
            .ok()?;
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            let mut process = 0;
            GetWindowThreadProcessId(GetForegroundWindow(), Some(&mut process));
            if process == GetCurrentProcessId() {
                break;
            }
            assert!(
                Instant::now() < deadline,
                "fixture did not acquire foreground"
            );
            window.SetFocus().ok()?;
            std::thread::sleep(Duration::from_millis(20));
        }
    }
    let icon = icon_window();
    let original = open(&automation, kind, icon, "Command")?;
    shell_event(icon, 0x400)?;
    phase(&automation, &completed, 1);
    let current = automation.wait("Command", true, true).unwrap();
    assert_eq!(automation.find("Command", true)?.len(), 1);
    unsafe {
        assert!(
            automation
                .client
                .CompareElements(&original, &current)?
                .as_bool()
        );
        original
            .GetCurrentPatternAs::<IUIAutomationInvokePattern>(INVOKE_PATTERN)?
            .Invoke()
            .ok()?;
    }
    phase(&automation, &completed, 2);
    if kind != "command" {
        automation.wait("More", true, false);
    }

    let replaced = unsafe {
        open(&automation, kind, icon, "Command")?
            .GetCurrentPatternAs::<IUIAutomationInvokePattern>(INVOKE_PATTERN)?
    };
    shell_event(icon, 0x400)?;
    phase(&automation, &completed, 3);
    closed(&automation, kind, "Command");
    stale(&replaced);
    let removed = unsafe {
        open(&automation, kind, icon, "Changed command")?
            .GetCurrentPatternAs::<IUIAutomationInvokePattern>(INVOKE_PATTERN)?
    };
    shell_event(icon, 0x400)?;
    phase(&automation, &completed, 4);
    closed(&automation, kind, "Changed command");
    stale(&removed);
    shell_event(icon, 0x400)?;
    phase(&automation, &completed, 5);
    let owner_replaced = unsafe {
        open(&automation, kind, icon, "Command")?
            .GetCurrentPatternAs::<IUIAutomationInvokePattern>(INVOKE_PATTERN)?
    };
    stale(&replaced);
    stale(&removed);
    shell_event(icon, 0x400)?;
    phase(&automation, &completed, 6);
    closed(&automation, kind, "Command");
    stale(&owner_replaced);
    let icon = icon_window();
    let original = open(&automation, kind, icon, "Command")?;
    shell_event(icon, 0x400)?;
    phase(&automation, &completed, 7);
    let current = automation.wait("Command", true, true).unwrap();
    unsafe {
        assert!(
            automation
                .client
                .CompareElements(&original, &current)?
                .as_bool()
        );
    }
    stale(&owner_replaced);
    automation.invoke("Command", true)?;
    phase(&automation, &completed, 8);
    Ok(())
}

fn main() -> windows_core::Result<()> {
    let kind = std::env::args().nth(1).unwrap();
    assert!(matches!(
        kind.as_str(),
        "menu" | "dropdown" | "menubar" | "command" | "tray"
    ));
    std::thread::spawn(|| {
        std::thread::sleep(Duration::from_secs(45));
        eprintln!("Menu lifetimes fixture timed out");
        std::process::exit(1);
    });
    let (completed, receiver) = channel();
    let driver_kind = kind.clone();
    let driver = std::thread::spawn(move || {
        unsafe {
            CoInitializeEx(None, COINIT_MULTITHREADED as u32)
                .ok()
                .unwrap();
        }
        let result = drive(&driver_kind, receiver);
        unsafe {
            CoUninitialize();
        }
        result.unwrap();
    });
    App::run_application::<MenuApp>((kind, completed))?;
    driver.join().unwrap();
    Ok(())
}
