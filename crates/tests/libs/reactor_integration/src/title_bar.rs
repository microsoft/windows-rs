use std::time::{Duration, Instant};
use windows::Win32::*;
use windows_reactor::*;

// UIAutomationClient.h identifiers are not included in the generated Win32 projection.
const PROCESS_ID: i32 = 30002;
const INVOKE_PATTERN: i32 = 10000;

struct Page;

impl Component for Page {
    type Input = bool;
    type Message = ();

    fn create(_: &bool, _: &ComponentContext<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: (), context: &ComponentContext<Self>) {
        assert!(context.open_window::<Self>(false));
    }

    fn view(&self, main: &bool, context: &mut ViewContext<Self>) -> View {
        let content: View = if *main {
            Button::new()
                .content("Open")
                .on_click(context.message(()))
                .into()
        } else {
            TextBlock::new().text("Secondary").into()
        };
        TitleBar::new()
            .content(content)
            .right_header(TextBlock::new().text("Right header"))
            .into()
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

fn named(
    client: &IUIAutomation,
    window: &IUIAutomationElement,
    name: &str,
) -> Option<IUIAutomationElement> {
    unsafe {
        let all = window
            .FindAll(TreeScope_Descendants, &client.CreateTrueCondition().ok()?)
            .ok()?;
        for index in 0..all.Length().ok()? {
            let element = all.GetElement(index).ok()?;
            if String::from_utf16_lossy(&element.CurrentName().ok()?) == name {
                return Some(element);
            }
        }
        None
    }
}

fn windows(client: &IUIAutomation) -> windows_core::Result<Vec<IUIAutomationElement>> {
    unsafe {
        let process =
            client.CreatePropertyCondition(PROCESS_ID, &integer(GetCurrentProcessId() as i32))?;
        let windows = client
            .GetRootElement()?
            .FindAll(TreeScope_Children, &process)?;
        (0..windows.Length()?)
            .map(|index| windows.GetElement(index))
            .collect()
    }
}

// Distance from each window's right edge to its right header.
fn right_header_gaps(client: &IUIAutomation) -> windows_core::Result<Vec<i32>> {
    let mut gaps = Vec::new();
    for window in windows(client)? {
        if let Some(header) = named(client, &window, "Right header") {
            unsafe {
                gaps.push(
                    window.CurrentBoundingRectangle()?.right
                        - header.CurrentBoundingRectangle()?.right,
                );
            }
        }
    }
    Ok(gaps)
}

fn drive() {
    let client: IUIAutomation =
        unsafe { CoCreateInstance(&CUIAutomation, None, CLSCTX_INPROC_SERVER).unwrap() };
    let deadline = Instant::now() + Duration::from_secs(10);
    let open = loop {
        if let Some(open) = windows(&client)
            .ok()
            .and_then(|windows| named(&client, windows.first()?, "Open"))
        {
            break open;
        }
        assert!(Instant::now() < deadline, "main window did not appear");
        std::thread::sleep(Duration::from_millis(50));
    };
    unsafe {
        open.GetCurrentPatternAs::<IUIAutomationInvokePattern>(INVOKE_PATTERN)
            .unwrap()
            .Invoke()
            .unwrap();
    }
    // Layout settles asynchronously, so poll until both windows agree.
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        let gaps = right_header_gaps(&client).unwrap_or_default();
        if let [main, secondary] = gaps[..]
            && main == secondary
        {
            return;
        }
        assert!(
            Instant::now() < deadline,
            "right headers are misplaced: found gaps {gaps:?}"
        );
        std::thread::sleep(Duration::from_millis(50));
    }
}
fn main() -> windows_core::Result<()> {
    std::thread::spawn(|| {
        unsafe {
            CoInitializeEx(None, COINIT_MULTITHREADED as u32)
                .ok()
                .unwrap();
        }
        let passed = std::panic::catch_unwind(drive).is_ok();
        std::process::exit(if passed { 0 } else { 1 });
    });
    App::run_component::<Page>(true)
}
