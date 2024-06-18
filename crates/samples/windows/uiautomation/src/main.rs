use windows::{
    core::*, Win32::System::Com::*, Win32::UI::Accessibility::*, Win32::UI::WindowsAndMessaging::*,
    UI::UIAutomation::*,
};

fn main() -> Result<()> {
    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED).ok()?;
        let window = FindWindowA(None, s!("Calculator"))?;

        // Start with COM API
        let automation: IUIAutomation = CoCreateInstance(&CUIAutomation, None, CLSCTX_ALL)?;
        let element: IUIAutomationElement = automation.ElementFromHandle(window)?;

        // Use COM API
        let name = element.CurrentName()?;
        println!("window name: {name}");

        // Query for WinRT API (will fail on earlier versions of Windows)
        let element: Result<AutomationElement> = element.cast();

        if let Ok(element) = element {
            // Use WinRT API
            println!("file name: {}", element.ExecutableFileName()?);
        }
    }

    Ok(())
}
