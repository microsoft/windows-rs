use tests::*;

#[test]
fn overrides() {
    let overrides: windows::ui::xaml::IApplicationOverrides = App {}.into();
    overrides.on_activated(None).unwrap();
}

#[::windows::implement(windows::ui::xaml::{IApplicationOverrides, IApplicationOverrides2})]
struct App {}

impl App {
    fn on_activated(
        &self,
        _args: &Option<windows::application_model::activation::IActivatedEventArgs>,
    ) -> windows::Result<()> {
        Ok(())
    }

    fn on_launched(
        &self,
        _args: &Option<windows::application_model::activation::LaunchActivatedEventArgs>,
    ) -> windows::Result<()> {
        Ok(())
    }

    fn on_file_activated(
        &self,
        _args: &Option<windows::application_model::activation::FileActivatedEventArgs>,
    ) -> windows::Result<()> {
        Ok(())
    }

    fn on_search_activated(
        &self,
        _args: &Option<windows::application_model::activation::SearchActivatedEventArgs>,
    ) -> windows::Result<()> {
        Ok(())
    }

    fn on_share_target_activated(
        &self,
        _args: &Option<windows::application_model::activation::ShareTargetActivatedEventArgs>,
    ) -> windows::Result<()> {
        Ok(())
    }

    fn on_file_open_picker_activated(
        &self,
        _args: &Option<windows::application_model::activation::FileOpenPickerActivatedEventArgs>,
    ) -> windows::Result<()> {
        Ok(())
    }

    fn on_file_save_picker_activated(
        &self,
        _args: &Option<windows::application_model::activation::FileSavePickerActivatedEventArgs>,
    ) -> windows::Result<()> {
        Ok(())
    }

    fn on_cached_file_updater_activated(
        &self,
        _args: &Option<windows::application_model::activation::CachedFileUpdaterActivatedEventArgs>,
    ) -> windows::Result<()> {
        Ok(())
    }

    fn on_window_created(
        &self,
        _args: &Option<windows::ui::xaml::WindowCreatedEventArgs>,
    ) -> windows::Result<()> {
        Ok(())
    }

    fn on_background_activated(
        &self,
        _args: &Option<windows::application_model::activation::BackgroundActivatedEventArgs>,
    ) -> windows::Result<()> {
        Ok(())
    }
}
