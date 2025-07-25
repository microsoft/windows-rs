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

#[allow(non_snake_case)]
impl IFrameworkViewSource_Impl for CoreApp_Impl {
    fn CreateView(&self) -> Result<IFrameworkView, HRESULT> {
        Ok(CoreAppView().into())
    }
}

#[implement(IFrameworkView)]
struct CoreAppView();

#[allow(non_snake_case)]
impl IFrameworkView_Impl for CoreAppView_Impl {
    fn Initialize(&self, _: Ref<CoreApplicationView>) -> Result<(), HRESULT> {
        Ok(())
    }

    fn Load(&self, _: &HSTRING) -> Result<(), HRESULT> {
        Ok(())
    }

    fn Uninitialize(&self) -> Result<(), HRESULT> {
        Ok(())
    }

    fn Run(&self) -> Result<(), HRESULT> {
        let window = CoreWindow::GetForCurrentThread()?;
        window.Activate()?;

        let dispatcher = window.Dispatcher()?;
        dispatcher.ProcessEvents(CoreProcessEventsOption::ProcessUntilQuit)?;

        Ok(())
    }

    fn SetWindow(&self, _: Ref<CoreWindow>) -> Result<(), HRESULT> {
        Ok(())
    }
}

fn main() -> Result<(), HRESULT> {
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
