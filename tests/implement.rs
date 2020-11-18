mod windows {
    pub use winrt::foundation;
}

use winrt::Interface; // for .cast()

#[test]
fn implement() -> winrt::Result<()> {
    let (sender, receiver) = std::sync::mpsc::channel();
    {
        let t = Thing {
            value: "hello".to_string(),
            sender,
        };

        let s: windows::foundation::IStringable = t.into();
        assert!(s.to_string()? == "hello");

        let c: windows::foundation::IClosable = s.cast()?;
        c.close()?;
        assert!(receiver.recv().unwrap() == "close: hello");
    }
    assert!(receiver.recv().unwrap() == "drop: hello");

    let (sender, receiver) = std::sync::mpsc::channel();
    {
        let t = Thing {
            value: "world".to_string(),
            sender,
        };

        let c: windows::foundation::IClosable = t.into();
        c.close()?;
        assert!(receiver.recv().unwrap() == "close: world");

        let s: windows::foundation::IStringable = c.cast()?;
        assert!(s.to_string()? == "world");
    }
    assert!(receiver.recv().unwrap() == "drop: world");

    let (sender, receiver) = std::sync::mpsc::channel();
    {
        let t = Thing {
            value: "object".to_string(),
            sender,
        };

        let s: windows::foundation::IStringable = t.into();
        assert!(s.to_string()? == "object");

        // Confirms that the conversion to `Object` properly handles
        // reference counting.
        let _: winrt::Object = s.into();
    }
    assert!(receiver.recv().unwrap() == "drop: object");

    Ok(())
}

#[winrt::implement(windows::foundation::{IStringable, IClosable})]
struct Thing {
    value: String,
    sender: std::sync::mpsc::Sender<String>,
}

impl Drop for Thing {
    fn drop(&mut self) {
        self.sender.send(format!("drop: {}", self.value)).unwrap();
    }
}

impl Thing {
    fn to_string(&self) -> winrt::Result<winrt::HString> {
        Ok(winrt::HString::from(&self.value))
    }

    fn close(&self) -> winrt::Result<()> {
        self.sender.send(format!("close: {}", self.value)).unwrap();
        Ok(())
    }
}
