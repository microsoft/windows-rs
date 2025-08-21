use windows::{core::*, Foundation::*};

#[implement(IStringable, IClosable)]
struct Test;

impl IStringable_Impl for Test_Impl {
    fn ToString(&self) -> Result<HSTRING> {
        todo!()
    }
}

impl IClosable_Impl for Test_Impl {
    fn Close(&self) -> Result<()> {
        todo!()
    }
}

// This tests that the interface_hierarchy macro correctly implement From<T> and From<&T> for interfaces.
#[test]
fn identity_from() -> Result<()> {
    {
        let inspectable: IInspectable = Test.into();
        assert_eq!(
            inspectable.GetRuntimeClassName()?,
            "Windows.Foundation.IStringable"
        );

        let unknown = Into::<&IUnknown>::into(&inspectable);
        assert_eq!(
            unknown.cast::<IInspectable>()?.GetRuntimeClassName()?,
            "Windows.Foundation.IStringable"
        );

        let unknown = Into::<IUnknown>::into(inspectable);
        assert_eq!(
            unknown.cast::<IInspectable>()?.GetRuntimeClassName()?,
            "Windows.Foundation.IStringable"
        );
    }
    {
        let stringable: IStringable = Test.into();

        let inspectable = Into::<&IInspectable>::into(&stringable);
        assert_eq!(
            inspectable.GetRuntimeClassName()?,
            "Windows.Foundation.IStringable"
        );

        let inspectable = Into::<IInspectable>::into(stringable.clone());
        assert_eq!(
            inspectable.GetRuntimeClassName()?,
            "Windows.Foundation.IStringable"
        );

        let unknown = Into::<&IUnknown>::into(&stringable);
        assert_eq!(
            unknown.cast::<IInspectable>()?.GetRuntimeClassName()?,
            "Windows.Foundation.IStringable"
        );

        let unknown = Into::<IUnknown>::into(stringable);
        assert_eq!(
            unknown.cast::<IInspectable>()?.GetRuntimeClassName()?,
            "Windows.Foundation.IStringable"
        );
    }
    {
        let closable: IClosable = Test.into();

        let inspectable = Into::<&IInspectable>::into(&closable);
        assert_eq!(
            inspectable.GetRuntimeClassName()?,
            "Windows.Foundation.IClosable"
        );

        let inspectable = Into::<IInspectable>::into(closable.clone());
        assert_eq!(
            inspectable.GetRuntimeClassName()?,
            "Windows.Foundation.IClosable"
        );

        let unknown = Into::<&IUnknown>::into(&closable);
        assert_eq!(
            unknown.cast::<IInspectable>()?.GetRuntimeClassName()?,
            "Windows.Foundation.IStringable"
        );

        let unknown = Into::<IUnknown>::into(closable);
        assert_eq!(
            unknown.cast::<IInspectable>()?.GetRuntimeClassName()?,
            "Windows.Foundation.IStringable"
        );
    }
    Ok(())
}
