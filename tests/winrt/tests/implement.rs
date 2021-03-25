use ::windows::Interface;
use test_winrt::Windows;

#[test]
fn implement() -> ::windows::Result<()> {
    let (sender, receiver) = std::sync::mpsc::channel();
    {
        let t = Thing {
            value: "hello".to_string(),
            sender,
        };

        let s: Windows::Foundation::IStringable = t.into();
        assert!(s.ToString()? == "hello");

        let c: Windows::Foundation::IClosable = s.cast()?;
        c.Close()?;
        assert!(receiver.recv().unwrap() == "close: hello");
    }
    assert!(receiver.recv().unwrap() == "drop: hello");

    let (sender, receiver) = std::sync::mpsc::channel();
    {
        let t = Thing {
            value: "world".to_string(),
            sender,
        };

        let c: Windows::Foundation::IClosable = t.into();
        c.Close()?;
        assert!(receiver.recv().unwrap() == "close: world");

        let s: Windows::Foundation::IStringable = c.cast()?;
        assert!(s.ToString()? == "world");
    }
    assert!(receiver.recv().unwrap() == "drop: world");

    let (sender, receiver) = std::sync::mpsc::channel();
    {
        let t = Thing {
            value: "object".to_string(),
            sender,
        };

        let s: Windows::Foundation::IStringable = t.into();
        assert!(s.ToString()? == "object");

        // Confirms that the conversion to `Object` properly handles
        // reference counting.
        let _: ::windows::Object = s.into();
    }
    assert!(receiver.recv().unwrap() == "drop: object");

    Ok(())
}

#[::windows::implement(Windows::Foundation::{IStringable, IClosable})]
struct Thing {
    value: String,
    sender: std::sync::mpsc::Sender<String>,
}

impl Drop for Thing {
    fn drop(&mut self) {
        self.sender.send(format!("drop: {}", self.value)).unwrap();
    }
}

#[allow(non_snake_case)]
impl Thing {
    fn ToString(&self) -> ::windows::Result<::windows::HString> {
        Ok(::windows::HString::from(&self.value))
    }

    fn Close(&self) -> ::windows::Result<()> {
        self.sender.send(format!("close: {}", self.value)).unwrap();
        Ok(())
    }
}
