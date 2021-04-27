use test_weak::*;
use windows::*;
use Windows::Foundation::IStringable;
use Windows::Win32::WinRT::{IWeakReference, IWeakReferenceSource};

#[test]
fn test_implement() -> Result<()> {
    let (sender, receiver) = std::sync::mpsc::channel();

    {
    let strong: IStringable = Stringable {sender }.into();

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
fn test_queries() {
    let (sender, receiver) = std::sync::mpsc::channel();

    {
        let strong: IStringable = Stringable {sender}.into();
        assert!(strong.cast::<Object>().is_ok());
        assert!(strong.cast::<IStringable>().is_ok());
        assert!(strong.cast::<IWeakReferenceSource>().is_ok());
        assert!(strong.cast::<IWeakReference>().is_err());

        let source = strong.cast::<IWeakReferenceSource>().unwrap();
        // assert!(source.cast::<IWeakReferenceSource>().is_ok());   
        // assert!(source.cast::<IWeakReference>().is_err());
        // assert!(source.cast::<IUnknown>().unwrap() == strong.cast::<IUnknown>().unwrap());

        // let mut weak = None;
        // let weak = unsafe { source.GetWeakReference(&mut weak).and_some(weak).unwrap() };
        // assert!(weak.cast::<IWeakReference>().is_ok());
        // assert!(weak.cast::<IWeakReferenceSource>().is_err());
        // assert!(weak.cast::<Object>().is_err());
        // assert!(weak.cast::<IStringable>().is_err());
        // assert!(weak.cast::<IWeakReference>().unwrap() == weak);
        // assert!(weak.cast::<IUnknown>().unwrap() != strong.cast::<IUnknown>().unwrap());
    }

    assert!(receiver.recv().unwrap() == "drop");
}

#[implement(Windows::Foundation::IStringable)]
struct Stringable {
    sender: std::sync::mpsc::Sender<String>,
}

#[allow(non_snake_case)]
impl Stringable {
    fn ToString(&self) -> Result<HString> {
        Ok("Stringable".into())
    }
}

impl Drop for Stringable {
    fn drop(&mut self) {
        self.sender.send("drop".to_string()).unwrap();
    }
}
