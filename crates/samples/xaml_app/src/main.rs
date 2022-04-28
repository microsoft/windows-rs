#![windows_subsystem = "windows"]
#![allow(non_snake_case)]

use windows::{
    core::*,
    ApplicationModel::{Activation::LaunchActivatedEventArgs, Package},
    Win32::System::Com::*,
    Win32::{
        Foundation::HWND,
        UI::WindowsAndMessaging::{MessageBoxW, MB_ICONSTOP, MB_OK},
    },
    UI::Xaml::Controls::*,
    UI::Xaml::*,
};

#[implement(IApplicationOverrides)]
struct MyApp {
    text: &'static str,
    placeholder: &'static str,
}

impl IApplicationOverrides_Impl for MyApp {
    fn OnLaunched(&self, _: &Option<LaunchActivatedEventArgs>) -> Result<()> {
        let window = Window::Current()?;
        let text_box = TextBox::new()?;
        text_box.SetText(self.text)?;
        text_box.SetPlaceholderText(self.placeholder)?;
        window.SetContent(text_box)?;
        window.Activate()
    }
}

fn main() -> Result<()> {
    unsafe {
        CoInitializeEx(std::ptr::null(), COINIT_MULTITHREADED)?;

        if let Err(result) = Package::Current() {
            MessageBoxW(HWND::default(), "This sample must be registered (via register.cmd) and launched from Start.", "Error", MB_ICONSTOP | MB_OK);
            return Err(result);
        }
    }

    Application::Start(ApplicationInitializationCallback::new(|_| {
        Application::compose(MyApp { text: "Hello world", placeholder: "What are you going to build today?" })?;
        Ok(())
    }))
}
