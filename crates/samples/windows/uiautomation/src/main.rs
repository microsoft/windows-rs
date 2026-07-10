fn main() -> windows::core::Result<()> {
    use windows::{
        UI::UIAutomation::*, combaseapi::*, core::*, objbase::*, uiautomationclient::*, winuser::*,
    };

    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED as u32).ok()?;
        let window = FindWindowA(None, s!("Calculator"));
        if window.is_invalid() {
            return Err(Error::from_thread());
        }

        // Start with COM API
        let automation: IUIAutomation = CoCreateInstance(&CUIAutomation, None, CLSCTX_ALL)?;
        let element: IUIAutomationElement = automation.ElementFromHandle(UIA_HWND(window.0))?;

        // Use COM API
        let name = element.CurrentName()?;
        println!("window name: {name:?}");

        // Query for WinRT API (will fail on earlier versions of Windows)
        let element: Result<AutomationElement> = element.cast();

        if let Ok(element) = element {
            // Use WinRT API
            println!("file name: {:?}", element.ExecutableFileName()?);
        }
    }

    Ok(())
}
