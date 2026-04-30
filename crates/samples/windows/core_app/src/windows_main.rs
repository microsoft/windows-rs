#![windows_subsystem = "windows"]

use windows::{
    core::*,
    ApplicationModel::{Core::*, Package},
    Win32::{
        System::Com::*,
        UI::WindowsAndMessaging::{MessageBoxW, MB_ICONSTOP, MB_OK},
    },
    UI::Core::*,
};

#[implement(IFrameworkViewSource)]
struct CoreApp();

impl IFrameworkViewSource_Impl for CoreApp_Impl {
    fn CreateView(&self) -> Result<IFrameworkView> {
        Ok(CoreAppView().into())
    }
}

#[implement(IFrameworkView)]
struct CoreAppView();

impl IFrameworkView_Impl for CoreAppView_Impl {
    fn Initialize(&self, _: Ref<CoreApplicationView>) -> Result<()> {
        Ok(())
    }

    fn Load(&self, _: &HSTRING) -> Result<()> {
        Ok(())
    }

    fn Uninitialize(&self) -> Result<()> {
        Ok(())
    }

    fn Run(&self) -> Result<()> {
        let window = CoreWindow::GetForCurrentThread()?;
        window.Activate()?;

        let dispatcher = window.Dispatcher()?;
        dispatcher.ProcessEvents(CoreProcessEventsOption::ProcessUntilQuit)?;

        Ok(())
    }

    fn SetWindow(&self, _: Ref<CoreWindow>) -> Result<()> {
        Ok(())
    }
}

fn main() -> Result<()> {
    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED).ok()?;

        if let Err(result) = Package::Current() {
            MessageBoxW(
                None,
                w!("This sample must be registered (via register.cmd) and launched from Start."),
                w!("Error"),
                MB_ICONSTOP | MB_OK,
            );
            return Err(result);
        }
    }

    let app: IFrameworkViewSource = CoreApp().into();
    CoreApplication::Run(&app)?;
    Ok(())
}
