use test_winrt_weak::*;
use windows::core::*;
use Component::Classes::{Activatable, NoWeakRef};
use Windows::Foundation::IStringable;
use Windows::Win32::Foundation::E_NOINTERFACE;
use Windows::Win32::System::WinRT::{IWeakReference, IWeakReferenceSource};

// The C++ Activatable class supports weak references, so it is used to test a successful downgrade and upgrade.
#[test]
fn test_cpp_success() -> Result<()> {
    let strong = Activatable::new()?;
    let weak = strong.downgrade()?;

    assert_eq!(weak.upgrade().unwrap(), strong);
    drop(strong);
    assert_eq!(weak.upgrade(), None);

    Ok(())
}

// The C++ NoWeakRef class does not support weak references, so it is used to test an unsuccessful downgrade.
#[test]
fn test_cpp_failure() -> Result<()> {
    let strong = NoWeakRef::new()?;
    let result = strong.downgrade();

    assert_eq!(result.is_err(), true);

    // E_NOINTERFACE is returned because IWeakReferenceSource is not implemented.
    assert_eq!(result.unwrap_err().code(), E_NOINTERFACE);

    Ok(())
}

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

#[test]
fn test_no_tearoff() {
    let (sender, receiver) = std::sync::mpsc::channel();

    {
        let strong: IStringable = Stringable { sender }.into();
        let _ = strong.clone();
        let _ = strong.cast::<IStringable>().unwrap();
        assert_eq!(strong.ToString().unwrap(), "Stringable");
    }

    assert!(receiver.recv().unwrap() == "drop");
}

#[test]
fn test_queries() {
    let (sender, receiver) = std::sync::mpsc::channel();

    {
        let strong: IStringable = Stringable { sender }.into();
        assert!(strong.cast::<IInspectable>().is_ok());
        assert!(strong.cast::<IStringable>().is_ok());
        assert!(strong.cast::<IWeakReferenceSource>().is_ok());
        assert!(strong.cast::<IWeakReference>().is_err());

        let source = strong.cast::<IWeakReferenceSource>().unwrap();
        assert!(source.cast::<IWeakReferenceSource>().is_ok());
        assert!(source.cast::<IWeakReference>().is_err());
        assert!(source.cast::<IUnknown>().unwrap() == strong.cast::<IUnknown>().unwrap());

        let weak = unsafe { source.GetWeakReference().unwrap() };
        assert!(weak.cast::<IWeakReference>().is_ok());
        assert!(weak.cast::<IWeakReferenceSource>().is_err());
        assert!(weak.cast::<IInspectable>().is_err());
        assert!(weak.cast::<IStringable>().is_err());
        assert!(weak.cast::<IWeakReference>().unwrap() == weak);
        assert!(weak.cast::<IUnknown>().unwrap() != strong.cast::<IUnknown>().unwrap());
    }

    assert!(receiver.recv().unwrap() == "drop");
}

#[implement(Windows::Foundation::IStringable)]
struct Stringable {
    sender: std::sync::mpsc::Sender<String>,
}

#[allow(non_snake_case)]
impl Stringable {
    fn ToString(&self) -> Result<HSTRING> {
        Ok("Stringable".into())
    }
}

impl Drop for Stringable {
    fn drop(&mut self) {
        self.sender.send("drop".to_string()).unwrap();
    }
}
