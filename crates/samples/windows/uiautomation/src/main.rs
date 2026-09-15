fn main() -> windows::core::Result<()> {
    use windows::{UI::UIAutomation::*, Win32::*, core::*};

    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED as u32).ok()?;
        let window = FindWindowA(None, s!("Calculator"));
        if window.is_null() {
            println!("Open Calculator before running this sample.");
            return Ok(());
        }

        let automation: IUIAutomation = CoCreateInstance(&CUIAutomation, None, CLSCTX_ALL)?;
        let element: IUIAutomationElement = automation.ElementFromHandle(window as UIA_HWND)?;

        let name = element.CurrentName()?;
        println!("window name: {name:?}");

        let element: Result<AutomationElement> = element.cast();

        if let Ok(element) = element {
            println!("file name: {:?}", element.ExecutableFileName()?);
        }
    }

    Ok(())
}
