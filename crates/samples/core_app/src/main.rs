#![windows_subsystem = "windows"]

use windows::{
    core::*,
    ApplicationModel::{Core::*, Package},
    Win32::{
        Foundation::HWND,
        System::Com::*,
        UI::WindowsAndMessaging::{MessageBoxW, MB_ICONSTOP, MB_OK},
    },
    UI::Core::*,
};

#[implement(IFrameworkViewSource)]
struct CoreApp();

#[allow(non_snake_case)]
impl IFrameworkViewSource_Impl for CoreApp {
    fn CreateView(&mut self) -> Result<IFrameworkView> {
        // TODO: need self query `self.into()` to support implementing both IFrameworkViewSource and IFrameworkView on the same object.
        Ok(CoreAppView().into())
    }
}

#[implement(IFrameworkView)]
struct CoreAppView();

#[allow(non_snake_case)]
impl IFrameworkView_Impl for CoreAppView {
    fn Initialize(&mut self, _: &Option<CoreApplicationView>) -> Result<()> {
        Ok(())
    }

    fn Load(&mut self, _: &HSTRING) -> Result<()> {
        Ok(())
    }

    fn Uninitialize(&mut self) -> Result<()> {
        Ok(())
    }

    fn Run(&mut self) -> Result<()> {
        let window = CoreWindow::GetForCurrentThread()?;
        window.Activate()?;

        let dispatcher = window.Dispatcher()?;
        dispatcher.ProcessEvents(CoreProcessEventsOption::ProcessUntilQuit)?;

        Ok(())
    }

    fn SetWindow(&mut self, _: &Option<CoreWindow>) -> Result<()> {
        Ok(())
    }
}

fn main() -> Result<()> {
    unsafe {
        CoInitializeEx(std::ptr::null_mut(), COINIT_MULTITHREADED)?;

        if let Err(result) = Package::Current() {
            MessageBoxW(HWND::default(), "This sample must be registered (via register.cmd) and launched from Start.", "Error", MB_ICONSTOP | MB_OK);
            return Err(result);
        }
    }

    let app: IFrameworkViewSource = CoreApp().into();
    CoreApplication::Run(app)?;
    Ok(())
}
