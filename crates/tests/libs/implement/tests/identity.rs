#![cfg(windows)]
use std::sync::atomic::*;
use windows::{Foundation::*, core::*};

static COUNTER: AtomicIsize = AtomicIsize::new(0);

#[implement(IStringable, IClosable)]
struct Test(String);

impl Test {
    fn new(value: &str) -> Self {
        COUNTER.fetch_add(1, Ordering::Relaxed);
        Self(value.to_string())
    }
}

impl Drop for Test {
    fn drop(&mut self) {
        COUNTER.fetch_sub(1, Ordering::Release);
    }
}

impl IStringable_Impl for Test_Impl {
    fn ToString(&self) -> Result<HSTRING> {
        Ok(self.0.as_str().into())
    }
}

impl IClosable_Impl for Test_Impl {
    fn Close(&self) -> Result<()> {
        Ok(())
    }
}

#[test]
fn identity() -> Result<()> {
    assert_eq!(COUNTER.load(Ordering::Acquire), 0);
    {
        let a: IStringable = Test::new("test").into();
        assert_eq!(COUNTER.load(Ordering::Acquire), 1);
        assert!(a.ToString()? == "test");

        let b: IClosable = a.cast()?;
        b.Close()?;

        let c: IUnknown = b.cast()?;

        let d: IInspectable = c.cast()?;

        assert!(a == d.cast()?);
    }
    {
        let a: IUnknown = Test::new("test").into();
        let b: IClosable = a.cast()?;
        let c: IStringable = b.cast()?;
        assert!(c.ToString()? == "test");
    }
    {
        let a: IInspectable = Test::new("test").into();
        let b: IStringable = a.cast()?;
        assert!(b.ToString()? == "test");
    }
    {
        let a: IInspectable = Test::new("test").into();
        assert_eq!(a.GetRuntimeClassName()?, "Windows.Foundation.IStringable");

        let b: IStringable = a.cast()?;
        let c: &IInspectable = &b.cast()?;
        assert_eq!(c.GetRuntimeClassName()?, "Windows.Foundation.IStringable");

        // The runtime class name belongs to the object, so every interface reports the same name.
        let d: IClosable = a.cast()?;
        let e: &IInspectable = (&d).into();
        assert_eq!(e.GetRuntimeClassName()?, "Windows.Foundation.IStringable");

        let f: IInspectable = e.cast()?;
        assert_eq!(f.GetRuntimeClassName()?, "Windows.Foundation.IStringable");
    }
    assert_eq!(COUNTER.load(Ordering::Acquire), 0);
    Ok(())
}

#[interface("7bc7af8e-454b-4daa-8731-988f3f306522")]
unsafe trait IClassic: IUnknown {}

#[interface("3287dfc7-a1ba-4095-a4c6-e61202871265")]
unsafe trait ICustomClosable: IClosable {}

#[interface("591f03b7-fb1d-4076-ab85-550175be152e")]
unsafe trait ICustomClosable2: ICustomClosable {}

#[implement(IClassic, ICustomClosable2)]
struct Mixed;

impl IClassic_Impl for Mixed_Impl {}
impl ICustomClosable_Impl for Mixed_Impl {}
impl ICustomClosable2_Impl for Mixed_Impl {}
impl IClosable_Impl for Mixed_Impl {
    fn Close(&self) -> Result<()> {
        Ok(())
    }
}

struct MixedDecl;

implement_decl! {
    impl MixedDecl as MixedDecl_Impl: [IClassic, ICustomClosable2]
}

impl IClassic_Impl for MixedDecl_Impl {}
impl ICustomClosable_Impl for MixedDecl_Impl {}
impl ICustomClosable2_Impl for MixedDecl_Impl {}
impl IClosable_Impl for MixedDecl_Impl {
    fn Close(&self) -> Result<()> {
        Ok(())
    }
}

#[test]
fn mixed_interface_identity() -> Result<()> {
    for unknown in [IUnknown::from(Mixed), IUnknown::from(MixedDecl)] {
        let closable: ICustomClosable2 = unknown.cast()?;
        let parent: &IClosable = &closable;
        let upcast: &IInspectable = parent.into();
        let inspectable: IInspectable = unknown.cast()?;
        assert_eq!(inspectable.as_raw(), upcast.as_raw());
        assert_ne!(unknown.as_raw(), inspectable.as_raw());

        for source in [
            unknown.clone(),
            IUnknown::from(closable),
            IUnknown::from(inspectable.clone()),
        ] {
            assert_eq!(source.cast::<IUnknown>()?.as_raw(), unknown.as_raw());
            assert_eq!(source.cast::<IClassic>()?.as_raw(), unknown.as_raw());
            assert_eq!(
                source.cast::<IInspectable>()?.as_raw(),
                inspectable.as_raw()
            );
            let closable: ICustomClosable2 = source.cast()?;
            closable.Close()?;
            assert_eq!(inspectable.GetTrustLevel()?, 0);
        }
    }
    Ok(())
}

#[implement(IClassic)]
struct Classic;

impl IClassic_Impl for Classic_Impl {}

struct ClassicDecl;

implement_decl! {
    impl ClassicDecl as ClassicDecl_Impl: [IClassic]
}

impl IClassic_Impl for ClassicDecl_Impl {}

#[implement]
struct Empty;

#[test]
fn classic_identity_is_not_inspectable() -> Result<()> {
    for unknown in [IUnknown::from(Classic), IUnknown::from(ClassicDecl)] {
        let error = unknown.cast::<IInspectable>().unwrap_err();
        assert_eq!(error.code(), windows::Win32::E_NOINTERFACE);
        assert_eq!(unknown.cast::<IClassic>()?.as_raw(), unknown.as_raw());
    }

    let unknown = IUnknown::from(Empty);
    let inspectable: IInspectable = unknown.cast()?;
    assert_eq!(inspectable.as_raw(), unknown.as_raw());
    Ok(())
}

#[implement(ICustomClosable2)]
struct Custom;

impl ICustomClosable_Impl for Custom_Impl {}
impl ICustomClosable2_Impl for Custom_Impl {}
impl IClosable_Impl for Custom_Impl {
    fn Close(&self) -> Result<()> {
        Ok(())
    }
}

#[test]
fn custom_inspectable_conversion() -> Result<()> {
    let object = ComObject::new(Custom);
    let inspectable: IInspectable = object.to_interface();
    assert_eq!(
        inspectable.as_raw(),
        object.to_interface::<IUnknown>().as_raw()
    );
    inspectable.cast::<ICustomClosable2>()?.Close()?;

    let inspectable: IInspectable = Custom.into();
    inspectable.cast::<ICustomClosable2>()?.Close()?;
    Ok(())
}
