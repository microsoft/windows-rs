use tests::*;

#[test]
fn overrides() {
    let overrides: windows::ui::xaml::IApplicationOverrides = App {}.into();
    overrides.on_activated(None).unwrap();
}

#[winrt::implement(windows::ui::xaml::{IApplicationOverrides, IApplicationOverrides2})]
struct App {}

impl App {
    fn on_activated(
        &self,
        _args: &Option<windows::application_model::activation::IActivatedEventArgs>,
    ) -> winrt::Result<()> {
        Ok(())
    }

    fn on_launched(
        &self,
        _args: &Option<windows::application_model::activation::LaunchActivatedEventArgs>,
    ) -> winrt::Result<()> {
        Ok(())
    }

    fn on_file_activated(
        &self,
        _args: &Option<windows::application_model::activation::FileActivatedEventArgs>,
    ) -> winrt::Result<()> {
        Ok(())
    }

    fn on_search_activated(
        &self,
        _args: &Option<windows::application_model::activation::SearchActivatedEventArgs>,
    ) -> winrt::Result<()> {
        Ok(())
    }

    fn on_share_target_activated(
        &self,
        _args: &Option<windows::application_model::activation::ShareTargetActivatedEventArgs>,
    ) -> winrt::Result<()> {
        Ok(())
    }

    fn on_file_open_picker_activated(
        &self,
        _args: &Option<windows::application_model::activation::FileOpenPickerActivatedEventArgs>,
    ) -> winrt::Result<()> {
        Ok(())
    }

    fn on_file_save_picker_activated(
        &self,
        _args: &Option<windows::application_model::activation::FileSavePickerActivatedEventArgs>,
    ) -> winrt::Result<()> {
        Ok(())
    }

    fn on_cached_file_updater_activated(
        &self,
        _args: &Option<windows::application_model::activation::CachedFileUpdaterActivatedEventArgs>,
    ) -> winrt::Result<()> {
        Ok(())
    }

    fn on_window_created(
        &self,
        _args: &Option<windows::ui::xaml::WindowCreatedEventArgs>,
    ) -> winrt::Result<()> {
        Ok(())
    }

    fn on_background_activated(
        &self,
        _args: &Option<windows::application_model::activation::BackgroundActivatedEventArgs>,
    ) -> winrt::Result<()> {
        Ok(())
    }
}
