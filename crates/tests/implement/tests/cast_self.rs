use windows::core::*;
use windows::UI::Xaml::*;
use windows::ApplicationModel::Activation::*;

// TODO: This is a compile-only test for now until #81 is further along and can provide composable test classes.

#[implement(IApplicationOverrides)]
struct App();

#[allow(non_snake_case)]
impl IApplicationOverrides_Impl for App {
    fn OnLaunched(&mut self, _: &Option<LaunchActivatedEventArgs>) -> Result<()> {
        let app: Application = self.cast()?;
        assert!(app.FocusVisualKind()? == FocusVisualKind::DottedLine);
        Ok(())
    }
}
