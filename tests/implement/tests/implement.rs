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

// TODO: should be able to extend without implementing any additional interfaces.
