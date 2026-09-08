#![cfg_attr(windows, windows_subsystem = "windows")]

fn main() -> windows::core::Result<()> {
    use windows::{
        ApplicationModel::{Core::*, Package},
        UI::Core::*,
        Win32::*,
        core::*,
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

    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED as u32).ok()?;

        if let Err(result) = Package::Current() {
            MessageBoxW(
                None,
                w!(
                    "Run crates\\samples\\windows\\core_app\\register.cmd, then launch \"Rust CoreApp\" from Start."
                ),
                w!("Error"),
                (MB_ICONSTOP | MB_OK) as u32,
            );
            return Err(result);
        }
    }

    let app: IFrameworkViewSource = CoreApp().into();
    CoreApplication::Run(&app)?;
    Ok(())
}
