#![allow(non_snake_case)]

use ::windows::core::Result;
use windows::UI::Xaml::*;
use windows::ApplicationModel::Activation::*;

#[::windows::core::implement(IApplicationOverrides, IApplicationOverrides2)]
struct App {}

impl IApplicationOverrides_Impl for App {
    fn OnActivated(&mut self, _args: &Option<IActivatedEventArgs>) -> Result<()> {
        Ok(())
    }

    fn OnLaunched(&mut self, _args: &Option<LaunchActivatedEventArgs>) -> Result<()> {
        Ok(())
    }
}

impl IApplicationOverrides2_Impl for App {
    fn OnBackgroundActivated(&mut self, _args: &Option<BackgroundActivatedEventArgs>) -> Result<()> {
        Ok(())
    }
}
