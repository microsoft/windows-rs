use ::windows::runtime::Result;
use test_winrt::*;

#[::windows::runtime::implement(Windows::UI::Xaml::{IApplicationOverrides, IApplicationOverrides2})]
struct App {}

#[allow(non_snake_case)]
impl App {
    fn OnActivated(
        &self,
        _args: &Option<Windows::ApplicationModel::Activation::IActivatedEventArgs>,
    ) -> Result<()> {
        Ok(())
    }

    fn OnLaunched(
        &self,
        _args: &Option<Windows::ApplicationModel::Activation::LaunchActivatedEventArgs>,
    ) -> Result<()> {
        Ok(())
    }

    fn OnFileActivated(
        &self,
        _args: &Option<Windows::ApplicationModel::Activation::FileActivatedEventArgs>,
    ) -> Result<()> {
        Ok(())
    }

    fn OnSearchActivated(
        &self,
        _args: &Option<Windows::ApplicationModel::Activation::SearchActivatedEventArgs>,
    ) -> Result<()> {
        Ok(())
    }

    fn OnShareTargetActivated(
        &self,
        _args: &Option<Windows::ApplicationModel::Activation::ShareTargetActivatedEventArgs>,
    ) -> Result<()> {
        Ok(())
    }

    fn OnFileOpenPickerActivated(
        &self,
        _args: &Option<Windows::ApplicationModel::Activation::FileOpenPickerActivatedEventArgs>,
    ) -> Result<()> {
        Ok(())
    }

    fn OnFileSavePickerActivated(
        &self,
        _args: &Option<Windows::ApplicationModel::Activation::FileSavePickerActivatedEventArgs>,
    ) -> Result<()> {
        Ok(())
    }

    fn OnCachedFileUpdaterActivated(
        &self,
        _args: &Option<Windows::ApplicationModel::Activation::CachedFileUpdaterActivatedEventArgs>,
    ) -> Result<()> {
        Ok(())
    }

    fn OnWindowCreated(
        &self,
        _args: &Option<Windows::UI::Xaml::WindowCreatedEventArgs>,
    ) -> Result<()> {
        Ok(())
    }

    fn OnBackgroundActivated(
        &self,
        _args: &Option<Windows::ApplicationModel::Activation::BackgroundActivatedEventArgs>,
    ) -> Result<()> {
        Ok(())
    }
}
