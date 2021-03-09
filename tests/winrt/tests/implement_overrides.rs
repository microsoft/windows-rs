use ::windows::Result;
use test_winrt::*;

#[::windows::implement(windows::ui::xaml::{IApplicationOverrides, IApplicationOverrides2})]
struct App {}

impl App {
    fn on_activated(
        &self,
        _args: &Option<windows::application_model::activation::IActivatedEventArgs>,
    ) -> Result<()> {
        Ok(())
    }

    fn on_launched(
        &self,
        _args: &Option<windows::application_model::activation::LaunchActivatedEventArgs>,
    ) -> Result<()> {
        Ok(())
    }

    fn on_file_activated(
        &self,
        _args: &Option<windows::application_model::activation::FileActivatedEventArgs>,
    ) -> Result<()> {
        Ok(())
    }

    fn on_search_activated(
        &self,
        _args: &Option<windows::application_model::activation::SearchActivatedEventArgs>,
    ) -> Result<()> {
        Ok(())
    }

    fn on_share_target_activated(
        &self,
        _args: &Option<windows::application_model::activation::ShareTargetActivatedEventArgs>,
    ) -> Result<()> {
        Ok(())
    }

    fn on_file_open_picker_activated(
        &self,
        _args: &Option<windows::application_model::activation::FileOpenPickerActivatedEventArgs>,
    ) -> Result<()> {
        Ok(())
    }

    fn on_file_save_picker_activated(
        &self,
        _args: &Option<windows::application_model::activation::FileSavePickerActivatedEventArgs>,
    ) -> Result<()> {
        Ok(())
    }

    fn on_cached_file_updater_activated(
        &self,
        _args: &Option<windows::application_model::activation::CachedFileUpdaterActivatedEventArgs>,
    ) -> Result<()> {
        Ok(())
    }

    fn on_window_created(
        &self,
        _args: &Option<windows::ui::xaml::WindowCreatedEventArgs>,
    ) -> Result<()> {
        Ok(())
    }

    fn on_background_activated(
        &self,
        _args: &Option<windows::application_model::activation::BackgroundActivatedEventArgs>,
    ) -> Result<()> {
        Ok(())
    }
}
