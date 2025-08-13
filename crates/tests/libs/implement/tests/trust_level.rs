use windows::core::*;
use windows::Foundation::*;

#[implement(IStringable)]
struct BaseTrust;

impl IStringable_Impl for BaseTrust_Impl {
    fn ToString(&self) -> Result<HSTRING> {
        Ok("BaseTrust".into())
    }
}

#[implement(IClosable, TrustLevel = Partial, IStringable)]
struct PartialTrust;

impl IStringable_Impl for PartialTrust_Impl {
    fn ToString(&self) -> Result<HSTRING> {
        Ok("PartialTrust".into())
    }
}

impl IClosable_Impl for PartialTrust_Impl {
    fn Close(&self) -> Result<()> {
        Ok(())
    }
}

#[implement(IStringable, TrustLevel = Full)]
struct FullTrust;

impl IStringable_Impl for FullTrust_Impl {
    fn ToString(&self) -> Result<HSTRING> {
        Ok("FullTrust".into())
    }
}

#[test]
fn test() -> Result<()> {
    let base: IStringable = BaseTrust.into();
    assert_eq!(base.ToString()?, "BaseTrust");
    assert_eq!(base.cast::<IInspectable>()?.GetTrustLevel()?, 0);

    let partial: IStringable = PartialTrust.into();
    assert_eq!(partial.ToString()?, "PartialTrust");
    assert_eq!(partial.cast::<IInspectable>()?.GetTrustLevel()?, 1);

    let full: IStringable = FullTrust.into();
    assert_eq!(full.ToString()?, "FullTrust");
    assert_eq!(full.cast::<IInspectable>()?.GetTrustLevel()?, 2);

    Ok(())
}
