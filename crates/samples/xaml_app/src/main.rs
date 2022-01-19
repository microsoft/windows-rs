#![windows_subsystem = "windows"]
#![allow(non_snake_case)]

use windows::{
    core::*, ApplicationModel::Activation::*, Win32::System::Com::*, UI::Xaml::Controls::*,
    UI::Xaml::*,
};

#[implement(IApplicationOverrides)]
struct MyApp();

impl IApplicationOverrides_Impl for MyApp {
    fn OnLaunched(&mut self, _: &Option<LaunchActivatedEventArgs>) -> Result<()> {
        let window = Window::Current()?;
        window.SetContent(TextBox::new()?)?;
        window.Activate()
    }
}

fn main() -> Result<()> {
    unsafe {
        CoInitializeEx(std::ptr::null_mut(), COINIT_MULTITHREADED)?;
    }
    Application::Start(ApplicationInitializationCallback::new(|_| {
        Application::compose(MyApp())?;
        Ok(())
    }))
}
