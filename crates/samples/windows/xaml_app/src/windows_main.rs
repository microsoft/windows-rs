mod bindings;
use bindings::*;

use windows::{
    core::*,
    ApplicationModel::{
        Activation::{
            CachedFileUpdaterActivatedEventArgs, FileActivatedEventArgs,
            FileOpenPickerActivatedEventArgs, FileSavePickerActivatedEventArgs,
            IActivatedEventArgs, LaunchActivatedEventArgs, SearchActivatedEventArgs,
            ShareTargetActivatedEventArgs,
        },
        Package,
    },
    Win32::{
        System::Com::*,
        UI::WindowsAndMessaging::{MessageBoxW, MB_ICONSTOP, MB_OK},
    },
};

#[implement(IApplicationOverrides)]
struct MyApp {
    text: HSTRING,
    placeholder: HSTRING,
}

impl IApplicationOverrides_Impl for MyApp_Impl {
    fn OnLaunched(&self, _: Ref<LaunchActivatedEventArgs>) -> Result<()> {
        let window = Window::Current()?;
        let text_box = TextBox::new()?;
        text_box.SetText(&self.text)?;
        text_box.SetPlaceholderText(&self.placeholder)?;
        window.SetContent(&text_box)?;
        window.Activate()
    }

    fn OnActivated(&self, _: Ref<IActivatedEventArgs>) -> Result<()> {
        Ok(())
    }
    fn OnFileActivated(&self, _: Ref<FileActivatedEventArgs>) -> Result<()> {
        Ok(())
    }
    fn OnSearchActivated(&self, _: Ref<SearchActivatedEventArgs>) -> Result<()> {
        Ok(())
    }
    fn OnShareTargetActivated(&self, _: Ref<ShareTargetActivatedEventArgs>) -> Result<()> {
        Ok(())
    }
    fn OnFileOpenPickerActivated(&self, _: Ref<FileOpenPickerActivatedEventArgs>) -> Result<()> {
        Ok(())
    }
    fn OnFileSavePickerActivated(&self, _: Ref<FileSavePickerActivatedEventArgs>) -> Result<()> {
        Ok(())
    }
    fn OnCachedFileUpdaterActivated(
        &self,
        _: Ref<CachedFileUpdaterActivatedEventArgs>,
    ) -> Result<()> {
        Ok(())
    }
    fn OnWindowCreated(&self, _: Ref<WindowCreatedEventArgs>) -> Result<()> {
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

    Application::Start(&ApplicationInitializationCallback::new(|_| {
        Application::compose(MyApp {
            text: h!("Hello world").clone(),
            placeholder: h!("What are you going to build today?").clone(),
        })?;
        Ok(())
    }))
}
