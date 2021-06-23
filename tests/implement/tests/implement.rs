use test_implement::*;
use windows::*;
use Windows::Foundation::IClosable;

// TODO: this just tests the syntax until #81 is further along.

#[test]
fn test_implement() -> Result<()> {
    let app: IClosable = AppWithOverrides {}.into();
    app.Close()?;

    let app: IClosable = AppNoOverrides {}.into();
    app.Close()?;

    let app: IClosable = NoExtend {}.into();
    app.Close()?;

    Ok(())
}

#[implement(
    extend Windows::UI::Xaml::Application,
    override OnLaunched OnBackgroundActivated,
    Windows::Foundation::IStringable,
    Windows::Foundation::IClosable,
)]
struct AppWithOverrides {}

#[allow(non_snake_case)]
impl AppWithOverrides {
    fn ToString(&self) -> Result<HSTRING> {
        Ok("Stringable".into())
    }

    fn Close(&self) -> Result<()> {
        Ok(())
    }

    fn OnLaunched(
        &self,
        _: &Option<Windows::ApplicationModel::Activation::LaunchActivatedEventArgs>,
    ) -> Result<()> {
        Ok(())
    }

    fn OnBackgroundActivated(
        &self,
        _: &Option<Windows::ApplicationModel::Activation::BackgroundActivatedEventArgs>,
    ) -> Result<()> {
        Ok(())
    }
}

#[implement(
    extend Windows::UI::Xaml::Application,
    Windows::Foundation::{IStringable, IClosable},
)]
struct AppNoOverrides {}

#[allow(non_snake_case)]
impl AppNoOverrides {
    fn ToString(&self) -> Result<HSTRING> {
        Ok("Stringable".into())
    }

    fn Close(&self) -> Result<()> {
        Ok(())
    }
}

#[implement(
    Windows::Foundation::{IStringable, IClosable},
)]
struct NoExtend {}

#[allow(non_snake_case)]
impl NoExtend {
    fn ToString(&self) -> Result<HSTRING> {
        Ok("Stringable".into())
    }

    fn Close(&self) -> Result<()> {
        Ok(())
    }
}

#[implement(
    // TODO: make sure we have the same error reporting as the build macro for when the types/overrides aren't found.
    extend Windows::UI::Xaml::Controls::Button,
    override OnContentChanged, // ContentControl
    override OnPointerEntered, // Control
    override OnApplyTemplate, // FrameworkElement
)]
struct Button {}

#[allow(non_snake_case)]
impl Button {
    fn OnContentChanged(&self, _: &Option<IInspectable>, _: &Option<IInspectable>) -> Result<()> {
        Ok(())
    }

    // TODO: need option to omit Option and/or reference.
    fn OnPointerEntered(
        &self,
        _: &Option<Windows::UI::Xaml::Input::PointerRoutedEventArgs>,
    ) -> Result<()> {
        Ok(())
    }

    fn OnApplyTemplate(&self) -> Result<()> {
        Ok(())
    }
}
