use windows::{core::*, Foundation::*};

#[test]
fn implement() -> Result<()> {
    let (sender, receiver) = std::sync::mpsc::channel();
    {
        let t = Thing {
            value: "hello".to_string(),
            sender,
        };

        let s: IStringable = t.into();
        assert_eq!(s.ToString()?, "hello");

        let c: IClosable = s.cast()?;
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

        let c: IClosable = t.into();
        c.Close()?;
        assert!(receiver.recv().unwrap() == "close: world");

        let s: IStringable = c.cast()?;
        assert!(s.ToString()? == "world");
    }
    assert!(receiver.recv().unwrap() == "drop: world");

    let (sender, receiver) = std::sync::mpsc::channel();
    {
        let t = Thing {
            value: "object".to_string(),
            sender,
        };

        let s: IStringable = t.into();
        assert!(s.ToString()? == "object");

        // Confirms that the conversion to `IInspectable` properly handles
        // reference counting.
        let _: IInspectable = s.cast()?;
    }
    assert!(receiver.recv().unwrap() == "drop: object");

    Ok(())
}

#[implement({IStringable, IClosable})]
struct Thing {
    value: String,
    sender: std::sync::mpsc::Sender<String>,
}

impl Drop for Thing {
    fn drop(&mut self) {
        self.sender.send(format!("drop: {}", self.value)).unwrap();
    }
}

impl IStringable_Impl for Thing_Impl {
    fn ToString(&self) -> Result<HSTRING> {
        Ok(HSTRING::from(&self.value))
    }
}

impl IClosable_Impl for Thing_Impl {
    fn Close(&self) -> Result<()> {
        self.sender.send(format!("close: {}", self.value)).unwrap();
        Ok(())
    }
}
