use test_implement::*;
use windows::*;
use Windows::Foundation::IStringable;

#[test]
fn test_implement() -> Result<()> {
    let (sender, receiver) = std::sync::mpsc::channel();

    {
        let strong: IStringable = Stringable { sender }.into();

        let weak = strong.downgrade()?;
        assert_eq!(weak.upgrade().unwrap(), strong);

        assert_eq!(strong.ToString()?, "Stringable");
        drop(strong);

        assert_eq!(weak.upgrade(), None);
    }

    assert!(receiver.recv().unwrap() == "drop");
    Ok(())
}

// TODO: this just tests the syntax until #81 is further along.
#[implement(
    extend Windows::UI::Xaml::Application,
    override OnLaunched OnBackgroundActivated,
    Windows::Foundation::IStringable,
    Windows::Foundation::IClosable,
)]
struct Stringable {
    sender: std::sync::mpsc::Sender<String>,
}

#[allow(non_snake_case)]
impl Stringable {
    fn ToString(&self) -> Result<HSTRING> {
        Ok("Stringable".into())
    }

    fn Close(&self) -> Result<()> {
        Ok(())
    }
}

impl Drop for Stringable {
    fn drop(&mut self) {
        self.sender.send("drop".to_string()).unwrap();
    }
}
