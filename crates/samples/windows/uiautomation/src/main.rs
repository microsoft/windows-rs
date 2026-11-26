fn main() -> windows::core::Result<()> {
    use windows::{
        UI::UIAutomation::*, Win32::System::Com::*, Win32::UI::Accessibility::*,
        Win32::UI::WindowsAndMessaging::*, core::*,
    };

    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED).ok()?;
        let Ok(window) = FindWindowA(None, s!("Calculator")) else {
            println!("Open Calculator before running this sample.");
            return Ok(());
        };

        let automation: IUIAutomation = CoCreateInstance(&CUIAutomation, None, CLSCTX_ALL)?;
        let element: IUIAutomationElement = automation.ElementFromHandle(window)?;

        let name = element.CurrentName()?;
        println!("window name: {name:?}");

        let element: Result<AutomationElement> = element.cast();

        if let Ok(element) = element {
            println!("file name: {:?}", element.ExecutableFileName()?);
        }
    }

    Ok(())
}
