use windows::core::*;
use windows::UI::Xaml::*;
use windows as Windows;

// TODO: This is a compile-only test for now until #81 is further along and can provide composable test classes.

#[implement(extend Windows::UI::Xaml::Application, override OnLaunched)]
struct App();

#[allow(non_snake_case)]
impl App {
    fn OnLaunched(&self, _: &Option<Windows::ApplicationModel::Activation::LaunchActivatedEventArgs>) -> Result<()> {
        let app: Application = self.cast()?;
        assert!(app.FocusVisualKind()? == FocusVisualKind::DottedLine);
        Ok(())
    }
}
